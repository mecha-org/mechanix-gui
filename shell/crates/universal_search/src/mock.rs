use bevy::{asset::AssetPath, ecs::system::command::trigger, prelude::*};
use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use freedesktop_icons::lookup;
use std::path::Path;

use crate::{
    types::{DesktopApp, SearchResult, SearchResultType},
    ui::{BrowserApps, FrequentlyUsedApps, SearchItems, SearchResults, SearchText},
};

pub struct MockPlugin;

impl Plugin for MockPlugin {
    fn build(&self, app: &mut App) {
        let apps = get_apps(app);
        app.insert_resource(FrequentlyUsedApps(apps));

        let searches = get_past_searches(app);
        app.insert_resource(SearchItems(searches));

        let search_results = get_search_results(app);
        app.insert_resource(SearchResults(search_results));

        app.insert_resource(SearchText("Disco".to_string()));

        let browser_apps = get_browser_apps(app);
        app.insert_resource(BrowserApps(browser_apps));
    }
}

fn get_apps(app: &mut App) -> Vec<DesktopApp> {
    let locales = get_languages_from_env();

    let entries = Iter::new(default_paths())
        .entries(Some(&locales))
        .collect::<Vec<_>>();

    let app_list = vec![
        "firefox_firefox".to_string(),
        "code".to_string(),
        "microsoft-edge".to_string(),
        "discord_discord".to_string(),
        "zulip_zulip".to_string(),
        "vim".to_string(),
    ];

    let apps = entries
        .into_iter()
        .filter_map(|entry| {
            if entry.name(&locales).is_some() && app_list.contains(&entry.appid.to_string()) {
                Some(entry)
            } else {
                None
            }
        })
        .map(|entry| {
            let icon_name = entry.icon().unwrap_or_default();

            let icon = lookup(&icon_name)
                .with_size(84)
                .with_theme("Papirus")
                .find()
                .unwrap_or_default()
                .into_os_string()
                .into_string()
                .unwrap();
            let default_icon = AssetPath::from_static(format!("icons/universal_search/files.png",));

            let path = Path::new(&icon);

            let icon: Handle<Image> = match path.extension() {
                Some(ext) if ext == "svg" => app.world_mut().load_asset(default_icon),
                Some(ext) if ext == "png" => app.world_mut().load_asset(AssetPath::from_path(path)),
                _ => {
                    println!("Unsupported icon format: {:?}", path.extension());
                    app.world_mut().load_asset(default_icon)
                }
            };

            let on_click = app.world_mut().register_system(|mut commands: Commands| {
                info!("App clicked");
            });

            let app = DesktopApp {
                app_id: entry.appid.clone(),
                categories: entry
                    .categories()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                exec: entry.exec().unwrap_or_default().to_string(),
                on_click,
                name: "".to_string(),
                icon,
            };
            app
        })
        .collect::<Vec<_>>();
    apps
}

fn get_past_searches(app: &mut App) -> Vec<SearchResult> {
    let files_icon = app.world_mut().load_asset(AssetPath::from_static(format!(
        "icons/universal_search/files.png",
    )));
    let kitty_icon = app.world_mut().load_asset(AssetPath::from_static(format!(
        "icons/universal_search/kitty.png",
    )));
    let mecha_connect_icon = app.world_mut().load_asset(AssetPath::from_static(format!(
        "icons/universal_search/mecha_connect.png",
    )));
    let python_icon = app.world_mut().load_asset(AssetPath::from_static(format!(
        "icons/universal_search/python.png",
    )));

    vec![
        SearchResult {
            name: "exec.py".to_string(),
            icon: python_icon,
            on_click: None,
            _type: SearchResultType::File,
        },
        SearchResult {
            name: "Files".to_string(),
            icon: files_icon,
            on_click: None,
            _type: SearchResultType::File,
        },
        SearchResult {
            name: "Kitty".to_string(),
            icon: kitty_icon,
            on_click: None,
            _type: SearchResultType::File,
        },
        SearchResult {
            name: "Mecha Connect".to_string(),
            icon: mecha_connect_icon,
            on_click: None,
            _type: SearchResultType::File,
        },
    ]
}

fn get_search_results(app: &mut App) -> Vec<SearchResult> {
    let default_icon = Path::new("../../assets/icons/universal_search/default_app_icon.png");
    let icon: Handle<Image> = app
        .world_mut()
        .load_asset(AssetPath::from_path(default_icon));

    vec![
        SearchResult {
            name: "Discord".to_string(),
            icon: icon.clone(),
            on_click: None,
            _type: SearchResultType::App,
        },
        SearchResult {
            name: "Firefox".to_string(),
            icon: icon.clone(),
            on_click: None,
            _type: SearchResultType::App,
        },
        SearchResult {
            name: "main.rs".to_string(),
            icon: icon.clone(),
            on_click: None,
            _type: SearchResultType::File,
        },
    ]
}

fn get_browser_apps(app: &mut App) -> Vec<DesktopApp> {
    let default_icon = Path::new("../../assets/icons/universal_search/default_app_icon.png");
    let icon: Handle<Image> = app
        .world_mut()
        .load_asset(AssetPath::from_path(default_icon));
    let on_click = app.world_mut().register_system(|mut commands: Commands| {
        info!("App clicked");
    });
    vec![
        DesktopApp {
            app_id: "firefox_firefox".to_string(),
            categories: vec!["Web Browser".to_string()],
            exec: "/usr/bin/firefox".to_string(),
            name: "Firefox".to_string(),
            icon: icon.clone(),
            on_click,
        },
        DesktopApp {
            app_id: "microsoft-edge".to_string(),
            categories: vec!["Web Browser".to_string()],
            exec: "/usr/bin/msedge".to_string(),
            name: "Edge".to_string(),
            icon: icon.clone(),
            on_click,
        },
    ]
}
