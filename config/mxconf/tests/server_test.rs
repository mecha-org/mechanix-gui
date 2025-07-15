use mechanix_conf_server::server::{ConfigServerInterface, SERVED_AT, APPLICATION_SCHEMA_TREE_NAME};
use mechanix_conf_server::database::Database;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use std::env;
use tokio::test;
use zbus::{Connection, ConnectionBuilder};

// Helper function to set up a test database in a temporary directory
async unsafe fn setup_test_db_and_interface() -> (TempDir, ConfigServerInterface) {
    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Override the home directory for testing
    env::set_var("HOME", temp_dir.path().to_str().unwrap());
    
    // Create a new database instance
    let db = Database::new();
    
    // Create a D-Bus connection
    let connection = Connection::session().await.expect("Failed to connect to D-Bus");
    
    // Create the interface
    let interface = ConfigServerInterface {
        db: Arc::new(Mutex::new(db)),
        conn: connection,
    };
    
    (temp_dir, interface)
}

// Helper function to insert a test schema into the database
fn insert_test_schema(db: &mut Database, schema_name: &str) {
    let schema_content = r#"
    [section]
    key = { type = "string", default = "default_value", description = "A test key" }
    bool_key = { type = "bool", default = "true", description = "A boolean test key" }
    number_key = { type = "number", default = "42", description = "A number test key" }
    enum_key = { type = "enum", default = "option1", options = ["option1", "option2"], description = "An enum test key" }
    "#;
    
    db.insert(
        APPLICATION_SCHEMA_TREE_NAME,
        schema_name,
        schema_content.as_bytes(),
        None,
        None,
    ).expect("Failed to insert test schema");
}

#[tokio::test]
async fn test_list_schemas_empty() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // List schemas (should be empty)
    let result = interface.list_schemas().await;
    assert!(result.is_ok(), "List schemas should succeed");
    
    let schemas_json = result.unwrap();
    assert_eq!(schemas_json, "[]", "Schemas list should be empty");
}

#[tokio::test]
async fn test_list_schemas_with_data() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // List schemas
    let result = interface.list_schemas().await;
    assert!(result.is_ok(), "List schemas should succeed");
    
    let schemas_json = result.unwrap();
    assert!(schemas_json.contains("org.mechanix.test.toml"), "Schemas list should contain the test schema");
}

#[tokio::test]
async fn test_list_keys() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // List keys
    let result = interface.list_keys("org.mechanix.test".to_string()).await;
    assert!(result.is_ok(), "List keys should succeed");
    
    let keys = result.unwrap();
    assert!(keys.contains(&"section.key".to_string()), "Keys list should contain section.key");
    assert!(keys.contains(&"section.bool_key".to_string()), "Keys list should contain section.bool_key");
    assert!(keys.contains(&"section.number_key".to_string()), "Keys list should contain section.number_key");
    assert!(keys.contains(&"section.enum_key".to_string()), "Keys list should contain section.enum_key");
}

#[tokio::test]
async fn test_list_keys_nonexistent_schema() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // List keys for a nonexistent schema
    let result = interface.list_keys("org.mechanix.nonexistent".to_string()).await;
    assert!(result.is_err(), "List keys should fail for nonexistent schema");
}

#[tokio::test]
async fn test_describe_key() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // Describe a key
    let result = interface.describe_key(
        "org.mechanix.test".to_string(),
        "section.key".to_string()
    ).await;
    assert!(result.is_ok(), "Describe key should succeed");
    
    let description = result.unwrap();
    assert_eq!(description, "A test key", "Description should match");
}

#[tokio::test]
async fn test_describe_key_nonexistent() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // Describe a nonexistent key
    let result = interface.describe_key(
        "org.mechanix.test".to_string(),
        "section.nonexistent".to_string()
    ).await;
    assert!(result.is_err(), "Describe key should fail for nonexistent key");
}

#[tokio::test]
async fn test_get_setting_nonexistent() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // Get a nonexistent setting
    let result = interface.get_setting("org.mechanix.test.section.key").await;
    assert!(result.is_ok(), "Get setting should succeed even for nonexistent settings");
    
    let value = result.unwrap();
    assert_eq!(value, "", "Value should be empty for nonexistent setting");
}

#[tokio::test]
async fn test_set_and_get_setting() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // Set a setting
    let result = interface.set_setting("org.mechanix.test.section.key", "test_value").await;
    assert_eq!(result, "Success", "Set setting should succeed");
    
    // Get the setting
    let result = interface.get_setting("org.mechanix.test.section.key").await;
    assert!(result.is_ok(), "Get setting should succeed");
    
    let value = result.unwrap();
    assert_eq!(value, "test_value", "Retrieved value should match set value");
}

#[tokio::test]
async fn test_set_setting_invalid_value() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Insert a test schema
    {
        let mut db = interface.db.lock().unwrap();
        insert_test_schema(&mut db, "org.mechanix.test.toml");
    }
    
    // Set a boolean setting with an invalid value
    let result = interface.set_setting("org.mechanix.test.section.bool_key", "not_a_bool").await;
    assert!(result.contains("Validation error"), "Set setting should fail for invalid value");
}

#[tokio::test]
async fn test_set_setting_nonexistent_schema() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;
    
    // Set a setting for a nonexistent schema
    let result = interface.set_setting("org.mechanix.nonexistent.section.key", "test_value").await;
    assert!(result.contains("Schema error"), "Set setting should fail for nonexistent schema");
}