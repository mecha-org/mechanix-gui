use super::handler::{PasswordRequest, PinRequest};
use crate::handlers::login::errors::{LoginHandlerError, LoginHandlerErrorCodes};
use anyhow::{bail, Result};
use greetd_ipc::{
    codec::SyncCodec, AuthMessageType, ErrorType, Request as GreetdRequest,
    Response as GreetdResponse,
};
use std::{env, os::unix::net::UnixStream, sync::mpsc};
use tracing::error;

#[derive(Debug)]
pub enum SessionMessages {
    PasswordEntered { password: String },
    CaptchaEntered { captcha: String },
}

pub fn login(username: String, message_rx: mpsc::Receiver<SessionMessages>) -> Result<bool> {
    let login_manager_url = match env::var("LOGIN_MANAGER_URL") {
        Ok(v) => v,
        Err(_) => {
            bail!(LoginHandlerError::new(
                LoginHandlerErrorCodes::FindLoginManagerUrlError,
                format!("unable to read LOGIN_MANAGER_URL in env"),
            ));
        }
    };

    let mut stream = match UnixStream::connect(login_manager_url) {
        Ok(v) => v,
        Err(e) => {
            bail!(LoginHandlerError::new(
                LoginHandlerErrorCodes::LoginManagerStreamConnectError,
                format!("unable to connect to login manager stream error: {}", e),
            ));
        }
    };

    let mut next_request = GreetdRequest::CreateSession { username };
    let mut starting = false;
    loop {
        match next_request.write_to(&mut stream) {
            Ok(_) => (),
            Err(e) => {
                bail!(LoginHandlerError::new(
                    LoginHandlerErrorCodes::StreamWriteError,
                    format!("unable to write in stream error: {}", e),
                ));
            }
        };

        let auth_response = match GreetdResponse::read_from(&mut stream) {
            Ok(r) => r,
            Err(e) => {
                bail!(LoginHandlerError::new(
                    LoginHandlerErrorCodes::StreamReadError,
                    format!("unable to read from stream error: {}", e),
                ));
            }
        };

        match auth_response {
            GreetdResponse::AuthMessage {
                auth_message_type,
                auth_message,
            } => {
                let response = match auth_message_type {
                    AuthMessageType::Visible => {
                        let session_message_res = message_rx.recv();
                        let session_message: SessionMessages = match session_message_res {
                            Ok(v) => v,
                            Err(e) => bail!(LoginHandlerError::new(
                                LoginHandlerErrorCodes::StreamReadError,
                                format!("unable to read from stream error: {}", e),
                            )),
                        };

                        let captcha = match session_message {
                            SessionMessages::CaptchaEntered { captcha } => captcha,
                            _ => "".to_string(),
                        };

                        Some(captcha)
                    }
                    AuthMessageType::Secret => {
                        let session_message_res = message_rx.recv();
                        let session_message: SessionMessages = match session_message_res {
                            Ok(v) => v,
                            Err(e) => bail!(LoginHandlerError::new(
                                LoginHandlerErrorCodes::StreamReadError,
                                format!("unable to read from stream error: {}", e),
                            )),
                        };

                        let password = match session_message {
                            SessionMessages::PasswordEntered { password } => password,
                            _ => "".to_string(),
                        };

                        Some(password)
                    }
                    AuthMessageType::Info => Some(String::from("")),
                    AuthMessageType::Error => {
                        error!("error {}", auth_message);
                        None
                    }
                };
                next_request = GreetdRequest::PostAuthMessageResponse { response };
            }
            GreetdResponse::Success => {
                if starting {
                    return Ok(true);
                } else {
                    starting = true;
                    next_request = GreetdRequest::StartSession {
                        env: vec![],
                        cmd: vec!["gnome-calculator".to_string()],
                    };
                }
            }
            GreetdResponse::Error {
                error_type,
                description,
            } => match error_type {
                ErrorType::AuthError => {
                    bail!(LoginHandlerError::new(
                        LoginHandlerErrorCodes::LoginError,
                        format!("login error: {}", description),
                    ))
                }
                ErrorType::Error => {
                    bail!(LoginHandlerError::new(
                        LoginHandlerErrorCodes::LoginError,
                        format!("login error: {}", description),
                    ))
                }
            },
        };
    }
}

pub fn login_with_password(request: PasswordRequest) -> Result<bool> {
    let PasswordRequest { username, password } = request;

    login(username)
}

pub fn login_with_pin(request: PinRequest) -> Result<bool> {
    let PinRequest { username, pin } = request;

    login(username)
}
