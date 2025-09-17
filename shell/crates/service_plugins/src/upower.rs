use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::IoTaskPool;
use upower::interfaces::device::{
    BatteryLevel as UPowerBatteryLevel, BatteryState,
};
use upower::service::UPowerService;
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
pub struct DevicePercentage(pub f64);

#[derive(Resource, Default)]
pub struct BatteryLevel(pub UPowerBatteryLevel);

#[derive(Default)]
pub struct DeviceStateReceiver(pub Option<mpsc::Receiver<BatteryState>>);

#[derive(Default)]
pub struct DevicePercentageReceiver(pub Option<mpsc::Receiver<f64>>);

#[derive(Event)]
pub struct UPowerActionEvent(pub UPowerAction);

#[derive(Debug, Clone)]
pub enum UPowerAction {
    StreamDeviceState,
    StreamDevicePercentage,
    // StreamBatteryLevel,
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
            .insert_non_send_resource(DeviceStateReceiver::default())
            .insert_non_send_resource(DevicePercentageReceiver::default())
            .add_systems(Startup, (init_upower_service, setup_upower_channel)) // Async task so temp move service result to static // Once a service is initialized, it will move service from static to resource
            .add_systems(
                Update,
                (
                    poll_service_init,
                    start_initial_streams_if_service_ready.after(poll_service_init),
                ),
            )
            .add_systems(Update, handle_upower_action_events)
            .add_systems(Update, (poll_device_state, poll_device_percentage));
    }
}

fn poll_device_state(
    mut device_state_recv_res: NonSendMut<DeviceStateReceiver>,
    mut device_state_res: ResMut<DeviceState>,
) {
    if let Some(device_state_recv) = &mut device_state_recv_res.0 {
        if let Ok(battery_state) = device_state_recv.try_recv() {
            info!("upower device state: {:?}", battery_state);
            device_state_res.0 = battery_state;
        }
    }
}

fn poll_device_percentage(
    mut device_percentage_recv_res: NonSendMut<DevicePercentageReceiver>,
    mut device_percentage_res: ResMut<DevicePercentage>,
) {
    if let Some(device_percentage_recv) = &mut device_percentage_recv_res.0 {
        if let Ok(device_percentage) = device_percentage_recv.try_recv() {
            info!("upower device percentage: {:?}", device_percentage);
            device_percentage_res.0 = device_percentage;
        }
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
            // events.write(UPowerActionEvent(UPowerAction::StreamBatteryLevel));
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

static SERVICE_RESULT: LazyLock<Mutex<Option<UPowerService>>> = LazyLock::new(|| Mutex::new(None));

/// Initializes the `UPowerService` asynchronously on startup.
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
    mut device_state_recv_res: NonSendMut<DeviceStateReceiver>,
    mut device_percentage_recv_res: NonSendMut<DevicePercentageReceiver>,
) {
    let pool = IoTaskPool::get();
    for event in events.read() {
        let UPowerActionEvent(action) = event;
        match action {
            UPowerAction::StreamDeviceState => {
                info!("upower action: stream device state");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver: Receiver<BatteryState> =
                        pollster::block_on(service.stream_device_state());
                    device_state_recv_res.0 = Some(receiver);
                }
            }
            UPowerAction::StreamDevicePercentage => {
                info!("upower action: stream device percentage");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver = pollster::block_on(service.stream_device_percentage());
                    device_percentage_recv_res.0 = Some(receiver);
                }
            } // UPowerAction::StreamBatteryLevel => {
            //     info!("upower action: stream battery level");
            //     if let Some(service) = &service.service {
            //         let service = service.clone();
            //         let sender = sender.0.clone();
            //         pool.spawn(async move {
            //             let receiver: Receiver<UPowerBatteryLevel> =
            //                 service.stream_battery_level().await;
            //             let sender2 = sender.clone();
            //             IoTaskPool::get()
            //                 .spawn(async move {
            //                     while let Ok(status) = receiver.recv() {
            //                         if let Err(err) =
            //                             sender2.send(UPowerResult::BatteryLevel(status))
            //                         {
            //                             error!("failed to send upower batter level: {err}");
            //                         }
            //                     }
            //                 })
            //                 .detach();
            //         })
            //         .detach();
            //     }
            // }
        }
    }
}
