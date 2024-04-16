use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::time::Duration;
use tokio::{
    sync::{mpsc, oneshot},
    time,
};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelEvent, ToplevelHandler, ToplevelMessage, ToplevelWState,
};

use tracing::error;

use crate::{gui::Message, AppMessage, RunningAppsMessage};

use super::service::RunningAppsService;

pub struct RunningAppsServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl RunningAppsServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
        let task = "run";
        println!("RunningAppsServiceHandle::run()");
        // create mpsc channel for interacting with the toplevel handler
        let (toplevel_msg_tx, toplevel_msg_rx) = mpsc::channel(128);

        // create mpsc channel for receiving events from the toplevel handler
        let (toplevel_event_tx, mut toplevel_event_rx) = mpsc::channel(128);

        // create the handler instance
        let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);

        // start the toplevel handler
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            let _ = runtime.block_on(toplevel_handler.run(toplevel_msg_rx));
        });

        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let (tx, rx) = oneshot::channel();
            let _ = toplevel_msg_tx
                .send(ToplevelMessage::GetToplevels { reply_to: tx })
                .await;

            match rx.await {
                Ok(top_levels) => {
                    // println!(
                    //     "RunningAppsServiceHandle::run() top level count{}",
                    //     top_levels.len()
                    // );
                    let _ = self.app_channel.send(AppMessage::RunningApps {
                        message: RunningAppsMessage::Count {
                            count: top_levels.len() as i32,
                        },
                    });
                }
                Err(e) => {
                    error!(task, "error while getting running apps {}", e);
                    let _ = self.app_channel.send(AppMessage::RunningApps {
                        message: RunningAppsMessage::Count { count: 0 },
                    });
                }
            };
        }
    }
}
