use crate::{
    UniversalSearchWindowCamera, WINDOW_SIZE,
    icons::UniversalSearchIcons,
    prelude::*,
    ui::{
        BAR_SIZE, BrowserApps, Container, FrequentlyUsedApps, SearchItems, SearchResults,
        SearchText, frequently_used_apps, search_input, search_items,
    },
};
use animation::{
    combinator::{event, parallel, sequence, tween},
    interpolate::node_to,
    prelude::*,
};
use bevy::color::palettes::css::*;
use types::prelude::FontAssets;

#[derive(Component)]
pub struct AnimateContainer;

pub fn listen_open_event(
    mut trigger: Trigger<UniversalSearchOpen>,
    mut commands: Commands,
    q_container: Query<(Entity), With<Container>>,
    asset_server: Res<AssetServer>,
    apps: Res<FrequentlyUsedApps>,
    searches: Res<SearchItems>,
    results: Res<SearchResults>,
    results_for: Res<SearchText>,
    browser_apps: Res<BrowserApps>,
    camera_ent: Res<UniversalSearchWindowCamera>,
    font_assets: Res<FontAssets>,
    icons: Res<UniversalSearchIcons>,
) {
    trigger.propagate(false);

    if let Ok((e_container)) = q_container.single() {
        println!("container found");
        let from = Node {
            width: Val::Percent(100.),
            height: Val::Px(WINDOW_SIZE.1 - BAR_SIZE.1),
            position_type: PositionType::Absolute,
            top: Val::Px(500.),
            left: Val::Px(0.),
            flex_direction: FlexDirection::Column,
            padding: UiRect::horizontal(Val::Px(16.)),
            row_gap: Val::Px(8.),
            ..Default::default()
        };

        let mut to = from.clone();

        to.top = Val::Px(0.);
        to.left = Val::Px(0.);

        let animate_container = commands
            .spawn((
                from.clone(),
                AnimateContainer,
                BackgroundColor(BLACK.into()),
                children![
                    frequently_used_apps(apps.0.clone()),
                    search_items(searches.0.clone(), &font_assets, &icons),
                    search_input(&font_assets, &icons) // search_results(results, results_for, browser_apps, asset_server)
                ],
            ))
            .id();
        commands.entity(e_container).add_child(animate_container);
        let node_target = animate_container.into_target();
        let mut node_position_state = node_target.state(from);
        commands
            .spawn((TweenPriorityToOthersOfType(10)))
            .animation()
            .insert(parallel((
                tween(
                    Duration::from_millis(800),
                    EaseKind::QuadraticOut,
                    node_position_state.with(node_to(to)),
                ),
                event("UniversalSearchOpened"),
            )));
    }
}

pub fn listen_close_event(
    mut trigger: Trigger<UniversalSearchClose>,
    mut commands: Commands,
    q_animate_container: Query<(Entity, &Node), With<AnimateContainer>>,
) {
    trigger.propagate(false);

    if let Ok((animate_container, from)) = q_animate_container.single() {
        let from = from.clone();
        let mut to = from.clone();
        to.top = Val::Px(576.);
        let node_target = animate_container.into_target();
        let mut node_position_state = node_target.state(from);
        commands
            .spawn((TweenPriorityToOthersOfType(20)))
            .animation()
            .insert(sequence((
                tween(
                    Duration::from_millis(800),
                    EaseKind::QuadraticOut,
                    node_position_state.with(node_to(to)),
                ),
                event("UniversalSearchClosed"),
            )));
    }
}

pub fn listen_close_completed(
    mut event: EventReader<TweenEvent<&'static str>>,
    mut commands: Commands,
    q_animate_container: Query<Entity, With<AnimateContainer>>,
) {
    event.read().for_each(|event| match event.data {
        "Closed" => {
            if let Ok(animate_container) = q_animate_container.single() {
                commands.entity(animate_container).despawn();
            }
        }
        _ => (),
    });
}
