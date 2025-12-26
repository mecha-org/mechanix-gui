use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("D-Bus error: `{0}`")]
    DbusString(String),
    #[error("Receiver error: `{0}`")]
    Receiver(#[from] std::sync::mpsc::RecvError),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;