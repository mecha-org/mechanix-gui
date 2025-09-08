use bevy::prelude::*;
use headless_widgets::prelude::*;
use utils::prelude::*;

pub fn app_button(app: &DesktopApp) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![(
            ImageNode::new(app.icon.clone()),
            Node {
                width: Val::Px(55.),
                height: Val::Px(55.),
                ..default()
            }
        )],
        CoreButton::new().on_click(app.on_click),
        Button,
        BorderRadius::all(Val::Px(13.91)),
        BackgroundColor(Color::oklch(0.2891, 0., 0.)),
    )
}
