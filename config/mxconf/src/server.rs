use crate::database;
use crate::validator::{validate_setting, validate_value};
use anyhow::Result;
use log::{debug, error, info, trace, warn};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use toml::Value;
use zbus::{dbus_interface, fdo::Error as ZbusError, Connection, SignalContext};

/// The D-Bus path where the ConfigServer interface is served
pub const SERVED_AT: &str = "/org/mechanix/MxConf";

/// ConfigServerInterface struct for D-Bus interface.
///
/// This struct implements the D-Bus interface for the configuration server.
/// It provides methods for listing schemas, listing keys, describing keys,
/// getting settings, and setting settings. It also emits signals when
/// settings are changed.
///
/// The interface is served at the path defined by the SERVED_AT constant.
#[derive(Clone)]
pub struct ConfigServerInterface {
    /// The database instance for storing and retrieving settings
    pub db: Arc<Mutex<database::Database>>,

    /// The D-Bus connection
    pub conn: Connection,

    /// The directory containing key files for validation
    pub key_file_dir: PathBuf,

    /// The directory containing schema files
    pub schema_dir: PathBuf,
}

#[dbus_interface(name = "org.mechanix.MxConf")]
impl ConfigServerInterface {
    /// Signal emitted when a setting is changed.
    ///
    /// This signal is emitted whenever a setting is changed through the set_setting method.
    /// Clients can listen for this signal to be notified of changes to settings they are
    /// interested in.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the setting that was changed
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the signal was emitted successfully
    /// * `Err(...)` if there was an error during emission

    #[dbus_interface(signal)]
    async fn schema_key_changed(
        &self,
        ctxt: &SignalContext<'_>,
        schema: &str,
        key: &str,
        value: &str,
    ) -> Result<(), zbus::Error>;

    /// List all available schemas.
    ///
    /// This method scans the schema directory and returns a list of all available schemas.
    /// Schemas are identified by their file names without the .toml or .tom extension.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - A JSON string containing an array of schema names
    /// * `Err(ZbusError)` - If there was an error during schema listing or JSON conversion
    ///
    /// # Example
    ///
    /// ```json
    /// ["org.mechanix.app1", "org.mechanix.app2"]
    /// ```
    pub async fn list_schemas(&self) -> Result<String, ZbusError> {
        info!("Listing all schemas in directory: {}", self.schema_dir.display());
        let mut keys = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.schema_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    debug!("Found file in schema dir: {}", file_name);
                    if file_name.ends_with(".toml") || file_name.ends_with(".tom") {
                        if let Some(schema_name) = file_name
                            .strip_suffix(".toml")
                            .or(file_name.strip_suffix(".tom"))
                        {
                            keys.push(schema_name.to_string());
                        }
                    }
                }
            }
        } else {
            warn!("Could not read schema directory: {}", self.schema_dir.display());
        }
        let json = serde_json::to_string(&keys)
            .map_err(|e| ZbusError::Failed(format!("JSON error: {}", e)))?;
        debug!("Returning schemas JSON: {}", json);
        Ok(json)
    }

    /// List all keys for a given schema.
    ///
    /// This method reads the schema file for the specified schema and extracts all the keys.
    /// Keys are returned as dotted paths relative to the schema (e.g., "section.key").
    ///
    /// # Arguments
    ///
    /// * `schema` - The name of the schema to list keys for (e.g., "org.mechanix.app")
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)` - A vector of key names
    /// * `Err(ZbusError)` - If there was an error reading or parsing the schema file
    ///
    /// # Errors
    ///
    /// This method returns an error if:
    /// - The schema file cannot be read
    /// - The schema file contains invalid TOML
    pub async fn list_keys(&self, schema: String) -> Result<Vec<String>, ZbusError> {
        info!("Listing all keys for schema: {}", schema);
        let schema_name = extract_schema_name(&schema);
        let schema_path = self.schema_dir.join(format!("{}.toml", schema_name));
        debug!("Schema file path: {}", schema_path.display());
        let toml_str = fs::read_to_string(&schema_path)
            .map_err(|e| ZbusError::Failed(format!("Failed to read schema file: {}", e)))?;
        let toml_value: toml::Value = toml::from_str(&toml_str)
            .map_err(|e| ZbusError::Failed(format!("Invalid TOML: {}", e)))?;
        let mut keys: Vec<String> = Vec::new();
        extract_leaf_tables(&toml_value, "".to_string(), &mut keys);
        debug!("Extracted keys: {:?}", keys);
        Ok(keys)
    }

    /// Get the description of a key in a schema.
    ///
    /// This method reads the schema file for the specified schema and extracts the description
    /// of the specified key. The description is a string that explains the purpose and usage
    /// of the key.
    ///
    /// # Arguments
    ///
    /// * `schema` - The name of the schema (e.g., "org.mechanix.app")
    /// * `key` - The key to describe (e.g., "section.key")
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The description of the key
    /// * `Err(ZbusError)` - If there was an error reading or parsing the schema file,
    ///   or if the key was not found
    ///
    /// # Errors
    ///
    /// This method returns an error if:
    /// - The schema file cannot be read
    /// - The schema file contains invalid TOML
    /// - The key is not found in the schema
    pub async fn describe_key(&self, schema: String, key: String) -> Result<String, ZbusError> {
        info!("Describing key '{}' of schema: {}", key, schema);
        let schema_name = extract_schema_name(&schema);
        let schema_path = self.schema_dir.join(format!("{}.toml", schema_name));
        debug!("Schema file path: {}", schema_path.display());
        let toml_str = fs::read_to_string(&schema_path)
            .map_err(|e| ZbusError::Failed(format!("Failed to read schema file: {}", e)))?;
        let toml_value: toml::Value = toml::from_str(&toml_str)
            .map_err(|e| ZbusError::Failed(format!("Invalid TOML: {}", e)))?;
        trace!("TOML VALUE: {}", toml_value);
        let Some(description) = extract_description(&toml_value, &key) else {
            warn!("Key not found in schema: {}", key);
            return Err(ZbusError::Failed(format!("Key not found: {}", key)));
        };
        debug!("Description for key '{}': {}", key, description);
        Ok(description.to_string())
    }

    /// Get a setting value from the database.
    ///
    /// This method retrieves a setting value from the database using the specified key.
    /// The key is used to determine the schema, and the value is validated against the schema
    /// before being returned.
    ///
    /// # Arguments
    ///
    /// * `key` - Can be the fully qualified key of the setting or the wildcard key (e.g., "org.mechanix.app.*")
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The setting value as a string if found, or an empty string if not found
    /// * `Err(ZbusError)` - If there was an error retrieving or validating the setting
    ///
    /// # Errors
    ///
    /// This method returns an error if:
    /// - There was a database error
    /// - The schema file cannot be read
    /// - The schema file contains invalid TOML
    /// - The retrieved value fails validation against the schema
    pub async fn get_setting(&self, key: &str) -> Result<HashMap<String, String>, ZbusError> {
        info!("Get Setting: Received key: {}", key);

        let schema_name = extract_schema_name(key);
        let schema_path = self.schema_dir.join(format!("{}.toml", schema_name));
        let schema = fs::read_to_string(&schema_path)
            .map_err(|e| ZbusError::Failed(format!("Failed to read schema file: {}", e)))?;
        let schema_as_toml = toml::from_str(&schema)
            .map_err(|e| ZbusError::Failed(format!("Invalid TOML: {}", e)))?;

        let db = &self.db.lock().unwrap();

        let settings: HashMap<String, String> = if key.contains('*') {
            debug!("Wildcard Schema Name: {}", schema_name);
            db.scan_with_prefix(&schema_name, key.split('*').next().unwrap_or_default())
                .map_err(|e| ZbusError::Failed(format!("Database error: {}", e)))?
        } else {
            debug!("Schema Name: {}", schema_name);
            db.get(&schema_name, key)
                .map_err(|e| ZbusError::Failed(format!("Database error: {}", e)))?
        };

        let mut results = HashMap::new();

        for (k, v) in settings {
            match validate_value(&schema_as_toml, &k, &v) {
                Ok(_) => {
                    results.insert(k, v);
                }
                Err(e) => {
                    warn!("Validation error for key {}: {}", k, e);
                    // Skip invalid entry
                }
            }
        }

        if results.is_empty() && !key.contains('*') {
            warn!("No value found for key: {}", key);
        }

        Ok(results)
    }


    /// Set a setting value in the database.
    ///
    /// This method validates and stores a setting value in the database using the specified key.
    /// The key is used to determine the schema, and the value is validated against the schema
    /// and any key files before being stored. After the value is stored, a notification signal
    /// is emitted to inform clients of the change.
    ///
    /// # Arguments
    ///
    /// * `key` - The fully qualified key of the setting (e.g., "org.mechanix.app.section.key")
    /// * `value` - The value to store
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - "Success" if the setting was stored successfully
    /// * `Err(ZbusError)` - If there was an error validating or storing the setting
    ///
    /// # Errors
    ///
    /// This method returns an error if:
    /// - The setting is locked by a key file
    /// - The schema file cannot be read
    /// - The schema file contains invalid TOML
    /// - The value fails validation against the schema
    /// - There was a database error
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<String, ZbusError> {
        info!("Set Setting: Received key: {}, value: {}", key, value);
        let schema_name = extract_schema_name(key);
        debug!("Schema Name: {}", schema_name);
        match validate_with_key_file(&self.key_file_dir, &schema_name, key, value) {
            Ok(_) => debug!("Setting validated with key file for key: {}", key),
            Err(e) => {
                error!("Validation error with key file for key {}: {}", key, e);
                return Err(ZbusError::Failed(format!("Validation error: {}", e)));
            }
        }
        let schema_path = self.schema_dir.join(format!("{}.toml", schema_name));
        debug!("Schema file path: {}", schema_path.display());
        let schema = fs::read_to_string(&schema_path)
            .map_err(|e| ZbusError::Failed(format!("Failed to read schema file: {}", e)))?;
        let schema_as_toml = toml::from_str(&schema)
            .map_err(|e| ZbusError::Failed(format!("Failed to read schema file: {}", e)))?;
        match validate_setting(&schema_as_toml, key, value) {
            Ok(_) => debug!("Setting validated against schema for key: {}", key),
            Err(e) => {
                error!("Validation error against schema for key {}: {}", key, e);
                return Err(ZbusError::Failed(format!("Validation error: {}", e)));
            }
        }
        let insert_result = {
            let mut db = self.db.lock().unwrap();
            db.insert_settings(&schema_name, key, value.as_bytes())
        };
        match insert_result {
            Ok(_) => {
                info!("Setting updated: key={}, value={}", key, value);
                match SignalContext::new(&self.conn, SERVED_AT) {
                    Ok(ctxt) => {
                        let key = key.split('.').skip(3).collect::<Vec<&str>>().join(".");
                        info!("Emitting notification for key: {}", key);
                        if let Err(e) = self.schema_key_changed(&ctxt, &schema_name, &key, &value).await {
                            error!("Failed to emit notification for key {}: {}", key, e);
                        } else {
                            debug!("Successfully emitted notification for key: {}", key);
                        }
                    }
                    Err(e) => {
                        error!("Failed to create signal context: {}", e);
                    }
                }
                Ok("Success".to_string())
            }
            Err(e) => {
                error!("Database error for key {}: {}", key, e);
                Err(ZbusError::Failed(format!("Database error: {}", e)))
            }
        }
    }
}

/// Extract schema name from a key.
///
/// This function extracts the schema name from a fully qualified key.
/// The schema name is the first three components of the key, joined by dots.
/// For example, from "org.mechanix.app.section.key", it extracts "org.mechanix.app".
///
/// # Arguments
///
/// * `key` - The fully qualified key to extract schema name from
///
/// # Returns
///
/// The schema name derived from the key
fn extract_schema_name(key: &str) -> String {
    key.split('.').take(3).collect::<Vec<&str>>().join(".")
}

/// Extract leaf tables from a TOML value.
///
/// This function recursively traverses a TOML value and extracts all leaf tables.
/// A leaf table is a table that contains other tables, but is not at the root level.
/// The keys are returned as dotted paths.
///
/// # Arguments
///
/// * `value` - The TOML value to extract leaf tables from
/// * `prefix` - The current prefix for the path
/// * `keys` - A mutable vector to store the extracted keys
fn extract_leaf_tables(value: &Value, prefix: String, keys: &mut Vec<String>) {
    if let Value::Table(table) = value {
        for (k, v) in table {
            let new_prefix = if prefix.is_empty() {
                k.clone()
            } else {
                format!("{}.{}", prefix, k)
            };
            // If the value is a table and not at the root, it's a leaf table (inline table)
            if let Value::Table(_) = v {
                // Only push if the parent is not empty (i.e., not root)
                if !prefix.is_empty() {
                    keys.push(new_prefix.clone());
                }
                // Recurse in case of nested tables
                extract_leaf_tables(v, new_prefix, keys);
            }
        }
    }
}

/// Extract description from a TOML value for a given key.
///
/// This function traverses a TOML value using a dotted key path and extracts
/// the "description" field from the resulting table.
///
/// # Arguments
///
/// * `value` - The TOML value to extract the description from
/// * `dotted_key` - The dotted key path to the table containing the description
///
/// # Returns
///
/// * `Some(&str)` - The description if found
/// * `None` - If the key path doesn't exist or doesn't contain a description
fn extract_description<'a>(value: &'a Value, dotted_key: &str) -> Option<&'a str> {
    let mut current = value;
    for part in dotted_key.split('.') {
        if let Value::Table(table) = current {
            current = table.get(part)?;
        } else {
            return None;
        }
    }
    if let Value::Table(table) = current {
        if let Some(Value::String(desc)) = table.get("description") {
            return Some(desc);
        }
    }
    None
}

/// Validate a setting against a key file.
///
/// This function checks if a setting is locked by a key file. If a key file exists
/// for the schema and the setting is locked, an error is returned.
///
/// # Arguments
///
/// * `key_file_dir` - The directory containing key files
/// * `schema_name` - The name of the schema
/// * `key` - The fully qualified key of the setting
/// * `value` - The value to validate
///
/// # Returns
///
/// * `Ok(())` if the setting is not locked or no key file exists
/// * `Err(...)` if the setting is locked by a key file or there was an error reading the key file
fn validate_with_key_file(
    key_file_dir: &PathBuf,
    schema_name: &str,
    key: &str,
    value: &str,
) -> Result<()> {
    debug!("Validating with key file for schema: {}", schema_name);
    if let Some(key_file) = find_latest_schema_file(key_file_dir, schema_name) {
        info!("Found key file: {}", key_file.display());
        let file_str = fs::read_to_string(key_file)?;
        let key_file: Value = toml::from_str(&file_str)?;
        trace!("key file: {}", key_file);
        trace!("key {}, value {}", key, value);
        if let Some((val, locked)) = get_value_and_locked_by_path(&key_file, schema_name, key) {
            if let Some(_val) = val {
                if let Some(locked) = locked {
                    if locked == "true" {
                        warn!("Setting is locked by key file for key: {}", key);
                        return Err(anyhow::anyhow!("Setting is locked by key file"));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Get the value and locked status of a setting from a key file.
///
/// This function traverses a TOML value using a key path and extracts the value
/// and locked status of a setting.
///
/// # Arguments
///
/// * `value` - The TOML value to extract from
/// * `schema` - The schema name
/// * `full_key` - The fully qualified key of the setting
///
/// # Returns
///
/// * `Some((Some(&str), Some(&str)))` - If the setting has both a value and locked status
/// * `Some((Some(&str), None))` - If the setting has a value but no locked status
/// * `Some((None, Some(&str)))` - If the setting has a locked status but no value
/// * `Some((None, None))` - If the setting exists but has neither a value nor locked status
/// * `None` - If the setting doesn't exist in the key file
fn get_value_and_locked_by_path<'a>(
    value: &'a toml::Value,
    schema: &str,
    full_key: &str,
) -> Option<(Option<&'a str>, Option<&'a str>)> {
    let prefix = format!("{}.", schema);
    let key_path = full_key.strip_prefix(&prefix).unwrap_or(full_key);
    let keys: Vec<&str> = key_path.split('.').collect();
    let mut current = value;
    for key in &keys[..keys.len() - 1] {
        current = current.get(*key)?;
    }
    let last_key = keys[keys.len() - 1];
    let entry = current.get(last_key)?;
    match entry {
        toml::Value::Table(map) => {
            let val = map.get("value").and_then(|v| v.as_str());
            let locked = map.get("locked").and_then(|v| v.as_str());
            Some((val, locked))
        }
        toml::Value::String(s) => Some((Some(s), None)),
        _ => None,
    }
}

/// Find the latest schema file based on numeric prefix.
///
/// This function searches a directory for files matching a pattern with a numeric prefix
/// and the schema name. It returns the path to the file with the highest prefix.
///
/// # Arguments
///
/// * `directory` - The directory to search
/// * `schema_name` - The name of the schema
///
/// # Returns
///
/// * `Some(PathBuf)` - The path to the latest schema file if found
/// * `None` - If no matching files were found
fn find_latest_schema_file<P: AsRef<Path>>(directory: P, schema_name: &str) -> Option<PathBuf> {
    debug!("Searching for latest schema file for schema: {} in dir: {}", schema_name, directory.as_ref().display());
    let pattern = format!(
        r"^(?P<prefix>\\d+)-?{}\\.(toml|tom)$",
        regex::escape(schema_name)
    );
    let re = Regex::new(&pattern).unwrap();
    let mut latest: Option<(u32, PathBuf)> = None;
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            debug!("File name: {}", file_name.to_string_lossy());
            let file_name = file_name.to_string_lossy();
            if let Some(caps) = re.captures(&file_name) {
                let prefix: u32 = caps
                    .name("prefix")
                    .map_or(0, |m| m.as_str().parse().unwrap_or(0));
                debug!("Prefix: {} for file: {}", prefix, file_name);
                let path = entry.path();
                debug!("Path: {}", path.display());
                if latest.is_none() || prefix > latest.as_ref().unwrap().0 {
                    latest = Some((prefix, path));
                }
            }
        }
    }
    latest.map(|(_, path)| path)
}
