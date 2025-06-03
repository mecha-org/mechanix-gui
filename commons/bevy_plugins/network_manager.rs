use bevy::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, LazyLock, Mutex};
use bevy::log::{error, info};
use bevy::prelude::{Event, Resource};
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool};
use freedesktop_network_manager_client::interfaces::wireless::{AccessPointEvent, WifiState, WirelessNetworkInfo};
use freedesktop_network_manager_client::service::NetworkManagerService;

/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct NetworkManagerServiceResource {
    pub service: Option<NetworkManagerService>,
}

#[derive(Resource, Default)]
struct WifiEventChannelInitialized(bool);

#[derive(Resource, Debug, Clone)]
pub struct WifiStatus {
    pub connected: bool,
    pub last_error: Option<String>,
}
#[derive(Resource)]
pub struct NetworkResultReceiver {
    receiver: Mutex<Receiver<NetworkResult>>,
}

#[derive(Event)]
pub struct NetworkActionEvent(pub NetworkAction);

#[derive(Event)]
pub struct NetworkResultEvent(pub NetworkResult);

// Resource to hold the sender
#[derive(Resource, Clone)]
pub struct NetworkResultSender(pub Sender<NetworkResult>);

#[derive(Debug, Clone)]
pub enum NetworkAction {
    ToggleWifi(bool),
    ListNetworks,
    ConnectNetwork(String, Option<String>),
    ConnectToSavedNetwork(String),
    ForgetSavedNetwork(String),
    DisconnectNetwork,
    SubscribeDeviceEvents,
    SubscribeAccessPointsEvents,
}

#[derive(Debug)]
pub enum NetworkResult {
    ToggleWifi(WifiStatus),
    ListNetworks(Vec<WirelessNetworkInfo>),
    NetworkDeviceEvent(WifiState),
    NetworkAccessPointEvent(AccessPointEvent),
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
pub struct NetworkManagerServicePlugin;

impl Plugin for NetworkManagerServicePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NetworkManagerServiceResource { service: None })
            .insert_resource(WifiEventChannelInitialized(false))
            .insert_resource(WifiStatus {
                connected: false,
                last_error: None,
            })
            .add_event::<NetworkActionEvent>()
            .add_event::<NetworkResultEvent>()
            .add_systems(Startup, init_network_manager_service) // Async task so temp move service result to static
            .add_systems(Update, poll_service_init) // Once a service is initialized, it will move service from static to resource
            .add_systems(Update, setup_wifi_event_channel_async) // Async task so temp move result to static
            .add_systems(
                Update,
                (
                    handle_network_action_events,
                    poll_network_action_result_events.after(handle_network_action_events),
                ),
            );
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

// Startup system: create channel and spawn async/event producer
static WIFI_RX_RESULT: LazyLock<Mutex<Option<Receiver<WifiState>>>> =
    LazyLock::new(|| Mutex::new(None));


fn setup_wifi_event_channel_async(
    service_res: Res<NetworkManagerServiceResource>,
    mut wifi_channel_flag: ResMut<WifiEventChannelInitialized>,
) {
    if wifi_channel_flag.0 {
        // Already initialized, do nothing
        return;
    }
    if let Some(service) = &service_res.service {
        let service = service.clone();
        bevy::tasks::IoTaskPool::get()
            .spawn(async move {
                let receiver = service.subscribe_device_events().await;
                *WIFI_RX_RESULT.lock().unwrap() = Some(receiver);
            })
            .detach();
        wifi_channel_flag.0 = true; // Mark as initialized
    }
}


// Polling system to move service from static to resource
fn poll_service_init(mut resource: ResMut<NetworkManagerServiceResource>) {
    let mut lock = SERVICE_RESULT.lock().unwrap();
    if let Some(service) = lock.take() {
        resource.service = Some(service);
    }
}

fn handle_network_action_events(
    mut events: EventReader<NetworkActionEvent>,
    mut service: ResMut<NetworkManagerServiceResource>,
    mut commands: Commands,
) {
    let (tx, rx): (Sender<NetworkResult>, Receiver<NetworkResult>) = mpsc::channel();
    let pool = AsyncComputeTaskPool::get();
    for event in events.read() {
        let NetworkActionEvent(action) = event;
        match action {
            NetworkAction::ToggleWifi(enable) => {
                info!("network action: toggle wifi: {enable}");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let enable = *enable;
                    let result_sender = tx.clone();
                    pool.spawn(async move {
                        match service.toggle_wireless(enable).await {
                            Ok(status) => {
                                let wifi_status = WifiStatus {
                                    connected: enable,
                                    last_error: None,
                                };
                                if let Err(err) = result_sender.send(NetworkResult::ToggleWifi(wifi_status)) {
                                    error!("failed to send wifi status: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to toggle wifi: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ToggleWifi(enable),
                                    message: "Failed to toggle WiFi".to_string(),
                                };
                                if let Err(err) = result_sender.send(NetworkResult::Error(error_type)) {
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
                    let result_sender = tx.clone();
                    pool.spawn(async move {
                        match service.list_networks().await {
                            Ok(networks) => {
                                if let Err(err) = result_sender.send(NetworkResult::ListNetworks(networks)) {
                                    error!("failed to send networks: {err}");
                                }
                            }
                            Err(err) => {
                                error!("failed to list networks: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ListNetworks,
                                    message: "Failed to list networks".to_string(),
                                };
                                if let Err(err) = result_sender.send(NetworkResult::Error(error_type)) {
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
                    let result_sender = tx.clone();
                    pool.spawn(async move {
                        match service.connect_network(&ssid, &password).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to connect network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ConnectNetwork(ssid, password),
                                    message: "Failed to connect network".to_string(),
                                };
                                if let Err(err) = result_sender.send(NetworkResult::Error(error_type)) {
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
                    let result_sender = tx.clone();
                    pool.spawn(async move {
                        match service.connect_to_saved_network(&ssid).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to connect to saved network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ConnectToSavedNetwork(ssid),
                                    message: "Failed to connect to saved network".to_string(),
                                };
                                if let Err(err) = result_sender.send(NetworkResult::Error(error_type)) {
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
                    let result_sender = tx.clone();
                    pool.spawn(async move {
                        match service.forget_saved_network(&ssid).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to forget saved network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::ForgetSavedNetwork(ssid),
                                    message: "Failed to forget saved network".to_string(),
                                };
                                if let Err(err) = result_sender.send(NetworkResult::Error(error_type)) {
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
                    let result_sender = tx.clone();
                    pool.spawn(async move {
                        match service.disconnect_network().await {
                            Ok(_) => {}
                            Err(err) => {
                                error!("failed to disconnect network: {err}");
                                let error_type = ErrorType::ActionFailed {
                                    action: NetworkAction::DisconnectNetwork,
                                    message: "Failed to disconnect network".to_string(),
                                };
                                if let Err(err) = result_sender.send(NetworkResult::Error(error_type)) {
                                    error!("failed to send disconnect network error: {err}");
                                }
                            }
                        }
                    })
                        .detach();
                }
            }
            NetworkAction::SubscribeDeviceEvents => {
                info!("network action: subscribe device events");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let result_sender = tx.clone();
                    bevy::tasks::IoTaskPool::get()
                        .spawn(async move {
                            let receiver = service.subscribe_device_events().await;
                            while let Ok(device_event) = receiver.try_recv() {
                                if let Err(err) = result_sender.send(NetworkResult::NetworkDeviceEvent(device_event)) {
                                    error!("failed to send device event: {err}");
                                }
                            }
                        }).detach();
                }
            }
            NetworkAction::SubscribeAccessPointsEvents => {
                info!("network action: subscribe access points events");
                if let Some(service) = &service.service {
                    let service = service.clone();
                    let result_sender = tx.clone();
                    bevy::tasks::IoTaskPool::get()
                        .spawn(async move {
                            let receiver = service.subscribe_access_point_events().await;
                            while let Ok(access_point_event_result) = receiver.try_recv() {
                                let access_point_event = match access_point_event_result{
                                    Ok(access_point_event) => access_point_event,
                                    Err(err) => {
                                        error!("error in access point event: {err}");
                                        continue;
                                    }
                                };
                                if let Err(err) = result_sender.send(NetworkResult::NetworkAccessPointEvent(access_point_event)) {
                                    error!("failed to send access point event: {err}");
                                }
                            }
                        }).detach();
                }
            }
        } // Add more as needed
    }
    commands.insert_resource(NetworkResultReceiver {
        receiver: Mutex::new(rx),
    });
}

// Polling system to insert write error into an event
fn poll_network_action_result_events(
    mut network_result_event_writer: EventWriter<NetworkResultEvent>,
    event_receiver: ResMut<NetworkResultReceiver>,
) {
    let receiver = event_receiver.receiver.lock().unwrap();
    while let Ok(network_result) = receiver.try_recv() {
        network_result_event_writer.write(NetworkResultEvent(network_result));
    }
}