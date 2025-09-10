use bevy::{ecs::system::SystemId, prelude::*};
use headless_widgets::CoreButton;

use crate::{icons::SettingsDrawerIcons, ui::ButtonType3};

pub fn camera(on_click: SystemId, icons: &SettingsDrawerIcons) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(8.)),
        BackgroundColor(Color::oklch(0.2435, 0., 0.)),
        CoreButton::new().on_click(on_click),
        ButtonType3,
        children![(
            ImageNode::new(icons.camera.clone()),
            Node {
                width: Val::Px(40.),
                height: Val::Px(40.),
                ..default()
            },
        ),],
    )
}
