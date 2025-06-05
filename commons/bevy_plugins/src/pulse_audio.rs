use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use freedesktop_pulseaudio_client::service::{DeviceInfo, PulseAudioService};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

const DISCOVER_DURATION: std::time::Duration = std::time::Duration::from_secs(5);
/// Holds the async-initialized service, or None if not ready yet.
pub struct PulseAudioServiceResource {
    pub service: Option<PulseAudioService>,
}

#[derive(Resource, Debug, Clone)]
pub struct PulseAudioStatus {
    pub connected: bool,
    pub last_error: Option<String>,
}
#[derive(Resource)]
pub struct PulseAudioResultReceiver {
    receiver: Mutex<Receiver<PulseAudioResult>>,
}

#[derive(Resource, Clone)]
pub struct PulseAudioResultSender(pub Sender<PulseAudioResult>);
#[derive(Event)]
pub struct PulseAudioActionEvent(pub PulseAudioAction);

#[derive(Event)]
pub struct PulseAudioResultEvent(pub PulseAudioResult);

#[derive(Debug, Clone)]
pub enum PulseAudioAction {
    ListSinks,
    // ListSources,
    // GetDefaultSink,
    // GetDefaultSource,
    // SetDefaultSink(String),
    // SetDefaultSource(String),
    // SetSinkVolumeByName(String, ChannelVolumes),
    // SetSourceVolumeByName(String, ChannelVolumes),
}

#[derive(Debug)]
pub enum PulseAudioResult {
    ListSinks(Vec<DeviceInfo>),
    ListSources(Vec<String>),
    GetDefaultSink(String),
    GetDefaultSource(String),
    SetDefaultSink(bool),
    SetDefaultSource(bool),
    SetSinkVolumeByName(bool),
    SetSourceVolumeByName(bool),
    Error(ErrorType),
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ActionFailed {
        action: PulseAudioAction,
        message: String,
    },
}

/// PulseAudioManager plugin for Bevy
///
/// This plugin provides a resource for the `PulseAudioService` which is
/// initialized asynchronously on startup. It also provides a system for enabling
/// bluetooth.
///
/// The `PulseAudioService` is not available until the `init_pulse_audion_service`
/// system has completed. This is checked with the `service_ready` function.
pub struct PulseAudioPlugin;

impl Plugin for PulseAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(PulseAudioServiceResource { service: None })
            .insert_resource(PulseAudioStatus {
                connected: false,
                last_error: None,
            })
            .add_event::<PulseAudioActionEvent>()
            .add_event::<PulseAudioResultEvent>()
            .add_systems(
                Startup,
                (init_pulse_audion_service, setup_bluetooth_channel),
            ) // Async task so temp move service result to static
            .add_systems(
                Update,
                (
                    handle_pulse_audio_action_events,
                    poll_bluetooth_action_result_events.after(handle_pulse_audio_action_events),
                ),
            );
    }
}

/// Initializes the `PulseAudioService` asynchronously on startup.
///
/// This system is spawned as an IoTaskPool task and will detach itself once
/// the service is initialized or an error occurs.
///
/// The service is stored in a static variable after initialization and can be
/// accessed using the `service_ready` function.
///
/// # Errors
///
/// If the service fails to initialize, an error message will be logged.
fn init_pulse_audion_service(mut resource: NonSendMut<PulseAudioServiceResource>) {
    match PulseAudioService::new() {
        Ok(service) => {
            resource.service = Some(service);
            info!("PulseAudioService initialized!");
        }
        Err(e) => {
            error!("Failed to initialize PulseAudioService: {e}");
        }
    }
}

// In your plugin setup or a startup system:
fn setup_bluetooth_channel(mut commands: Commands) {
    let (tx, rx) = std::sync::mpsc::channel();
    commands.insert_resource(PulseAudioResultReceiver {
        receiver: Mutex::new(rx),
    });
    commands.insert_resource(PulseAudioResultSender(tx)); // You define this
}
/// Handles all `PulseAudioActionEvent`s and sends the result to the channel.
///
/// This system should be run after the `PulseAudioService` is initialized and
/// ready to use. It will spawn a new task for each action, which will send the
/// result of the action to the channel once the task is complete.
///
/// # Errors
///
/// If any action fails, an error message will be logged and the error will be
/// sent to the channel as a `PulseAudioResult::Error`.
fn handle_pulse_audio_action_events(
    mut events: EventReader<PulseAudioActionEvent>,
    resource_service: NonSend<PulseAudioServiceResource>,
    sender: Res<PulseAudioResultSender>,
) {
    for event in events.read() {
        let PulseAudioActionEvent(action) = event;
        match action {
            PulseAudioAction::ListSinks => {
                info!("audio action: list sinks");
                if let Some(service) = &resource_service.service {
                    let server = &service.server;
                    match server.get_sinks() {
                        Ok(sinks) => {
                            let result = PulseAudioResult::ListSinks(sinks);
                            if let Err(e) = sender.0.send(result) {
                                error!("failed to send list sinks result: {e}");
                            }
                        }
                        Err(e) => {
                            error!("error getting sinks: {e}");
                            let result = PulseAudioResult::Error(ErrorType::ActionFailed {
                                action: PulseAudioAction::ListSinks,
                                message: format!("Error getting sinks: {e}"),
                            });
                        }
                    }
                }
            } // PulseAudioAction::ListSources => {}
              // PulseAudioAction::GetDefaultSink => {}
              // PulseAudioAction::GetDefaultSource => {}
              // PulseAudioAction::SetDefaultSink(_) => {}
              // PulseAudioAction::SetDefaultSource(_) => {}
              // PulseAudioAction::SetSinkVolumeByName(_, _) => {}
              // PulseAudioAction::SetSourceVolumeByName(_, _) => {}
        }
    }
}

/// Polls the internal event receiver for new events and writes them to the
/// `bluetooth_result_event_writer` as `PulseAudioResultEvent`s.
///
/// This system is typically run once per frame and is used to dispatch events
/// from the PulseAudio service to the rest of the app.
///
/// The event receiver is accessed by a lock, and if the lock can't be acquired,
/// the system will print an error message and do nothing.
fn poll_bluetooth_action_result_events(
    mut bluetooth_result_event_writer: EventWriter<PulseAudioResultEvent>,
    event_receiver: ResMut<PulseAudioResultReceiver>,
) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            bluetooth_result_event_writer.write(PulseAudioResultEvent(event));
        }
    } else {
        error!("failed to acquire receiver lock");
    }
}
