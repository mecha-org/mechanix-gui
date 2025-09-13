use log::error;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ActionSchema {
    pub name: String,
    pub icon: String,
    pub exec: String,

    #[serde(flatten)]
    pub actions: HashMap<String, ActionSetting>,
}

#[derive(Debug, Deserialize)]
pub struct ActionSetting {
    pub action: String,
    pub description: String,
    pub arg: Arg,
}

#[derive(Debug, Deserialize)]
pub struct Arg {
    pub path: String,
}

pub fn parse_action_schema(schema_path: &PathBuf) -> Option<ActionSchema> {
    // Read file contents
    let toml_str = match fs::read_to_string(schema_path) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to read a schema file: {}", err);
            return None;
        }
    };

    // Parse to struct
    let schema: ActionSchema = match toml::from_str(&toml_str) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to parse a schema file: {}", err);
            return None;
        }
    };
    Some(schema)
}
