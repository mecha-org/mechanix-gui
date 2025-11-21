use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to build connection: {0}")]
    FailedBuildConnection(zbus::Error),
    #[error("Failed to register object: {0}")]
    FailedRegisterObject(zbus::Error),
    #[error("Failed to start app search service: {0}")]
    FailedStartAppSearchService(anyhow::Error),
    #[error("Failed to start file search service: {0}")]
    FailedStartFileSearchService(anyhow::Error),
    #[error("Failed to start app actions service: {0}")]
    FailedStartAppActionsService(anyhow::Error),
    #[error("Failed to start dbus server: {0}")]
    FailedStartDBusServer(zbus::Error),
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Failed to build connection: {0}")]
    CreateProxyError(String),
}
