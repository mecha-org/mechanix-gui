use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Cli Error: {0}")]
    CliError(CliError),
    #[error("Failed to watch schema directory: {0}")]
    DirWatcherFailed(String),
    #[error("Failed to load profile: {0}")]
    FailedLoadProfile(anyhow::Error),
    #[error("Failed to build connection: {0}")]
    FailedBuildConnection(zbus::Error),
    #[error("Failed to register object: {0}")]
    FailedRegisterObject(zbus::Error),
}

impl From<CliError> for ServerError {
    fn from(err: CliError) -> Self {
        ServerError::CliError(err)
    }
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Failed to get settings: {0}")]
    FailedToGetSetting(anyhow::Error),
    #[error("Failed to set settings: {0}")]
    FailedToSetSetting(anyhow::Error),
    #[error("Failed to connect to D-Bus session: {0}")]
    FailedToConnectToDBus(zbus::Error),
    #[error("Failed to create ConfigServer proxy: {0}")]
    FailedToCreateProxy(zbus::Error),
    #[error("Failed to create ConfigServer stream: {0}")]
    FailedToCreateStream(zbus::Error),
    #[error("Failed to list schemas: {0}")]
    FailedToListSchemas(zbus::Error),
    #[error("Failed to parse schema: {0}")]
    FailedToParseJSON(serde_json::Error),
    #[error("Failed to list keys: {0}")]
    FailedToListKeys(zbus::Error),
    #[error("Failed to describe key: {0}")]
    FailedToDescribeKey(zbus::Error),
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("io error: {0}")]
    IoError(std::io::Error),
    #[error("toml error: {0}")]
    TomlError(toml::de::Error),
}

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("Validator error: {0}")]
    ValidationError(String),
    #[error("Invalid schema name: {0}")]
    InvalidSchemaName(String),
    #[error("Regex error: {0}")]
    RegexError(regex::Error),
    #[error("Invalid schema type")]
    InvalidSchemaType,
}