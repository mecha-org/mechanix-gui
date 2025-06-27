use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool};
use freedesktop_network_manager_client::interfaces::wireless::{
    AccessPointEvent, NMState, WirelessNetworkInfo,
};
use freedesktop_network_manager_client::service::NetworkManagerService;
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
    StreamDeviceEvents,
    StreamAccessPointsEvents,
    StreamWirelessEnabledStatus,
    StreamActiveNetworkStrength,
}

#[derive(Debug)]
pub enum NetworkResult {
    ToggleWifi(WirelessEnabled),
    ListNetworks(Vec<WirelessNetworkInfo>),
    NetworkDeviceEvent(NMState),
    NetworkAccessPointEvent(AccessPointEvent),
    NetworkStrength(u8),
    WirelessEnabled(bool),
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
            .insert_resource(NetworkManagerState::default())
            .insert_resource(WirelessEnabled::default())
            .insert_resource(ActiveNetworkStrength::default())
            .insert_resource(NetworkManagerDeviceStatus::default())
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
            .add_systems(
                Update,
                (
                    handle_network_action_events,
                    poll_network_action_result_events.after(handle_network_action_events),
                ),
            )
            .add_systems(Update, start_dependent_streams.run_if(resource_changed::<NetworkManagerDeviceStatus>));
    }
}


fn start_dependent_streams(
    state: ResMut<NetworkManagerDeviceStatus>,
    mut events: EventWriter<NetworkActionEvent>,
) {
    // Only start once, and only when the service is initialized
    if state.0 == NMState::ConnectedGlobal {
        events.write(NetworkActionEvent(NetworkAction::StreamActiveNetworkStrength));
        events.write(NetworkActionEvent(NetworkAction::StreamAccessPointsEvents));
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
    sender: Res<NetworkResultSender>,
) {
    let pool = AsyncComputeTaskPool::get();
    for event in action_events.read() {
        let NetworkActionEvent(action) = event;
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
            NetworkAction::StreamDeviceEvents => {
                info!("network action: subscribe device events");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver: mpsc::Receiver<NMState> =
                                service.stream_device_events().await;
                            while let Ok(device_event) = receiver.recv() {
                                if let Err(err) = result_sender
                                    .send(NetworkResult::NetworkDeviceEvent(device_event))
                                {
                                    error!("failed to send device event: {err}");
                                }
                            }
                        })
                        .detach();
                }
            }
            NetworkAction::StreamAccessPointsEvents => {
                info!("network action: subscribe access points events");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver = service.stream_access_point_events().await;
                            while let Ok(access_point_event_result) = receiver.recv() {
                                let access_point_event = match access_point_event_result {
                                    Ok(access_point_event) => access_point_event,
                                    Err(err) => {
                                        error!("error in access point event: {err}");
                                        continue;
                                    }
                                };
                                if let Err(err) = result_sender.send(
                                    NetworkResult::NetworkAccessPointEvent(access_point_event),
                                ) {
                                    error!("failed to send access point event: {err}");
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
                    let sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver: mpsc::Receiver<bool> =
                                service.stream_wireless_enabled_status().await;
                            while let Ok(is_enabled) = receiver.recv() {
                                if let Err(err) =
                                    sender.send(NetworkResult::WirelessEnabled(is_enabled))
                                {
                                    error!("failed to send wireless enabled status: {err}");
                                }
                            }
                        })
                        .detach();
                }
            }
            NetworkAction::StreamActiveNetworkStrength => {
                info!("network action: stream active network strength");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let sender = sender.0.clone();
                    IoTaskPool::get()
                        .spawn(async move {
                            let receiver: mpsc::Receiver<u8> =
                                service.stream_active_network_strength().await;
                            while let Ok(network_strength) = receiver.recv() {
                                if let Err(err) =
                                    sender.send(NetworkResult::NetworkStrength(network_strength))
                                {
                                    error!("failed to send wireless enabled status: {err}");
                                }
                            }
                        })
                        .detach();
                }
            }
        } // Add more as needed
    }
}

// Polling system to insert write error into an event
fn poll_network_action_result_events(event_receiver: ResMut<NetworkResultReceiver>, mut wifi_state: ResMut<WirelessEnabled>, mut network_strength: ResMut<ActiveNetworkStrength>, mut network_device_status: ResMut<NetworkManagerDeviceStatus>) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            match event {
                NetworkResult::WirelessEnabled(is_enabled) => {
                    info!("network result: wireless enabled status: {is_enabled}");
                    wifi_state.0 = is_enabled;
                }
                NetworkResult::ListNetworks(networks) => {
                    info!("network result: list of available networks: {:?}", networks);
                }
                NetworkResult::NetworkStrength(strength) => {
                    info!("network result: active network strength: {strength}");
                    network_strength.0 = strength;
                }
                NetworkResult::NetworkDeviceEvent(device_event) => {
                    network_device_status.0 = device_event;
                }
                _ => {}
            }
        }
    } else {
        println!("Failed to acquire receiver lock");
    }
}
