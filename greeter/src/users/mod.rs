use crate::errors::{GreeterError, GreeterErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub name: Option<String>,
    pub pin_enabled: Option<bool>,
    pub avatar: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            name: None,
            pin_enabled: None,
            avatar: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsersSettings {
    pub users: Vec<User>,
}

impl Default for UsersSettings {
    fn default() -> Self {
        Self { users: vec![] }
    }
}

pub fn read_users_yml(path: &str) -> Result<UsersSettings> {
    let mut file_path = PathBuf::from(path); // Get path of the library

    info!(task = "read_users_yml", "file location - {:?}", file_path);

    // open file
    let file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(GreeterError::new(
                GreeterErrorCodes::UsersSettingsReadError,
                format!("cannot read the users.yml in the path - {}", e),
            ));
        }
    };

    // read and parse
    let users_settings: UsersSettings = match serde_yaml::from_reader(file_handle) {
        Ok(user_settings) => user_settings,
        Err(e) => {
            bail!(GreeterError::new(
                GreeterErrorCodes::UsersSettingsParseError,
                format!("error parsing the users.yml - {}", e),
            ));
        }
    };

    Ok(users_settings)
}
