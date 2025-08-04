use crate::universal_search::ErrorType::ActionFailed;
use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool};
use mxsearch::service::MxSearchService;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{LazyLock, Mutex};

/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct MxSearchServiceResource {
    pub service: Option<MxSearchService>,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct WirelessEnabled(bool);
#[derive(Resource)]
pub struct MxSearchResultReceiver {
    receiver: Mutex<Receiver<MxSearchResult>>,
}

#[derive(Resource, Clone)]
pub struct MxSearchResultSender(pub Sender<MxSearchResult>);

#[derive(Event)]
pub struct MxSearchActionEvent(pub MxSearchAction);

#[derive(Debug, Clone)]
pub enum MxSearchAction {
    ListApplications,
    SearchApplications(String),
    SearchFiles(String),
}

#[derive(Debug)]
pub enum MxSearchResult {
    Error(ErrorType),
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    ActionFailed {
        action: MxSearchAction,
        message: String,
    },
}

/// Plugin to search files and applications
pub struct UniversalSearchPlugin;

impl Plugin for UniversalSearchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MxSearchServiceResource { service: None })
            .add_event::<MxSearchActionEvent>()
            .add_systems(Startup, (init_mxsearch_service, setup_channel)) // Async task so temp move service result to static
            .add_systems(
                Update,
                (
                    handle_action_events,
                    poll_action_result_events.after(handle_action_events),
                ),
            );
    }
}

static SERVICE_RESULT: LazyLock<Mutex<Option<MxSearchService>>> =
    LazyLock::new(|| Mutex::new(None));

/// Initializes the `MxSearchService` asynchronously using `IoTaskPool`.
///
/// If initialization succeeds, the `MxSearchService` is stored in the static
/// `SERVICE_RESULT` and a message is logged indicating success. If it fails, an
/// error is logged.
///
/// This function is used by the `UniversalSearchPlugin` to initialize the service
/// on startup.
fn init_mxsearch_service() {
    IoTaskPool::get()
        .spawn(async {
            match MxSearchService::new().await {
                Ok(service) => {
                    let mut lock = SERVICE_RESULT.lock().unwrap();
                    *lock = Some(service);
                    info!("MxSearchService initialized!");
                }
                Err(e) => {
                    error!("Failed to initialize MxSearchService: {e}");
                }
            }
        })
        .detach();
}

// Polling system to move service from static to resource
fn poll_service_init(mut resource: ResMut<MxSearchServiceResource>) {
    let mut lock = SERVICE_RESULT.lock().unwrap();
    if let Some(service) = lock.take() {
        resource.service = Some(service);
    }
}

// In your plugin setup or a startup system:
fn setup_channel(mut commands: Commands) {
    let (tx, rx) = std::sync::mpsc::channel();
    commands.insert_resource(MxSearchResultReceiver {
        receiver: Mutex::new(rx),
    });
    commands.insert_resource(MxSearchResultSender(tx)); // You define this
}
fn handle_action_events(
    mut action_events: EventReader<MxSearchActionEvent>,
    mut service: ResMut<MxSearchServiceResource>,
    sender: Res<MxSearchResultSender>,
) {
    let pool = AsyncComputeTaskPool::get();
    for event in action_events.read() {
        let MxSearchActionEvent(action) = event;
        match action {
            MxSearchAction::ListApplications => {
                info!("list applications");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    pool.spawn(async move {
                        match service.list_applications().await {
                            Ok(status) => {}
                            Err(err) => {
                                error!("failed to list applications: {err}");
                                let error_type = ActionFailed {
                                    action: MxSearchAction::ListApplications,
                                    message: "Failed to get list of applications".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(MxSearchResult::Error(error_type))
                                {
                                    error!("failed to send list applications error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            MxSearchAction::SearchApplications(query) => {
                info!("search applications: {query}");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    let query = query.clone();
                    pool.spawn(async move {
                        match service.search_applications(&query).await {
                            Ok(status) => {}
                            Err(err) => {
                                error!("failed to list applications: {err}");
                                let error_type = ActionFailed {
                                    action: MxSearchAction::ListApplications,
                                    message: "Failed to get list of applications".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(MxSearchResult::Error(error_type))
                                {
                                    error!("failed to send list applications error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
            MxSearchAction::SearchFiles(query) => {
                info!("search files: {query}");
                if let Some(service) = &mut service.service {
                    let service = service.clone();
                    let result_sender = sender.0.clone();
                    let query = query.clone();
                    pool.spawn(async move {
                        match service.search_files(&query).await {
                            Ok(status) => {}
                            Err(err) => {
                                error!("failed to search files: {err}");
                                let error_type = ActionFailed {
                                    action: MxSearchAction::ListApplications,
                                    message: "Failed to search files".to_string(),
                                };
                                if let Err(err) =
                                    result_sender.send(MxSearchResult::Error(error_type))
                                {
                                    error!("failed to send search files error: {err}");
                                }
                            }
                        }
                    })
                    .detach();
                }
            }
        }
    }
}

// Polling system to insert write error into an event
fn poll_action_result_events(
    event_receiver: ResMut<MxSearchResultReceiver>,
    mut wifi_state: ResMut<WirelessEnabled>,
) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            match event {
                _ => {}
            }
        }
    } else {
        println!("Failed to acquire receiver lock");
    }
}
