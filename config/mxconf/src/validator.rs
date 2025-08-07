use crate::error::ValidatorError;
use anyhow::Context;
use log::{debug, error, info, trace, warn};
use regex::Regex;
use toml::Value;

const ALLOWED_TYPES: &[&str] = &["string", "number", "bool", "array", "tuple", "object", "enum"];

/// Validate a TOML schema against predefined rules.
///
/// This function checks that each section and key in the given TOML schema
/// adheres to specified constraints. It ensures that all entries have valid
/// types and defaults, and performs type-specific validations such as checking
/// for enumeration options and numeric bounds.
///
/// # Arguments
///
/// * `toml_file` - A reference to a TOML value representing the schema to be validated.
///
/// # Returns
///
/// * `Ok(())` if the schema is valid.
/// * `Err(message)` if the schema is invalid, with a descriptive error message.
///
/// # Errors
///
/// This function returns an error if:
/// - The root of the TOML is not a table.
/// - A section or key is not a table.
/// - A type is missing, invalid, or not in the allowed types.
/// - A default value is missing.
/// - Type-specific constraints are not met, such as:
///   - Non-integer `max_length` for strings.
///   - Non-numeric `min` or `max` for numbers.
///   - An empty or invalid `options` array for enums, or a default not in options.
///

pub fn validate_schema(toml_file: &Value) -> Result<(), ValidatorError> {
    let table = as_table(toml_file, "root")?;

    for (section_name, section_val) in table {
        let section = as_table(section_val, &format!("section '{}'", section_name))?;

        for (key, val) in section {
            let entry = as_table(val, &format!("key '{}' in section '{}'", key, section_name))?;

            debug!("Validating key '{}' in section '{}'", key, section_name);

            let type_val = get_required_str(entry, "type", &key, &section_name)?;
            validate_type(type_val, &key, &section_name)?;

            if !entry.contains_key("default") {
                return Err(missing_field_error("default", key, section_name));
            }

            if !entry.contains_key("description") {
                return Err(missing_field_error("description", key, section_name));
            }

            match type_val {
                "string" => validate_string_type(entry, &key, &section_name)?,
                "number" => validate_number_type(entry, &key, &section_name)?,
                "enum" => validate_enum_type(entry, &key, &section_name)?,
                _ => {} // If new types are added, define new validation
            }
        }
    }

    info!("Schema validation successful");
    Ok(())
}

//-------------------------
// Helper Functions
//-------------------------

/// Convert a TOML value to a table reference.
///
/// This function attempts to treat the given TOML value as a table. If the value is not a table,
/// it returns an error indicating an invalid schema type.
///
/// # Arguments
///
/// * `value` - A reference to the TOML value to be converted.
/// * `context` - A string representing the context in which this conversion is occurring, used for error reporting.
///
/// # Returns
///
/// * `Ok(&toml::value::Table)` - A reference to the TOML table if the conversion is successful.
/// * `Err(ValidatorError)` - An error indicating that the value is not a table.
///
/// # Errors
///
/// This function returns an `InvalidSchemaType` error if the provided value is not a table.

fn as_table<'a>(value: &'a Value, _context: &str) -> Result<&'a toml::value::Table, ValidatorError> {
    value.as_table().ok_or_else(|| {
        ValidatorError::InvalidSchemaType
    })
}

fn get_required_str<'a>(entry: &'a toml::value::Table, field: &str, key: &str, section: &str) -> Result<&'a str, ValidatorError> {
    entry.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| ValidatorError::ValidationError(format!(
            "Key '{}' in section '{}' missing or invalid '{}'", key, section, field
        )))
}

fn missing_field_error(field: &str, key: &str, section: &str) -> ValidatorError {
    ValidatorError::ValidationError(format!(
        "Key '{}' in section '{}' missing '{}'", key, section, field
    ))
}

fn validate_type(type_val: &str, key: &str, section: &str) -> Result<(), ValidatorError> {
    if type_val.trim().is_empty() {
        return Err(ValidatorError::ValidationError(format!(
            "Key '{}' in section '{}' has empty 'type'", key, section,
        )));
    }
    if !ALLOWED_TYPES.contains(&type_val) {
        return Err(ValidatorError::ValidationError(format!(
            "Key '{}' in section '{}' has invalid 'type': '{}'", key, section, type_val,
        )));
    }
    Ok(())
}

fn validate_string_type(entry: &toml::value::Table, key: &str, section: &str) -> Result<(), ValidatorError> {
    if let Some(max_length) = entry.get("max_length") {
        if !max_length.is_integer() {
            return Err(ValidatorError::ValidationError(format!(
                "Key '{}' in section '{}' has non-integer 'max_length'", key, section,
            )));
        }
    }
    Ok(())
}

fn validate_number_type(entry: &toml::value::Table, key: &str, section: &str) -> Result<(), ValidatorError> {
    for field in ["min", "max"] {
        if let Some(val) = entry.get(field) {
            if !val.is_integer() && !val.is_float() {
                return Err(ValidatorError::ValidationError(format!(
                    "Key '{}' in section '{}' has non-numeric '{}'", key, section, field,
                )));
            }
        }
    }
    Ok(())
}

fn validate_enum_type(entry: &toml::value::Table, key: &str, section: &str) -> Result<(), ValidatorError> {
    let options = match entry.get("options") {
        Some(Value::Array(options)) => options,
        _ => {
            return Err(ValidatorError::ValidationError(format!(
                "Key '{}' in section '{}' is enum but missing valid 'options' array", key, section,
            )));
        }
    };

    if options.is_empty() {
        return Err(ValidatorError::ValidationError(format!(
            "Key '{}' in section '{}' is enum but 'options' array is empty", key, section,
        )));
    }

    let default = get_required_str(entry, "default", key, section)?;
    let found = options.iter().any(|opt| opt.as_str() == Some(default));
    if !found {
        return Err(ValidatorError::ValidationError(format!(
            "Key '{}' in section '{}' has default '{}' not in options {:?}", key, section, default, options,
        )));
    }

    Ok(())
}


/// Validate a new setting with a given value, using the schema for the namespace.
///
/// This function validates that a setting value conforms to the type and constraints
/// defined in the schema. It extracts the schema entry for the given namespace,
/// determines the expected type, and performs type-specific validation.
///
/// # Arguments
///
/// * `schema` - The schema TOML to validate against
/// * `namespace` - The namespace of the setting (e.g., "org.mechanix.app.section.key")
/// * `value` - The string value to validate
///
/// # Returns
///
/// * `Ok(())` if the value is valid according to the schema
/// * `Err(message)` if the value is invalid, with a descriptive error message
///
/// # Errors
///
/// This function returns an error if:
/// - The schema entry for the namespace is not found
/// - The schema entry is not a table
/// - The type is not specified in the schema
/// - The value does not conform to the specified type
/// - For enums, the value is not in the options list
pub fn validate_setting(schema: &toml::Value, namespace: &str, value: &str) -> Result<(), String> {
    debug!("Validating setting '{}' with value '{}'", namespace, value);

    // This function is similar to validate_value but with a different entry point
    // Both functions extract a schema entry and validate a value against it

    let parts: Vec<&str> = namespace.split('.').skip(3).collect();
    let entry = get_schema_entry(schema, &parts)
        .ok_or_else(|| format!("Schema for '{}' not found", namespace))?;
    let entry_table = entry.as_table().ok_or("Schema entry is not a table")?;
    let type_str = entry_table.get("type")
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
        "number" | "int" | "float" => {
            if value.parse::<f64>().is_err() {
                return Err(format!("Value '{}' is not a valid number", value));
            }
        }
        "enum" => {
            let options = entry_table.get("options")
                .and_then(|v| v.as_array())
                .ok_or("Enum options not specified")?;
            let found = options.iter().any(|opt| opt.as_str() == Some(value));
            if !found {
                return Err(format!("Value '{}' is not in enum options", value));
            }
        }
        // Add other types as needed
        _ => return Err(format!("Unknown type '{}'", type_str)),
    }
    info!("Setting '{}' validated successfully", namespace);
    Ok(())
}

/// Validate a value against a schema entry.
///
/// This function validates that a value conforms to the type and constraints
/// defined in the schema. It extracts the schema entry for the given namespace,
/// determines the expected type, and performs type-specific validation.
///
/// # Arguments
///
/// * `schema_entry` - The schema TOML to validate against
/// * `namespace` - The namespace of the setting (e.g., "org.mechanix.app.section.key")
/// * `value` - The string value to validate
///
/// # Returns
///
/// * `Ok(())` if the value is valid according to the schema
/// * `Err(message)` if the value is invalid, with a descriptive error message
///
/// # Errors
///
/// This function returns an error if:
/// - The schema entry for the namespace is not found
/// - The type is not specified in the schema
/// - The value does not conform to the specified type
/// - For enums, the value is not in the options list
pub fn validate_value(schema_entry: &toml::Value, namespace: &str, value: &str) -> Result<(), String> {
    debug!("Validating value '{}' for namespace '{}'", value, namespace);

    let parts: Vec<&str> = namespace.split('.').skip(3).collect();
    let entry = get_schema_entry(schema_entry, &parts)
        .ok_or_else(|| format!("Schema for '{}' not found", namespace))?;

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
    info!("Value '{}' validated successfully for namespace '{}'", value, namespace);
    Ok(())
}

/// Get a schema entry from a path.
///
/// This function traverses a TOML value using a path of keys to find a specific entry.
/// It's used to extract schema entries for validation.
///
/// # Arguments
///
/// * `schema` - The TOML value to traverse
/// * `path` - A slice of string slices representing the path to the entry
///
/// # Returns
///
/// * `Some(&toml::Value)` if the entry is found
/// * `None` if any part of the path doesn't exist
fn get_schema_entry<'a>(schema: &'a toml::Value, path: &[&str]) -> Option<&'a toml::Value> {
    trace!("Getting schema entry for path: {:?}", path);
    let mut current = schema;
    for key in path {
        println!("key: {}", key);
        current = current.get(*key)?;
    }
    Some(current)
}

/// Validate a schema name against a predefined pattern.
///
/// This function checks if the given schema name matches the expected pattern
/// for schema files. The pattern requires the schema name to be in the format
/// `org.<domain>.<app>.toml`, where `<domain>` and `<app>` are alphanumeric
/// strings with underscores allowed.
///
/// # Arguments
///
/// * `schema_name` - A string slice representing the schema file name.
///
/// # Returns
///
/// * `Ok(())` if the schema name is valid.
/// * `Err(anyhow::Error)` if the schema name does not match the expected pattern.
///
/// # Errors
///
/// This function returns an error if:
/// - The schema name does not conform to the pattern `org.<domain>.<app>.toml`.
pub fn validate_schema_name(schema_name: &str) -> anyhow::Result<(), ValidatorError> {
    debug!("Validating schema name: {}", schema_name);

    // Corrected regex!
    let re = Regex::new(r"^org\.([a-zA-Z0-9_]+)\.([a-zA-Z0-9_]+)\.toml$")
        .map_err(ValidatorError::RegexError)?;

    if !re.is_match(schema_name) {
        error!("Invalid schema name: {}", schema_name);
        return Err(ValidatorError::InvalidSchemaName(schema_name.to_string()));
    }
    Ok(())
}

pub fn generate_checksum(namespace: &str, schema_toml: &Value) -> anyhow::Result<u32> {
    debug!("Generating checksum for namespace: {}", namespace);
    // Convert to JSON
    let application_schema =
        serde_json::to_value(&schema_toml).context("Unable to convert TOML to JSON")?;
    let application_schema_bytes =
        serde_json::to_vec(&application_schema).context("Unable to convert JSON to bytes")?;
    // Generate checksum
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(namespace.as_bytes());
    hasher.update(&application_schema_bytes);
    let checksum = hasher.finalize();
    info!("Generated checksum: {:08x} for namespace: {}", checksum, namespace);
    Ok(checksum)
}