use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use tokio::sync::{mpsc, oneshot};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelEvent, ToplevelHandler, ToplevelWState,
};

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
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            let _ = runtime.block_on(toplevel_handler.run(toplevel_msg_rx));
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

                                    let _ = sender.send(Message::Window {
                                        title: updated_title,
                                    });
                                }
                            }
                            None => (),
                        }
                    }
                    ToplevelEvent::Closed { key } => {
                        println!("close event");
                        let _ = sender.send(Message::Window {
                            title: "".to_string(),
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

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}

// fn init_wl_handler(
//     toplevel_event_tx: tokio::sync::mpsc::Sender<ToplevelEvent>,
//     toplevel_msg_rx: tokio::sync::mpsc::Receiver<ToplevelMessage>,
// ) {
//     println!("WAYLAND:: create toplevel handler()");
//     // create the handler instance
//     let mut toplevel_handler = ToplevelHandler::new(toplevel_event_tx);

//     std::thread::spawn(move || {
//         let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
//         let _ = runtime.block_on(toplevel_handler.run(toplevel_msg_rx));
//     });
// }
