use std::fmt;

use tracing::error;

/// # Greeter error codes
///
/// Implements standard errors for the Greeter
#[derive(Debug, Default, Clone, Copy)]
pub enum GreeterErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    UsersSettingsReadError,
    UsersSettingsParseError,
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

impl fmt::Display for GreeterErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GreeterErrorCodes::UnknownError => write!(f, "UnknownError"),
            GreeterErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            GreeterErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            GreeterErrorCodes::UsersSettingsReadError => write!(f, "UsersSettingsReadError"),
            GreeterErrorCodes::UsersSettingsParseError => {
                write!(f, "UsersSettingsParseError")
            }
            GreeterErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            GreeterErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            GreeterErrorCodes::FindLoginManagerUrlError => {
                write!(f, "FindLoginManagerUrlError")
            }
            GreeterErrorCodes::LoginManagerStreamConnectError => {
                write!(f, "LoginManagerStreamConnectError")
            }
            GreeterErrorCodes::StreamWriteUsernameError => {
                write!(f, "StreamWriteUsernameError")
            }
            GreeterErrorCodes::StreamReadEnterPasswordError => {
                write!(f, "StreamReadEnterPasswordError")
            }
            GreeterErrorCodes::StreamWritePasswordError => {
                write!(f, "StreamWritePasswordError")
            }
            GreeterErrorCodes::StreamReadCaptchaError => {
                write!(f, "StreamReadCaptchaError")
            }
            GreeterErrorCodes::StreamWriteCaptchaError => {
                write!(f, "StreamWriteCaptchaError")
            }
            GreeterErrorCodes::StreamReadAuthResponseError => {
                write!(f, "StreamReadAuthResponseError")
            }
        }
    }
}

/// # GreeterError
///
/// Implements a standard error type for all Greeter related errors
/// includes the error code (`GreeterErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct GreeterError {
    pub code: GreeterErrorCodes,
    pub message: String,
}

impl GreeterError {
    pub fn new(code: GreeterErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for GreeterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
