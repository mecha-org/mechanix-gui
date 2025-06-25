use bevy::{asset::Handle, image::Image, platform::collections::HashMap};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HomeBundleType {
    #[default]
    App,
    Widget,
}

#[derive(Debug, Clone)]
pub struct HomeEntry {
    pub app_id: String,
    pub icon_name: Option<String>,
    pub icon_path: Option<Handle<Image>>,
    pub name: String,
    pub bundle_type: HomeBundleType, 
}

#[derive(Debug, Clone)]
pub struct HomeScreenSettings {
    pub width: f32,
    pub height: f32,
    pub grid_template_columns: u16,
    pub grid_template_rows: u16,
    pub home_entries: HashMap<String, Vec<HomeEntry>>,
}

impl Default for HomeScreenSettings {
    fn default() -> Self {
        Self {
            width: 100.,
            height: 100.,
            grid_template_columns: 4,
            grid_template_rows: 4,
            home_entries: HashMap::from([(
                "sm".to_string(),
                Vec::from([
                    HomeEntry {
                        app_id: "App 1".to_string(),
                        icon_name: Some("Images".to_string()),
                        icon_path: None,
                        name: "App 1".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 2".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 2".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "Widget 1".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "Widget 1".to_string(),
                        bundle_type: HomeBundleType::Widget,
                    },
                    HomeEntry {
                        app_id: "App 4".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 4".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 4".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 4".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 5".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 5".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 6".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 6".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 7".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 7".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 8".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 8".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 9".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 9".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 10".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 10".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 11".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 11".to_string(),
                        bundle_type: HomeBundleType::App,
                    },
                    HomeEntry {
                        app_id: "App 12".to_string(),
                        icon_name: None,
                        icon_path: None,
                        name: "App 12".to_string(),
                        bundle_type: HomeBundleType::App,
                    }
                ]),
            )]),
        }
    }
}
