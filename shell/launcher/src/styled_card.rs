use bevy::prelude::*;
use bevy_styled_widgets::prelude::ThemeManager;

#[derive(Component)]
pub struct StyledCard;

fn update_bg(
    theme_manager: Res<ThemeManager>,
    mut query: Query<&mut BackgroundColor, With<StyledCard>>,
) {
    for mut bg_color in query.iter_mut() {
        let theme_styles = theme_manager.styles.clone();
        let color = theme_styles.panel.background_color;
        bg_color.0 = color;
    }
}

pub struct StyledCardPlugin;

impl Plugin for StyledCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bg);
    }
}
