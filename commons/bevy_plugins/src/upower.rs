use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::IoTaskPool;
use freedesktop_upower_client::interfaces::device::{BatteryLevel as UPowerBatteryLevel, BatteryState};
use freedesktop_upower_client::service::UPowerService;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, LazyLock, Mutex};

/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct UPowerServiceResource {
    pub service: Option<UPowerService>,
}

#[derive(Resource)]
pub struct UPowerResultReceiver {
    receiver: Mutex<Receiver<UPowerResult>>,
}

#[derive(Resource, Clone)]
pub struct UPowerResultSender(pub Sender<UPowerResult>);

#[derive(Resource, Default)]
pub struct ServiceState {
    pub initialized: bool,
    pub stream_started: bool,
}
#[derive(Resource, Default)]
pub struct DeviceState(pub BatteryState);

#[derive(Resource, Default)]
pub struct DevicePercentage(f64);

#[derive(Resource, Default)]
pub struct BatteryLevel(pub UPowerBatteryLevel);
#[derive(Event)]
pub struct UPowerActionEvent(pub UPowerAction);

#[derive(Debug, Clone)]
pub enum UPowerAction {
    StreamDeviceState,
    StreamDevicePercentage,
    StreamBatteryLevel,
}

#[derive(Debug)]
pub enum UPowerResult {
    DeviceState(BatteryState),
    DevicePercentage(f64),
    BatteryLevel(UPowerBatteryLevel),
    Error(ErrorType),
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ActionFailed {
        action: UPowerAction,
        message: String,
    },
}

/// UPowerManager plugin for Bevy
///
/// This plugin provides a resource for the `UPowerService` which is
/// initialized asynchronously on startup. It also provides a system for enabling
/// upower.
///
/// The `UPowerService` is not available until the `init_upower_service`
/// system has completed. This is checked with the `service_ready` function.
pub struct UPowerPlugin;

impl Plugin for UPowerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UPowerServiceResource { service: None })
            .add_event::<UPowerActionEvent>()
            .insert_resource(ServiceState::default())
            .insert_resource(DeviceState::default())
            .insert_resource(DevicePercentage::default())
            .insert_resource(BatteryLevel::default())
            .add_systems(Startup, (init_upower_service, setup_upower_channel)) // Async task so temp move service result to static // Once a service is initialized, it will move service from static to resource
            .add_systems(
                Update,
                (
                    handle_upower_action_events,
                    poll_upower_action_result_events.after(handle_upower_action_events),
                ),
            ).add_systems(
            Update,
            (
                poll_service_init,
                start_initial_streams_if_service_ready.after(poll_service_init),
            ),
        );
    }
}

fn start_initial_streams_if_service_ready(
    mut state: ResMut<ServiceState>,
    service_res: Res<UPowerServiceResource>,
    mut events: EventWriter<UPowerActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if !state.stream_started {
        if let Some(service) = &service_res.service {
            println!("Starting initial streams...");
            events.write(UPowerActionEvent(UPowerAction::StreamDeviceState));
            events.write(UPowerActionEvent(UPowerAction::StreamDevicePercentage));
            events.write(UPowerActionEvent(UPowerAction::StreamBatteryLevel));
            state.stream_started = true;
        }
    }
}

/// Checks if the `UPowerService` is ready (i.e. not None).
///
/// This is used to gate the execution of systems that depend on the service
/// being available.
fn service_ready(resource: Res<UPowerServiceResource>) -> bool {
    resource.service.is_some()
}

static SERVICE_RESULT: LazyLock<Mutex<Option<UPowerService>>> =
    LazyLock::new(|| Mutex::new(None));

/// Initializes the `UPowerService` asynchronously on startup.
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
fn init_upower_service() {
    IoTaskPool::get()
        .spawn(async {
            match UPowerService::new().await {
                Ok(service) => {
                    let mut lock = SERVICE_RESULT.lock().unwrap();
                    *lock = Some(service);
                    info!("UPowerService initialized!");
                }
                Err(e) => {
                    error!("Failed to initialize UPowerService: {e}");
                }
            }
        })
        .detach();
}

// Polling system to move service from static to resource
fn poll_service_init(mut resource: ResMut<UPowerServiceResource>) {
    let mut lock = SERVICE_RESULT.lock().unwrap();
    if let Some(service) = lock.take() {
        resource.service = Some(service);
    }
}

// In your plugin setup or a startup system:
fn setup_upower_channel(mut commands: Commands) {
    let (tx, rx) = std::sync::mpsc::channel();
    commands.insert_resource(UPowerResultReceiver {
        receiver: Mutex::new(rx),
    });
    commands.insert_resource(UPowerResultSender(tx)); // You define this
}
/// Handles all `UPowerActionEvent`s and sends the result to the channel.
///
/// This system should be run after the `UPowerService` is initialized and
/// ready to use. It will spawn a new task for each action, which will send the
/// result of the action to the channel once the task is complete.
///
/// # Errors
///
/// If any action fails, an error message will be logged and the error will be
/// sent to the channel as a `UPowerResult::Error`.
fn handle_upower_action_events(
    mut events: EventReader<UPowerActionEvent>,
    service: ResMut<UPowerServiceResource>,
    sender: Res<UPowerResultSender>,
) {
    for event in events.read() {
        let UPowerActionEvent(action) = event;
        match action {
            UPowerAction::StreamDeviceState => {
                info!("upower action: stream device state");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver: Receiver<BatteryState> =
                                service.stream_device_state().await;
                            while let Ok(status) = receiver.recv() {
                                if let Err(err) =
                                    sender.send(UPowerResult::DeviceState(status))
                                {
                                    error!("failed to send upower device state: {err}");
                                }
                            }
                        })
                        .detach();
                }
            }
            UPowerAction::StreamDevicePercentage => {
                info!("upower action: stream device percentage");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver: Receiver<f64> =
                                service.stream_device_percentage().await;
                            while let Ok(status) = receiver.recv() {
                                if let Err(err) =
                                    sender.send(UPowerResult::DevicePercentage(status))
                                {
                                    error!("failed to send upower device percentage: {err}");
                                }
                            }
                        })
                        .detach();
                }
            }
            UPowerAction::StreamBatteryLevel => {
                info!("upower action: stream battery level");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver: Receiver<UPowerBatteryLevel> =
                                service.stream_battery_level().await;
                            while let Ok(status) = receiver.recv() {
                                if let Err(err) =
                                    sender.send(UPowerResult::BatteryLevel(status))
                                {
                                    error!("failed to send upower batter level: {err}");
                                }
                            }
                        })
                        .detach();
                }
            }
        }
    }
}

/// Polls the internal event receiver for new events and writes them to the
/// `upower_result_event_writer` as `UPowerResultEvent`s.
///
/// This system is typically run once per frame and is used to dispatch events
/// from the UPower service to the rest of the app.
///
/// The event receiver is accessed by a lock, and if the lock can't be acquired,
/// the system will print an error message and do nothing.
fn poll_upower_action_result_events(
    event_receiver: ResMut<UPowerResultReceiver>,
    mut device_state_res: ResMut<DeviceState>,
    mut device_percentage_res: ResMut<DevicePercentage>,
    mut battery_level_res: ResMut<BatteryLevel>,
) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            match event {
                UPowerResult::DeviceState(state) => {
                    info!("upower device state updated: {:?}", state);
                    device_state_res.0 = state;
                }
                UPowerResult::DevicePercentage(percentage) => {
                    info!("upower device percentage updated: {percentage}");
                    device_percentage_res.0 = percentage;
                }
                UPowerResult::BatteryLevel(level) => {
                    info!("upower battery level updated: {:?}", level);
                    battery_level_res.0 = level;
                }
                _ => {}
            }
        }
    } else {
        error!("failed to acquire receiver lock");
    }
}
