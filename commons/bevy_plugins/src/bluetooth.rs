use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool};
use freedesktop_bluez_client::interfaces::device::BluetoothDevice;
use freedesktop_bluez_client::service::BluetoothService;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{LazyLock, Mutex};

const DISCOVER_DURATION: std::time::Duration = std::time::Duration::from_secs(5);
/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct BluetoothServiceResource {
    pub service: Option<BluetoothService>,
}

#[derive(Resource, Debug, Clone)]
pub struct BluetoothStatus {
    pub connected: bool,
    pub last_error: Option<String>,
}
#[derive(Resource)]
pub struct BluetoothResultReceiver {
    receiver: Mutex<Receiver<BluetoothResult>>,
}

#[derive(Resource, Clone)]
pub struct BluetoothResultSender(pub Sender<BluetoothResult>);
#[derive(Event)]
pub struct BluetoothActionEvent(pub BluetoothAction);

#[derive(Event)]
pub struct BluetoothResultEvent(pub BluetoothResult);

#[derive(Debug, Clone)]
pub enum BluetoothAction {
    ToggleBluetooth(bool),
    ListAvailableDevices,
    ConnectToDevice(String),
    DisconnectDevice(String),
    ListConnectedDevices,
}

#[derive(Debug)]
pub enum BluetoothResult {
    ToggleBluetooth(BluetoothStatus),
    ListAvailableDevices(Vec<BluetoothDevice>),
    ConnectDevice(bool),
    DisconnectDevice(bool),
    ListConnectedDevices(Vec<BluetoothDevice>),
    Error(ErrorType),
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ActionFailed {
        action: BluetoothAction,
        message: String,
    },
}

/// BluetoothManager plugin for Bevy
///
/// This plugin provides a resource for the `BluetoothService` which is
/// initialized asynchronously on startup. It also provides a system for enabling
/// bluetooth.
///
/// The `BluetoothService` is not available until the `init_bluetooth_service`
/// system has completed. This is checked with the `service_ready` function.
pub struct BluetoothPlugin;

impl Plugin for BluetoothPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BluetoothServiceResource { service: None })
            .insert_resource(BluetoothStatus {
                connected: false,
                last_error: None,
            })
            .add_event::<BluetoothActionEvent>()
            .add_event::<BluetoothResultEvent>()
            .add_systems(Startup, (init_bluetooth_service, setup_bluetooth_channel)) // Async task so temp move service result to static
            .add_systems(Update, poll_service_init) // Once a service is initialized, it will move service from static to resource
            .add_systems(
                Update,
                (
                    handle_bluetooth_action_events,
                    poll_bluetooth_action_result_events.after(handle_bluetooth_action_events),
                ),
            );
    }
}
/// Checks if the `BluetoothService` is ready (i.e. not None).
///
/// This is used to gate the execution of systems that depend on the service
/// being available.
fn service_ready(resource: Res<BluetoothServiceResource>) -> bool {
    resource.service.is_some()
}

static SERVICE_RESULT: LazyLock<Mutex<Option<BluetoothService>>> =
    LazyLock::new(|| Mutex::new(None));

/// Initializes the `BluetoothService` asynchronously on startup.
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
fn init_bluetooth_service() {
    IoTaskPool::get()
        .spawn(async {
            match BluetoothService::new().await {
                Ok(service) => {
                    let mut lock = SERVICE_RESULT.lock().unwrap();
                    *lock = Some(service);
                    info!("BluetoothService initialized!");
                }
                Err(e) => {
                    error!("Failed to initialize BluetoothService: {e}");
                }
            }
        })
        .detach();
}

// Polling system to move service from static to resource
fn poll_service_init(mut resource: ResMut<BluetoothServiceResource>) {
    let mut lock = SERVICE_RESULT.lock().unwrap();
    if let Some(service) = lock.take() {
        resource.service = Some(service);
    }
}

// In your plugin setup or a startup system:
fn setup_bluetooth_channel(mut commands: Commands) {
    let (tx, rx) = std::sync::mpsc::channel();
    commands.insert_resource(BluetoothResultReceiver {
        receiver: Mutex::new(rx),
    });
    commands.insert_resource(BluetoothResultSender(tx)); // You define this
}
/// Handles all `BluetoothActionEvent`s and sends the result to the channel.
///
/// This system should be run after the `BluetoothService` is initialized and
/// ready to use. It will spawn a new task for each action, which will send the
/// result of the action to the channel once the task is complete.
///
/// # Errors
///
/// If any action fails, an error message will be logged and the error will be
/// sent to the channel as a `BluetoothResult::Error`.
fn handle_bluetooth_action_events(
    mut events: EventReader<BluetoothActionEvent>,
    mut service: ResMut<BluetoothServiceResource>,
    sender: Res<BluetoothResultSender>,
) {
    let pool = AsyncComputeTaskPool::get();
    for event in events.read() {
        let BluetoothActionEvent(action) = event;
        match action {
            BluetoothAction::ToggleBluetooth(enable) => {
                info!("bluetooth action: toggle bluetooth: {enable}");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let enable = *enable;
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.toggle_bluetooth(true).await {
                            Ok(status) => {
                                info!("toggle wireless status: {status:?}");
                                let bluetooth_status = BluetoothStatus {
                                    connected: enable,
                                    last_error: None,
                                };
                                match result_sender
                                    .send(BluetoothResult::ToggleBluetooth(bluetooth_status))
                                {
                                    Ok(res) => {
                                        info!("sent toggle bluetooth status: {res:?}");
                                    }
                                    Err(err) => {
                                        error!("failed to send toggle bluetooth status: {err}");
                                    }
                                }
                            }
                            Err(err) => {
                                error!("failed to toggle bluetooth: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::ToggleBluetooth(enable),
                                    message: "Failed to toggle bluetooth".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send toggle bluetooth error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            BluetoothAction::ListAvailableDevices => {
                info!("bluetooth action: list available devices");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.get_available_devices(DISCOVER_DURATION).await {
                            Ok(devices) => {
                                if let Err(err) = result_sender
                                    .send(BluetoothResult::ListAvailableDevices(devices))
                                {
                                    error!("failed to send list available devices: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to list available devices: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::ListAvailableDevices,
                                    message: "Failed to list available devices".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send list available devices error: {err}");
                                }
                            }
                        };
                    })
                    .detach();
                }
            }
            BluetoothAction::ConnectToDevice(device_address) => {
                info!("bluetooth action: connect to device: {device_address}");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let device_address = device_address.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.connect(&device_address).await {
                            Ok(()) => {
                                info!("connected to device: {device_address}");
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::ConnectDevice(true))
                                {
                                    error!("failed to send connect to device: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to connect to device: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::ConnectToDevice(device_address),
                                    message: "Failed to connect to device".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send connect to device error: {err}");
                                }
                            }
                        };
                    })
                    .detach();
                }
            }
            BluetoothAction::DisconnectDevice(device_address) => {
                info!("bluetooth action: disconnect device");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let device_address = device_address.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.disconnect(&device_address).await {
                            Ok(()) => {
                                info!("disconnected from device");
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::DisconnectDevice(true))
                                {
                                    error!("failed to send disconnect device: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to disconnect from device: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::DisconnectDevice(device_address),
                                    message: "Failed to disconnect from device".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send disconnect device error: {err}");
                                }
                            }
                        };
                    })
                    .detach();
                }
            }
            BluetoothAction::ListConnectedDevices => {
                info!("bluetooth action: list connected devices");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.get_connected_devices().await {
                            Ok(devices) => {
                                if let Err(err) = result_sender
                                    .send(BluetoothResult::ListConnectedDevices(devices))
                                {
                                    error!("failed to send list connected devices: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to list connected devices: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::ListConnectedDevices,
                                    message: "Failed to list connected devices".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send list connected devices error: {err}");
                                }
                            }
                        };
                    })
                    .detach();
                }
            }
        } // Add more as needed
    }
}

/// Polls the internal event receiver for new events and writes them to the
/// `bluetooth_result_event_writer` as `BluetoothResultEvent`s.
///
/// This system is typically run once per frame and is used to dispatch events
/// from the Bluetooth service to the rest of the app.
///
/// The event receiver is accessed by a lock, and if the lock can't be acquired,
/// the system will print an error message and do nothing.
fn poll_bluetooth_action_result_events(
    mut bluetooth_result_event_writer: EventWriter<BluetoothResultEvent>,
    event_receiver: ResMut<BluetoothResultReceiver>,
) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            bluetooth_result_event_writer.write(BluetoothResultEvent(event));
        }
    } else {
        error!("failed to acquire receiver lock");
    }
}
