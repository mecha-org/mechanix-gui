use mxconf::database::Database;
use serial_test::serial;
use tempfile::TempDir;

// Helper function to set up a test database in a temporary directory
fn setup_test_db() -> (TempDir, Database) {
    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a database path within the temporary directory
    let db_path = temp_dir.path().join("test_db");

    // Create a new database instance
    let db = Database::new(db_path);

    (temp_dir, db)
}

#[test]
#[serial]
fn test_database_new() {
    let (temp_dir, _db) = setup_test_db();

    // Check that the database directory was created
    let db_path = temp_dir.path().join("test_db");
    assert!(db_path.exists(), "Database directory should exist");
}

#[test]
#[serial]
fn test_database_insert_settings_and_get() {
    let (_temp_dir, mut db) = setup_test_db();

    // Insert a value
    let schema_identifier = "test_schema";
    let key = "test_key";
    let value = b"test_value";

    let result = db.insert_settings(schema_identifier, key, value);
    assert!(result.is_ok(), "Insert settings should succeed");

    // Get the value
    let result = db.get(schema_identifier, key);
    assert!(result.is_ok(), "Get should succeed");

    let values = result.unwrap();
    assert!(!values.is_empty(), "Values should not be empty");
    assert!(values.contains_key(key), "Values should contain the key");
    assert_eq!(values.get(key).unwrap(), &String::from_utf8_lossy(value).to_string(),
               "Retrieved value should match inserted value");
}

#[test]
#[serial]
fn test_database_get_nonexistent() {
    let (_temp_dir, db) = setup_test_db();

    // Try to get a nonexistent value
    let result = db.get("nonexistent_schema", "nonexistent_key");
    assert!(result.is_ok(), "Get should succeed even for nonexistent keys");

    let values = result.unwrap();
    assert!(values.is_empty(), "Values should be empty for nonexistent key");
}

#[test]
#[serial]
fn test_database_insert_checksum_and_get() {
    let (_temp_dir, mut db) = setup_test_db();

    // Insert a checksum
    let schema_name = "test_schema";
    let checksum_identifier = "test_checksum";
    let checksum_value = 12345u32;

    let result = db.insert_checksum(schema_name, checksum_identifier, &checksum_value);
    assert!(result.is_ok(), "Insert checksum should succeed");

    // Get the checksum
    let result = db.get_checksum(checksum_identifier, schema_name);
    assert!(result.is_ok(), "Get checksum should succeed");

    let checksum_opt = result.unwrap();
    assert!(checksum_opt.is_some(), "Checksum should exist");

    let retrieved_checksum = checksum_opt.unwrap();
    assert_eq!(retrieved_checksum, checksum_value, "Retrieved checksum should match inserted checksum");
}

#[test]
#[serial]
fn test_database_scan_with_prefix() {
    let (_temp_dir, mut db) = setup_test_db();

    // Insert multiple values with a common prefix
    let schema_identifier = "test_schema";
    let prefix = "prefix_";
    let keys = [
        format!("{}{}", prefix, "key1"),
        format!("{}{}", prefix, "key2"),
        format!("{}{}", prefix, "key3")
    ];
    let value = b"test_value";

    for key in &keys {
        let result = db.insert_settings(schema_identifier, key, value);
        assert!(result.is_ok(), "Insert settings should succeed");
    }

    // Also insert a key without the prefix
    let non_prefix_key = "different_key";
    let result = db.insert_settings(schema_identifier, non_prefix_key, value);
    assert!(result.is_ok(), "Insert settings should succeed");

    // Scan with prefix
    let result = db.scan_with_prefix(schema_identifier, prefix);
    assert!(result.is_ok(), "Scan with prefix should succeed");

    let values = result.unwrap();
    assert_eq!(values.len(), keys.len(), "Should find all keys with the prefix");

    // Check that all keys with the prefix are in the result
    for key in &keys {
        assert!(values.contains_key(key), "Values should contain the key with prefix");
    }

    // Check that the key without the prefix is not in the result
    assert!(!values.contains_key(non_prefix_key), "Values should not contain the key without prefix");
}

#[test]
#[serial]
fn test_database_get_nonexistent_checksum() {
    let (_temp_dir, db) = setup_test_db();

    // Try to get a nonexistent checksum
    let result = db.get_checksum("nonexistent_checksum", "nonexistent_key");
    assert!(result.is_ok(), "Get checksum should succeed even for nonexistent keys");

    let checksum_opt = result.unwrap();
    assert!(checksum_opt.is_none(), "Checksum should not exist");
}

#[test]
#[serial]
fn test_database_insert_settings_update() {
    let (_temp_dir, mut db) = setup_test_db();

    // Insert a value
    let schema_identifier = "test_schema";
    let key = "test_key";
    let value1 = b"test_value1";

    let result = db.insert_settings(schema_identifier, key, value1);
    assert!(result.is_ok(), "Insert settings should succeed");

    // Update the value
    let value2 = b"test_value2";
    let result = db.insert_settings(schema_identifier, key, value2);
    assert!(result.is_ok(), "Update should succeed");

    // Get the updated value
    let result = db.get(schema_identifier, key);
    assert!(result.is_ok(), "Get should succeed");

    let values = result.unwrap();
    assert!(!values.is_empty(), "Values should not be empty");
    assert!(values.contains_key(key), "Values should contain the key");
    assert_eq!(values.get(key).unwrap(), &String::from_utf8_lossy(value2).to_string(),
               "Retrieved value should match updated value");
}
