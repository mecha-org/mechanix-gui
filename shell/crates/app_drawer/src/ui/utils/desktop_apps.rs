use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::prelude::Icon;
use crate::prelude::IconName;
use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use freedesktop_icons::lookup;
use shlex::Shlex;

/// Represents a single desktop entry (application).
#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub app_id: String,
    pub name: String,
    /// Path to the icon file if found (png/svg...), or None if not found.
    pub icon_path: Option<PathBuf>,
    pub categories: Vec<String>,
    pub exec: String,
}

impl DesktopApp {
    pub fn new(
        app_id: impl Into<String>,
        name: impl Into<String>,
        icon_path: Option<PathBuf>,
        exec: impl Into<String>,
        categories: Vec<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            name: name.into(),
            icon_path,
            exec: exec.into(),
            categories,
        }
    }

    pub fn resolved_icon(app_icon: &Option<PathBuf>) -> Icon {
        match app_icon {
            Some(path) => Icon::default().path(path.to_string_lossy().to_string()),
            None => Icon::from(IconName::DefaultApp),
        }
    }
}

/// Collection of desktop apps discovered on the system.
#[derive(Debug, Clone, Default)]
pub struct DesktopApps {
    pub apps: Vec<DesktopApp>,
}

impl DesktopApps {
    /// Scan the freedesktop dirs and return discovered apps.
    pub fn scan() -> Self {
        let apps = get_desktop_apps();
        Self { apps }
    }

    pub fn add_app(&mut self, app: DesktopApp) {
        self.apps.push(app)
    }

    pub fn remove_app_by_name(&mut self, name: &str) {
        self.apps.retain(|a| a.name != name);
    }

    pub fn get_app_by_name(&self, name: &str) -> Option<&DesktopApp> {
        self.apps.iter().find(|a| a.name == name)
    }

    pub fn get_app_by_id(&self, id: &str) -> Option<&DesktopApp> {
        self.apps.iter().find(|a| a.app_id == id)
    }

    pub fn get_apps_by_category(&self, category: &str) -> Vec<DesktopApp> {
        self.apps
            .iter()
            .filter(|a| a.categories.iter().any(|c| c == category))
            .cloned()
            .collect()
    }

    pub fn grouped_by_category(&self) -> HashMap<String, Vec<DesktopApp>> {
        let mut map: HashMap<String, Vec<DesktopApp>> = HashMap::new();
        for app in &self.apps {
            for c in &app.categories {
                map.entry(c.clone()).or_default().push(app.clone());
            }
        }
        map
    }

    pub fn get_apps_by_categories(&self) -> HashMap<String, Vec<DesktopApp>> {
        let mut map: HashMap<String, Vec<DesktopApp>> = HashMap::new();

        for app in &self.apps {
            // If an app has NO categories, put it in "Uncategorized"
            if app.categories.is_empty() {
                map.entry("Uncategorized".to_string())
                    .or_default()
                    .push(app.clone());
            } else {
                for category in &app.categories {
                    if !category.is_empty() {
                        map.entry(category.clone()).or_default().push(app.clone());
                    }
                }
            }
        }

        map
    }

    /// Tokenize and run a desktop entry Exec string honoring %X fields removal.
    /// This will spawn the executable and print the child PID on success.
    pub fn run_app_exec(exec_str: &str) -> std::io::Result<u32> {
        let mut lexer = Shlex::new(exec_str);

        // Extract executable
        let executable = match lexer.next() {
            Some(e) if !e.contains('=') => e,
            Some(e) => e, // fallback — allow starting with env vars
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Exec string is empty",
                ));
            }
        };

        // Build command
        let mut cmd = std::process::Command::new(&executable);

        // Append valid command-line args (skip %U %F %f %u etc.)
        for token in lexer {
            if !token.starts_with('%') {
                cmd.arg(token);
            }
        }

        // Spawn application
        let child = cmd.spawn()?;
        Ok(child.id())
    }
}

/// Internal helper: find icons and build DesktopApp list.
fn get_desktop_apps() -> Vec<DesktopApp> {
    let locales = get_languages_from_env();
    let entries = Iter::new(default_paths())
        .entries(Some(&locales))
        .collect::<Vec<_>>();

    let mut apps: Vec<DesktopApp> = entries
        .into_iter()
        .filter_map(|entry| {
            // require at least a name
            let opt_name = entry.name(&locales);
            if opt_name.is_none() {
                return None;
            }

            let app_id = entry.appid.clone();
            let name = opt_name.unwrap_or_else(|| app_id.clone().into());
            let categories = entry
                .categories()
                .unwrap_or_default()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            // try to resolve icon path using freedesktop_icons::lookup
            let icon_name = entry.icon().unwrap_or_default();
            let icon_path: Option<PathBuf> = lookup(&icon_name)
                .with_size(84)
                .find()
                .map(|os_str| os_str.into_os_string().into());

            // exec line (raw from .desktop). We keep it verbatim; run_app_exec will tokenize.
            let exec = entry.exec().unwrap_or_default();

            Some(DesktopApp::new(
                app_id,
                name,
                icon_path,
                exec.to_string(),
                categories,
            ))
        })
        .collect();

    // sort by name (case-insensitive)
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}
