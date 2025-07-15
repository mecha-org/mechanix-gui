use mxconf::database::Database;
use mxconf::server::ConfigServerInterface;
use std::env;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use zbus::Connection;

// Helper function to insert a test schema into the database
fn insert_test_schema(db: &mut Database, schema_name: &str) {
    // Insert schema checksum
    db.insert_checksum(schema_name, "checksums", &1234).expect("Failed to insert schema checksum");

    // Insert schema metadata
    let schema_id = schema_name.replace(".toml", "");

    // Insert test keys with their descriptions and default values
    db.insert_settings(&schema_id, "section.key", b"A test key").expect("Failed to insert key");
    db.insert_settings(&schema_id, "section.bool_key", b"A boolean key").expect("Failed to insert bool key");
    db.insert_settings(&schema_id, "section.number_key", b"A number key").expect("Failed to insert number key");
    db.insert_settings(&schema_id, "section.enum_key", b"An enum key").expect("Failed to insert enum key");
}

// Helper function to set up a test database in a temporary directory
async unsafe fn setup_test_db_and_interface() -> (TempDir, ConfigServerInterface) {
    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Override the home directory for testing
    env::set_var("HOME", temp_dir.path().to_str().unwrap());

    // Create a new database instance
    let db = Database::new("test_db".into());

    // Create a D-Bus connection
    let connection = Connection::session().await.expect("Failed to connect to D-Bus");

    // Create the interface
    let interface = ConfigServerInterface {
        db: Arc::new(Mutex::new(db)),
        conn: connection,
        key_file_dir: Default::default(),
        schema_dir: Default::default(),
    };

    (temp_dir, interface)
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
        "section.key".to_string(),
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
        "section.nonexistent".to_string(),
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
    assert!(value.is_empty(), "Value should be empty for nonexistent setting");
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
    assert!(result.is_ok(), "Set setting should succeed");
    assert_eq!(result.unwrap(), "Success", "Set setting should return 'Success'");

    // Get the setting
    let result = interface.get_setting("org.mechanix.test.section.key").await;
    assert!(result.is_ok(), "Get setting should succeed");

    let value = result.unwrap();
    assert!(value.contains_key("org.mechanix.test.section.key"), "Result should contain the key");
    assert_eq!(value.get("org.mechanix.test.section.key").unwrap(), "test_value", "Retrieved value should match set value");
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
    assert!(result.is_err(), "Set setting should fail for invalid value");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Validation error"), "Error should contain 'Validation error'");
}

#[tokio::test]
async fn test_set_setting_nonexistent_schema() {
    let (_temp_dir, interface) = unsafe { setup_test_db_and_interface() }.await;

    // Set a setting for a nonexistent schema
    let result = interface.set_setting("org.mechanix.nonexistent.section.key", "test_value").await;
    assert!(result.is_err(), "Set setting should fail for nonexistent schema");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to read schema file"), "Error should contain 'Failed to read schema file'");
}
