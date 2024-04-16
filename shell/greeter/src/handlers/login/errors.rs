use std::fmt;

use tracing::error;

/// # Login Handler error codes
///
/// Implements standard errors for the Login Handler
#[derive(Debug, Default, Clone, Copy)]
pub enum LoginHandlerErrorCodes {
    #[default]
    FindLoginManagerUrlError,
    LoginManagerStreamConnectError,
    StreamWriteError,
    StreamReadError,
    StreamWriteUsernameError,
    StreamReadEnterPasswordError,
    StreamWritePasswordError,
    StreamReadCaptchaError,
    StreamWriteCaptchaError,
    StreamReadAuthResponseError,
    LoginError,
}

impl fmt::Display for LoginHandlerErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoginHandlerErrorCodes::FindLoginManagerUrlError => {
                write!(f, "FindLoginManagerUrlError")
            }
            LoginHandlerErrorCodes::LoginManagerStreamConnectError => {
                write!(f, "LoginManagerStreamConnectError")
            }
            LoginHandlerErrorCodes::StreamWriteUsernameError => {
                write!(f, "StreamWriteUsernameError")
            }
            LoginHandlerErrorCodes::StreamReadEnterPasswordError => {
                write!(f, "StreamReadEnterPasswordError")
            }
            LoginHandlerErrorCodes::StreamWritePasswordError => {
                write!(f, "StreamWritePasswordError")
            }
            LoginHandlerErrorCodes::StreamReadCaptchaError => {
                write!(f, "StreamReadCaptchaError")
            }
            LoginHandlerErrorCodes::StreamWriteCaptchaError => {
                write!(f, "StreamWriteCaptchaError")
            }
            LoginHandlerErrorCodes::StreamReadAuthResponseError => {
                write!(f, "StreamReadAuthResponseError")
            }
            LoginHandlerErrorCodes::LoginError => {
                write!(f, "LoginError")
            }
            LoginHandlerErrorCodes::StreamWriteError => {
                write!(f, "StreamWriteError")
            }
            LoginHandlerErrorCodes::StreamReadError => {
                write!(f, "StreamWriteError")
            }
        }
    }
}

/// # LoginHandlerError
///
/// Implements a standard error type for all Login Handler related errors
/// includes the error code (`LoginHandlerErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct LoginHandlerError {
    pub code: LoginHandlerErrorCodes,
    pub message: String,
}

impl LoginHandlerError {
    pub fn new(code: LoginHandlerErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for LoginHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
