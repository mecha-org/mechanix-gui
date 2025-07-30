use std::path::Path;

use bevy::{asset::AssetPath, ecs::system::SystemId, platform::collections::HashMap, prelude::*};
use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use freedesktop_icons::lookup;

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub name: String,
    pub icon: Handle<Image>,
    pub categories: Vec<String>,
    pub exec: String,
    pub on_click: SystemId,
}

impl DesktopApp {
    pub fn new(
        name: &str,
        icon: Handle<Image>,
        exec: String,
        categories: Vec<String>,
        on_click: SystemId,
    ) -> Self {
        Self {
            name: name.to_string(),
            icon: icon,
            exec,
            categories,
            on_click,
        }
    }
}

#[derive(Debug, Resource)]
pub struct DesktopApps {
    pub apps: Vec<DesktopApp>,
}

impl DesktopApps {
    pub fn new(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Self {
        let apps = get_desktop_apps(commands, asset_server);
        Self { apps }
    }

    pub fn add_app(&mut self, app: DesktopApp) {
        self.apps.push(app);
    }

    pub fn get_app_by_name(&self, name: &str) -> Option<&DesktopApp> {
        self.apps.iter().find(|app| app.name == name)
    }

    pub fn get_apps_by_category(&self, category: &str) -> Vec<&DesktopApp> {
        self.apps
            .iter()
            .filter(|app| app.categories.contains(&category.to_string()))
            .collect()
    }
    pub fn remove_app(&mut self, name: &str) {
        self.apps.retain(|app| app.name != name);
    }

    pub fn get_apps_by_categories(&self) -> HashMap<String, Vec<DesktopApp>> {
        let mut category_map: HashMap<String, Vec<DesktopApp>> = HashMap::new();
        for app in &self.apps {
            for category in &app.categories {
                category_map
                    .entry(category.clone())
                    .or_default()
                    .push(app.clone());
            }
        }
        category_map
    }

    pub fn run_app_exec(exec: String) {
        let mut exec = shlex::Shlex::new(&exec);
        let executable = match exec.next() {
            Some(executable) if !executable.contains('=') => executable,
            _ => "".to_string(),
        };
        let mut cmd = std::process::Command::new(&executable);
        for arg in exec {
            if !arg.starts_with('%') {
                cmd.arg(arg);
            }
        }

        if let Ok(child) = cmd.spawn() {
            // Write PID to pipe
            println!("Spawned process: {:?}", child.id());
        }
    }
}

fn get_desktop_apps(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Vec<DesktopApp> {
    let locales = get_languages_from_env();

    let entries = Iter::new(default_paths())
        .entries(Some(&locales))
        .collect::<Vec<_>>();

    let mut apps: Vec<DesktopApp> = entries
        .into_iter()
        .filter_map(|entry| {
            if entry.name(&locales).is_some() {
                Some(entry)
            } else {
                None
            }
        })
        .filter_map(|entry| {
            let name = &entry.name(&locales);
            let icon_name = entry.icon().unwrap_or_default();
            let categories = entry
                .categories()
                .unwrap_or_default()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let icon = lookup(&icon_name)
                .with_size(84)
                .with_theme("Papirus")
                .find()
                .unwrap_or_default()
                .into_os_string()
                .into_string()
                .unwrap();
            let default_icon = Path::new("icons/default_app_icon.png");

            let path = Path::new(&icon);

            let icon: Handle<Image> = match path.extension() {
                Some(ext) if ext == "svg" => asset_server.load(AssetPath::from_path(default_icon)),
                Some(ext) if ext == "png" => asset_server.load(AssetPath::from_path(path)),
                _ => {
                    println!("Unsupported icon format: {:?}", path.extension());
                    asset_server.load(AssetPath::from_path(default_icon))
                }
            };

            let exec = entry.exec().unwrap_or_default();
            let exec_cloned = exec.to_string().clone();
            let on_click = commands.register_system(move || {
                let _ = DesktopApps::run_app_exec(exec_cloned.to_string());
            });

            return Some(DesktopApp::new(
                &name.clone().unwrap(),
                icon,
                exec.to_string(),
                categories,
                on_click,
            ));
        })
        .collect();

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

pub struct DesktopAppsPlugin;

impl Plugin for DesktopAppsPlugin {
    fn build(&self, app: &mut App) {}
}
