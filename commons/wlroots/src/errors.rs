use std::fmt;

use tracing::error;

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum WlrootsErrorCodes {
    #[default]
    UnknownError,
    ConnectWaylandServerError,
    ConnectionError,
    WaylandCompositorSupportError,
    WaylandServerFlushingError,
    ToplevelManagerFinishedError,
}

impl fmt::Display for WlrootsErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WlrootsErrorCodes::UnknownError => write!(f, "UnknownError"),
            WlrootsErrorCodes::ConnectWaylandServerError => write!(f, "ConnectWaylandServerError"),
            WlrootsErrorCodes::ConnectionError => write!(f, "ConnectionError"),
            WlrootsErrorCodes::WaylandCompositorSupportError => {
                write!(f, "WaylandCompositorSupportError")
            }
            WlrootsErrorCodes::WaylandServerFlushingError => {
                write!(f, "WaylandServerFlushingError")
            }
            WlrootsErrorCodes::ToplevelManagerFinishedError => {
                write!(f, "ToplevelManagerFinishedError")
            }
        }
    }
}

/// # WlrootsError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`WlrootsErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct WlrootsError {
    pub code: WlrootsErrorCodes,
    pub message: String,
}

impl WlrootsError {
    pub fn new(code: WlrootsErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for WlrootsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
