use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use sled::Tree;
use sled::{Config, Db};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::{mpsc, oneshot};


pub enum DbCmd {
    GetLastModified {
        identifier: String,
        key: String,
        rsp: oneshot::Sender<Option<String>>,
    },
    InsertLastModified {
        identifier: String,
        key: String,
        value: String,
    },
    Get {
        identifier: String,
        key: String,
        rsp: oneshot::Sender<HashMap<String, String>>,
    },
    Set {
        identifier: String,
        key: String,
        value: Vec<u8>,
        rsp: oneshot::Sender<Result<()>>,
    },
}

pub fn start_db_actor(db: Database) -> mpsc::Sender<DbCmd> {
    let (tx, mut rx) = mpsc::channel::<DbCmd>(128);
    tokio::spawn(async move {
        let mut db = db; // owned by this task
        while let Some(cmd) = rx.recv().await {
                match cmd {
                    DbCmd::GetLastModified {
                        identifier,
                        key,
                        rsp,
                    } => match db.get_last_modified(&identifier, &key) {
                        Ok(last_modified) => {
                            let _ = rsp.send(last_modified);
                        }
                        Err(e) => {
                            error!("Failed to get last_modified: {}", e);
                            let _ = rsp.send(None);
                        }
                    },
                    DbCmd::InsertLastModified {
                        identifier,
                        key,
                        value,
                    } => match db.insert_last_modified(&identifier, &key, &value) {
                        Ok(()) => {}
                        Err(e) => {
                            error!("Failed to insert last_modified: {}", e);
                        }
                    },
                    DbCmd::Get {
                        identifier,
                        key,
                        rsp,
                    } => match db.get(&identifier, &key) {
                        Ok(settings) => {
                            let _ = rsp.send(settings);
                        }
                        Err(e) => {
                            error!("Failed to get settings: {}", e);
                            let _ = rsp.send(HashMap::new());
                        }
                    },
                    DbCmd::Set {
                        identifier,
                        key,
                        value,
                        rsp,
                    } => {
                        if let Err(err) = rsp.send(db.insert_settings(&identifier, &key, &value)) {
                            error!("Failed to send an insert result on channel: {:?}", err);
                        }
                    }
                }
            }
        });
    tx
}
/// Database struct for storing and retrieving configuration data.
///
/// This struct provides an interface to the underlying sled database,
/// which is used to store configuration settings and schema last_modifieds.
/// The database is organized into trees, where each tree corresponds
/// to a schema or a collection of last_modifieds.
#[derive(Debug, Clone)]
pub struct Database {
    /// The underlying sled database instance
    db: Db,
}

impl Database {
    /// Create a new Database instance with the specified path.
    ///
    /// This function creates a new database instance at the specified path.
    /// If the parent directories don't exist, they will be created.
    ///
    /// # Arguments
    ///
    /// * `db_path` - The path where the database will be stored
    ///
    /// # Returns
    ///
    /// A new Database instance with a connection to the database at the specified path
    ///
    /// # Panics
    ///
    /// Panics if the database cannot be opened or if the parent directories cannot be created
    pub fn new(db_path: PathBuf) -> Self {
        info!("Opening database at path: {}", db_path.display());
        // Ensure parent directories exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create database directory");
        }

        let config = Config::new().path(&db_path);
        let db = config.open().expect("Failed to open database");

        Self { db }
    }

    /// Get a tree from the database.
    ///
    /// Trees in sled are similar to tables in a relational database.
    /// Each tree contains a collection of key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the tree to get
    ///
    /// # Returns
    ///
    /// * `Ok(Tree)` if the tree was retrieved successfully
    /// * `Err(...)` if there was an error during retrieval
    fn get_tree(&self, identifier: &str) -> Result<Tree> {
        debug!("Opening tree: {}", identifier);
        let tree = self
            .db
            .open_tree(identifier)
            .with_context(|| format!("Failed to open tree: {}", identifier))?;
        Ok(tree)
    }

    /// Insert a last_modified into the database.
    ///
    /// This function stores a last_modified value for a schema in the specified last_modified tree.
    /// last_modifieds are used to detect changes in schema files.
    ///
    /// # Arguments
    ///
    /// * `last_modified_identifier` - The identifier for the last_modified tree
    /// * `schema_name` - The name of the schema
    /// * `last_modified_value` - The last_modified value to store
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the insertion was successful
    /// * `Err(...)` if there was an error during insertion
    pub fn insert_last_modified(
        &mut self,
        identifier: &str,
        schema_name: &str,
        last_modified: &str,
    ) -> Result<()> {
        info!(
            "Inserting last_modified for schema: {} value: {}",
            schema_name, last_modified
        );
        let last_modified_tree = self.get_tree(identifier)?;
        // let last_modified_bytes = last_modified.bytes();
        last_modified_tree
            .insert(schema_name, last_modified)
            .with_context(|| {
                format!(
                    "Failed to insert last_modified with schema_name: {}",
                    schema_name
                )
            })?;
        debug!("last_modified inserted for schema: {}", schema_name);
        Ok(())
    }

    /// Insert a setting into the database.
    ///
    /// This function stores a setting value in the specified schema tree.
    /// If the key already exists, its value will be updated.
    ///
    /// # Arguments
    ///
    /// * `schema_identifier` - The identifier of the schema tree to insert into
    /// * `key` - The key to insert
    /// * `value` - The value to insert as a byte slice
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the insertion was successful
    /// * `Err(...)` if there was an error during insertion
    pub fn insert_settings(
        &mut self,
        schema_identifier: &str,
        key: &str,
        value: &[u8],
    ) -> Result<()> {
        info!(
            "Inserting setting: {} in schema: {}",
            key, schema_identifier
        );
        let tree = self.get_tree(schema_identifier)?;
        debug!("Tree opened for schema: {}", schema_identifier);
        tree.insert(key, value)
            .with_context(|| format!("Failed to insert setting with key: {}", key))?;
        debug!("Setting inserted: {}", key);
        Ok(())
    }
    /// Retrieve a value from the database.
    ///
    /// This function retrieves a value from the specified tree using the given key.
    /// If the key doesn't exist, it returns None.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the tree to retrieve from
    /// * `key` - The key of the value to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(None)` if the key is not present in the database
    /// * `Ok(Some(value))` if the key is present, where `value` is the associated value
    /// * `Err(...)` if there was an error during retrieval
    pub fn get(&self, identifier: &str, key: &str) -> Result<HashMap<String, String>> {
        debug!("Getting value for key: {} from tree: {}", key, identifier);
        let tree = self.get_tree(identifier)?;

        let settings = if let Some(prefix) = key.split('*').next().filter(|p| !p.is_empty()) {
            // Wildcard / prefix case
            self.scan_with_prefix(identifier, prefix)?
        } else {
            // Exact key case
            let mut results = HashMap::new();
            if let Some(value) = tree
                .get(key)
                .with_context(|| format!("Failed to get value with key: {}", key))?
            {
                results.insert(key.to_string(), String::from_utf8_lossy(&value).to_string());
            }
            results
        };

        Ok(settings)
    }

    /// Perform a prefix scan on a tree.
    ///
    /// This function performs a prefix scan on the specified tree, using the given key as a prefix.
    /// It returns a map of all key-value pairs in the tree that match the prefix.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier of the tree to scan
    /// * `key` - The key to use as a prefix for the scan
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<String, String>)` - A map of all key-value pairs in the tree that match the prefix
    /// * `Err(...)` - If there was an error during the scan
    fn scan_with_prefix(&self, identifier: &str, key: &str) -> Result<HashMap<String, String>> {
        debug!("Prefix scan for key: {} in tree: {}", key, identifier);
        let tree = self.get_tree(identifier)?;

        let prefix = key.split('*').next().unwrap_or_default();
        debug!("Scanning with prefix: {}", prefix);

        let mut results = HashMap::new();

        for result in tree.scan_prefix(prefix.as_bytes()) {
            let (k, v) = result.with_context(|| "Failed to scan key-value pair")?;
            let key_str = String::from_utf8_lossy(&k).to_string();
            let value_str = String::from_utf8_lossy(&v).to_string();
            results.insert(key_str, value_str);
        }
        debug!(
            "Found {} matching entries for prefix {}",
            results.len(),
            prefix
        );

        Ok(results)
    }
    /// Retrieve a last_modified from the database.
    ///
    /// This function retrieves a last_modified from the specified last_modified tree using the given key.
    /// last_modifieds are stored as 4-byte little-endian u32 values.
    /// If the key doesn't exist or the value is not a valid 4-byte last_modified, it returns None.
    ///
    /// # Arguments
    ///
    /// * `last_modified_identifier` - The identifier of the last_modified tree
    /// * `key` - The key of the last_modified to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(None)` if the key is not present or the value is not a valid last_modified
    /// * `Ok(Some(last_modified))` if the key is present and the value is a valid last_modified
    /// * `Err(...)` if there was an error during retrieval
    pub fn get_last_modified(&self, identifier: &str, key: &str) -> Result<Option<String>> {
        debug!(
            "Getting last_modified for key: {} from tree: {}",
            key, identifier
        );
        let tree = self.get_tree(identifier)?;
        let value_opt = tree
            .get(key)
            .with_context(|| format!("Failed to get last_modified with key: {}", key))?;
        if let Some(value) = value_opt {
            debug!("last_modified value found for key: {}: {:?}", key, value);
            Ok(Some(String::from_utf8_lossy(&value).to_string()))
        } else {
            warn!(
                "No last_modified found for key: {} in tree: {}",
                key, identifier
            );
            Ok(None)
        }
    }
}
