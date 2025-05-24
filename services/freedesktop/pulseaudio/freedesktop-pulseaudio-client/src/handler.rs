//! Handler to handle audio requests and events.

use crate::errors::PulseAudioError;
use crate::service::{DeviceInfo, Message, PulseHandle};
use anyhow::Result;
use libpulse_binding::volume::ChannelVolumes;
use log::{error, info};
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

/// Response type for operations returning multiple audio devices
type SinkDeviceResponse = Result<Vec<DeviceInfo>, PulseAudioError>;
/// Response type for operations returning a single audio device
type SingleDeviceResponse = Result<DeviceInfo, PulseAudioError>;
/// Response type for volume control operations
type VolumeResponse = Result<(), PulseAudioError>;
/// Channel type for sending responses containing multiple devices
type ResponseChannel = mpsc::Sender<SinkDeviceResponse>;

/// String constant representing an input (source) audio device
const DEVICE_TYPE_INPUT: &str = "input";
const DEVICE_TYPE_OUTPUT: &str = "output";
const DEVICE_TYPE_DEFAULT_INPUT: &str = "default input";
const DEVICE_TYPE_DEFAULT_OUTPUT: &str = "default output";

/// Represents different types of requests that can be made to the PulseAudio service.
///
/// This enum defines all possible operations that can be performed on audio devices,
/// including retrieving device lists, setting default devices, and controlling volume/mute states.
/// Each variant includes a response channel to communicate the operation result back to the caller.
#[derive(Debug)]
pub enum PulseAudioRequest {
    /// Request to get the list of all available output (sink) devices.
    ///
    /// The response will be sent through the provided channel containing either
    /// a Vec<DeviceInfo> with sink details or a PulseAudioError.
    GetSinks { reply_to: ResponseChannel },

    /// Request to get the list of all available input (source) devices.
    ///
    /// The response will be sent through the provided channel containing either
    /// a Vec<DeviceInfo> with source details or a PulseAudioError.
    GetSources { reply_to: ResponseChannel },

    /// Request to get the output device.
    GetDefaultSink {
        reply_to: Sender<SingleDeviceResponse>,
    },

    /// Request to get the input device.
    GetDefaultSource {
        reply_to: Sender<SingleDeviceResponse>,
    },

    /// Request to set the output device volume by name.
    SetSinkVolumeByName {
        device: String,
        volume: ChannelVolumes,
        reply_to: Sender<VolumeResponse>,
    },

    /// Request to set the input device volume by name.
    SetSourceVolumeByName {
        device: String,
        volume: ChannelVolumes,
        reply_to: Sender<VolumeResponse>,
    },

    /// Request to set the output device mute by name.
    SetSinkMuteByName {
        device: String,
        mute: bool,
        reply_to: Sender<Sender<VolumeResponse>>,
    },

    /// Request to set the input device mute by name.
    SetSourceMuteByName {
        device: String,
        mute: bool,
        reply_to: Sender<Sender<VolumeResponse>>,
    },
}

/// A client for interacting with the PulseAudio freedesktop-pulseaudio-client server.
///
/// This struct provides an interface for handling PulseAudio operations asynchronously.
/// It processes incoming requests through a channel-based communication system and
/// manages the underlying PulseAudio connection.
pub struct PulseAudioClient {
    pulse_handler: PulseHandle,
}

impl PulseAudioClient {
    /// Creates a new PulseAudioClient instance.
    ///
    /// Initializes a new client that can handle PulseAudio operations.
    /// This client will manage the connection to the PulseAudio server
    /// and process audio-related requests.
    pub fn new() -> Self {
        // Initialize the PulseAudio handler
        let pulse_handler = PulseHandle::new();
        Self { pulse_handler }
    }

    /// Handles incoming PulseAudio requests and communicates with the PulseAudio backend.
    ///
    /// This asynchronous event loop listens for requests on the `request` channel,
    /// processes each request by sending the appropriate message to the PulseAudio handler,
    /// and sends the response back via the provided reply channel.
    ///
    /// Supported requests include:
    /// - Getting source (input) and sink (output) device lists
    /// - Getting default source/sink devices
    /// - Setting source/sink volumes
    /// - (Stubbed) Setting source/sink mute states
    pub async fn run(
        &mut self,
        mut request: mpsc::Receiver<PulseAudioRequest>,
    ) -> Result<(), PulseAudioError> {
        loop {
            select! {
                // Await the next request from the channel.
                msg = request.recv() => {
                    if let Some(request) = msg {
                        match request {
                            PulseAudioRequest::GetSources { reply_to } => {
                                self.handle_device_list_request(
                                    Message::GetSources,
                                    reply_to,
                                    |msg| match msg {
                                        Message::SetSource(sources) => sources,
                                        _ => Err(PulseAudioError::UnexpectedMessage),
                                    },
                                    DEVICE_TYPE_INPUT
                                ).await;
                            }
                            PulseAudioRequest::GetSinks { reply_to } => {
                                self.handle_device_list_request(
                                    Message::GetSinks,
                                    reply_to,
                                    |msg| match msg {
                                        Message::SetSink(sinks) => sinks,
                                        _ => Err(PulseAudioError::UnexpectedMessage),
                                    },
                                    DEVICE_TYPE_DEFAULT_OUTPUT
                                ).await;
                            }
                            PulseAudioRequest::GetDefaultSink { reply_to } => {
                                self.handle_single_device_request(
                                    Message::GetDefaultSink,
                                    reply_to,
                                    |msg| match msg {
                                        Message::SetDefaultSink(sink) => sink,
                                        _ => Err(PulseAudioError::UnexpectedMessage),
                                    },
                                    DEVICE_TYPE_DEFAULT_OUTPUT
                                ).await;
                            }
                            PulseAudioRequest::GetDefaultSource { reply_to } => {
                                self.handle_single_device_request(
                                    Message::GetDefaultSource,
                                    reply_to,
                                    |msg| match msg {
                                        Message::SetDefaultSource(source) => source,
                                        _ => Err(PulseAudioError::UnexpectedMessage),
                                    },
                                    DEVICE_TYPE_DEFAULT_INPUT
                                ).await;
                            }
                            PulseAudioRequest::SetSinkVolumeByName { device, volume, reply_to } => {
                                self.handle_volume_request(
                                    Message::SetSinkVolumeByName(device, volume),
                                    reply_to,
                                    |msg| matches!(msg, Message::SetSinkVolumeByName { .. }),
                                    DEVICE_TYPE_OUTPUT
                                ).await;
                            }
                            PulseAudioRequest::SetSourceVolumeByName { device, volume, reply_to } => {
                                self.handle_volume_request(
                                    Message::SetSourceVolumeByName(device, volume),
                                    reply_to,
                                    |msg| matches!(msg, Message::SetSourceVolumeByName { .. }),
                                    DEVICE_TYPE_INPUT
                                ).await;
                            }
                            // Mute functionality is currently not implemented.
                            PulseAudioRequest::SetSinkMuteByName { .. } => {}
                            PulseAudioRequest::SetSourceMuteByName { .. } => {}
                        }
                    }
                }
            }
        }
    }
}

impl PulseAudioClient {
    /// Handles requests that expect a list of devices as a response (sources or sinks).
    async fn handle_device_list_request<F>(
        &mut self,
        request_msg: Message,
        reply_to: ResponseChannel,
        parse_response: F, // Closure function to parse the response message
        device_type: &str,
    ) where
        F: Fn(Message) -> Result<Vec<DeviceInfo>, PulseAudioError>,
    {
        info!("Getting {} device list request", device_type);

        match self.pulse_handler.to_pulse.send(request_msg).await {
            Ok(_) => match self.pulse_handler.from_pulse.recv().await {
                Some(msg) => match parse_response(msg) {
                    Ok(devices) => {
                        info!("received {} device list: {:?}", device_type, devices);
                        reply_to.send(Ok(devices)).await.ok();
                    }
                    Err(e) => {
                        error!("unexpected message type received");
                        reply_to.send(Err(e)).await.ok();
                    }
                },
                None => {
                    error!("failed to receive message from PulseAudio");
                    reply_to
                        .send(Err(PulseAudioError::ReceiveMessageError))
                        .await
                        .ok();
                }
            },
            Err(e) => {
                error!("failed to send message to PulseAudio: {}", e);
                reply_to
                    .send(Err(PulseAudioError::SendMessageError(e.to_string())))
                    .await
                    .ok();
            }
        }
    }

    /// Handles requests that expect a single device as a response (default source or sink).
    async fn handle_single_device_request<F>(
        &mut self,
        request_msg: Message,
        reply_to: Sender<Result<DeviceInfo, PulseAudioError>>,
        parse_response: F,
        device_type: &str,
    ) where
        F: Fn(Message) -> Result<DeviceInfo, PulseAudioError>,
    {
        info!("Getting {} device request", device_type);

        match self.pulse_handler.to_pulse.send(request_msg).await {
            Ok(_) => match self.pulse_handler.from_pulse.recv().await {
                Some(msg) => match parse_response(msg) {
                    Ok(device) => {
                        info!("Received {} device: {:?}", device_type, device);
                        reply_to.send(Ok(device)).await.ok();
                    }
                    Err(e) => {
                        error!("Unexpected message type received");
                        reply_to.send(Err(e)).await.ok();
                    }
                },
                None => {
                    error!("failed to receive message from PulseAudio");
                    reply_to
                        .send(Err(PulseAudioError::ReceiveMessageError))
                        .await
                        .ok();
                }
            },
            Err(e) => {
                error!("failed to send message to PulseAudio: {}", e);
                reply_to
                    .send(Err(PulseAudioError::SendMessageError(e.to_string())))
                    .await
                    .ok();
            }
        }
    }

    /// Handles requests to set the volume of a device (source or sink).
    async fn handle_volume_request<F>(
        &mut self,
        request_msg: Message,
        reply_to: Sender<Result<(), PulseAudioError>>,
        is_expected_response: F,
        device_type: &str,
    ) where
        F: Fn(&Message) -> bool,
    {
        info!("setting {} device volume request", device_type);

        match self.pulse_handler.to_pulse.send(request_msg).await {
            Ok(_) => match self.pulse_handler.from_pulse.recv().await {
                Some(ref msg) if is_expected_response(msg) => {
                    info!("set {} device volume", device_type);
                    reply_to.send(Ok(())).await.ok();
                }
                Some(_) => {
                    error!("unexpected message type received");
                    reply_to
                        .send(Err(PulseAudioError::UnexpectedMessage))
                        .await
                        .ok();
                }
                None => {
                    error!("failed to receive message from PulseAudio");
                    reply_to
                        .send(Err(PulseAudioError::ReceiveMessageError))
                        .await
                        .ok();
                }
            },
            Err(e) => {
                error!("failed to send message to PulseAudio: {}", e);
                reply_to
                    .send(Err(PulseAudioError::SendMessageError(e.to_string())))
                    .await
                    .ok();
            }
        }
    }
}
