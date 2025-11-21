use gpui::*;
use tokio::sync::mpsc;
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::ToplevelKey;
use tokio::{ sync::{ oneshot } };

pub struct RunningApps {
    pub scroll_offset: Pixels,
    pub is_dragging: bool,
    pub drag_start_x: Pixels,
    pub target_scroll_offset: Pixels,
    pub drag_start_y: Pixels,
    pub drag_start_offset: Pixels,
    pub apps: Vec<AppCard>,
    pub dragging_card: Option<usize>,
    pub drag_direction: Option<DragDirection>,
    pub is_animating: bool,
    pub is_removing: bool,
    pub removing_card_id: Option<usize>,
    pub current_center_index: usize,
    pub is_cleaning_up: bool, // Flag for clean up animation
    pub position: f32,
    pub bar_drag_offset: f32,
    pub bar_drag_start_y: Option<f32>,
    pub show_apps: bool,
    pub message_tx: mpsc::Sender<AppManagerMessage>,
}
#[derive(Debug)]
pub enum AppManagerMessage {
    CloseAppInstance {
        instance: ToplevelKey,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    ActivateAppInstance {
        instance: ToplevelKey,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    CloseAllApps {
        reply_to: oneshot::Sender<Result<bool>>,
    },
    LaunchApp {
        app_id: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    CloseApp {
        app_id: String,
        reply_to: oneshot::Sender<Result<bool>>,
    },
    MinimizeAll,
}
#[derive(Clone, Copy, PartialEq)]
pub enum DragDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct AppCard {
    pub id: usize,
    pub app_id: String,
    pub offset_y: Pixels,
    pub target_offset_y: Pixels,
    pub app_name: Option<String>,
    pub app_icon_path: Option<String>,
}

#[derive(Debug)]
pub enum AppMessage {
    CPUUsage {
        usage: f32,
    },
    Uptime {
        uptime: String,
    },
    MachineName {
        name: String,
    },
    Net {
        online: bool,
    },
    Memory {
        total: u64,
        used: u64,
    },

    // RunningApps {
    //     message: RunningAppsMessage,
    // },
    // ChangeLayer(Layer),
    // Bluetooth {
    //     message: BluetoothMessage,
    // },
    // Sound {
    //     message: SoundMessage,
    // },
    // Brightness {
    //     message: BrightnessMessage,
    // },
    AppsUpdated {
        apps: Vec<AppDetails>,
        app_id: String,
        active_apps_count: i32,
    },
    ShutDown,
    Restart,
    Unlock,
    AppOpen {
        app_id: String,
    },
    AppClose {
        app_id: String,
    },
    MinimizeAll,
}

#[derive(Debug, Clone)]
pub struct AppDetails {
    pub app_id: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub icon: Option<String>,
    // pub icon_type: Option<IconType>,
    pub icon_path: Option<String>,
    pub instances: Vec<AppInstance>,
}

#[derive(Debug, Clone)]
pub struct AppInstance {
    pub title: Option<String>,
    pub instance_key: ToplevelKey,
    pub icon: Option<String>,
}
