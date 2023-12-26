use relm4::Sender;
use tokio::sync::{mpsc, oneshot};
use zwlr_foreign_toplevel_v1_async::handler::{ToplevelEvent, ToplevelHandler, ToplevelWState};

use crate::Message;

#[derive(Debug)]
pub enum ServiceMessage {
    Start { respond_to: oneshot::Sender<u32> },
    Stop { respond_to: oneshot::Sender<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

pub struct WindowServiceHandle {
    status: ServiceStatus,
}

impl WindowServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        // create mpsc channel for interacting with the toplevel handler
        let (toplevel_msg_tx, toplevel_msg_rx) = mpsc::channel(128);

        // create mpsc channel for receiving events from the toplevel handler
        let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);

        // start the toplevel handler
        let toplevel_t = tokio::spawn(async move {
            let _ = toplevel_handler.run(toplevel_msg_rx).await;
        });

        // receive all toplevel events
        let toplevel_event_t = tokio::spawn(async move {
            loop {
                let msg = toplevel_event_rx.recv().await;
                if msg.is_none() {
                    continue;
                }

                match msg.unwrap() {
                    ToplevelEvent::Done {
                        key,
                        title,
                        app_id,
                        state,
                    } => {
                        let mut window_title = "".to_string();

                        match state {
                            Some(top_level_state) => {
                                if top_level_state.contains(&ToplevelWState::Activated) {
                                    window_title = title.clone();
                                }
                            }
                            None => (),
                        }
                        let _ = sender.send(Message::CurrentWindowTitleUpdate(window_title));
                    }
                    ToplevelEvent::Closed { key } => {
                        let _ = sender.send(Message::CurrentWindowTitleUpdate("".to_string()));
                    }
                    _ => {}
                }
            }
        });
        let _ = toplevel_t.await.unwrap();
        let _ = toplevel_event_t.await.unwrap();
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
