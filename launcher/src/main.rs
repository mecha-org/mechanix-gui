use std::collections::HashMap;
use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
mod settings;
mod errors;
use crate::settings::LauncherSettings;
use tracing::{info, error};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Apps {
    StatusBar,
    SettingsPanel,
    AppDock,
    AppDrawer,
    ActionBar,
    LockScreen,
}

struct Launcher {
    running_apps: HashMap<Apps, Arc<Mutex<Child>>>,
}

impl Launcher {
    fn new() -> Launcher {
        Launcher {
            running_apps: HashMap::new(),
        }
    }

    fn start_app(&mut self, app: Apps, bin_path: &str, settings_path: &str) {
        if self.running_apps.contains_key(&app) {
            info!("{:?} is already running.", app);
            return;
        }

        let child = Command::new(bin_path)
            .arg("--settings")
            .arg(settings_path)
            .spawn()
            .expect("Failed to start the app");

        self.running_apps.insert(app, Arc::new(Mutex::new(child)));
    }

    fn stop_app(&mut self, app: Apps) {
        if let Some(child_mutex) = self.running_apps.get(&app) {
            let mut child = child_mutex.lock().unwrap();
            if let Err(err) = child.kill() {
                error!("Failed to stop {:?}: {}", app, err);
            }
        }
    }

    fn list_running_apps(&self) {
        info!("Running Apps:");
        for (app, _) in &self.running_apps {
            info!("{:?}", app);
        }
    }

    fn clean_up_exited_apps(&mut self) {
        let apps_to_remove: Vec<Apps> = self.running_apps
            .iter()
            .filter_map(|(app, child_mutex)| {
                let mut child = child_mutex.lock().unwrap();
                match child.try_wait() {
                    Ok(status) => {
                        match status {
                            Some(_) => {
                                info!("{:?} has exited.", app);
                                Some(app.clone())
                            }
                            None => None,
                        }
                    }
                    Err(_) => {
                        error!("Error checking {:?} status.", app);
                        None
                    }
                }
            })
            .collect();

        for app in apps_to_remove {
            self.running_apps.remove(&app);
            self.list_running_apps();
        }
    }
}

fn main() {
    let mut app_manager = Launcher::new();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => LauncherSettings::default(),
    };

    if settings.components.status_bar.enabled {
        app_manager.start_app(Apps::StatusBar, &settings.components.status_bar.bin_path, &settings.components.status_bar.settings_path);
    }
    if settings.components.app_dock.enabled {
        app_manager.start_app(Apps::AppDock, &settings.components.app_dock.bin_path, &settings.components.app_dock.settings_path);
    }
    if settings.components.settings_panel.enabled {
        app_manager.start_app(Apps::SettingsPanel, &settings.components.settings_panel.bin_path, &settings.components.settings_panel.settings_path);
    }
    if settings.components.action_bar.enabled {
        app_manager.start_app(Apps::ActionBar, &settings.components.action_bar.bin_path, &settings.components.action_bar.settings_path);
    }
    if settings.components.app_drawer.enabled {
        app_manager.start_app(Apps::AppDrawer, &settings.components.app_drawer.bin_path, &settings.components.app_drawer.settings_path);
    }
    if settings.components.lock_screen.enabled {
        app_manager.start_app(Apps::LockScreen, &settings.components.lock_screen.bin_path, &settings.components.lock_screen.settings_path);
    }

    app_manager.list_running_apps();

    // Clean up
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5)); 
            app_manager.clean_up_exited_apps();
        }
    });
}
