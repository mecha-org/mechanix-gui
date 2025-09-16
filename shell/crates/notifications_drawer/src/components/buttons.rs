use bevy::prelude::*;

#[derive(Component)]
pub struct ActionElement {
    pub id: u32,
    pub action_id: String,
}

#[derive(Component)]
pub struct ClearAllButton;

#[derive(Component)]
pub struct ClearButton {
    pub app_name: String,
}

#[derive(Component)]
pub struct CardCloseButton {
    pub card_entity: Entity,
}

#[derive(Component)]
pub struct StackButton {
    pub app_name: String,
    pub is_stacked: bool,
}

pub fn get_actions_row(actions: Vec<String>) -> impl Bundle {
    // Check if we have display text actions (pairs: action_id, display_text)
    let has_actions = actions.len() >= 2;

    (
        Node {
            width: Val::Px(509.0),
            height: if has_actions {
                Val::Auto
            } else {
                Val::Px(0.0)
            },
            margin: if has_actions {
                UiRect::top(Val::Px(10.0))
            } else {
                UiRect::ZERO
            },
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            // row_gap: Val::Px(1.0),
            ..default()
        },
    )
}