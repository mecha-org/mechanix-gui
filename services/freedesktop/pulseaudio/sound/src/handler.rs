//! Handler to handle audio requests and events.

use crate::errors::PulseAudioError;
use log::{error, info};
use tokio::select;
use tokio::sync::mpsc;
use crate::service::{Message, PulseHandle};

#[derive(Debug)]
pub enum PulseAudioRequest {
    GetOutputDeviceList {
        reply_to: mpsc::Sender<Result<(), PulseAudioError>>,
    },
}

/// Handler for managing NetworkManager operations and requests.
///
/// This struct maintains a connection to the system D-Bus and processes incoming requests
/// through an async channel.
pub struct PulseAudioClient {}

impl PulseAudioClient {
    /// Creates a new Client with a connection to the system D-Bus.
    pub fn new() -> Self {
        Self {}
    }

    /// Main event loop for handling incoming NetworkManager requests.
    ///
    /// # Arguments
    /// * `nm_request` - A receiver for incoming NetworkManagerRequest messages.
    ///
    /// # Returns
    /// * `Result<()>` - Returns Ok on normal operation, or an error if the proxy cannot be created.
    pub async fn run(
        &mut self,
        mut nm_request: mpsc::Receiver<PulseAudioRequest>,
    ) -> Result<(), PulseAudioError> {


        // Initialize the PulseAudio handler
        let pulse_handler = PulseHandle::new()?;

        // 2. Event loop: handle each incoming request.
        loop {
            select! {
                // Wait for the next request from the channel.
                msg = nm_request.recv() => {
                    // If a request was received, process it.
                    if let Some(request) = msg {
                        match request {
                            PulseAudioRequest::GetOutputDeviceList { reply_to } => {
                                info!("Getting output device list request");
                                let result = pulse_handler.to_pulse(Message::GetSinks);

                            }
                        }
                    }
                }
            }
        }
    }
}
