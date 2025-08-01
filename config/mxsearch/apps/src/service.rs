use log::{debug, error, info, warn};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::read_dir;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::utils::{parse_desktop_entry, DesktopEntry};
use crate::Apps;
use tantivy::query::TermQuery;
use tantivy::schema::{Field, IndexRecordOption, Value, STRING};
use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::{Schema, STORED, TEXT}, Document, Index,
    IndexReader,
    IndexWriter,
    TantivyDocument,
    Term,
};
use tokio::{sync::mpsc, task::JoinHandle, time};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Type, SerializeDict, DeserializeDict, Debug, Default, Clone)]
#[zvariant(signature = "dict")]
pub struct AppInfo {
    pub type_: String,
    pub name: String,
    pub generic_name: String,
    pub keywords: Vec<String>,
    pub comment: String,
    pub icon: String,
    pub categories: Vec<String>,
    pub exec: String,
    pub path: String,
    pub score: f32,
}
/// Public entry point for the app search service.

#[derive()]
pub struct AppSearchService {
    config: Apps,
    schema: Schema,
    index: Index,
    writer: Arc<Mutex<IndexWriter>>,
    index_worker_handle: Option<JoinHandle<()>>,
    watcher_handler: Option<JoinHandle<()>>,
}

impl AppSearchService {
    fn load_existing_desktop_entries(
        desktop_app_dir: &str,
        index_reader: &IndexReader,
        index_writer: &Arc<Mutex<IndexWriter>>,
        schema: &Schema,
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
            let existing_entry = extract_doc_given_app_path(index_reader, &term).unwrap();
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
                        let term = Term::from_field_text(
                            schema.get_field("path").unwrap(),
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
                    }
                } else {
                    debug!("Checksum match for entry: {}", path.display());
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

            let doc = feed_doc(&schema, &desktop_entry, checksum, &path);
            if let Ok(writer) = index_writer.lock() {
                match writer.add_document(doc) {
                    Ok(_) => (),
                    Err(e) => error!("Failed to index app entry: {}", e),
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
        let mut schema_builder = tantivy::schema::Schema::builder();
        schema_builder.add_text_field("type", STRING | STORED);
        schema_builder.add_text_field("name", STRING | STORED);
        schema_builder.add_text_field("exec", STORED);
        schema_builder.add_text_field("comment", TEXT);
        schema_builder.add_text_field("generic_name", STRING | STORED);
        schema_builder.add_text_field("categories", STRING | STORED);
        schema_builder.add_text_field("keywords", TEXT | STORED);
        schema_builder.add_text_field("icon", STORED);
        schema_builder.add_text_field("checksum", STORED);
        schema_builder.add_text_field("path", STRING);

        schema_builder.build()
    }

    /// Create a new service instance.
    pub fn new(config: &Apps) -> anyhow::Result<Self> {
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

    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Starting AppSearchService watcher...");
        // Load existing entries, this should be in a separate task
        let schema = self.schema.clone(); // make sure schema is Arc or Clone
        let index_reader = self.index.reader()?; // Make sure this is thread safe
        Self::load_existing_desktop_entries(
            &self.config.desktop_apps_dir,
            &index_reader,
            &self.writer.clone(),
            &schema,
        );
        let watch_path: PathBuf = self.config.desktop_apps_dir.clone().into();
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
                            if let Some(path) = event.paths.get(0) {
                                if path.extension().and_then(|s| s.to_str()) == Some("desktop") &&
                                   (event.kind.is_create() || event.kind.is_modify()) || event.kind.is_remove() {
                                    debug!("File created or modified: {}", path.display());
                                    unique_paths.insert(path.clone(), event.kind.clone());
                                }
                            }
                        }

                        for (path, kind) in unique_paths {
                            if kind.is_create() || kind.is_modify() {
                                if let Some(desktop_entry) = parse_desktop_entry(&path) {
                                info!("Indexing changed desktop entry: {}", desktop_entry.name);
                                    let checksum = match generate_checksum(&path) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            warn!("Failed to generate checksum for {}: {}", path.display(), e);
                                            String::new()
                                        }
                                    };
                                    debug!("Checksum while storing: {}",checksum );
                                let doc = feed_doc(&schema, &desktop_entry, checksum, &path);
                                if let Ok(writer) = writer.lock() {
                                    match writer.add_document(doc) {
                                        Ok(_) => info!("Indexed desktop entry: {}", desktop_entry.name),
                                        Err(e) => error!("Failed to index app entry: {}", e),
                                        }
                                    }
                                }
                            } else if kind.is_remove() {
                                info!("Removing indexed app entry: {:?}", path.file_name());
                                if let Ok(writer) = writer.lock() {
                                    let term = Term::from_field_text(schema.get_field("path").unwrap(), &path.to_string_lossy().to_string());
                                   let doc= extract_doc_given_app_path(&reader, &term).unwrap_or_else(|e| {
                                            error!("Failed to extract doc: {}", e);
                                            None
                                        });
                                    if let Some(_doc) = doc {
                                        let _result =writer.delete_term(term);
                                        info!("Removed indexed app entry: {:?}", path.file_name());
                                    }
                                }
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
            }

            results.push(app);
        }

        Ok(results)
    }
    pub fn list_applications(&self, limit: usize) -> tantivy::Result<Vec<AppInfo>> {
        info!("List applications: limit {}", limit);
        let field_name = "type";
        let search_term = "Application";
        let field_to_lookup = match self.schema.get_field(field_name) {
            Ok(field) => field,
            Err(err) => {
                error!("Failed to get field {}: {}", field_name, err);
                return Err(err);
            }
        };

        let reader = self
            .index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let searcher = reader.searcher();
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
            }

            results.push(app);
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
fn set_app_field(app: &mut AppInfo, field_name: &str, joined_values: String) {
    match field_name {
        "type" => app.type_ = joined_values,
        "name" => app.name = joined_values,
        "exec" => app.exec = joined_values,
        "comment" => app.comment = joined_values,
        "generic_name" => app.generic_name = joined_values,
        "categories" => {
            app.categories = joined_values.split(';').map(|s| s.to_string()).collect();
        }
        "keywords" => {
            app.keywords = joined_values.split(';').map(|s| s.to_string()).collect();
        }
        "icon" => app.icon = joined_values,
        "path" => app.path = joined_values,
        _ => {}
    }
}

/// Creates a `TantivyDocument` from a `DesktopEntry`, with the given checksum and path.
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
/// - `checksum`: the checksum of the `.desktop` file
///
/// If any of the fields are missing in the `DesktopEntry`, they will be filled with default values.
///
/// # Arguments
///
/// * `schema`: the `Schema` to use for creating the `TantivyDocument`
/// * `desktop_entry`: the `DesktopEntry` to create the `TantivyDocument` from
/// * `checksum`: the checksum of the `.desktop` file
/// * `path`: the path to the `.desktop` file
///
/// # Returns
///
/// A `TantivyDocument` with the fields filled in from the `DesktopEntry`, checksum and path.
fn feed_doc(
    schema: &Schema,
    desktop_entry: &DesktopEntry,
    checksum: String,
    path: &Path,
) -> TantivyDocument {
    doc!(
        schema.get_field("type").unwrap() => desktop_entry.type_,
        schema.get_field("name").unwrap() => desktop_entry.name,
        schema.get_field("exec").unwrap() => desktop_entry.exec.clone().unwrap_or_default(),
        schema.get_field("comment").unwrap() => desktop_entry.comment.clone().unwrap_or_default(),
        schema.get_field("generic_name").unwrap() => desktop_entry.generic_name.clone().unwrap_or_default(),
        schema.get_field("categories").unwrap() => desktop_entry.categories.join(";"),
        schema.get_field("keywords").unwrap() => desktop_entry.keywords.join(";"),
        schema.get_field("icon").unwrap() => desktop_entry.icon.clone().unwrap_or_default(),
        schema.get_field("path").unwrap() => path.to_string_lossy().to_string(),
        schema.get_field("checksum").unwrap() => checksum
    )
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
    let top_docs = searcher.search(&term_query, &TopDocs::with_limit(200))?;

    if let Some((_score, doc_address)) = top_docs.first() {
        let doc = searcher.doc(*doc_address)?;
        Ok(Some(doc))
    } else {
        // no doc matching this ID.
        Ok(None)
    }
}
