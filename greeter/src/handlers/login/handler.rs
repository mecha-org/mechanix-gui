use anyhow::Result;
use greetd_ipc::{AuthMessageType, ErrorType, Response};
use relm4::AsyncComponentSender;
use std::{sync::Arc, time::Duration};
use tokio::{
    select,
    sync::{mpsc, oneshot, Mutex},
    time::sleep,
};

use crate::{CommandMsg, Greeter, Prompt};
use tracing::{debug, error, info, warn};

use super::greetd_client::{AuthStatus, GreetdClient};

#[derive(Debug)]
pub struct SessionInfo {
    pub username: String,
}

pub struct LoginHandler {
    greetd_client: Arc<Mutex<GreetdClient>>,
    // gui_sender: AsyncComponentSender<Greeter>,
    session_info: Option<SessionInfo>,
}

#[derive(Debug)]
pub enum LoginHandlerMessage {
    Login {
        username: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    CaptchaInput {
        captcha: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    PasswordInput {
        password: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    CancelSession {
        reply_to: oneshot::Sender<Result<bool>>,
    },
}

impl LoginHandler {
    pub async fn new() -> Result<LoginHandler> {
        let greetd_client = Arc::new(Mutex::new(
            GreetdClient::new()
                .await
                .expect("Couldn't initialize greetd client"),
        ));
        Ok(Self {
            // gui_sender: sender,
            greetd_client,
            session_info: None,
        })
    }

    pub async fn run(
        &mut self,
        mut message_rx: mpsc::Receiver<LoginHandlerMessage>,
        gui_sender: AsyncComponentSender<Greeter>,
    ) {
        loop {
            select! {
                            msg = message_rx.recv() => {
                                if msg.is_none() {
                                     continue;
                                }

                            debug!("msg received {:?}", msg);

                            match msg.unwrap() {
                                LoginHandlerMessage::Login{ username, reply_to } => {
                                    let res = self.login(username, &gui_sender.clone()).await;
                                    let _ =reply_to.send(res);
                                }
                                LoginHandlerMessage::CaptchaInput { captcha, reply_to } => {
                                    let res = self.login(captcha, &gui_sender.clone()).await;
                                    let _ = reply_to.send(res);
                                },
                                LoginHandlerMessage::PasswordInput { password, reply_to } => {
                                    let res = self.login(password, &gui_sender.clone()).await;
                                    let _ = reply_to.send(res);
                                },
                                LoginHandlerMessage::CancelSession { reply_to } => {
                                    let res = self.cancel().await;
                                    let _ = reply_to.send(res);
                                }
                        }
                }
            }
        }
    }
    pub async fn login(
        &mut self,
        input: String,
        sender: &AsyncComponentSender<Greeter>,
    ) -> Result<bool> {
        // Check if a password is needed. If not, then directly start the session.
        let auth_status = self.greetd_client.lock().await.get_auth_status().clone();
        match auth_status {
            AuthStatus::Done => {
                // No password is needed, but the session should've been already started by
                // `create_session`.
                self.start_session(sender).await;
            }
            AuthStatus::InProgress => {
                self.send_input(sender, input).await;
            }
            AuthStatus::NotStarted => {
                self.create_session(sender, input).await;
            }
        };

        Ok(true)
    }
    async fn start_session(&mut self, sender: &AsyncComponentSender<Greeter>) {
        let environment = vec![];
        let args: Vec<String> = std::env::args().collect();
        let mut cmd = vec![];
        if args.len() > 5 && (args[5] == "-c" || args[5] == "--command") {
            cmd = vec![String::from(args[6].clone())];
        }
        // Start the session.
        let response = self
            .greetd_client
            .lock()
            .await
            .start_session(cmd, environment)
            .await
            .unwrap_or_else(|err| panic!("Failed to start session: {err}"));

        match response {
            Response::Success => {
                debug!("Session successfully started");
                std::process::exit(0);
            }

            Response::AuthMessage { .. } => unimplemented!(),

            Response::Error { description, .. } => {
                self.cancel().await;
                self.display_error(
                    sender,
                    "Failed to start session",
                    &format!("Failed to start session; error: {description}"),
                );
            }
        }
    }

    pub async fn cancel(&mut self) -> Result<bool> {
        if let Err(err) = self.greetd_client.lock().await.cancel_session().await {
            println!("Couldn't cancel greetd session: {err}");
        };
        Ok(true)
    }

    fn display_error(
        &mut self,
        sender: &AsyncComponentSender<Greeter>,
        display_text: &str,
        log_text: &str,
    ) {
        error!("{log_text}");

        let error_message = String::from(display_text.clone());
        sender.oneshot_command(async move { CommandMsg::ShowErr(error_message) });

        sender.oneshot_command(async move {
            sleep(Duration::from_secs(5)).await;
            CommandMsg::ClearErr
        });
    }

    async fn create_session(&mut self, sender: &AsyncComponentSender<Greeter>, username: String) {
        debug!("Creating session for user: {username}");

        // Create a session for the current user.
        let response = self
            .greetd_client
            .lock()
            .await
            .create_session(&username)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to create session for username '{username}': {err}",)
            });

        self.handle_greetd_response(sender, response).await;
    }

    pub async fn handle_greetd_response(
        &mut self,
        sender: &AsyncComponentSender<Greeter>,
        response: Response,
    ) {
        match response {
            Response::Success => {
                // Authentication was successful and the session may be started.
                // This may happen on the first request, in which case logging in
                // as the given user requires no authentication.
                debug!("Successfully logged in; starting session");
                self.start_session(sender).await;
                return;
            }
            Response::AuthMessage {
                auth_message,
                auth_message_type,
            } => {
                match auth_message_type {
                    AuthMessageType::Secret => {
                        // Greetd has requested input that should be hidden
                        // e.g.: a password
                        debug!("greetd asks for a secret auth input: {auth_message}");
                        //Send message to gui to prompt password
                        sender.oneshot_command(async move {
                            CommandMsg::Prompts(Prompt::Password {
                                message: auth_message.clone(),
                            })
                        });
                        return;
                    }
                    AuthMessageType::Visible => {
                        // Greetd has requested input that need not be hidden
                        debug!("greetd asks for a visible auth input: {auth_message}");
                        //Send message to gui to prompt captcha
                        sender.oneshot_command(async move {
                            CommandMsg::Prompts(Prompt::Captcha {
                                message: auth_message.clone(),
                            })
                        });
                        return;
                    }
                    AuthMessageType::Info => {
                        // Greetd has sent an info message that should be displayed
                        // e.g.: asking for a fingerprint
                        debug!("greetd sent an info: {auth_message}");
                        //Send message to gui and show info
                    }
                    AuthMessageType::Error => {
                        // Greetd has sent an error message that should be displayed and logged
                        self.display_error(
                            sender,
                            &capitalize(&auth_message),
                            &format!("Authentication message error from greetd: {auth_message}"),
                        );
                    }
                }
            }
            Response::Error {
                description,
                error_type,
            } => {
                // some general response error. This can be an authentication failure or a general error
                self.display_error(
                    sender,
                    &format!("Login failed: {}", capitalize(&description)),
                    &format!("Error from greetd: {description}"),
                );

                // In case this is an authentication error (e.g. wrong password), the session should be cancelled.
                if let ErrorType::AuthError = error_type {
                    let _ = self.cancel().await;
                    sender.oneshot_command(async move { CommandMsg::AuthError });
                }

                return;
            }
        }

        debug!("Sending empty auth response to greetd");
        let client = Arc::clone(&self.greetd_client);
        sender.oneshot_command(async move {
            debug!("Sending empty auth response to greetd");
            let response = client
                .lock()
                .await
                .send_auth_response(None)
                .await
                .unwrap_or_else(|err| panic!("Failed to respond to greetd: {err}"));
            CommandMsg::HandleGreetdResponse(response)
        });
    }

    async fn send_input(&mut self, sender: &AsyncComponentSender<Greeter>, input: String) {
        // Reset the password field, for convenience when the user has to re-enter a password.

        // Send the password, as authentication for the current user.
        let resp = self
            .greetd_client
            .lock()
            .await
            .send_auth_response(Some(input))
            .await
            .unwrap_or_else(|err| panic!("Failed to send input: {err}"));

        self.handle_greetd_response(sender, resp).await;
    }
}

/// Capitalize the first letter of the string.
fn capitalize(string: &str) -> String {
    string[0..1].to_uppercase() + &string[1..]
}
