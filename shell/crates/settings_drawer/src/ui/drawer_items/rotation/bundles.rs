use bevy::{ecs::system::SystemId, prelude::*};
use headless_widgets::CoreButton;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        ButtonType1,
        drawer_items::rotation::{AutoRotation, AutoRotationIcon},
        on_rotation_click,
    },
};

pub fn auto_rotation(on_click: SystemId, icons: &SettingsDrawerIcons) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            border: UiRect::all(Val::Px(1.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(8.)),
        BorderColor(Color::oklcha(0.4202, 0., 0., 0.2)),
        BackgroundColor(Color::oklcha(0.209, 0., 0., 0.2)),
        CoreButton::new().on_click(on_click),
        ButtonType1(true),
        AutoRotation,
        children![(
            ImageNode::new(icons.rotation_on.clone()),
            Node {
                width: Val::Px(40.),
                height: Val::Px(40.),
                ..default()
            },
            AutoRotationIcon,
        ),],
    )
}
