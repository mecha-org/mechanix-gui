use bevy::{ecs::system::SystemId, prelude::*};
use headless_widgets::CoreButton;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{
        ButtonType1,
        drawer_items::power_saving_mode::{PowerSavingMode, PowerSavingModeIcon},
        on_power_saving_mode_click,
    },
};

pub fn power_saving_mode(on_click: SystemId, icons: &SettingsDrawerIcons) -> impl Bundle {
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
        ButtonType1(false),
        PowerSavingMode,
        children![(
            ImageNode::new(icons.power_mode_high.clone()),
            Node {
                width: Val::Px(40.),
                height: Val::Px(40.),
                ..default()
            },
            PowerSavingModeIcon,
        ),],
    )
}
