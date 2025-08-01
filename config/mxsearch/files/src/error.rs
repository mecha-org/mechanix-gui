use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileSearchError {
    #[error("Failed to build connection: {0}")]
    FailedBuildConnection(zbus::Error),
}
