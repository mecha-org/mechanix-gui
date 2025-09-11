use animation::{
    combinator::{event, parallel, tween},
    interpolate::node_to,
    prelude::*,
};
use bevy_wayland::prelude::InputRegion;
use types::prelude::FontAssets;

use crate::{
    icons::SettingsDrawerIcons,
    setup::{SettingsDrawerWindow, WINDOW_SIZE},
    ui::{
        BAR_SIZE, ButtonType1, ButtonType2, ButtonType3, ButtonType4, DrawerItemsRoot,
        SettingsDrawerRoot, drawer_items,
    },
};

pub fn spawn_drawer_items(
    mut commands: Commands,
    q_root: Single<Entity, With<SettingsDrawerRoot>>,
    fonts: Res<FontAssets>,
    icons: Res<SettingsDrawerIcons>,
) {
    let root = q_root.into_inner();

    let drawer_items = drawer_items(&mut commands, &fonts, &icons);

    // commands
    //     .entity(root)
    //     .with_children(|parent: &mut RelatedSpawnerCommands<ChildOf>| {
    //         parent.spawn(drawer_items);
    //     });
    // println!("spawn_drawer_items()");

    let from = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        position_type: PositionType::Absolute,
        top: Val::Px(500.),
        left: Val::Px(0.),
        ..Default::default()
    };

    let mut to = from.clone();

    to.top = Val::Px(0.);
    to.left = Val::Px(0.);

    let animate_container = commands.spawn((from.clone(), children![drawer_items])).id();
    commands.entity(root).add_child(animate_container);
    let node_target = animate_container.into_target();
    let mut node_position_state = node_target.state(from);
    commands
        .spawn((TweenPriorityToOthersOfType(15)))
        .animation()
        .insert(parallel((
            tween(
                Duration::from_millis(800),
                EaseKind::QuadraticOut,
                node_position_state.with(node_to(to)),
            ),
            event("SettingsDrawerOpened"),
        )));
}

pub fn despawn_drawer_items(
    mut commands: Commands,
    q_drawer_items: Single<Entity, With<DrawerItemsRoot>>,
) {
    let root = q_drawer_items.into_inner();

    // commands.entity(drawer_items_root).despawn();

    let from = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        position_type: PositionType::Absolute,
        top: Val::Px(0.),
        left: Val::Px(0.),
        ..Default::default()
    };

    let mut to = from.clone();

    to.top = Val::Px(500.);
    to.left = Val::Px(0.);

    let node_target = root.into_target();
    let mut node_position_state = node_target.state(from);
    commands.entity(root).animation().insert(parallel((
        tween(
            Duration::from_millis(800),
            EaseKind::QuadraticOut,
            node_position_state.with(node_to(to)),
        ),
        event("SettingsDrawerClosed"),
    )));
}

pub fn update_button_type1_styles(
    mut q_buttons: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Node,
        &ButtonType1,
    )>,
) {
    for (mut bg_color, mut border_color, mut node, is_on) in q_buttons.iter_mut() {
        if is_on.0 {
            node.border = UiRect::all(Val::Px(0.));
            bg_color.0 = Color::oklch(0.2435, 0., 0.);
        } else {
            node.border = UiRect::all(Val::Px(1.));
            bg_color.0 = Color::oklcha(0.209, 0., 0., 0.2);
            border_color.0 = Color::oklcha(0.4202, 0., 0., 0.2);
        }
    }
}

pub fn update_button_type2_styles(
    mut q_buttons: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Node,
        &ButtonType2,
    )>,
) {
    for (mut bg_color, mut border_color, mut node, is_on) in q_buttons.iter_mut() {
        if is_on.0 {
            node.border = UiRect::all(Val::Px(0.));
            bg_color.0 = Color::oklch(0.7157, 0.151951, 73.643);
        } else {
            node.border = UiRect::all(Val::Px(1.));
            bg_color.0 = Color::oklcha(0.209, 0., 0., 0.2);
            border_color.0 = Color::oklcha(0.4202, 0., 0., 0.2);
        }
    }
}

pub fn update_button_type3_styles(
    mut q_buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonType3>),
    >,
) {
    for (interaction, mut bg_color) in q_buttons.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                bg_color.0 = Color::oklch(0.3329, 0., 0.);
            }
            Interaction::None => {
                bg_color.0 = Color::oklch(0.2435, 0., 0.);
            }
            Interaction::Hovered => {
                bg_color.0 = Color::oklch(0.2435, 0., 0.);
            }
        }
    }
}

pub fn update_button_type4_styles(
    mut q_buttons: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Node,
        &ButtonType4,
    )>,
) {
    for (mut bg_color, mut border_color, mut node, is_on) in q_buttons.iter_mut() {
        if is_on.0 {
            bg_color.0 = Color::oklch(0.2435, 0., 0.);
        } else {
            bg_color.0 = Color::oklch(0.209, 0., 0.);
        }
    }
}
