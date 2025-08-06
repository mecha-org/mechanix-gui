use crate::mxsearch::ErrorType::ActionFailed;
use bevy::ecs::system::SystemId;
use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::prelude::{Event, Resource};
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool};
use mxsearch::service::MxSearchService;
use mxsearch::{AppInfo, FileInfo};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{LazyLock, Mutex};

/// Holds the async-initialized service, or None if not ready yet.
#[derive(Resource)]
pub struct MxSearchServiceResource {
    pub service: Option<MxSearchService>,
}

#[derive(Resource)]
pub struct MxSearchResultReceiver {
    receiver: Mutex<Receiver<MxSearchResult>>,
}

#[derive(Resource, Clone)]
pub struct MxSearchResultSender(pub Sender<MxSearchResult>);

#[derive(Debug, Clone)]
pub enum SearchResultType {
    App,
    File,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub icon: String,
    pub on_click: Option<SystemId>,
    pub _type: SearchResultType,
}

#[derive(Resource, Clone, Default)]
pub struct AppSearchResult(pub Vec<SearchResult>);

#[derive(Resource, Clone, Default)]
pub struct FileSearchResult(pub Vec<SearchResult>);

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
    Applications(Vec<AppInfo>),
    Files(Vec<FileInfo>),
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
            .insert_resource(AppSearchResult::default())
            .insert_resource(FileSearchResult::default())
            .add_event::<MxSearchActionEvent>()
            .add_systems(Startup, (init_mxsearch_service, setup_channel)) // Async task so temp move service result to static
            .add_systems(Update, poll_service_init)
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
                        info!("about to send query: {query}");
                        match service.search_applications(&query).await {
                            Ok(apps) => {
                                info!("found {} applications", apps.len());
                                if let Err(err) =
                                    result_sender.send(MxSearchResult::Applications(apps))
                                {
                                    error!("failed to send list applications: {err}");
                                }
                            }
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
                } else {
                    error!("service not initialized");
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
                            Ok(files) => {
                                info!("found {} files", files.len());
                                if let Err(err) = result_sender.send(MxSearchResult::Files(files)) {
                                    error!("failed to send files a search result: {err}");
                                }
                            }
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
    mut app_search_result: ResMut<AppSearchResult>,
    mut file_search_result: ResMut<FileSearchResult>,
) {
    if let Ok(receiver) = event_receiver.receiver.lock() {
        while let Ok(event) = receiver.try_recv() {
            match event {
                MxSearchResult::Error(_) => {}
                MxSearchResult::Applications(apps) => {
                    let mut app_list: Vec<SearchResult> = Vec::new();
                    for app in apps {
                        app_list.push(SearchResult {
                            name: app.name,
                            icon: app.icon,
                            on_click: None,
                            _type: SearchResultType::App,
                        });
                    }
                    app_search_result.0 = app_list;
                }
                MxSearchResult::Files(files) => {
                    let mut file_result: Vec<SearchResult> = Vec::new();
                    for file_info in files {
                        file_result.push(SearchResult {
                            name: format!("{}.{}",file_info.name, file_info.file_type),
                            icon: String::new(),
                            on_click: None,
                            _type: SearchResultType::File,
                        });
                    }
                    file_search_result.0 = file_result;
                }
            }
        }
    } else {
        println!("Failed to acquire receiver lock");
    }
}
