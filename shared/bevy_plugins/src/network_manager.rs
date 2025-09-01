use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::{IoTaskPool};
use networkmanager::interfaces::wireless::{ NMState, WirelessNetworkInfo,
};
use networkmanager::service::NetworkManagerService;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, LazyLock, Mutex};

/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct NetworkManagerServiceResource {
    pub service: Option<NetworkManagerService>,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct WirelessEnabled(pub bool);

#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveNetworkStrength(pub u8);

#[derive(Resource)]
pub struct NetworkResultReceiver {
    receiver: Mutex<Receiver<NetworkResult>>,
}

#[derive(Default)]
pub struct WirelessStatusReceiver(pub Option<mpsc::Receiver<bool>>);

#[derive(Default)]
pub struct ActiveNetworkStrengthReceiver(pub Option<mpsc::Receiver<u8>>);

#[derive(Default)]
pub struct DeviceStateReceiver(pub Option<mpsc::Receiver<NMState>>);

#[derive(Resource, Clone)]
pub struct NetworkResultSender(pub Sender<NetworkResult>);

#[derive(Resource, Default)]
pub struct NetworkManagerState {
    pub initialized: bool,
    pub stream_started: bool,
}
#[derive(Resource, Default, Debug)]
pub struct NetworkManagerDeviceStatus(pub NMState);
#[derive(Event)]
pub struct NetworkActionEvent(pub NetworkAction);

#[derive(Debug, Clone)]
pub enum NetworkAction {
    ToggleWifi(bool),
    ListNetworks,
    ConnectNetwork(String, Option<String>),
    ConnectToSavedNetwork(String),
    ForgetSavedNetwork(String),
    DisconnectNetwork,
    StreamWirelessEnabledStatus,
    StreamActiveNetworkStrength,
    StreamDeviceEvents,
}

#[derive(Debug)]
pub enum NetworkResult {
    ToggleWifi(WirelessEnabled),
    ListNetworks(Vec<WirelessNetworkInfo>),
    WirelessEnabled(bool),
    NetworkDeviceEvent(NMState),
    Error(ErrorType),
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ActionFailed {
        action: NetworkAction,
        message: String,
    },
}

/// NetworkManager plugin for Bevy
///
/// This plugin provides a resource for the `NetworkManagerService` which is
/// initialized asynchronously on startup. It also provides a system for enabling
/// WiFi.
///
/// The `NetworkManagerService` is not available until the `init_network_manager_service`
/// system has completed. This is checked with the `service_ready` function.
pub struct NetworkManagerPlugin;

impl Plugin for NetworkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManagerServiceResource { service: None })
            .insert_resource(WirelessEnabled::default())
            .insert_resource(NetworkManagerState::default())
            .insert_resource(NetworkManagerDeviceStatus::default())
            .insert_resource(ActiveNetworkStrength::default())
            .insert_non_send_resource(WirelessStatusReceiver::default())
            .insert_non_send_resource(ActiveNetworkStrengthReceiver::default())
            .insert_non_send_resource(DeviceStateReceiver::default())
            .add_event::<NetworkActionEvent>()
            .add_systems(
                Startup,
                (init_network_manager_service, setup_network_channel),
            ) // Async task so temp move service result to static
            .add_systems(
                Update,
                (
                    poll_service_init,
                    start_stream_if_service_ready.after(poll_service_init),
                ),
            )
            .add_systems(Update, handle_network_action_events)
            .add_systems(
                Update,
                start_dependent_streams.run_if(resource_changed::<NetworkManagerDeviceStatus>),
            )
            .add_systems(Update, poll_wifi_status.after(handle_network_action_events))
            .add_systems(Update, poll_active_network_strength)
            .add_systems(Update, poll_device_state);
    }
}

fn poll_wifi_status(
    wifi_status_receiver: NonSendMut<WirelessStatusReceiver>,
    mut wifi_state: ResMut<WirelessEnabled>,
) {
    if let Some(receiver) = &wifi_status_receiver.0 {
        if let Ok(is_enabled) = receiver.try_recv() {
            wifi_state.0 = is_enabled;
        }
    }
}
fn poll_active_network_strength(
    active_network_strength_receiver: NonSendMut<ActiveNetworkStrengthReceiver>,
    mut active_network_strength: ResMut<ActiveNetworkStrength>,
) {
    if let Some(receiver) = &active_network_strength_receiver.0 {
        if let Ok(strength) = receiver.try_recv() {
            active_network_strength.0 = strength;
        }
    }
}
fn poll_device_state(
    device_state_receiver: NonSendMut<DeviceStateReceiver>,
    mut device_state: ResMut<NetworkManagerDeviceStatus>,
) {
    if let Some(receiver) = &device_state_receiver.0 {
        if let Ok(nm_state) = receiver.try_recv() {
            device_state.0 = nm_state;
        }
    }
}
fn start_stream_if_service_ready(
    mut state: ResMut<NetworkManagerState>,
    service_res: Res<NetworkManagerServiceResource>,
    mut events: EventWriter<NetworkActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if !state.stream_started {
        if let Some(service) = &service_res.service {
            println!("Starting stream...");
            events.write(NetworkActionEvent(
                NetworkAction::StreamWirelessEnabledStatus,
            ));
            events.write(NetworkActionEvent(NetworkAction::StreamDeviceEvents));
            state.stream_started = true;
        }
    }
}

fn start_dependent_streams(
    state: ResMut<NetworkManagerDeviceStatus>,
    mut events: EventWriter<NetworkActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if state.0 == NMState::ConnectedGlobal {
        events.write(NetworkActionEvent(
            NetworkAction::StreamActiveNetworkStrength,
        ));
        // events.write(NetworkActionEvent(NetworkAction::StreamAccessPointsEvents));
    }
}
/// Checks if the `NetworkManagerService` is ready (i.e. not None).
///
/// This is used to gate the execution of systems that depend on the service
/// being available.
fn service_ready(resource: Res<NetworkManagerServiceResource>) -> bool {
    resource.service.is_some()
}

static SERVICE_RESULT: LazyLock<Mutex<Option<NetworkManagerService>>> =
    LazyLock::new(|| Mutex::new(None));

/// This plugin provides a resource for the `NetworkManagerService` which is
/// initialized asynchronously on startup. It also provides a system for enabling
/// WiFi.
///
/// The `NetworkManagerService` is not available until the `init_network_manager_service`
/// system has completed. This is checked with the `service_ready` function.
fn init_network_manager_service() {
    IoTaskPool::get()
        .spawn(async {
            match NetworkManagerService::new().await {
                Ok(service) => {
                    let mut lock = SERVICE_RESULT.lock().unwrap();
                    *lock = Some(service);
                    info!("NetworkManagerService initialized!");
                }
                Err(e) => {
                    error!("Failed to initialize NetworkManagerService: {e}");
                }
            }
        })
        .detach();
}

// Polling system to move service from static to resource
fn poll_service_init(mut resource: ResMut<NetworkManagerServiceResource>) {
    let mut lock = SERVICE_RESULT.lock().unwrap();
    if let Some(service) = lock.take() {
        resource.service = Some(service);
    }
}

// In your plugin setup or a startup system:
fn setup_network_channel(mut commands: Commands) {
    let (tx, rx) = std::sync::mpsc::channel();
    commands.insert_resource(NetworkResultReceiver {
        receiver: Mutex::new(rx),
    });
    commands.insert_resource(NetworkResultSender(tx)); // You define this
}
fn handle_network_action_events(
    mut action_events: EventReader<NetworkActionEvent>,
    mut service: ResMut<NetworkManagerServiceResource>,
    mut wifi_status_receiver: NonSendMut<WirelessStatusReceiver>,
    mut active_network_strength_receiver: NonSendMut<ActiveNetworkStrengthReceiver>,
    mut device_state_receiver: NonSendMut<DeviceStateReceiver>,
    sender: Res<NetworkResultSender>,
) {
    let pool = IoTaskPool::get();
    for event in action_events.read() {
        let NetworkActionEvent(action) = event;
        println!("network action: {action:?}");
        match action {
            NetworkAction::ToggleWifi(enable) => {
                info!("network action: toggle wifi: {enable}");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let enable = *enable;
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.toggle_wireless(enable).await {
                            Ok(status) => {}
                            Err(err) => {
                                error!("failed to toggle wifi: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ToggleWifi(enable),
                                    message: "Failed to toggle WiFi".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(NetworkResult::Error(error_type))
                                {
                                    error!("failed to send toggle wifi error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            NetworkAction::ListNetworks => {
                info!("network action: list networks");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.list_networks().await {
                            Ok(networks) => {
                                if let Err(err) =
                                    result_sender.send(NetworkResult::ListNetworks(networks))
                                {
                                    error!("failed to send networks: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to list networks: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ListNetworks,
                                    message: "Failed to list networks".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(NetworkResult::Error(error_type))
                                {
                                    error!("failed to send list networks error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            NetworkAction::ConnectNetwork(ssid, password) => {
                info!("network action: connect network");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let ssid = ssid.clone();
                    let password = password.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.connect_network(&ssid, &password).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to connect network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ConnectNetwork(ssid, password),
                                    message: "Failed to connect network".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(NetworkResult::Error(error_type))
                                {
                                    error!("failed to send connect network error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            NetworkAction::ConnectToSavedNetwork(ssid) => {
                info!("network action: connect to saved network");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let ssid = ssid.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.connect_to_saved_network(&ssid).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to connect to saved network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ConnectToSavedNetwork(ssid),
                                    message: "Failed to connect to saved network".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(NetworkResult::Error(error_type))
                                {
                                    error!("failed to send connect to saved network error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            NetworkAction::ForgetSavedNetwork(ssid) => {
                info!("network action: forget saved network");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let ssid = ssid.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.forget_saved_network(&ssid).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to forget saved network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ForgetSavedNetwork(ssid),
                                    message: "Failed to forget saved network".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(NetworkResult::Error(error_type))
                                {
                                    error!("failed to send forget saved network error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            NetworkAction::DisconnectNetwork => {
                info!("network action: disconnect network");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.disconnect_network().await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to disconnect network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::DisconnectNetwork,
                                    message: "Failed to disconnect network".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(NetworkResult::Error(error_type))
                                {
                                    error!("failed to send disconnect network error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            NetworkAction::StreamWirelessEnabledStatus => {
                info!("network action: subscribe wireless enabled status");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver: mpsc::Receiver<bool> =
                        pollster::block_on(service.stream_wireless_enabled_status());
                    wifi_status_receiver.0 = Some(receiver);
                }
            }
            NetworkAction::StreamActiveNetworkStrength => {
                info!("network action: active network strength");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver: mpsc::Receiver<u8> =
                        pollster::block_on(service.stream_active_network_strength());
                    active_network_strength_receiver.0 = Some(receiver);
                }
            }
            NetworkAction::StreamDeviceEvents => {
                info!("network action: stream device events");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let receiver: mpsc::Receiver<NMState> =
                        pollster::block_on(service.stream_device_events());
                    device_state_receiver.0 = Some(receiver);
                }
            }
        } // Add more as needed
    }
}
