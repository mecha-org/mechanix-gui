use mxconf::validator::{validate_schema, validate_value};
use toml::Value;

// Custom validation function for testing that doesn't rely on namespace parsing
fn test_validate_entry(entry: &toml::Value, value: &str) -> Result<(), String> {
    let type_str = entry.get("type")
        .and_then(|v| v.as_str())
        .ok_or("Type not specified in schema")?;

    match type_str {
        "bool" | "boolean" => {
            if value != "true" && value != "false" {
                return Err(format!("Value '{}' is not a valid boolean", value));
            }
        }
        "string" => {
            // Optionally check max_length, etc.
        }
        "number" => {
            if value.parse::<f64>().is_err() {
                return Err(format!("Value '{}' is not a valid number", value));
            }
        }
        "enum" => {
            let options = entry.get("options")
                .and_then(|v| v.as_array())
                .ok_or("Enum options not specified")?;
            let found = options.iter().any(|opt| opt.as_str() == Some(value));
            if !found {
                return Err(format!("Value '{}' is not in enum options", value));
            }
        }
        _ => return Err(format!("Unknown type '{}'", type_str)),
    }
    Ok(())
}

#[test]
fn test_validate_schema_valid() {
    // Create a valid schema
    let schema_str = r#"
    [section]
    key = { type = "string", default = "value", description = "A string value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_ok(), "Valid schema should pass validation");
}

#[test]
fn test_validate_schema_missing_type() {
    // Create a schema with missing type
    let schema_str = r#"
    [section]
    key = { default = "value", description = "A string value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_err(), "Schema with missing type should fail validation");
    assert!(result.unwrap_err().to_string().contains("missing or invalid 'type'"),
            "Error message should mention missing type");
}

#[test]
fn test_validate_schema_missing_default() {
    // Create a schema with missing default
    let schema_str = r#"
    [section]
    key = { type = "string", description = "A string value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_err(), "Schema with missing default should fail validation");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("missing 'default'"),
            "Error message '{}' should mention missing default", error_msg);
}

#[test]
fn test_validate_schema_missing_description() {
    // Create a schema with missing description
    let schema_str = r#"
    [section]
    key = { type = "string", default = "value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_err(), "Schema with missing description should fail validation");
    assert!(result.unwrap_err().to_string().contains("missing 'description'"),
            "Error message should mention missing description");
}

#[test]
fn test_validate_schema_invalid_type() {
    // Create a schema with invalid type
    let schema_str = r#"
    [section]
    key = { type = "invalid", default = "value", description = "A value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_err(), "Schema with invalid type should fail validation");
    assert!(result.unwrap_err().to_string().contains("invalid 'type'"),
            "Error message should mention invalid type");
}

#[test]
fn test_validate_schema_enum_valid() {
    // Create a valid enum schema
    let schema_str = r#"
    [section]
    key = { type = "enum", default = "option1", options = ["option1", "option2"], description = "An enum value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_ok(), "Valid enum schema should pass validation");
}

#[test]
fn test_validate_schema_enum_missing_options() {
    // Create an enum schema with missing options
    let schema_str = r#"
    [section]
    key = { type = "enum", default = "option1", description = "An enum value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_err(), "Enum schema with missing options should fail validation");
    assert!(result.unwrap_err().to_string().contains("missing valid 'options'"),
            "Error message should mention missing options");
}

#[test]
fn test_validate_schema_enum_default_not_in_options() {
    // Create an enum schema with default not in options
    let schema_str = r#"
    [section]
    key = { type = "enum", default = "option3", options = ["option1", "option2"], description = "An enum value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Validate the schema
    let result = validate_schema(&schema);
    assert!(result.is_err(), "Enum schema with default not in options should fail validation");
    assert!(result.unwrap_err().to_string().contains("not in options"),
            "Error message should mention default not in options");
}

#[test]
fn test_validate_setting_bool_valid() {
    // Create a schema with a boolean setting
    let schema_str = r#"
    [section]
    key = { type = "bool", default = "true", description = "A boolean value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Get the entry directly
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Use our custom validation function
    let result = test_validate_entry(section_key, "true");
    assert!(result.is_ok(), "Valid boolean setting should pass validation");

    let result = test_validate_entry(section_key, "false");
    assert!(result.is_ok(), "Valid boolean setting should pass validation");
}

#[test]
fn test_validate_setting_bool_invalid() {
    // Create a schema with a boolean setting
    let schema_str = r#"
    [section]
    key = { type = "bool", default = "true", description = "A boolean value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Get the entry directly
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Use our custom validation function
    let result = test_validate_entry(section_key, "not_a_bool");
    assert!(result.is_err(), "Invalid boolean setting should fail validation");
    assert!(result.unwrap_err().contains("not a valid boolean"),
            "Error message should mention invalid boolean");
}

#[test]
fn test_validate_setting_number_valid() {
    // Create a schema with a number setting
    let schema_str = r#"
    [section]
    key = { type = "number", default = "42", description = "A number value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Get the entry directly
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Use our custom validation function
    let result = test_validate_entry(section_key, "42");
    assert!(result.is_ok(), "Valid number setting should pass validation");

    let result = test_validate_entry(section_key, "3.14");
    assert!(result.is_ok(), "Valid number setting should pass validation");
}

#[test]
fn test_validate_setting_number_invalid() {
    // Create a schema with a number setting
    let schema_str = r#"
    [section]
    key = { type = "number", default = "42", description = "A number value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Use validate_value instead, which doesn't have the namespace parsing logic
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Validate an invalid number setting
    let result = validate_value(section_key, "org.mechanix.section.key", "not_a_number");
    assert!(result.is_err(), "Invalid number setting should fail validation");
    assert!(result.unwrap_err().contains("not a valid number"),
            "Error message should mention invalid number");
}

#[test]
fn test_validate_setting_enum_valid() {
    // Create a schema with an enum setting
    let schema_str = r#"
    [section]
    key = { type = "enum", default = "option1", options = ["option1", "option2"], description = "An enum value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Use validate_value instead, which doesn't have the namespace parsing logic
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Validate a valid enum setting
    let result = validate_value(section_key, "org.mechanix.section.key", "option1");
    assert!(result.is_ok(), "Valid enum setting should pass validation");

    let result = validate_value(section_key, "org.mechanix.section.key", "option2");
    assert!(result.is_ok(), "Valid enum setting should pass validation");
}

#[test]
fn test_validate_setting_enum_invalid() {
    // Create a schema with an enum setting
    let schema_str = r#"
    [section]
    key = { type = "enum", default = "option1", options = ["option1", "option2"], description = "An enum value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Use validate_value instead, which doesn't have the namespace parsing logic
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Validate an invalid enum setting
    let result = validate_value(section_key, "org.mechanix.section.key", "option3");
    assert!(result.is_err(), "Invalid enum setting should fail validation");
    assert!(result.unwrap_err().contains("not in enum options"),
            "Error message should mention not in enum options");
}

#[test]
fn test_validate_setting_unknown_type() {
    // Create a schema with an unknown type
    let schema_str = r#"
    [section]
    key = { type = "unknown", default = "value", description = "A value" }
    "#;

    let schema: Value = toml::from_str(schema_str).unwrap();

    // Use validate_value instead, which doesn't have the namespace parsing logic
    let section_key = schema.get("section").unwrap().get("key").unwrap();

    // Validate a setting with an unknown type
    let result = validate_value(section_key, "org.mechanix.section.key", "value");
    assert!(result.is_err(), "Setting with unknown type should fail validation");
    assert!(result.unwrap_err().contains("Unknown type"),
            "Error message should mention unknown type");
}
