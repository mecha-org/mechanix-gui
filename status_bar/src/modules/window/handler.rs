use relm4::Sender;
use std::time::Duration;
use tokio::{
    sync::{mpsc, oneshot},
    time,
};
use wlroots::wlr_toplevel_handler::{
    WlrToplevelHandler, WlrToplevelHandlerMessage, WlrToplevelHandlerOptions,
};

use crate::Message;

use super::service::WindowService;
use tracing::error;

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
        let (toplevel_message_tx, toplevel_message_rx) = mpsc::channel(32);
        let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(32);

        let mut wlr_toplevel_handler =
            WlrToplevelHandler::new(WlrToplevelHandlerOptions { toplevel_event_tx });

        let wlr_thread = tokio::spawn(async move {
            let _ = wlr_toplevel_handler.run(toplevel_message_rx).await;
        });

        while let Some(message) = toplevel_event_rx.recv().await {
            println!("main: toplevel event = {:?}", message);

            // get active window/toplevel title
            let (tx, rx) = oneshot::channel();
            let _ = toplevel_message_tx
                .send(WlrToplevelHandlerMessage::GetActiveToplevelTitle { reply_to: tx })
                .await;

            let res = rx.await.expect("no reply from service");

            match res {
                Ok(active_toplevel_title_op) => match active_toplevel_title_op {
                    Some(active_toplevel_title) => {
                        let _ =
                            sender.send(Message::CurrentWindowTitleUpdate(active_toplevel_title));
                    }
                    None => (),
                },
                Err(e) => {
                    error!("error while getting top level window title: {}", e)
                }
            }

            // get all open toplevels
            let (tx, rx) = oneshot::channel();
            let _ = toplevel_message_tx
                .send(WlrToplevelHandlerMessage::GetActiveToplevel { reply_to: tx })
                .await;

            let res = rx.await.expect("no reply from service");
            match res {
                Ok(active_toplevel_op) => {
                    let _ =
                        sender.send(Message::TopLevelActiveUpdate(active_toplevel_op.is_some()));
                }
                Err(e) => {
                    error!("error while getting top level window title: {}", e);
                    let _ = sender.send(Message::TopLevelActiveUpdate(false));
                }
            }
        }

        wlr_thread.await.unwrap();
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
