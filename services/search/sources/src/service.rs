use crate::indexer::Indexer;
use crate::utils::MetadataFields;
use crate::SourceSearchServiceConfig;
use log::{debug, error, info, warn};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema};
use tantivy::{doc, Document, Index, IndexReader, IndexWriter, TantivyDocument, Term};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

pub(crate) enum IndexCmd {
    Upsert(Vec<UpsertMetadata>),
    RemoveByUniqueIds(Vec<String>, bool),
    RemoveByPath(String),
    Flush,
    Shutdown,
}

#[derive(Type, SerializeDict, DeserializeDict, Debug, Clone)]
#[zvariant(signature = "dict")]
pub struct SourceSearchResult {
    pub source: String,
    pub uri: String,
    pub title: String,
    pub icon: String,
    pub thumbnail: String,
    pub last_modified: u64,
    pub content: String,
    pub unique_id: String,
    pub keywords: Vec<String>,
    pub score: f32,
}

//({'uri': <'file:///usr/bin/gedit'>})
#[derive(Type, SerializeDict, DeserializeDict, Debug, Clone)]
#[zvariant(signature = "a{sv}")]
pub struct UpsertMetadata {
    pub source: String,
    pub unique_id: String,
    pub uri: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub icon: String,
    pub thumbnail: String,
    pub last_modified: u64,
    pub content: Option<String>,
    pub source_entry_path: String,
}

impl Default for SourceSearchResult {
    fn default() -> Self {
        Self {
            source: "".to_string(),
            uri: "".to_string(),
            title: "".to_string(),
            icon: "".to_string(),
            thumbnail: "".to_string(),
            last_modified: 0,
            content: "".to_string(),
            unique_id: "".to_string(),
            keywords: Vec::new(),
            score: 0.0,
        }
    }
}

/// Simple debouncer for collecting events and flushing them after a delay.
struct Debouncer {
    pending: Vec<notify::Event>,
    next_flush: Option<tokio::time::Instant>,
    debounce: std::time::Duration,
}

impl Debouncer {
    fn new(debounce_ms: u64) -> Self {
        Self {
            pending: Vec::new(),
            next_flush: None,
            debounce: std::time::Duration::from_millis(debounce_ms),
        }
    }

    /// Add an event and schedule a flush if needed.
    fn push(&mut self, event: notify::Event) {
        self.pending.push(event);

        if self.next_flush.is_none() {
            self.next_flush = Some(tokio::time::Instant::now() + self.debounce);
        }
    }

    /// Returns the flush deadline, if any.
    fn deadline(&self) -> Option<tokio::time::Instant> {
        self.next_flush
    }

    /// Perform a flush and return the drained events.
    fn flush(&mut self) -> Vec<notify::Event> {
        let drained = std::mem::take(&mut self.pending);
        self.next_flush = None;
        drained
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

pub struct SourceSearchService {
    config: SourceSearchServiceConfig,
    schema: Schema,
    pub fields: MetadataFields,
    index: Index,
    index_reader: IndexReader,
    index_writer: Option<IndexWriter>,
    indexer_handle: Option<JoinHandle<()>>,
    watcher_handler: Option<JoinHandle<()>>,
    cmd_tx: Option<mpsc::Sender<IndexCmd>>,
}

impl SourceSearchService {
    pub fn new(config: &SourceSearchServiceConfig) -> anyhow::Result<Self> {
        info!("Creating an sources search service...");
        let bundle = Self::build_schema()?;
        let schema = bundle.schema.clone();
        let fields = bundle.fields;
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?;
        let index_path = home_dir.join(&config.index_dir);
        if !index_path.exists() {
            fs::create_dir_all(&index_path)?;
        }
        let mmap_dir = MmapDirectory::open(index_path)?;
        let index = match Index::open_or_create(mmap_dir, schema.clone()) {
            Ok(index) => index,
            Err(e) => {
                error!("Failed to open index: {}", e);
                return Err(e.into());
            }
        };
        let index_reader = index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let index_writer = index.writer(config.target_memory_usage_in_bytes)?;
        Ok(SourceSearchService {
            config: config.clone(),
            schema,
            index,
            index_reader,
            fields,
            index_writer: Some(index_writer),
            watcher_handler: None,
            cmd_tx: None,
            indexer_handle: None,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        info!("init sources search service runner!");
        let (cmd_tx, cmd_rx) = mpsc::channel::<IndexCmd>(256);
        self.cmd_tx = Some(cmd_tx.clone());
        let index_writer = self.index_writer.take().unwrap();
        let fields = self.fields.clone(); // if you implement Clone; else clone handles individually
        let indexer = Indexer::new(index_writer, fields);
        let indexer_handle = tokio::spawn(indexer.run(cmd_rx));
        self.indexer_handle = Some(indexer_handle); // add field to SourceService

        self.watcher_handler = Some(tokio::spawn({
            let cmd_tx = cmd_tx.clone();
            let watch_path = PathBuf::from(&self.config.app_dir);
            async move {
                // Create the file watcher with a synchronous callback
                let mut watcher = match RecommendedWatcher::new(
                    move |res: Result<Event, notify::Error>| match res {
                        Ok(event) => {
                            // Only handle remove events
                            if let EventKind::Remove(_) = event.kind {
                                for p in event.paths {
                                    if let Some(s) = p.to_str() {
                                        debug!("Desktop entry removed: {}", s);
                                        if let Err(e) = cmd_tx
                                            .blocking_send(IndexCmd::RemoveByPath(s.to_string()))
                                        {
                                            error!("Failed to send RemoveByPath command: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => error!("Watch error: {:?}", err),
                    },
                    notify::Config::default(),
                ) {
                    Ok(w) => w,
                    Err(e) => {
                        error!("Failed to create watcher: {}", e);
                        return;
                    }
                };

                if let Err(e) = watcher.watch(&watch_path, RecursiveMode::Recursive) {
                    error!("Failed to start watcher: {}", e);
                    return;
                }

                info!("Watching path: {:?}", watch_path);

                // Keep the task alive; notify watcher runs on its own thread.
                // Sleep forever until task is cancelled.
                futures::future::pending::<()>().await;
            }
        }));

        Ok(())
    }

    pub async fn upsert_metadata(&mut self, payload: Vec<UpsertMetadata>) -> anyhow::Result<bool> {
        debug!("Upserting metadata: {:?}", payload);
        let ids: Vec<String> = payload.iter().map(|m| m.unique_id.clone()).collect();
        if let Some(tx) = &self.cmd_tx {
            tx.send(IndexCmd::RemoveByUniqueIds(ids, false)).await.ok();
        }
        if let Some(tx) = &self.cmd_tx {
            tx.send(IndexCmd::Upsert(payload)).await.ok();
            Ok(true)
        } else {
            anyhow::bail!("Indexer not running")
        }
    }

    pub async fn delete_by_ids(&mut self, ids: Vec<String>) -> anyhow::Result<bool> {
        debug!("delete by ids, length: {:?}", ids.len());
        if let Some(tx) = &self.cmd_tx {
            tx.send(IndexCmd::RemoveByUniqueIds(ids, true)).await.ok();
            Ok(true)
        } else {
            anyhow::bail!("Indexer not running")
        }
    }
    pub fn search(
        &self,
        search_term: &str,
        limit: usize,
    ) -> tantivy::Result<Vec<SourceSearchResult>> {
        info!(target: "search", "Listing sources search results...");
        let searcher = self.index_reader.searcher();

        // Look up the field to search in.
        let fields_to_lookup: Vec<Field> = self
            .config
            .searchable_fields
            .iter()
            .filter_map(|field_name| match self.schema.get_field(field_name) {
                Ok(field) => Some(field),
                Err(err) => {
                    warn!("Failed to get field {}: {}", field_name, err);
                    None
                }
            })
            .collect();

        let query_parser = QueryParser::for_index(&self.index, fields_to_lookup);
        let query = query_parser.parse_query(search_term)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_addr) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_addr)?;
            let mut app = SourceSearchResult::default();
            app = self.map_doc(&doc);
            app.score = score;
            results.push(app);
        }

        Ok(results)
    }

    pub async fn shutdown(&mut self) {
        if let Some(cmd_tx) = self.cmd_tx.take() {
            let _ = cmd_tx.send(IndexCmd::Shutdown).await;
        }

        if let Some(h) = self.watcher_handler.take() {
            let _ = h.abort();
        }

        if let Some(handle) = self.indexer_handle.take() {
            match handle.await {
                Ok(_) => info!("Indexer stopped cleanly"),
                Err(e) => error!("Indexer task error: {:?}", e),
            }
        }
    }
}
