use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to build connection: {0}")]
    FailedBuildConnection(zbus::Error),
    #[error("Failed to register object: {0}")]
    FailedRegisterObject(zbus::Error),
    #[error("Failed to start dbus server: {0}")]
    FailedStartDBusServer(zbus::Error),
}