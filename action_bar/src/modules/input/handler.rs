use relm4::Sender;
use std::time::Duration;
use tokio::{
    io::unix::AsyncFd,
    sync::{mpsc, oneshot},
    time,
};
use wlroots::wl_input_method::InputFocusHandler;

use crate::Message;

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

pub struct InputServiceHandle {
    status: ServiceStatus,
}

impl InputServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let conn = wayland_client::Connection::connect_to_env()
            .map_err(|_| "Could not connect to wayland socket.")
            .unwrap();
        let wl_display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let _registry = wl_display.get_registry(&event_queue.handle(), ());
        let mut input_focus_handler = InputFocusHandler::new(&mut event_queue);

        let mut was_active = input_focus_handler.is_active();

        loop {
            // This would be required if other threads were reading from the socket.
            // event_queue.dispatch_pending(&mut state).unwrap();
            let read_guard = event_queue.prepare_read().unwrap();
            let fd = read_guard.connection_fd();
            let async_fd = AsyncFd::new(fd).unwrap();

            tokio::select! {
                async_guard = async_fd.readable() => {
                    async_guard.unwrap().clear_ready();
                    // Drop the async_fd since it's holding a reference to the read_guard,
                    // which is dropped on read. We don't need to read from it anyways.
                    std::mem::drop(async_fd);
                    // This should not block because we already ensured readiness
                    let event = read_guard.read();
                    match event {
                        // There are events but another thread processed them, we don't need to dispatch
                        Ok(0) => {}
                        // We have some events
                        Ok(_) => {
                            event_queue.dispatch_pending(&mut input_focus_handler).unwrap();
                        }
                        // No events to receive
                        Err(_) => {} // Err(e) => eprintln!("{}", e),
                    }

                    let is_active = input_focus_handler.is_active();
                    if was_active != is_active {
                        if is_active {
                            println!("Show the keyboard");
                            let _ = sender.send(Message::ShowKeyboard);
                            // let mut pkill = Command::new("pkill")
                            // // TODO: replace `TERM` to signal you want.
                            //     .args(["-USR2", "wvkbd-mobintl"])
                            //     .spawn().unwrap();
                            // let _ = pkill.wait().await;
                        } else {
                            println!("Hide the keyboard");
                            let _ = sender.send(Message::HideKeyboard);
                        }
                        was_active = is_active;
                    }
                },
            }

            // Send any new messages to the socket.
            event_queue.flush().unwrap();
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
