use bevy::{ prelude::*, winit::WinitSettings };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_stacked_cards)
        .add_systems(Update, card_hover_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
pub struct Card {
    pub index: usize,
}

fn spawn_stacked_cards(mut commands: Commands) {
    // Container for all cards, centered both vertically and horizontally
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Px(28.0),
                    right: Val::Px(28.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(12.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center, // Center vertically
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Create multiple stacked cards using the helper function
            for i in 0..5 {
                parent.spawn(create_card(i));
            }
        });
}

/// Creates a single card bundle with text and stacking offset.
fn create_card(index: usize) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(50.8),
            height: Val::Percent(8.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            // Stack cards with slight offset from the bottom
            bottom: Val::Percent(index as f32),
            ..default()
        },
        BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
        BorderRadius::all(Val::Px(8.0)),
        BackgroundColor(Color::linear_rgba(37.0, 36.0, 36.0, 1.0)),
        ZIndex(index as i32),
        Card { index },
        // Add text as a child node
        children![(
            Text::new(format!("Card {}", index + 1)),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::BLACK),
        )],
    )
}

fn card_hover_system(
    mut query: Query<
        (&Interaction, &Card, &mut Node),
        (Changed<Interaction>, With<Card>, With<Button>)
    >
) {
    for (interaction, card, mut node) in &mut query {
        // Base offset step (percent). Adjust as desired.
        let base_offset = (card.index as f32) * 2.0;
        match *interaction {
            Interaction::Hovered => {
                // Spread cards further apart on hover
                println!("Card {} hovered", card.index);
                node.bottom = Val::Percent(base_offset + 10.0);
            }
            Interaction::None => {
                // Reset to original stacking offset
                node.bottom = Val::Percent(base_offset);
            }
            Interaction::Pressed => {
                // Optionally bring pressed card even higher
                node.bottom = Val::Percent(base_offset + 8.0);
            }
        }
    }
}
