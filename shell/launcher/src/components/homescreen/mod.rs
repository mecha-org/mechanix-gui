use bevy::prelude::*;
use bevy_styled_widgets::prelude::StyledText;

use crate::styled_card::StyledCard;

pub fn apps_grid() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        StyledCard,
        children![
            StyledText::builder()
                .content("Apps / Widgets surface")
                .font_size(28.)
                .build()
        ],
    )
}
