use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use mxconf::utils::read_application_schema;

#[test]
fn test_read_application_schema_valid() {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temporary directory");
    let file_path = dir.path().join("test_schema.toml");
    let file_path_str = file_path.to_str().unwrap();

    // Create a test TOML file
    let test_content = r#"
[test]
name = "test_schema"
version = "1.0.0"

[settings.example]
type = "string"
default = "example value"
description = "An example setting"
"#;

    let mut file = File::create(&file_path).expect("Failed to create test file");
    file.write_all(test_content.as_bytes()).expect("Failed to write to test file");

    // Test reading the file
    let result = read_application_schema(file_path_str);
    assert!(result.is_ok(), "Failed to read application schema: {:?}", result.err());

    let content = result.unwrap();
    assert_eq!(content, test_content, "File content does not match expected content");
}

#[test]
fn test_read_application_schema_nonexistent_file() {
    // Test reading a file that doesn't exist
    let result = read_application_schema("nonexistent_file.toml");
    assert!(result.is_err(), "Expected error when reading nonexistent file");

    // Verify the error message contains information about the file
    let err = result.err().unwrap();
    let err_string = err.to_string();
    assert!(err_string.contains("Unable to open file"), 
            "Error message does not contain expected text: {}", err_string);
    assert!(err_string.contains("nonexistent_file.toml"), 
            "Error message does not contain the file name: {}", err_string);
}
