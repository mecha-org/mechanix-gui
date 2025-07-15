use mechanix_conf_server::database::Database;
use std::env;
use std::path::PathBuf;
use tempfile::TempDir;
use serial_test::serial;

// Helper function to set up a test database in a temporary directory
unsafe fn setup_test_db() -> (TempDir, Database) {
    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Override the home directory for testing
    env::set_var("HOME", temp_dir.path().to_str().unwrap());
    
    // Create a new database instance
    let db = Database::new();
    
    (temp_dir, db)
}

#[test]
#[serial]
fn test_database_new() {
    let (temp_dir, _db) = unsafe { setup_test_db() };
    
    // Check that the database directory was created
    let db_path = temp_dir.path().join(".config/mechanix/conf/store/db");
    assert!(db_path.exists(), "Database directory should exist");
}

#[test]
#[serial]
fn test_database_insert_and_get() {
    let (_temp_dir, mut db) = unsafe { setup_test_db() };
    
    // Insert a value
    let schema_identifier = "test_schema";
    let key = "test_key";
    let value = b"test_value";
    
    let result = db.insert(schema_identifier, key, value, None, None);
    assert!(result.is_ok(), "Insert should succeed");
    
    // Get the value
    let result = db.get(schema_identifier, key);
    assert!(result.is_ok(), "Get should succeed");
    
    let value_opt = result.unwrap();
    assert!(value_opt.is_some(), "Value should exist");
    
    let retrieved_value = value_opt.unwrap();
    assert_eq!(retrieved_value, value, "Retrieved value should match inserted value");
}

#[test]
#[serial]
fn test_database_get_nonexistent() {
    let (_temp_dir, db) = unsafe { setup_test_db() };
    
    // Try to get a nonexistent value
    let result = db.get("nonexistent_schema", "nonexistent_key");
    assert!(result.is_ok(), "Get should succeed even for nonexistent keys");
    
    let value_opt = result.unwrap();
    assert!(value_opt.is_none(), "Value should not exist");
}

#[test]
#[serial]
fn test_database_insert_with_checksum() {
    let (_temp_dir, mut db) = unsafe { setup_test_db() };
    
    // Insert a value with a checksum
    let schema_identifier = "test_schema";
    let key = "test_key";
    let value = b"test_value";
    let checksum_identifier = "test_checksum";
    let checksum_value = 12345u32;
    
    let result = db.insert(schema_identifier, key, value, Some(checksum_identifier), Some(&checksum_value));
    assert!(result.is_ok(), "Insert with checksum should succeed");
    
    // Get the value
    let result = db.get(schema_identifier, key);
    assert!(result.is_ok(), "Get should succeed");
    
    let value_opt = result.unwrap();
    assert!(value_opt.is_some(), "Value should exist");
    
    // Get the checksum
    let result = db.get_checksum(checksum_identifier, key);
    assert!(result.is_ok(), "Get checksum should succeed");
    
    let checksum_opt = result.unwrap();
    assert!(checksum_opt.is_some(), "Checksum should exist");
    
    let retrieved_checksum = checksum_opt.unwrap();
    assert_eq!(retrieved_checksum, checksum_value, "Retrieved checksum should match inserted checksum");
}

#[test]
#[serial]
fn test_database_list_keys() {
    let (_temp_dir, mut db) = unsafe { setup_test_db() };
    
    // Insert multiple values
    let schema_identifier = "test_schema";
    let keys = ["key1", "key2", "key3"];
    let value = b"test_value";
    
    for key in &keys {
        let result = db.insert(schema_identifier, key, value, None, None);
        assert!(result.is_ok(), "Insert should succeed");
    }
    
    // List the keys
    let result = db.list_keys(schema_identifier);
    assert!(result.is_ok(), "List keys should succeed");
    
    let listed_keys = result.unwrap();
    assert_eq!(listed_keys.len(), keys.len(), "Should list all inserted keys");
    
    // Check that all inserted keys are in the list
    for key in &keys {
        assert!(listed_keys.contains(&key.to_string()), "Listed keys should contain inserted key");
    }
}

#[test]
#[serial]
fn test_database_get_nonexistent_checksum() {
    let (_temp_dir, db) = unsafe { setup_test_db() };
    
    // Try to get a nonexistent checksum
    let result = db.get_checksum("nonexistent_checksum", "nonexistent_key");
    assert!(result.is_ok(), "Get checksum should succeed even for nonexistent keys");
    
    let checksum_opt = result.unwrap();
    assert!(checksum_opt.is_none(), "Checksum should not exist");
}

#[test]
#[serial]
fn test_database_insert_update() {
    let (_temp_dir, mut db) = unsafe { setup_test_db() };
    
    // Insert a value
    let schema_identifier = "test_schema";
    let key = "test_key";
    let value1 = b"test_value1";
    
    let result = db.insert(schema_identifier, key, value1, None, None);
    assert!(result.is_ok(), "Insert should succeed");
    
    // Update the value
    let value2 = b"test_value2";
    let result = db.insert(schema_identifier, key, value2, None, None);
    assert!(result.is_ok(), "Update should succeed");
    
    // Get the updated value
    let result = db.get(schema_identifier, key);
    assert!(result.is_ok(), "Get should succeed");
    
    let value_opt = result.unwrap();
    assert!(value_opt.is_some(), "Value should exist");
    
    let retrieved_value = value_opt.unwrap();
    assert_eq!(retrieved_value, value2, "Retrieved value should match updated value");
}