use bevy::prelude::*;
use types::prelude::FontAssets;
use utils::prelude::DesktopApps;

use crate::{
    icons::AppDrawerIcons,
    ui::{
        AppDrawerRoot, CategoriesSubState,
        apps_list::AppsList,
        categories::{AppsCategoriesList, CategoriesList, categories_list},
        category_expanded::{CategoryExpanded, CategoryPopupFor, category_exapanded},
        search_input::search_input,
    },
};

pub fn spawn_categories_list(
    mut commands: Commands,
    q_root: Single<(Entity, &Children), With<AppDrawerRoot>>,
    font_assets: Res<FontAssets>,
    desktop_apps: Res<DesktopApps>,
    icons: Res<AppDrawerIcons>,
) {
    let (root, children) = q_root.into_inner();

    //despawn second child
    for (i, child) in children.iter().enumerate() {
        if i == 1 {
            commands.entity(child).despawn();
        }
    }

    commands.entity(root).with_children(|parent| {
        parent.spawn(categories_list(
            desktop_apps.get_apps_by_categories(),
            icons.sm_curve_image.clone(),
            font_assets.primary_500.clone(),
        ));
    });
}

pub fn spawn_apps_category_popup(
    mut commands: Commands,
    q_apps_list: Single<Entity, With<AppDrawerRoot>>,
    popup_for: Res<CategoryPopupFor>,
    desktop_apps: Res<DesktopApps>,
    font_assets: Res<FontAssets>,
    icons: Res<AppDrawerIcons>,
) {
    println!("spawn_apps_category_popup() ");
    let on_popup_click =
        commands.register_system(|mut state: ResMut<NextState<CategoriesSubState>>| {
            state.set(CategoriesSubState::NoPopup);
        });

    let apps = desktop_apps.get_apps_by_category(&popup_for.0);
    let entity = q_apps_list.into_inner();
    commands.entity(entity).with_children(|parent| {
        parent.spawn(category_exapanded(
            popup_for.0.clone(),
            apps,
            on_popup_click,
            &font_assets,
            &icons,
        ));
    });
}

pub fn despawn_apps_category_popup(
    mut commands: Commands,
    q_category_expanded: Single<Entity, With<CategoryExpanded>>,
) {
    let entity = q_category_expanded.into_inner();
    commands.entity(entity).despawn();
    commands.remove_resource::<CategoryPopupFor>();
}
