use crate::utils::FileMetadata;
use crate::{utils, FilesConfig};
use log::{debug, error, info, warn};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tantivy::query::TermQuery;
use tantivy::schema::{Field, IndexRecordOption, Value, STRING};
use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::{Schema, STORED, TEXT}, Document, Index, IndexReader,
    IndexWriter,
    TantivyDocument,
    TantivyError,
    Term,
};
use tokio::{sync::mpsc, task::JoinHandle, time};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Type, SerializeDict, DeserializeDict, Debug, Default, Clone)]
#[zvariant(signature = "dict")]
pub struct FileInfo {
    pub file_type: String,
    pub name: String,
    pub path: String,
    pub score: f32,
}
/// Public entry point for the app search service.

#[derive()]
pub struct FileSearchService {
    config: FilesConfig,
    schema: Schema,
    index: Index,
    writer: Arc<Mutex<IndexWriter>>,
    index_worker_handle: Option<JoinHandle<()>>,
    watcher_handler: Option<JoinHandle<()>>,
}

pub enum FileIndexState {
    IndexedAndUpToDate, // indexed & modified_timestamp matches
    IndexedAndStale,    // indexed & modified_timestamp mismatch
    NotIndexed,         // not in the index at all
}

impl FileSearchService {
    /// Create the Tantivy schema for `.desktop` fields
    fn create_schema() -> Schema {
        let mut schema_builder = tantivy::schema::Schema::builder();
        schema_builder.add_text_field("file_type", STRING | STORED);
        schema_builder.add_text_field("name", STRING | STORED);
        schema_builder.add_text_field("content", TEXT);
        schema_builder.add_text_field("path", STRING | STORED);
        schema_builder.add_text_field("last_modified", STRING | STORED);
        schema_builder.build()
    }

    /// Returns the state of the given file in the index.
    ///
    /// Returns `IndexedAndUpToDate` if the file is already indexed and the last modified timestamp
    /// matches, `IndexedAndStale` if the file is indexed but the last modified timestamp does not
    /// match, and `NotIndexed` if the file is not indexed at all.
    ///
    fn get_index_state(
        schema: &Schema,
        file_metadata: &FileMetadata,
        file_path: &PathBuf,
        index_reader: &IndexReader,
    ) -> Result<FileIndexState, TantivyError> {
        // Check if last_modified is same then do not index
        let field = match schema.get_field("path") {
            Ok(field) => field,
            Err(err) => {
                error!("Failed to get field: {}", err);
                return Err(err);
            }
        };
        let term = Term::from_field_text(field, &file_path.to_string_lossy().to_string());
        // check if the entry is already in the index
        let existing_entry = extract_doc_given_file_path(index_reader, &term)?;
        if let Some(existing_entry) = existing_entry {
            // verify last modified
            let last_modified_field = schema.get_field("last_modified")?;
            let last_modified_field_value = match existing_entry.get_first(last_modified_field) {
                Some(v) => v,
                None => {
                    warn!(
                        "last_modified field not found for entry: {}",
                        file_path.display()
                    );
                    return Ok(FileIndexState::NotIndexed);
                }
            };
            debug!(
                "last_modified field value: {:?}",
                last_modified_field_value.as_str()
            );
            debug!(
                "file metadata last_modified: {}",
                file_metadata.last_modified
            );
            if let Some(last_modified_field_value) = last_modified_field_value.as_str() {
                return if last_modified_field_value == file_metadata.last_modified {
                    debug!(
                        "Entry already exists and last_modified is same for path: {}",
                        file_path.display()
                    );
                    Ok(FileIndexState::IndexedAndUpToDate)
                } else {
                    debug!(
                        "Entry already exists but last_modified is different for path: {}",
                        file_path.display()
                    );
                    Ok(FileIndexState::IndexedAndStale)
                };
            }
        }
        Ok(FileIndexState::NotIndexed)
    }
    /// Create a new service instance.
    pub fn new(config: &FilesConfig) -> anyhow::Result<Self> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get home directory"))?;
        let schema = Self::create_schema();
        let index_path = home_dir.join(&config.index_dir);

        // Create the index if it doesn't exist
        let index = if index_path.join("meta.json").exists() {
            Index::open_in_dir(&index_path)?
        } else {
            Index::create_in_dir(&index_path, schema.clone())?
        };

        // TODO: We can configure this to be more fine-grained
        let writer = Arc::new(Mutex::new(index.writer(50_000_000)?));
        Ok(Self {
            config: config.clone(),
            schema,
            index,
            writer,
            index_worker_handle: None,
            watcher_handler: None,
        })
    }

    /// Recursively traverse the directory at `path` up to `depth_level` and
    /// index all files with extensions in `allowed_extension`. If the file is
    /// already indexed, check if the last modified timestamp has changed. If
    /// it has, update the index. If not, do not update the index.
    ///
    /// The `index_reader` is used to check if the file is already indexed, and
    /// the `index_writer` is used to write new documents to the index.
    ///
    /// The `schema` is used to create the `Document` instances for the indexed
    /// files.
    ///
    /// The `buffer_size_kb` is the size of the buffer to use when reading the
    /// file contents.
    fn index_existing_files(
        path: &str,
        depth_level: i8,
        allowed_extension: &HashSet<String>,
        index_reader: &IndexReader,
        index_writer: &Arc<Mutex<IndexWriter>>,
        schema: &Schema,
        buffer_size_kb: usize,
    ) {
        // Vector to store collected file paths
        let mut files_to_index: Vec<PathBuf> = Vec::new();
        fn visit_dir(
            dir: &str,
            current_depth: i8,
            max_depth: i8,
            allowed_ext: &HashSet<String>,
            collected_files: &mut Vec<PathBuf>,
        ) {
            if current_depth > max_depth {
                return;
            }

            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    // Skip hidden files/folders (names starting with '.')
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with('.') {
                            continue;
                        }
                    }
                    if path.is_dir() {
                        visit_dir(
                            &path.display().to_string(),
                            current_depth + 1,
                            max_depth,
                            allowed_ext,
                            collected_files,
                        );
                    } else if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if allowed_ext.contains(&ext.to_string()) {
                                collected_files.push(path);
                            }
                        }
                    }
                }
            }
        }

        visit_dir(
            &path,
            0,
            depth_level,
            allowed_extension,
            &mut files_to_index,
        );

        // Now process the collected files
        for file_path in files_to_index {
            let file_metadata: FileMetadata =
                match utils::get_file_metadata(&file_path, buffer_size_kb) {
                    Ok(info) => info,
                    Err(e) => {
                        error!("Failed to get file info for {}: {}", file_path.display(), e);
                        continue;
                    }
                };
            // Check if last_modified is same then do not index
            let state =
                match Self::get_index_state(&schema, &file_metadata, &file_path, &index_reader) {
                    Ok(state) => state,
                    Err(err) => {
                        error!("Failed to get index state: {}", err);
                        continue;
                    }
                };
            match state {
                FileIndexState::IndexedAndUpToDate => {
                    debug!(
                        "Entry already exists and last_modified is same for path: {}",
                        file_path.display()
                    );
                    continue;
                }
                FileIndexState::IndexedAndStale => {
                    if let Ok(writer) = index_writer.lock() {
                        let field = match schema.get_field("path") {
                            Ok(field) => field,
                            Err(err) => {
                                error!("Failed to get field - path: {}", err);
                                continue;
                            }
                        };
                        let term = Term::from_field_text(
                            field,
                            &file_path.to_string_lossy().to_string(),
                        );
                        let doc =
                            extract_doc_given_file_path(&index_reader, &term).unwrap_or_else(|e| {
                                error!("Failed to extract doc: {}", e);
                                None
                            });
                        if let Some(_doc) = doc {
                            let _result = writer.delete_term(term);
                            info!("Removed indexed file entry: {:?}", file_path.file_name());
                        }
                        let doc = feed_doc(&schema, &file_metadata);
                        match writer.add_document(doc) {
                            Ok(_) => info!("Indexed new file: {}", file_metadata.name),
                            Err(e) => error!("Failed to index new file: {}", e),
                        }
                    }
                }
                FileIndexState::NotIndexed => {
                    let doc = feed_doc(&schema, &file_metadata);
                    if let Ok(writer) = index_writer.lock() {
                        match writer.add_document(doc) {
                            Ok(_) => info!("Indexed new file: {}", file_metadata.name),
                            Err(e) => error!("Failed to index new file: {}", e),
                        }
                    }
                }
            }
        }
        if let Ok(mut writer) = index_writer.lock() {
            if let Err(e) = writer.commit() {
                error!("Failed to commit index: {:?}", e);
            } else {
                debug!("Committed indexed files data to disk.");
            }
        };
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Starting FileSearchService watcher...");
        // Allowed file extensions
        let allowed_extensions: HashSet<String> = self.config.allowed_extensions.clone();
        let buffer_size_kb: usize = self.config.read_file_content_upto_in_kb;

        // Load existing entries, this should be in a separate task
        let schema = self.schema.clone(); // make sure schema is Arc or Clone
        let index_reader = self.index.reader()?; // Make sure this is thread safe
        Self::index_existing_files(
            &self.config.files_dir_to_watch,
            self.config.max_depth,
            &allowed_extensions,
            &index_reader,
            &self.writer.clone(),
            &schema,
            buffer_size_kb,
        );
        let watch_path: PathBuf = self.config.files_dir_to_watch.clone().into();
        debug!("Watching path: {}", watch_path.display());
        if !watch_path.exists() {
            anyhow::bail!("Watch path does not exist: {}", watch_path.display());
        }

        let (event_tx, mut event_rx) = mpsc::channel::<Event>(100);
        let tx_clone = event_tx.clone();

        // ========= Spawn async task that holds the watcher =========
        let watch_path_clone = watch_path.clone();
        self.watcher_handler = Some(tokio::spawn(async move {
            let mut watcher = RecommendedWatcher::new(
                move |res| {
                    if let Ok(event) = res {
                        let _ = tx_clone.blocking_send(event);
                    } else if let Err(err) = res {
                        error!("Watch error: {:?}", err);
                    }
                },
                notify::Config::default(),
            )
            .expect("Failed to create watcher");

            if let Err(e) = watcher.watch(&watch_path_clone, RecursiveMode::Recursive) {
                error!("Failed to start watcher: {}", e);
            } else {
                info!("Watching path: {:?}", watch_path_clone);
            }

            // Keep the watcher alive in background (task won't exit unless manually dropped)
            futures::future::pending::<()>().await;
        }));

        // ========= Event Debouncing & Indexing =========
        let writer = Arc::clone(&self.writer);
        let schema = self.schema.clone();
        let reader = match self.index.reader() {
            Ok(reader) => reader,
            Err(err) => {
                error!("Failed to get index reader: {}", err);
                return Ok(());
            }
        };
        self.index_worker_handle = Some(tokio::spawn(async move {
            let debounce_duration = Duration::from_secs(2);
            let mut pending = Vec::new();

            loop {
                tokio::select! {
                    Some(event) = event_rx.recv() => {
                        pending.push(event);
                    }
                    _ = time::sleep(debounce_duration), if !pending.is_empty() => {
                        let mut unique_paths = HashMap::new();
                        for event in pending.drain(..) {
                            // Ignore events that are not relevant, extension should be in allowed_extensions, and the file name should not start with "."
                            if event.kind.is_create() || event.kind.is_modify() || event.kind.is_remove() {
                                for path in &event.paths {
                                    if let (Some(file_name_os), Some(ext_os)) = (path.file_name(), path.extension()) {
                                        if let (Some(file_name), Some(file_extension)) = (file_name_os.to_str(), ext_os.to_str()) {
                                            // Skip hidden files and filter by allowed extensions
                                            if !file_name.starts_with('.') && allowed_extensions.contains(file_extension) {
                                                unique_paths.entry(path.clone()).or_insert(event.kind);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        for (path, kind) in unique_paths {
                            match kind {
                                EventKind::Any => {}
                                EventKind::Access(_) => {}
                                EventKind::Create(_) => {
                                let file_metadata: FileMetadata = match utils::get_file_metadata(&path, buffer_size_kb) {
                                    Ok(info) => info,
                                    Err(e) => {
                                        error!("Failed to get file info for {}: {}", path.display(), e);
                                        continue;
                                    }
                                };
                                // Check if last_modified is same then do not index
                                let doc = feed_doc(&schema, &file_metadata);
                                if let Ok(writer) = writer.lock() {
                                    match writer.add_document(doc) {
                                        Ok(_) => info!("Indexed new file: {}", file_metadata.name),
                                        Err(e) => error!("Failed to index new file: {}", e),
                                        }
                                    }
                                }
                                EventKind::Modify(_) => {
                                    debug!("===>>> File modified: {}", path.display());
                                    let file_metadata: FileMetadata = match utils::get_file_metadata(&path, buffer_size_kb) {
                                    Ok(info) => info,
                                    Err(e) => {
                                        error!("Failed to get file info for {}: {}", path.display(), e);
                                        continue;
                                    }
                                };
                                    debug!("file metadata: {:#?}", file_metadata);
                                    let state = match Self::get_index_state(&schema, &file_metadata, &path, &reader) {
                                        Ok(state) => state,
                                        Err(e) => {
                                            error!("Failed to get index state for {}: {}", path.display(), e);
                                            continue;
                                        }
                                    };
                                    match state {
                                        FileIndexState::IndexedAndUpToDate => {
                                            continue;
                                        }
                                        FileIndexState::IndexedAndStale => {
                                            if let Ok(writer) = writer.lock() {
                                                let field = match schema.get_field("path") {
                                                    Ok(field) => field,
                                                    Err(err) => {
                                                        error!("Failed to get field - path: {}", err);
                                                        continue;
                                                    }
                                                };
                                               let term = Term::from_field_text(field, &path.to_string_lossy().to_string());
                                               let doc= extract_doc_given_file_path(&reader, &term).unwrap_or_else(|e| {
                                                        error!("Failed to extract doc: {}", e);
                                                        None
                                                    });
                                                if let Some(_doc) = doc {
                                                    let _result =writer.delete_term(term);
                                                    info!("Removed indexed file entry: {:?}", path.file_name());
                                                }
                                                let doc = feed_doc(&schema, &file_metadata);
                                                match writer.add_document(doc) {
                                                    Ok(_) => info!("Indexed new file: {}", file_metadata.name),
                                                    Err(e) => error!("Failed to index new file: {}", e),
                                                }
                                            }
                                        }
                                        FileIndexState::NotIndexed => {
                                        // Check if last_modified is same then do not index
                                        let doc = feed_doc(&schema, &file_metadata);
                                        if let Ok(writer) = writer.lock() {
                                            match writer.add_document(doc) {
                                                Ok(_) => info!("Indexed new file: {}", file_metadata.name),
                                                Err(e) => error!("Failed to index new file: {}", e),
                                                }
                                            }
                                        }
                                    };
                                }
                                EventKind::Remove(_) => {
                                    debug!("Removing indexed file entry: {:?}", path.file_name());
                                    if let Ok(writer) = writer.lock() {
                                        let field = match schema.get_field("path") {
                                            Ok(field) => field,
                                            Err(err) => {
                                                error!("Failed to get field - path: {}", err);
                                                continue;
                                            }
                                        };
                                        let term = Term::from_field_text(field, &path.to_string_lossy().to_string());
                                       let doc= extract_doc_given_file_path(&reader, &term).unwrap_or_else(|e| {
                                                error!("Failed to extract doc: {}", e);
                                                None
                                            });
                                        if let Some(_doc) = doc {
                                            let _result =writer.delete_term(term);
                                            info!("Removed indexed file entry: {:?}", path.file_name());
                                        }
                                    }
                                }
                                EventKind::Other => {}
                            };
                        }
                        if let Ok(mut writer) = writer.lock() {
                            if let Err(e) = writer.commit() {
                                error!("Failed to commit index: {:?}", e);
                            } else {
                                info!("Committed indexed app data to disk.");
                            }
                        }
                    }
                }
            }
        }));
        Ok(())
    }

    /// Search indexed file using a free-form query.
    pub fn search(&self, query_str: &str, limit: usize) -> tantivy::Result<Vec<FileInfo>> {
        let fields: Vec<Field> = get_searchable_fields(&self.config, &self.schema);

        let reader = self
            .index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, fields);
        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();

        for (score, doc_addr) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_addr)?;

            let mut file_info = FileInfo::default();
            for (field, value) in doc.get_sorted_field_values() {
                let field_name = self.schema.get_field_name(field).to_string();
                // Join all values into a single string (semicolon-separated)
                let joined_values = value
                    .iter()
                    .filter_map(|val| val.as_str())
                    .collect::<Vec<_>>()
                    .join(";");

                set_file_field(&mut file_info, &field_name, joined_values);
                file_info.score = score;
            }

            results.push(file_info);
        }

        Ok(results)
    }

    /// Graceful shutdown (optional: cancels task)
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        if let Some(handle) = &self.index_worker_handle {
            debug!("Aborting indexer task");
            handle.abort(); // Stop a background debounce task
        }

        if let Some(handle) = &self.watcher_handler {
            debug!("Aborting watcher task");
            handle.abort(); // Stop background task
        }
        Ok(())
    }
}

// A simple helper function to fetch a single document
// given its id from our index.
// It will be helpful to check our work.
fn extract_doc_given_file_path(
    reader: &IndexReader,
    file_path: &Term,
) -> tantivy::Result<Option<TantivyDocument>> {
    let searcher = reader.searcher();

    // This is the simplest query you can think of.
    // It matches all of the documents containing a specific term.
    //
    // The second argument is here to tell we don't care about decoding positions,
    // or term frequencies.
    let term_query = TermQuery::new(file_path.clone(), IndexRecordOption::Basic);
    let top_docs = searcher.search(&term_query, &TopDocs::with_limit(200))?;

    if let Some((_score, doc_address)) = top_docs.first() {
        let doc = searcher.doc(*doc_address)?;
        Ok(Some(doc))
    } else {
        // no doc matching this ID.
        Ok(None)
    }
}

/// Creates a `TantivyDocument` from a `FileMetadata`, with the given fields.
///
/// The document will have the fields:
///
/// - `file_type`: the file type
/// - `name`: the file name
/// - `path`: the file path
/// - `content`: the file content
/// - `last_modified`: the last modified timestamp of the file
///
/// # Arguments
///
/// * `schema`: the `Schema` to use for creating the `TantivyDocument`
/// * `file_metadata`: the `FileMetadata` to create the `TantivyDocument` from
///
/// # Returns
///
/// A `TantivyDocument` with the fields filled in from the `FileMetadata`.
fn feed_doc(schema: &Schema, file_metadata: &FileMetadata) -> TantivyDocument {
    doc!(
        schema.get_field("file_type").unwrap() => file_metadata.file_type,
        schema.get_field("name").unwrap() => file_metadata.name,
        schema.get_field("path").unwrap() => file_metadata.path,
        schema.get_field("content").unwrap() => file_metadata.content,
        schema.get_field("last_modified").unwrap() => file_metadata.last_modified
    )
}
fn get_searchable_fields(config: &FilesConfig, schema: &Schema) -> Vec<Field> {
    config
        .searchable_fields
        .iter()
        .filter_map(|field_name| {
            debug!("Get field: {}", field_name);
            match schema.get_field(field_name) {
                Ok(field) => Some(field),
                Err(err) => {
                    warn!("Failed to get field {}: {}", field_name, err);
                    None
                }
            }
        })
        .collect()
}

fn set_file_field(file: &mut FileInfo, field_name: &str, joined_values: String) {
    match field_name {
        "file_type" => file.file_type = joined_values,
        "name" => file.name = joined_values,
        "path" => file.path = joined_values,
        _ => {}
    }
}
