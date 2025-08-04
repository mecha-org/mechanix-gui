use std::collections::HashMap;
use anyhow::{Context, Result};
use sled::{Tree};
use sled::{Config, Db};
use std::path::PathBuf;
use log::{info, debug, warn};

/// Database struct for storing and retrieving configuration data.
///
/// This struct provides an interface to the underlying sled database,
/// which is used to store configuration settings and schema checksums.
/// The database is organized into trees, where each tree corresponds
/// to a schema or a collection of checksums.
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
            std::fs::create_dir_all(parent)
                .expect("Failed to create database directory");
        }

        let config = Config::new().path(&db_path);
        let db = config.open()
            .expect("Failed to open database");

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
        let tree = self.db.open_tree(identifier)
            .with_context(|| format!("Failed to open tree: {}", identifier))?;
        Ok(tree)
    }

    /// Insert a checksum into the database.
    ///
    /// This function stores a checksum value for a schema in the specified checksum tree.
    /// Checksums are used to detect changes in schema files.
    ///
    /// # Arguments
    ///
    /// * `schema_name` - The name of the schema
    /// * `checksum_identifier` - The identifier for the checksum tree
    /// * `checksum_value` - The checksum value to store
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the insertion was successful
    /// * `Err(...)` if there was an error during insertion
    pub fn insert_checksum(
        &mut self,
        schema_name: &str,
        checksum_identifier: &str,
        checksum_value: &u32
    ) -> Result<()> {
        info!("Inserting checksum for schema: {} value: {}", schema_name, checksum_value);
        let checksum_tree = self.get_tree(checksum_identifier)?;
        debug!("Checksum tree opened: {}", checksum_identifier);
        let checksum = checksum_value.to_le_bytes();
        checksum_tree.insert(schema_name, checksum.as_ref())
            .with_context(|| format!("Failed to insert checksum with schema_name: {}", schema_name))?;
        debug!("Checksum inserted for schema: {}", schema_name);
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
        info!("Inserting setting: {} in schema: {}", key, schema_identifier);
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
        let value_opt = tree.get(key)
            .with_context(|| format!("Failed to get value with key: {}", key))?;

        let mut results = HashMap::new();
        if let Some(value) = value_opt {
            let value_str = String::from_utf8_lossy(&value).to_string();
            results.insert(key.to_string(), value_str);
        }
        Ok(results)
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
    pub fn scan_with_prefix(&self, identifier: &str, key: &str) -> Result<HashMap<String, String>> {
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
        debug!("Found {} matching entries for prefix {}", results.len(), prefix);

        Ok(results)
    }
    /// Retrieve a checksum from the database.
    ///
    /// This function retrieves a checksum from the specified checksum tree using the given key.
    /// Checksums are stored as 4-byte little-endian u32 values.
    /// If the key doesn't exist or the value is not a valid 4-byte checksum, it returns None.
    ///
    /// # Arguments
    ///
    /// * `checksum_identifier` - The identifier of the checksum tree
    /// * `key` - The key of the checksum to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(None)` if the key is not present or the value is not a valid checksum
    /// * `Ok(Some(checksum))` if the key is present and the value is a valid checksum
    /// * `Err(...)` if there was an error during retrieval
    pub fn get_checksum(&self, checksum_identifier: &str, key: &str) -> Result<Option<u32>> {
        debug!("Getting checksum for key: {} from tree: {}", key, checksum_identifier);
        let checksum_tree = self.get_tree(checksum_identifier)?;
        let value_opt = checksum_tree.get(key)
            .with_context(|| format!("Failed to get checksum with key: {}", key))?;
        if let Some(value) = value_opt {
            debug!("Checksum value found for key: {}: {:?}", key, value);
            if value.len() == 4 {
                let bytes: [u8; 4] = value
                    .as_ref()
                    .try_into()
                    .with_context(|| "Failed to convert checksum bytes")?;
                Ok(Some(u32::from_le_bytes(bytes)))
            } else {
                warn!("Invalid checksum length for key {}: {}", key, value.len());
                Ok(None)
            }
        } else {
            warn!("No checksum found for key: {} in tree: {}", key, checksum_identifier);
            Ok(None)
        }
    }
}
