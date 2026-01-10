use freedesktop_icons::lookup;
use gpui::{foreign_toplevel_management::ForeignToplevelHandle, *};
use mxsearch::service::MxSearchService;
use std::{collections::HashMap, path::PathBuf};

#[derive(Default, Clone, Debug)]
pub struct App {
    pub app_id: String,
    pub name: String,
    pub icon: Option<PathBuf>,
}

impl App {
    pub fn run_active_instance(&self, running_apps: Vec<ForeignToplevelHandle>) {
        for top_level in running_apps {
            if let Some(app_id) = top_level.app_id() {
                if app_id == self.app_id {
                    top_level.activate();
                    return;
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct InstalledApps {
    apps: HashMap<String, App>,
    search_service: Option<MxSearchService>,
}

impl InstalledApps {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            let search_service = MxSearchService::new().await.unwrap();
            let app_infos = search_service.list_applications().await.unwrap_or_default();

            this.update(cx, |this, cx| {
                this.apps = app_infos
                    .into_iter()
                    .map(|app_info| {
                        let app_id = std::path::Path::new(&app_info.app_path)
                            .file_stem()
                            .and_then(|os_str| os_str.to_str())
                            .unwrap_or_default()
                            .to_string();
                        let icon: Option<PathBuf> = lookup(&app_info.icon_name)
                            .with_size(84)
                            .find()
                            .map(|os_str| os_str.into_os_string().into());

                        (
                            app_id.clone(),
                            App {
                                app_id: app_id,
                                name: app_info.name,
                                icon,
                            },
                        )
                    })
                    .collect();
                this.search_service = Some(search_service);
                cx.notify();
            })
        })
        .detach();

        Self {
            apps: HashMap::new(),
            search_service: None,
        }
    }

    pub fn list(&self) -> Vec<App> {
        self.apps.values().cloned().collect()
    }

    pub fn find(&self, app_id: &str) -> Option<App> {
        self.apps.get(app_id).cloned()
    }
}
