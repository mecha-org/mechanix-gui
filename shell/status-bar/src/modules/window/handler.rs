use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use tokio::sync::{mpsc, oneshot};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelEvent, ToplevelHandler, ToplevelMessage, ToplevelWState,
};

use crate::AppMessage;

pub struct WindowServiceHandle {
    app_channel: Sender<AppMessage>,
}

impl WindowServiceHandle {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self { app_channel }
    }

    pub async fn run(&mut self) {
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

        let app_channel = self.app_channel.clone();

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
                        println!("done event {:?}", state);
                        match state {
                            Some(top_level_state) => {
                                if top_level_state.contains(&ToplevelWState::Activated) {
                                    let updated_title = if title.len() > 15 {
                                        title[..15].to_owned()
                                    } else {
                                        title
                                    };

                                    println!("updated_title {:?}", updated_title);

                                    let _ = app_channel.clone().send(AppMessage::Window {
                                        title: updated_title,
                                        activated: true,
                                    });
                                }
                            }
                            None => (),
                        }
                    }
                    ToplevelEvent::Closed { key } => {
                        println!("close event");
                        let _ = app_channel.clone().send(AppMessage::Window {
                            title: "".to_string(),
                            activated: false,
                        });
                    }
                    _ => {}
                }
            }
        });
        println!("before run");
        let _ = toplevel_event_t.await.unwrap();
        println!("after run");
    }
}
