use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::IoTaskPool;
use bluez::interfaces::device::BluetoothDevice;
use bluez::service::{BluetoothEvent, BluetoothService};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, LazyLock, Mutex};

const DISCOVER_DURATION: std::time::Duration = std::time::Duration::from_secs(5);
/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct BluetoothServiceResource {
    pub service: Option<BluetoothService>,
}

#[derive(Resource)]
pub struct BluetoothResultReceiver {
    receiver: Mutex<Receiver<BluetoothResult>>,
}

#[derive(Resource, Clone)]
pub struct BluetoothResultSender(pub Sender<BluetoothResult>);
#[derive(Resource, Clone, Default, Debug)]
pub struct BluetoothEnabledStatus(pub bool);

#[derive(Resource, Clone, Default)]
pub struct ConnectedDeviceCount(pub i32);

#[derive(Resource, Default)]
pub struct BluetoothState {
    pub initialized: bool,
    pub stream_started: bool,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct ListPairedDevices(pub Vec<BluetoothDevice>);

#[derive(Default)]
pub struct PoweredStatusReceiver(pub Option<mpsc::Receiver<bool>>);

#[derive(Default)]
pub struct BluetoothEventReceiver(pub Option<mpsc::Receiver<BluetoothEvent>>);

#[derive(Event)]
pub struct BluetoothActionEvent(pub BluetoothAction);

#[derive(Debug, Clone)]
pub enum BluetoothAction {
    ToggleBluetooth(bool),
    ListAvailableDevices,
    ConnectToDevice(String),
    DisconnectDevice(String),
    ListConnectedDevices,
    ConnectedDeviceCount,
    ListPairedDevices,
    StreamPoweredStatus,
    StreamBluetoothEvent,
}

#[derive(Debug)]
pub enum BluetoothResult {
    BluetoothStatus(bool),
    BluetoothEvent(BluetoothEvent),
    ListAvailableDevices(Vec<BluetoothDevice>),
    ListPairedDevices(Vec<BluetoothDevice>),
    ConnectDevice(bool),
    DisconnectDevice(bool),
    ConnectedDeviceCount(i32),
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
pub struct BluetoothPlugin;

impl Plugin for BluetoothPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BluetoothServiceResource { service: None })
            .insert_resource(BluetoothState::default())
            .insert_resource(BluetoothEnabledStatus::default())
            .insert_resource(ConnectedDeviceCount::default())
            .insert_resource(ListPairedDevices::default())
            .insert_non_send_resource(PoweredStatusReceiver::default())
            .insert_non_send_resource(BluetoothEventReceiver::default())
            .add_event::<BluetoothActionEvent>()
            .add_systems(Startup, (init_bluetooth_service, setup_bluetooth_channel)) // Async task so temp move service result to static
            .add_systems(
                Update,
                (
                    poll_service_init,
                    start_initial_streams_if_service_ready.after(poll_service_init),
                ),
            )
            .add_systems(
                Update,
                (
                    handle_bluetooth_action_events,
                    poll_bluetooth_action_result_events.after(handle_bluetooth_action_events),
                ),
            )
            .add_systems(
                Update,
                start_dependent_streams.run_if(resource_changed::<BluetoothEnabledStatus>),
            )
            .add_systems(Update, (poll_power_status, poll_bluetooth_event));
    }
}

fn poll_power_status(
    power_status_receiver: NonSendMut<PoweredStatusReceiver>,
    mut bluetooth_status: ResMut<BluetoothEnabledStatus>,
) {
    if let Some(power_status_receiver) = (&power_status_receiver.0) {
        if let Ok(power_status) = power_status_receiver.try_recv() {
            info!("bluetooth power status received: {power_status}");
            bluetooth_status.0 = power_status;
        }
    }
}
fn poll_bluetooth_event(
    mut connected_device_count: ResMut<ConnectedDeviceCount>,
    event_receiver: NonSendMut<BluetoothEventReceiver>,
) {
    if let Some(event) = &event_receiver.0 {
        if let Ok(bluetooth_event) = event.try_recv() {
            match bluetooth_event {
                BluetoothEvent::DeviceAdded => {
                    connected_device_count.0 = connected_device_count.0 + 1;
                }
                BluetoothEvent::DeviceRemoved => {
                    connected_device_count.0 = connected_device_count.0 - 1;
                }
            }
        }
    }
}
fn start_dependent_streams(
    enabled_status: ResMut<BluetoothEnabledStatus>,
    mut events: EventWriter<BluetoothActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if enabled_status.0 {
        events.write(BluetoothActionEvent(BluetoothAction::StreamBluetoothEvent));
    }
}
fn start_initial_streams_if_service_ready(
    mut state: ResMut<BluetoothState>,
    service_res: Res<BluetoothServiceResource>,
    mut events: EventWriter<BluetoothActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if !state.stream_started {
        if let Some(service) = &service_res.service {
            println!("Bluetooth Starting stream...");
            events.write(BluetoothActionEvent(BluetoothAction::StreamPoweredStatus));
            state.stream_started = true;
        }
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
/// This system is spawned as ComputeTaskPool task and will detach itself once
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
    mut powered_status_receiver: NonSendMut<PoweredStatusReceiver>,
    mut event_receiver: NonSendMut<BluetoothEventReceiver>,
    sender: Res<BluetoothResultSender>,
) {
    let pool = IoTaskPool::get();
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
            BluetoothAction::ListPairedDevices => {
                info!("bluetooth action: list paired devices");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service
                            .get_available_devices(std::time::Duration::from_secs(1))
                            .await
                        {
                            Ok(devices) => {
                                let mut devices: Vec<BluetoothDevice> = devices
                                    .iter()
                                    .filter(|device| device.paired || device.connected)
                                    .cloned()
                                    .collect();

                                devices.sort_by_key(|d| !(d.connected));

                                if let Err(err) =
                                    result_sender.send(BluetoothResult::ListPairedDevices(devices))
                                {
                                    error!("failed to send list paired devices: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to list paired devices: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::ListPairedDevices,
                                    message: "Failed to list paired devices".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send list paired devices error: {err}");
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
            BluetoothAction::StreamPoweredStatus => {
                info!("bluetooth action: stream powered status");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver: mpsc::Receiver<bool> =
                        pollster::block_on(service.stream_bluetooth_enabled_status());
                    info!("bluetooth power status receiver received");
                    powered_status_receiver.0 = Some(receiver);
                }
            }
            BluetoothAction::StreamBluetoothEvent => {
                info!("bluetooth action: stream connected status");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver: Receiver<BluetoothEvent> =
                        pollster::block_on(service.stream_bluetooth_device_status());
                    event_receiver.0 = Some(receiver);
                }
            }
            BluetoothAction::ConnectedDeviceCount => {
                info!("bluetooth action: connected device count");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.get_connected_devices().await {
                            Ok(devices) => {
                                if let Err(err) = result_sender.send(
                                    BluetoothResult::ConnectedDeviceCount(devices.len() as i32),
                                ) {
                                    error!("failed to send connected devices count : {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to get connected devices for count: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: BluetoothAction::ConnectedDeviceCount,
                                    message: "Failed to list connected devices".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(BluetoothResult::Error(error_type))
                                {
                                    error!("failed to send error while getting connecte device list: {err}");
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
    event_receiver: ResMut<BluetoothResultReceiver>,
    mut bluetooth_status: ResMut<BluetoothEnabledStatus>,
    mut connected_device_count: ResMut<ConnectedDeviceCount>,
    mut bluetooth_paired_devices: ResMut<ListPairedDevices>,
) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            match event {
                BluetoothResult::BluetoothStatus(status) => {
                    info!("bluetooth status updated: {status}");
                    bluetooth_status.0 = status;
                }
                BluetoothResult::BluetoothEvent(event) => match event {
                    BluetoothEvent::DeviceAdded => {}
                    BluetoothEvent::DeviceRemoved => {}
                },
                BluetoothResult::ListConnectedDevices(devices) => {
                    info!("list connected devices: total {:?} ", devices.len());
                }
                BluetoothResult::ListPairedDevices(devices) => {
                    info!("list of paired devices: {:?}", devices);
                    bluetooth_paired_devices.0 = devices;
                }
                BluetoothResult::ConnectedDeviceCount(device_count) => {
                    connected_device_count.0 = device_count;
                }
                _ => {
                    // ignore other events
                }
            }
        }
    } else {
        error!("failed to acquire receiver lock");
    }
}
