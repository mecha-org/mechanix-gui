use bevy::prelude::*;
use types::prelude::FontAssets;
use utils::prelude::DesktopApps;

use crate::{
    icons::AppDrawerIcons,
    ui::{
        AppDrawerRoot,
        apps_list::{AppsList, app_list_app, apps_list, separator},
        search_input::{SearchActive, SearchText, search_input},
    },
};
pub fn filter_apps(
    mut commands: Commands,
    q_apps_list: Single<Entity, With<AppsList>>,
    search_text: ResMut<SearchText>,
    desktop_apps: Res<DesktopApps>,
    font_assets: Res<FontAssets>,
) {
    let e_apps_list = q_apps_list.into_inner();

    //despawn all childs
    commands.entity(e_apps_list).despawn_related::<Children>();

    //filter apps and spawn
    //spawn all apps
    for app in desktop_apps
        .apps
        .iter()
        .filter(|app| {
            if search_text.0.is_empty() {
                return true;
            }
            app.name.to_lowercase().starts_with(&search_text.0)
        })
        .into_iter()
    {
        commands.entity(e_apps_list).with_children(|parent| {
            parent.spawn(app_list_app(&font_assets, app));
            parent.spawn(separator());
        });
    }
    return;
}

pub fn spawn_apps_list(
    mut commands: Commands,
    q_root: Single<(Entity, &Children), With<AppDrawerRoot>>,
    font_assets: Res<FontAssets>,
    desktop_apps: Res<DesktopApps>,
) {
    let (root, children) = q_root.into_inner();

    //despawn second child
    for (i, child) in children.iter().enumerate() {
        if i == 1 {
            commands.entity(child).despawn();
        }
    }

    commands.entity(root).with_children(|parent| {
        parent.spawn(apps_list(&font_assets, desktop_apps.apps.clone()));
    });
}
