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
    pub pin_enabled: bool,
    pub avatar: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            name: None,
            pin_enabled: false,
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

/// # Reads Users path from arg
///
/// Reads the `-u` or `--users` argument for the path
pub fn read_users_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 && (args[3] == "-u" || args[3] == "--users") {
        debug!("Using settings path from argument - {}", args[2]);
        return Some(String::from(args[4].clone()));
    }
    None
}

pub fn read_users_yml() -> Result<UsersSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_GREETER_USERS_PATH").unwrap_or(String::from("users.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_users_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

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
