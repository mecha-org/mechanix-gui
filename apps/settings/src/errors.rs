use std::fmt;

use tracing::error;

/// # Lock screen error codes
///
/// Implements standard errors for the lock screen
#[derive(Debug, Default, Clone, Copy)]
pub enum LockScreenErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    FindLoginManagerUrlError,
    LoginManagerStreamConnectError,
    StreamWriteUsernameError,
    StreamReadEnterPasswordError,
    StreamWritePasswordError,
    StreamReadCaptchaError,
    StreamWriteCaptchaError,
    StreamReadAuthResponseError,
}

impl fmt::Display for LockScreenErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LockScreenErrorCodes::UnknownError => write!(f, "UnknownError"),
            LockScreenErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            LockScreenErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            LockScreenErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            LockScreenErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            LockScreenErrorCodes::FindLoginManagerUrlError => write!(f, "FindLoginManagerUrlError"),
            LockScreenErrorCodes::LoginManagerStreamConnectError => {
                write!(f, "LoginManagerStreamConnectError")
            }
            LockScreenErrorCodes::StreamWriteUsernameError => {
                write!(f, "StreamWriteUsernameError")
            }
            LockScreenErrorCodes::StreamReadEnterPasswordError => {
                write!(f, "StreamReadEnterPasswordError")
            }
            LockScreenErrorCodes::StreamWritePasswordError => {
                write!(f, "StreamWritePasswordError")
            }
            LockScreenErrorCodes::StreamReadCaptchaError => {
                write!(f, "StreamReadCaptchaError")
            }
            LockScreenErrorCodes::StreamWriteCaptchaError => {
                write!(f, "StreamWriteCaptchaError")
            }
            LockScreenErrorCodes::StreamReadAuthResponseError => {
                write!(f, "StreamReadAuthResponseError")
            }
        }
    }
}

/// # LockScreenError
///
/// Implements a standard error type for all lock screen related errors
/// includes the error code (`LockScreenErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct LockScreenError {
    pub code: LockScreenErrorCodes,
    pub message: String,
}

impl LockScreenError {
    pub fn new(code: LockScreenErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for LockScreenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
