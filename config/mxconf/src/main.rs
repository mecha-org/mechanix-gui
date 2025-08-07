mod cli;
mod database;
mod error;
mod server;
mod utils;
mod validator;

use crate::cli::{
    describe_key, get_setting_table, list_keys, list_schemas, set_setting_table, watch_setting,
};
use crate::error::ServerError;
use crate::server::{ConfigServerInterface, SERVED_AT};
use crate::validator::validate_schema;
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use dirs::home_dir;
use log::{debug, error, info, trace, warn};
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use zbus::ConnectionBuilder;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    keystore: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct System {
    keyfiles: String,
}
/// CLI for mxconf
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Start the DBus server
    #[arg(short, long)]
    start_server: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a setting value
    Get {
        /// The key to get
        key: String,
    },
    /// Set a setting value
    Set {
        /// The key to set
        key: String,
        /// The value to set
        value: String,
    },
    /// Watch for changes to a key
    Watch {
        /// The schema to watch
        schema: String,
        /// The key to watch
        key: Option<String>,
    },
    /// List all available schemas
    ListSchemas,

    /// List Keys in schema
    ListKeys {
        /// The schema to list keys from
        schema: String,
    },

    /// The schema and key to describe
    Describe { schema: String, key: String },
}
const CHECKSUM_TREE_NAME: &str = "schema_checksum";
const CONNECTION_BUS_NAME: &str = "org.mechanix.MxConf";
const SCHEMA_DIR: &str = "/usr/share/mxconf/schemas";
const DEFAULT_PROFILE_PATH: &str = "/etc/mxconf/profile/default.toml";
const KEY_FILE_DIR: &str = "/etc/mxconf/keyfiles";
const DB_PATH: &str = ".config/mxconf";

/// Process a TOML file by validating it and storing it in the database
///
/// # Arguments
///
/// * `path` - The path to the TOML file
/// * `db` - The database instance to store the validated file
///
/// # Returns
///
/// * `Ok(())` if the file was processed successfully
/// * `Err(...)` if there was an error during processing
fn process_toml_file(path: &PathBuf, db: &mut database::Database) -> Result<()> {
    info!("Processing TOML file: {}", path.display());
    let application_schema_str = utils::read_application_schema(&path.display().to_string())?;

    // Extract the filename to use as namespace
    let schema_file_name = path
        .file_name()
        .context("Failed to get filename")?
        .to_str()
        .context("Failed to convert filename to string")?;

    validator::validate_schema_name(schema_file_name)?;
    let schema_toml: toml::Value = application_schema_str
        .parse()
        .context("Unable to parse TOML")?;
    let schema_checksum = validator::generate_checksum(&schema_file_name, &schema_toml)
        .context("Unable to generate checksum")?;

    // Check if file already exists with same checksum
    if let Some(existing_checksum) = db.get_checksum(CHECKSUM_TREE_NAME, schema_file_name)? {
        if schema_checksum == existing_checksum {
            info!("TOML file already exists with same checksum");
            return Ok(());
        }
    }

    // Validate the application schema file
    match validate_schema(&schema_toml).map_err(|err| anyhow::anyhow!("Validation error: {}", err))
    {
        Ok(()) => {
            trace!("Validation successful");
        }
        Err(e) => return Err(e),
    };

    // Insert the validated file and checksum into the database
    match db.insert_checksum(schema_file_name, CHECKSUM_TREE_NAME, &schema_checksum) {
        Ok(()) => (),
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to insert TOML file into database: {}",
                e
            ));
        }
    }

    println!("Checksum inserted into database");
    Ok(())
}

/// Handle a file system event
///
/// # Arguments
///
/// * `event` - The file system event to handle
/// * `db` - The database instance to store validated files
///
/// # Returns
///
/// * `Ok(())` if the event was handled successfully
/// * `Err(...)` if there was an error during handling
fn handle_event(event: Event, db: &mut database::Database) -> Result<()> {
    debug!("Received event: {:?}", event.kind);
    // Only process file creation events
    if event.kind.is_create() || event.kind.is_modify() {
        debug!("Event received: {:?}", event.kind);
        // Process each path in the event
        for path in &event.paths {
            // Only process TOML files
            if path.extension() == Some("toml".as_ref()) {
                if let Err(err) = process_toml_file(path, db) {
                    error!("Failed to process TOML file: {}", err);
                }
            }
        }
    }

    Ok(())
}


/// Load the profile, which contains user and system settings
mod profile {
    use crate::error::ProfileError;
    use crate::{System, User, DEFAULT_PROFILE_PATH};
    use log::{debug, error, info};
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Profile {
        pub user: User,
        pub system: System,
    }

    pub fn load_profile() -> Result<Profile, ProfileError> {
        info!("Loading profile...");
        let profile_path =
            std::env::var("MXCONF_PROFILE").unwrap_or(DEFAULT_PROFILE_PATH.to_string());
        debug!("Profile path: {}", profile_path);
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let profile_path = home_dir.join(profile_path);
        let profile_file = match File::open(profile_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open profile file: {}", e);
                return Err(ProfileError::IoError(e));
            }
        };
        let file_str = match io::read_to_string(profile_file) {
            Ok(str) => str,
            Err(e) => {
                error!("Failed to read profile file: {}", e);
                return Err(ProfileError::IoError(e));
            }
        };
        let profile: Profile = match toml::from_str(&file_str) {
            Ok(profile) => profile,
            Err(e) => {
                error!("Failed to parse profile file: {}", e);
                return Err(ProfileError::TomlError(e));
            }
        };
        info!("Profile Loaded!");
        Ok(profile)
    }
}
/// Main function that sets up a file system watcher and a D-Bus server
///
/// # Returns
///
/// * `Ok(())` if the program ran successfully
/// * `Err(...)` if there was an error during execution
#[tokio::main]
async fn main() -> Result<(), ServerError> {
    env_logger::init();
    // Parse command-line arguments
    let cli = Cli::parse();

    // If no command is provided and start_server is false, print help and exit
    if cli.command.is_none() && !cli.start_server {
        match Cli::command().print_help() {
            Ok(()) => (),
            Err(e) => {
                warn!("Failed to print help: {}", e);
            }
        }
        return Ok(());
    }

    // If start_server is true or no command is provided, start the server
    if cli.start_server {
        start_server().await?;
    }

    // Handle commands
    match cli.command {
        Some(Commands::Get { key }) => {
            get_setting_table(&key).await?;
        }
        Some(Commands::Set { key, value }) => {
            set_setting_table(&key, &value).await?;
        }
        Some(Commands::Watch { schema, key }) => {
            watch_setting(&schema, &key).await?;
        }
        Some(Commands::ListSchemas) => {
            list_schemas().await?;
        }
        Some(Commands::ListKeys { schema }) => {
            list_keys(&schema).await?;
        }
        Some(Commands::Describe { schema, key }) => {
            describe_key(&schema, &key).await?;
        }
        None => {
            warn!("No command provided");
            unreachable!();
        }
    }

    Ok(())
}

/// Start the configuration server
async fn start_server() -> Result<(), ServerError> {
    info!("Starting server...");
    // Load profile
    let profile = match profile::load_profile().context("Failed to load profile") {
        Ok(profile) => profile,
        Err(e) => {
            error!("Failed to load profile: {}", e);
            return Err(ServerError::FailedLoadProfile(e));
        }
    };
    let home_dir = home_dir().expect("Failed to get home directory");
    let db_path = home_dir.join(DB_PATH).join(profile.user.keystore);
    let schema_dir = home_dir.join(SCHEMA_DIR);
    let key_file_dir = home_dir.join(KEY_FILE_DIR).join(profile.system.keyfiles);
    debug!(
        "Database path: {}, Schema directory: {}",
        db_path.display(),
        schema_dir.display()
    );
    // Initialize database
    let db = Arc::new(Mutex::new(database::Database::new(db_path)));

    // Build the connection first
    let conn = match ConnectionBuilder::session() {
        Ok(builder) => match builder.name(CONNECTION_BUS_NAME) {
            Ok(named_builder) => match named_builder.build().await {
                Ok(conn) => conn,
                Err(e) => return Err(ServerError::FailedBuildConnection(e)),
            },
            Err(e) => return Err(ServerError::FailedBuildConnection(e)),
        },
        Err(e) => return Err(ServerError::FailedBuildConnection(e)),
    };

    info!("D-Bus connection built");

    // Now build the server struct with the connection
    let config_server = ConfigServerInterface {
        db: Arc::clone(&db),
        conn: conn.clone(),
        key_file_dir,
        schema_dir,
    };

    // Register the object
    match conn.object_server().at(SERVED_AT, config_server).await {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to register object at {}: {}", SERVED_AT, e);
            return Err(ServerError::FailedRegisterObject(e));
        }
    }

    info!("D-Bus server registered at {}", SERVED_AT);

    // Set up a file system watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher = match recommended_watcher(tx).context("Failed to create file watcher") {
        Ok(watcher) => watcher,
        Err(e) => {
            error!("Failed to create file watcher: {}", e);
            return Err(ServerError::DirWatcherFailed(e.to_string()));
        }
    };

    let schema_dir = std::env::var("MXCONF_SCHEMA_DIR").unwrap_or(SCHEMA_DIR.to_string());
    let schema_dir_to_watch = home_dir.join(schema_dir);
    // Watch the schemas directory for changes
    let schemas_dir = Path::new(&schema_dir_to_watch);
    match watcher.watch(schemas_dir, RecursiveMode::Recursive) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to watch schemas directory: {}", e);
            return Err(ServerError::DirWatcherFailed(e.to_string()));
        }
    };
    debug!(
        "Watching schemas directory: {}",
        schema_dir_to_watch.display()
    );
    // Process events as they come in
    while let Ok(event_result) = rx.recv() {
        match event_result {
            Ok(event) => {
                let mut db_guard = match db.lock() {
                    Ok(guard) => guard,
                    Err(e) => {
                        error!("Failed to lock database: {}", e);
                        continue;
                    }
                };
                if let Err(err) = handle_event(event, &mut db_guard) {
                    error!("Error handling event: {}", err);
                }
            }
            Err(err) => error!("Error receiving event: {}", err),
        }
    }
    Ok(())
}
