use crate::utils::{get_last_modified_timestamp, parse_desktop_entry, DesktopEntry};
use crate::Apps;
use freedesktop_icons::lookup;
use log::{debug, error, info, warn};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::read_dir;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use tantivy::directory::MmapDirectory;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, Query, TermQuery};
use tantivy::schema::{Field, FieldType, IndexRecordOption, Value, STRING};
use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::{Schema, STORED, TEXT}, Document, Index,
    IndexReader,
    IndexWriter,
    TantivyDocument,
    Term,
};
use tokio::{sync::mpsc, task::JoinHandle, time};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

const DESKTOP_APPS_DIR: &str = "/usr/share/applications";

pub mod fields {
    pub const TYPE: &str = "type";
    pub const NAME: &str = "name";
    pub const EXEC: &str = "exec";
    pub const COMMENT: &str = "comment";
    pub const GENERIC_NAME: &str = "generic_name";
    pub const CATEGORIES: &str = "categories";
    pub const KEYWORDS: &str = "keywords";
    pub const ICON_NAME: &str = "icon_name";
    pub const APP_PATH: &str = "app_path";
    pub const LAST_MODIFIED: &str = "last_modified";
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileAction {
    Upsert,
    Remove,
}

pub enum IndexCmd {
    FsEvent(Event),
    Shutdown,
}

#[derive(Type, SerializeDict, DeserializeDict, Debug, Default, Clone)]
#[zvariant(signature = "dict")]
pub struct AppInfo {
    pub possible_app_id: String,
    pub type_: String,
    pub name: String,
    pub generic_name: String,
    pub keywords: Vec<String>,
    pub comment: String,
    pub icon_name: String,
    pub icon_path: Option<String>,
    pub categories: Vec<String>,
    pub exec: String,
    pub app_path: String,
    pub score: f32,
}
/// Public entry point for the app search service.

#[derive()]
pub struct AppSearchService {
    config: Apps,
    schema: Schema,
    index: Index,
    reader: IndexReader,
    writer: Option<IndexWriter>,
    index_worker_handle: Option<JoinHandle<()>>,
    watcher_handler: Option<JoinHandle<()>>,
    cmd_tx: Option<mpsc::Sender<IndexCmd>>,
}

impl AppSearchService {
    /// Create a new service instance.
    pub fn new(config: &Apps) -> anyhow::Result<Self> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?;
        let schema = create_schema();
        let index_path = home_dir.join(&config.index_dir);
        if !index_path.exists() {
            fs::create_dir_all(&index_path)?;
        }
        let mmap_dir = MmapDirectory::open(index_path)?;
        let index = Index::open_or_create(mmap_dir, schema.clone())?;
        let reader = index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let writer = index.writer(config.target_memory_usage_in_bytes)?;
        Ok(Self {
            config: config.clone(),
            schema,
            index,
            reader,
            writer: Some(writer),
            index_worker_handle: None,
            watcher_handler: None,
            cmd_tx: None,
        })
    }

    pub fn control_tx(&self) -> Option<mpsc::Sender<IndexCmd>> {
        self.cmd_tx.clone()
    }
    fn load_existing_desktop_entries(
        index_reader: &IndexReader,
        writer: &mut IndexWriter,
        schema: &Schema,
    ) {
        info!("Loading existing desktop entries");
        let existing_desktop_entries = read_dir(&DESKTOP_APPS_DIR).unwrap();
        for entry in existing_desktop_entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let last_modified = match get_last_modified_timestamp(&path) {
                Ok(c) => c,
                Err(e) => {
                    warn!(
                        "Failed to generate last_modified for {}: {}",
                        path.display(),
                        e
                    );
                    continue;
                }
            };

            let term = Term::from_field_text(
                schema.get_field(fields::APP_PATH).unwrap(),
                &path.to_string_lossy().to_string(),
            );
            // check if the entry is already in the index
            let existing_entry = extract_doc_given_app_path(index_reader, &term).unwrap();
            if let Some(existing_entry) = existing_entry {
                debug!("Entry already exists for path: {}", path.display());
                // verify last_modified
                let last_modified_field = schema.get_field(fields::LAST_MODIFIED).unwrap();
                let last_modified_indexed_value =
                    match existing_entry.get_first(last_modified_field) {
                        Some(v) => v,
                        None => {
                            warn!(
                                "last_modified field not found for entry: {}",
                                path.display()
                            );
                            continue;
                        }
                    };
                let last_modified_indexed_value_str = match last_modified_indexed_value.as_str() {
                    Some(s) => s,
                    None => {
                        warn!(
                            "the last_modified field value is not a string for entry: {}",
                            path.display()
                        );
                        continue;
                    }
                };
                if last_modified != last_modified_indexed_value_str {
                    // If last_modifieds don't match, then delete the entry
                    warn!("Last modified mismatch for entry: {}", path.display());
                    let term = Term::from_field_text(
                        schema.get_field(fields::APP_PATH).unwrap(),
                        &path.to_string_lossy().to_string(),
                    );
                    let doc =
                        extract_doc_given_app_path(&index_reader, &term).unwrap_or_else(|e| {
                            error!("Failed to extract doc: {}", e);
                            None
                        });
                    if let Some(_doc) = doc {
                        let _result = writer.delete_term(term);
                        info!("Removed indexed app entry: {}", path.display());
                    }
                } else {
                    debug!("Last modified match for entry: {}", path.display());
                    continue;
                }
            }
            let desktop_entry = match parse_desktop_entry(&path) {
                Some(d) => d,
                None => {
                    warn!("Failed to parse desktop entry: {}", path.display());
                    continue;
                }
            };
            debug!(
                "Found desktop entry: {:?} {:?}",
                desktop_entry.name, desktop_entry.comment
            );

            let doc = feed_doc(&schema, &desktop_entry, last_modified, &path);
            match writer.add_document(doc) {
                Ok(_) => (),
                Err(e) => error!("Failed to index app entry: {}", e),
            }
        }

        if let Err(e) = writer.commit() {
            error!("Failed to commit index: {:?}", e);
        } else {
            debug!("Committed indexed app data to disk.");
        }
        info!("Finished loading existing entries");
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Starting AppSearchService watcher...");
        // Load existing entries, this should be in a separate task
        let schema = self.schema.clone(); // make sure schema is Arc or Clone
        let index_reader = self.index.reader()?; // Make sure this is thread safe
        Self::load_existing_desktop_entries(
            &index_reader,
            self.writer.as_mut().expect("index writer missing"),
            &schema,
        );
        let watch_path: PathBuf = DESKTOP_APPS_DIR.into();
        if !watch_path.exists() {
            error!("Watch path does not exist: {}", watch_path.display());
            return Ok(());
        }

        let (cmd_tx, mut cmd_rx) = mpsc::channel::<IndexCmd>(256);
        self.cmd_tx = Some(cmd_tx.clone());
        let tx_clone = cmd_tx.clone();

        // ========= Spawn async task that holds the watcher =========
        let watch_path_clone = watch_path.clone();
        debug!("Watching path for application: {:?}", watch_path_clone);
        self.watcher_handler = Some(tokio::spawn(async move {
            let mut watcher = RecommendedWatcher::new(
                move |res| {
                    if let Ok(event) = res {
                        let _ = tx_clone.blocking_send(IndexCmd::FsEvent(event));
                    } else if let Err(err) = res {
                        error!("Watch error: {:?}", err);
                    }
                },
                notify::Config::default(),
            )
            .expect("Failed to create watcher");

            if let Err(e) = watcher.watch(&watch_path_clone, RecursiveMode::NonRecursive) {
                error!("Failed to start watcher: {}", e);
            } else {
                info!("Watching path: {:?}", watch_path_clone);
            }

            // Keep the watcher alive indefinitely
            futures::future::pending::<()>().await;
        }));

        // ========= Event Debouncing & Indexing =========
        let writer = self.writer.take().expect("writer missing");
        let schema = self.schema.clone();
        self.index_worker_handle = Some(tokio::spawn(async move {
            let mut writer = writer;
            let mut pending = Vec::new();
            let debounce = Duration::from_millis(400);
            let mut next_flush: Option<time::Instant> = None;
            loop {
                tokio::select! {
                    maybe_cmd = cmd_rx.recv() => {
                        match maybe_cmd {
                            Some(IndexCmd::FsEvent(event)) => {
                                pending.push(event);
                                if next_flush.is_none() {
                                    next_flush = Some(time::Instant::now() + debounce);
                                }
                            }
                            Some(IndexCmd::Shutdown) => {
                            // Final flush before exiting
                            if !pending.is_empty() {
                                Self::process_batch(&mut pending, &mut writer, &schema);
                            }
                            info!("Indexer received shutdown. Exiting cleanly.");
                            break; // Exit the task
                            }
                            None => {
                                // Sender dropped unexpectedly; try to flush and exit.
                                if !pending.is_empty() {
                                    Self::process_batch(&mut pending, &mut writer, &schema);
                                }
                                warn!("Indexer channel closed. Exiting.");
                                break;
                            }
                        }
                    }
                    _ = async {
                        if let Some(deadline) = next_flush { time::sleep_until(deadline).await }
                    }, if next_flush.is_some() => {
                        if !pending.is_empty() { Self::process_batch(&mut pending, &mut writer, &schema); }
                        next_flush = None;
                    }
                }
            }
        }));
        Ok(())
    }

    fn process_batch(pending: &mut Vec<Event>, writer: &mut IndexWriter, schema: &Schema) {
        let mut actions: HashMap<PathBuf, FileAction> = HashMap::new();

        for event in pending.drain(..) {
            for path in event.paths {
                // iterate all
                let is_desktop = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|e| e.eq_ignore_ascii_case("desktop"))
                    .unwrap_or(false);
                if !is_desktop {
                    continue;
                }

                let next = if event.kind.is_remove() {
                    FileAction::Remove
                } else if event.kind.is_create() || event.kind.is_modify() {
                    FileAction::Upsert
                } else {
                    continue;
                };

                actions
                    .entry(path)
                    .and_modify(|cur| {
                        if next == FileAction::Remove {
                            *cur = FileAction::Remove;
                        }
                    })
                    .or_insert(next);
            }
        }

        for (path, kind) in actions {
            if kind == FileAction::Upsert {
                if let Some(desktop_entry) = parse_desktop_entry(&path) {
                    info!("Indexing changed desktop entry: {}", desktop_entry.name);
                    let last_modified = match get_last_modified_timestamp(&path) {
                        Ok(c) => c,
                        Err(e) => {
                            warn!(
                                "Failed to generate last_modified for {}: {}",
                                path.display(),
                                e
                            );
                            String::new()
                        }
                    };
                    debug!("Last modified while storing: {}", last_modified);
                    let doc = feed_doc(&schema, &desktop_entry, last_modified, &path);
                    // if let Ok(writer) = writer.lock() {
                    match writer.add_document(doc) {
                        Ok(_) => info!("Indexed desktop entry: {}", desktop_entry.name),
                        Err(e) => error!("Failed to index app entry: {}", e),
                    }
                    // }
                }
            } else if kind == FileAction::Remove {
                info!("Removing indexed app entry: {:?}", path.file_name());
                let term = Term::from_field_text(
                    schema.get_field(fields::APP_PATH).unwrap(),
                    &path.to_string_lossy().to_string(),
                );
                let _result = writer.delete_term(term);
                info!("Removed indexed app entry: {:?}", path.file_name());
            }
        }
        if let Err(e) = writer.commit() {
            error!("Failed to commit index: {:?}", e);
        } else {
            info!("Committed indexed app data to disk.");
        }
    }
    /// Search indexed applications using a free-form query.
    pub fn search(&self, query_str: &str, limit: usize) -> tantivy::Result<Vec<AppInfo>> {
        info!("Search Apps: {}", query_str);
        let fields: Vec<Field> = self
            .config
            .searchable_fields
            .iter()
            .filter_map(|field_name| {
                debug!("Get field: {}", field_name);
                match self.schema.get_field(field_name) {
                    Ok(field) => Some(field),
                    Err(err) => {
                        warn!("Failed to get field {}: {}", field_name, err);
                        None
                    }
                }
            })
            .collect();

        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, fields.clone());
        let parsed = query_parser.parse_query(query_str)?;

        // start with parsed query
        let mut subqueries: Vec<(Occur, Box<dyn Query>)> = vec![(Occur::Should, parsed)];
        // add fuzzy queries for every searchable field
        for field in &fields {
            //NOTE: Must check the field type: FuzzyTermQuery only works on STRING fields
            let field_entry = self.schema.get_field_entry(*field);
            if let FieldType::Str(_) = field_entry.field_type() {
                let term = Term::from_field_text(*field, &query_str);
                subqueries.push((
                    Occur::Should,
                    Box::new(FuzzyTermQuery::new_prefix(term, 2, true)),
                ));
            }
        }
        
        let query = BooleanQuery::new(subqueries);

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_addr) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_addr)?;

            let mut app = AppInfo::default();
            for (field, value) in doc.get_sorted_field_values() {
                let field_name = self.schema.get_field_name(field).to_string();
                // Join all values into a single string (semicolon-separated)
                let joined_values = value
                    .iter()
                    .filter_map(|val| val.as_str())
                    .collect::<Vec<_>>()
                    .join(";");

                set_app_field(&mut app, &field_name, joined_values);
                app.score = score;
                let (icon_path, possible_app_id) =
                    resolve_icon_and_app_id(&app.icon_name, &app.app_path);
                app.icon_path = icon_path;
                app.possible_app_id = possible_app_id;
            }

            results.push(app);
        }

        Ok(results)
    }
    pub fn list_applications(&self, limit: usize) -> tantivy::Result<Vec<AppInfo>> {
        info!("List applications: limit {}", limit);
        let search_term = "Application";
        let field_to_lookup = match self.schema.get_field(fields::TYPE) {
            Ok(field) => field,
            Err(err) => {
                error!("Failed to get field {}: {}", fields::TYPE, err);
                return Err(err);
            }
        };

        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, vec![field_to_lookup]);
        let query = query_parser.parse_query(search_term)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (_score, doc_addr) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_addr)?;

            let mut app = AppInfo::default();
            for (field, value) in doc.get_sorted_field_values() {
                let field_name = self.schema.get_field_name(field).to_string();
                // Join all values into a single string (semicolon-separated)
                let joined_values = value
                    .iter()
                    .filter_map(|val| val.as_str())
                    .collect::<Vec<_>>()
                    .join(";");

                set_app_field(&mut app, &field_name, joined_values);
                let (icon_path, possible_app_id) =
                    resolve_icon_and_app_id(&app.icon_name, &app.app_path);
                app.icon_path = icon_path;
                app.possible_app_id = possible_app_id;
            }

            results.push(app);
        }

        Ok(results)
    }

    /// Graceful shutdown (optional: cancels task)
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        // Signal the indexer to flush pending work and exit
        if let Some(tx) = self.cmd_tx.take() {
            let _ = tx.send(IndexCmd::Shutdown).await; // ignore error if task already exited
        }

        // Await the indexer task to allow a clean final commit
        if let Some(handle) = self.index_worker_handle.take() {
            match handle.await {
                Ok(()) => debug!("Indexer task exited cleanly"),
                Err(e) => warn!("Indexer task join error: {:?}", e),
            }
        }

        // Stop the watcher task (it is a source-only task)
        if let Some(handle) = self.watcher_handler.take() {
            debug!("Aborting watcher task");
            handle.abort();
        }
        Ok(())
    }
}
fn set_app_field(app: &mut AppInfo, field_name: &str, joined_values: String) {
    match field_name {
        fields::TYPE => app.type_ = joined_values,
        fields::NAME => app.name = joined_values,
        fields::EXEC => app.exec = joined_values,
        fields::COMMENT => app.comment = joined_values,
        fields::GENERIC_NAME => app.generic_name = joined_values,
        fields::CATEGORIES => {
            app.categories = joined_values.split(';').map(|s| s.to_string()).collect();
        }
        fields::KEYWORDS => {
            app.keywords = joined_values.split(';').map(|s| s.to_string()).collect();
        }
        fields::ICON_NAME => app.icon_name = joined_values,
        fields::APP_PATH => app.app_path = joined_values,
        _ => {}
    }
}

/// Creates a `TantivyDocument` from a `DesktopEntry`, with the given last_modified and path.
///
/// This function takes a `DesktopEntry` and creates a new `TantivyDocument` with the fields:
///
/// - `name`: the application name
/// - `exec`: the application executable
/// - `comment`: the application description
/// - `generic_name`: the application generic name
/// - `categories`: the application categories, joined with `;`
/// - `keywords`: the application keywords, joined with `;`
/// - `icon`: the application icon
/// - `path`: the path to the `.desktop` file
/// - `last_modified`: the last_modified of the `.desktop` file
///
/// If any of the fields are missing in the `DesktopEntry`, they will be filled with default values.
///
/// # Arguments
///
/// * `schema`: the `Schema` to use for creating the `TantivyDocument`
/// * `desktop_entry`: the `DesktopEntry` to create the `TantivyDocument` from
/// * `last_modified`: the last_modified of the `.desktop` file
/// * `path`: the path to the `.desktop` file
///
/// # Returns
///
/// A `TantivyDocument` with the fields filled in from the `DesktopEntry`, last_modified and path.
fn feed_doc(
    schema: &Schema,
    desktop_entry: &DesktopEntry,
    last_modified: String,
    path: &Path,
) -> TantivyDocument {
    doc!(
        schema.get_field(fields::TYPE).unwrap() => desktop_entry.type_,
        schema.get_field(fields::NAME).unwrap() => desktop_entry.name,
        schema.get_field(fields::EXEC).unwrap() => desktop_entry.exec.clone().unwrap_or_default(),
        schema.get_field(fields::COMMENT).unwrap() => desktop_entry.comment.clone().unwrap_or_default(),
        schema.get_field(fields::GENERIC_NAME).unwrap() => desktop_entry.generic_name.clone().unwrap_or_default(),
        schema.get_field(fields::CATEGORIES).unwrap() => desktop_entry.categories.join(";"),
        schema.get_field(fields::KEYWORDS).unwrap() => desktop_entry.keywords.join(";"),
        schema.get_field(fields::ICON_NAME).unwrap() => desktop_entry.icon.clone().unwrap_or_default(),
        schema.get_field(fields::APP_PATH).unwrap() => path.to_string_lossy().to_string(),
        schema.get_field(fields::LAST_MODIFIED).unwrap() => last_modified
    )
}

/// Create the Tantivy schema for `.desktop` fields
fn create_schema() -> Schema {
    let mut schema_builder = tantivy::schema::Schema::builder();
    schema_builder.add_text_field(fields::TYPE, STRING | STORED);
    schema_builder.add_text_field(fields::NAME, STRING | STORED);
    schema_builder.add_text_field(fields::EXEC, STORED);
    schema_builder.add_text_field(fields::COMMENT, TEXT);
    schema_builder.add_text_field(fields::GENERIC_NAME, STRING | STORED);
    schema_builder.add_text_field(fields::CATEGORIES, TEXT | STORED);
    schema_builder.add_text_field(fields::APP_PATH, STRING | STORED);
    schema_builder.add_text_field(fields::KEYWORDS, TEXT);
    schema_builder.add_text_field(fields::ICON_NAME, STORED);
    schema_builder.add_text_field(fields::LAST_MODIFIED, STORED);

    schema_builder.build()
}

// A simple helper function to fetch a single document
// given its id from our index.
// It will be helpful to check our work.
fn extract_doc_given_app_path(
    reader: &IndexReader,
    app_path: &Term,
) -> tantivy::Result<Option<TantivyDocument>> {
    let searcher = reader.searcher();

    // This is the simplest query you can think of.
    // It matches all of the documents containing a specific term.
    //
    // The second argument is here to tell we don't care about decoding positions,
    // or term frequencies.
    let term_query = TermQuery::new(app_path.clone(), IndexRecordOption::Basic);
    let top_docs = searcher.search(&term_query, &TopDocs::with_limit(1))?;

    if let Some((_score, doc_address)) = top_docs.first() {
        let doc = searcher.doc(*doc_address)?;
        Ok(Some(doc))
    } else {
        // no doc matching this ID.
        Ok(None)
    }
}

fn resolve_icon_and_app_id(icon_name: &str, app_path: &str) -> (Option<String>, String) {
    let icon_path = (!icon_name.is_empty()).then(|| {
        lookup(icon_name)
            .find()
            .and_then(|p| p.to_str().map(|p| p.to_owned()))
            .unwrap_or_default()
    });

    let possible_app_id = Path::new(app_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_owned)
        .unwrap_or_default();

    (icon_path, possible_app_id)
}
