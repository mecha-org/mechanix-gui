use crate::utils::{parse_action_schema, ActionSchema, ActionSetting, Arg};
use crate::{utils, AppActionsConfig};
use log::{debug, error, info, warn};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::fs::read_dir;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
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
pub struct AppActions {
    pub name: String,
    pub icon: String,
    pub exec: String,
    pub section: String,
    pub action: String,
    pub description: String,
    pub arg_key: String,
    pub arg_value: String,
    pub score: f32,
}
pub enum FileIndexState {
    IndexedAndUpToDate, // indexed & checksum matches
    IndexedAndStale,    // indexed & checksum mismatch
    NotIndexed,         // not in the index at all
}

/// Public entry point for the app actions service.
#[derive()]
pub struct AppActionsService {
    config: AppActionsConfig,
    schema: Schema,
    index: Index,
    writer: Arc<Mutex<IndexWriter>>,
    index_worker_handle: Option<JoinHandle<()>>,
    watcher_handler: Option<JoinHandle<()>>,
}

impl AppActionsService {
    fn get_index_state(
        schema: &Schema,
        new_checksum: &str,
        file_path: &PathBuf,
        index_reader: &IndexReader,
        search_limit: usize,
    ) -> Result<FileIndexState, TantivyError> {
        // Check if last_modified is same then do not index
        let field = match schema.get_field("path") {
            Ok(field) => field,
            Err(err) => {
                error!("Failed to get the field: {}", err);
                return Err(err);
            }
        };
        let term = Term::from_field_text(field, &file_path.to_string_lossy().to_string());
        // check if the entry is already in the index
        let existing_entry = extract_doc(index_reader, &term, search_limit)?;
        if let Some(existing_entry) = existing_entry {
            // verify checksum
            let old_checksum_field = schema.get_field("checksum")?;
            let old_checksum_value = match existing_entry.get_first(old_checksum_field) {
                Some(v) => v,
                None => {
                    warn!(
                        "The checksum field isn't found for entry: {}",
                        file_path.display()
                    );
                    return Ok(FileIndexState::NotIndexed);
                }
            };
            if let Some(last_modified_field_value) = old_checksum_value.as_str() {
                return if last_modified_field_value == new_checksum {
                    debug!(
                        "Entry already exists, and last_modified is the same for path: {}",
                        file_path.display()
                    );
                    Ok(FileIndexState::IndexedAndUpToDate)
                } else {
                    debug!(
                        "Entry already exists, but last_modified is different for a path: {}",
                        file_path.display()
                    );
                    Ok(FileIndexState::IndexedAndStale)
                };
            }
        }
        Ok(FileIndexState::NotIndexed)
    }
    fn load_existing_desktop_entries(
        desktop_app_dir: &str,
        index_reader: &IndexReader,
        index_writer: &Arc<Mutex<IndexWriter>>,
        schema: &Schema,
        search_limit: usize,
    ) {
        info!("Loading existing desktop entries");
        let existing_desktop_entries = read_dir(&desktop_app_dir).unwrap();
        for entry in existing_desktop_entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let checksum = match generate_checksum(&path) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to generate checksum for {}: {}", path.display(), e);
                    continue;
                }
            };

            let term = Term::from_field_text(
                schema.get_field("path").unwrap(),
                &path.to_string_lossy().to_string(),
            );
            // check if the entry is already in the index
            let existing_entry =
                extract_doc(index_reader, &term, search_limit).unwrap_or_else(|e| {
                    error!("Failed to extract doc: {}", e);
                    None
                });
            if let Some(existing_entry) = existing_entry {
                debug!("Entry already exists for path: {}", path.display());
                // verify checksum
                let checksum_field = schema.get_field("checksum").unwrap();
                let checksum_field_value = match existing_entry.get_first(checksum_field) {
                    Some(v) => v,
                    None => {
                        warn!("Checksum field not found for entry: {}", path.display());
                        continue;
                    }
                };
                let checksum_field_value_str = match checksum_field_value.as_str() {
                    Some(s) => s,
                    None => {
                        warn!(
                            "Checksum field value is not a string for entry: {}",
                            path.display()
                        );
                        continue;
                    }
                };
                if checksum != checksum_field_value_str {
                    // If checksums don't match, then delete the entry
                    warn!("Checksum mismatch for entry: {}", path.display());
                    if let Ok(writer) = index_writer.lock() {
                        let _result = writer.delete_term(term);
                        info!("Removed indexed action entry: {}", path.display());
                    }
                } else {
                    debug!("Checksum match for entry: {}", path.display());
                    continue;
                }
            }
            let action_schema = match parse_action_schema(&path) {
                Some(d) => d,
                None => {
                    warn!("Failed to parse action schema: {}", path.display());
                    continue;
                }
            };
            debug!("Found action schema: {:?}", action_schema.name);

            let docs = feed_docs(&schema, &action_schema, checksum, &path);
            if let Ok(writer) = index_writer.lock() {
                for doc in docs {
                    match writer.add_document(doc) {
                        Ok(_) => info!("Indexed existing action schema: {}", action_schema.name),
                        Err(e) => error!("Failed to index the existing action schema: {}", e),
                    }
                }
            }
        }
        if let Ok(mut writer) = index_writer.lock() {
            if let Err(e) = writer.commit() {
                error!("Failed to commit index: {:?}", e);
            } else {
                debug!("Committed indexed app data to disk.");
            }
        };
        info!("Finished loading existing entries");
    }
    /// Create the Tantivy schema for `.desktop` fields
    fn create_schema() -> Schema {
        let mut schema_builder = Schema::builder();
        // top-level fields
        schema_builder.add_text_field("name", STRING | STORED);
        schema_builder.add_text_field("icon", STRING | STORED);
        schema_builder.add_text_field("exec", STRING | STORED);
        schema_builder.add_text_field("checksum", STORED);
        schema_builder.add_text_field("path", STRING | STORED);

        // section fields (each config block)
        schema_builder.add_text_field("section", STRING | STORED); // e.g. EnableWifi, EnableBluetooth
        schema_builder.add_text_field("action", TEXT | STORED); // searchable action text
        schema_builder.add_text_field("description", TEXT | STORED); // searchable description
        schema_builder.add_text_field("arg_key", STRING | STORED); // e.g. "path"
        schema_builder.add_text_field("arg_value", STRING | STORED); // e.g. "network", "bluetooth"

        schema_builder.build()
    }

    /// Create a new service instance.
    pub fn new(config: &AppActionsConfig) -> anyhow::Result<Self> {
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

    /// Starts the App Actions service.
    ///
    /// This function starts a separate task which watches the `schema_dir` for new or modified
    /// TOML files. When a change is detected, it will parse the TOML file, generate a checksum,
    /// and index the file in the Tantivy index. The index will be committed to disk after every
    /// change.
    ///
    /// The service will also load existing entries from the index on startup.
    ///
    /// If the watch path does not exist, this function will return an error.
    ///
    /// This function is marked as `async` because it uses async I/O to read the watch path and
    /// index the files. However, it does not use `await` because it uses a separate task to do
    /// the work.
    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Starting App Action watcher...");
        // Load existing entries, this should be in a separate task
        let schema = self.schema.clone(); // make sure schema is Arc or Clone
        let index_reader = self.index.reader()?; // Make sure this is thread safe
        let search_limit = self.config.search_limit;
        Self::load_existing_desktop_entries(
            &self.config.schema_dir,
            &index_reader,
            &self.writer.clone(),
            &schema,
            search_limit,
        );
        let watch_path: PathBuf = self.config.schema_dir.clone().into();
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

            if let Err(e) = watcher.watch(&watch_path_clone, RecursiveMode::NonRecursive) {
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
                            if let Some(path) = event.paths.get(0) {
                                if path.extension().and_then(|s| s.to_str()) == Some("toml") &&
                                !path.starts_with(".") &&
                                   (event.kind.is_create() || event.kind.is_modify()) || event.kind.is_remove() {
                                    unique_paths.insert(path.clone(), event.kind.clone());
                                }
                            }
                        }

                        for (path, kind) in unique_paths {
                               match kind {
                                EventKind::Any => {}
                                EventKind::Access(_) => {}
                                EventKind::Create(_) => {
                                  debug!("New schema detected: {}", path.display());
                                  if let Some(action_schema) = utils::parse_action_schema(&path) {
                                    debug!("action_schema: {:?}", action_schema);
                                    let checksum = match generate_checksum(&path) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            warn!("Failed to generate checksum for {}: {}", path.display(), e);
                                            String::new()
                                        }
                                    };
                                    let docs = feed_docs(&schema, &action_schema, checksum, &path);
                                    if let Ok(writer) = writer.lock() {
                                            for doc in docs {
                                                match writer.add_document(doc) {
                                                Ok(_) => info!("Indexed action entry: {}", action_schema.name),
                                                Err(e) => error!("Failed to index action entry: {}", e),
                                                }
                                            }
                                        }
                                    }
                                }
                                EventKind::Modify(_) => {
                                    debug!("Schema modification detected: {}", path.display());
                                    if let Some(action_schema) = utils::parse_action_schema(&path) {
                                        let checksum = match generate_checksum(&path) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            warn!("Failed to generate checksum for {}: {}", path.display(), e);
                                            String::new()
                                            }
                                        };
                                        let state = match Self::get_index_state(&schema, &checksum, &path, &reader, search_limit) {
                                            Ok(state) => state,
                                            Err(e) => {
                                                error!("Failed to get index state for {}: {}", path.display(), e);
                                                continue;
                                            }
                                        };
                                        match state {
                                            FileIndexState::IndexedAndUpToDate => {
                                                debug!(
                                                    "Entry already exists, and checksum is the same for schema: {}",
                                                    action_schema.name
                                                );
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
                                                    //TODO: we can delete this in get_index_state or return from that function
                                                    let term = Term::from_field_text(
                                                        field,
                                                        &path.to_string_lossy().to_string(),
                                                    );
                                                    let doc =
                                                        extract_doc(&index_reader, &term, search_limit).unwrap_or_else(|e| {
                                                            error!("Failed to extract doc: {}", e);
                                                            None
                                                        });
                                                    if let Some(_doc) = doc {
                                                        let _result = writer.delete_term(term);
                                                        info!("Removed indexed file entry: {:?}", path.file_name());
                                                    }
                                                    let docs = feed_docs(&schema, &action_schema, checksum, &path);
                                                    for doc in docs {
                                                        match writer.add_document(doc) {
                                                            Ok(_) => info!("Indexed new action schema: {}", action_schema.name),
                                                            Err(e) => error!("Failed to index the new action schema: {}", e),
                                                        }
                                                    }
                                                }
                                            }
                                            FileIndexState::NotIndexed => {
                                                 let docs = feed_docs(&schema, &action_schema, checksum, &path);
                                                if let Ok(writer) = writer.lock() {
                                                    for doc in docs {
                                                        match writer.add_document(doc) {
                                                            Ok(_) => info!("Indexed new action schema: {}", action_schema.name),
                                                            Err(e) => error!("Failed to index new action schema: {}", e),
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                EventKind::Remove(_) => {
                                    debug!("Removing indexed action entry: {:?}", path.file_name());
                                    if let Ok(writer) = writer.lock() {
                                        let field = match schema.get_field("path") {
                                            Ok(field) => field,
                                            Err(err) => {
                                                error!("Failed to get field - path: {}", err);
                                                continue;
                                            }
                                        };
                                        let term = Term::from_field_text(field, &path.to_string_lossy().to_string());
                                       let doc= extract_doc(&reader, &term, search_limit).unwrap_or_else(|e| {
                                                error!("Failed to extract doc: {}", e);
                                                None
                                            });
                                        if let Some(_doc) = doc {
                                            let _result =writer.delete_term(term);
                                            info!("Removed indexed action entry: {:?}", path.file_name());
                                        }
                                    }
                                }
                                EventKind::Other => {}
                            }
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

    /// Search indexed action using a free-form query.
    pub fn search(&self, query_str: &str, limit: usize) -> tantivy::Result<Vec<AppActions>> {
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

        // Define reload policy to reload the index on commit
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

            let mut app_action = AppActions::default();
            for (field, value) in doc.get_sorted_field_values() {
                let field_name = self.schema.get_field_name(field).to_string();
                // Join all values into a single string (semicolon-separated)
                let joined_values = value
                    .iter()
                    .filter_map(|val| val.as_str())
                    .collect::<Vec<_>>()
                    .join(";");

                set_app_action_field(&mut app_action, &field_name, joined_values, &query_str);
                app_action.score = score;
            }

            results.push(app_action);
        }

        Ok(results)
    }

    /// Graceful shutdown (optional: cancels task)
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        if let Some(handle) = &self.index_worker_handle {
            debug!("Aborting indexer task");
            handle.abort(); // Stop background debounce task
        }

        if let Some(handle) = &self.watcher_handler {
            debug!("Aborting watcher task");
            handle.abort(); // Stop background task
        }
        Ok(())
    }
}

/// Populate fields of `AppActions` from a given `joined_values` string
/// (semicolon-separated) based on a `field_name`.
///
/// If `field_name` is "arg_value" and the value is "%KEYWORD%", the
/// `search_query` is used instead.
///
/// # Parameters
///
/// * `app`: The `AppActions` instance to populate.
/// * `field_name`: The name of the field to populate.
/// * `joined_values`: The value to assign to the field.
/// * `search_query`: The search string to use if the `field_name` is
///   "arg_value" and the value is "%KEYWORD%".
fn set_app_action_field(
    app: &mut AppActions,
    field_name: &str,
    joined_values: String,
    search_query: &str,
) {
    match field_name {
        "name" => app.name = joined_values,
        "icon" => app.icon = joined_values,
        "exec" => app.exec = joined_values,
        "section" => app.section = joined_values,
        "action" => app.action = joined_values,
        "description" => app.description = joined_values,
        "arg_key" => app.arg_key = joined_values,
        "arg_value" => {
            if joined_values.to_lowercase() == "%keyword%" {
                app.arg_value = search_query.to_string();
            } else {
                app.arg_value = joined_values;
            }
        }
        _ => {}
    }
}

/// Create a `Vec` of `TantivyDocument`s from a `&ActionSchema`, with the given checksum and path.
pub fn feed_docs(
    schema: &Schema,
    action_schema: &ActionSchema,
    checksum: String,
    path: &Path,
) -> Vec<TantivyDocument> {
    let mut docs = Vec::new();

    // Get tantivy fields once for efficiency:
    let name_field = schema.get_field("name").unwrap();
    let icon_field = schema.get_field("icon").unwrap();
    let exec_field = schema.get_field("exec").unwrap();
    let section_field = schema.get_field("section").unwrap();
    let action_field = schema.get_field("action").unwrap();
    let description_field = schema.get_field("description").unwrap();
    let arg_key_field = schema.get_field("arg_key").unwrap();
    let arg_value_field = schema.get_field("arg_value").unwrap();
    let path_field = schema.get_field("path").unwrap();
    let checksum_field = schema.get_field("checksum").unwrap();

    for (section_name, action_setting) in &action_schema.actions {
        // Create document per action section
        let mut doc = TantivyDocument::default();

        // top-level fields
        doc.add_text(name_field, &action_schema.name);
        doc.add_text(icon_field, &action_schema.icon);
        doc.add_text(exec_field, &action_schema.exec);

        // section-level fields
        doc.add_text(section_field, section_name); // e.g. "EnableWifi"
        doc.add_text(action_field, &action_setting.action); // e.g. "Enable WiFi"
        doc.add_text(description_field, &action_setting.description); // e.g. "Enable wireless network"

        doc.add_text(arg_key_field, "path"); // because your Arg has only `path` key
        doc.add_text(arg_value_field, &action_setting.arg.path); // e.g. "network"

        doc.add_text(path_field, &path.to_string_lossy());
        doc.add_text(checksum_field, &checksum);

        docs.push(doc);
    }
    docs
}

/// Generates a SHA256 checksum for the file at the given path.
///
/// This function reads the contents of the specified file and computes its SHA256 hash,
/// returning the resulting checksum as a hexadecimal string. If the file cannot be read,
/// an I/O error is returned.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the file for which the checksum is to be generated.
///
/// # Returns
///
/// A `Result` containing the SHA256 checksum as a `String` if successful, or a `std::io::Error` if an error occurs during file reading.

fn generate_checksum(file_path: &PathBuf) -> Result<String, std::io::Error> {
    let file_bytes = match std::fs::read(&file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read file: {}", e);
            return Err(e);
        }
    };

    // Generate checksum
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&file_bytes);
    let checksum = format!("{:x}", hasher.finalize());
    Ok(checksum)
}

// A simple helper function to fetch a single document
// given its id from our index.
// It will be helpful to check our work.
fn extract_doc(
    reader: &IndexReader,
    app_path: &Term,
    search_limit: usize,
) -> tantivy::Result<Option<TantivyDocument>> {
    let searcher = reader.searcher();

    // This is the simplest query you can think of.
    // It matches all of the documents containing a specific term.
    //
    // The second argument is here to tell we don't care about decoding positions,
    // or term frequencies.
    let term_query = TermQuery::new(app_path.clone(), IndexRecordOption::Basic);
    let top_docs = searcher.search(&term_query, &TopDocs::with_limit(search_limit))?;

    if let Some((_score, doc_address)) = top_docs.first() {
        let doc = searcher.doc(*doc_address)?;
        Ok(Some(doc))
    } else {
        // no doc matching this ID.
        Ok(None)
    }
}
