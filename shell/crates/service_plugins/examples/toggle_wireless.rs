use bevy::color::palettes::basic::RED;
use bevy::{prelude::*, winit::WinitSettings};
use service_plugins::network_manager::{NetworkAction, NetworkActionEvent};
use service_plugins::NetworkManagerPlugin;
use networkmanager::interfaces::wireless::NMState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NetworkManagerPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

#[derive(Event, Debug, Clone)]
pub struct WifiStateEvent(pub NMState);

#[derive(Resource, Component)]
struct WifiEventText(String);

#[derive(Clone, Copy, Component)]
struct WifiStatusText;

#[derive(Component)]
enum ButtonAction {
    Wifi,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2d);
    // Text with one section

    create_counter_text(&mut commands, &assets);

    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(150.0),
                left: Val::Px(120.0),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            ButtonAction::Wifi,
        ))
        .with_child((Text::new("WIFI"), TextColor(Color::srgb(0.9, 0.9, 0.9))));
}

fn create_counter_text(commands: &mut Commands, assets: &AssetServer) {
    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(65.0),
                // border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(45.0),
                left: Val::Px(70.0),
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
            Text::new("Connected"),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            WifiStatusText, // Mark the text component
        ));
}

fn button_system(
    mut queries: ParamSet<(
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                Option<&ButtonAction>, // Added ButtonAction component
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<&mut Text, With<WifiStatusText>>,
        Query<&mut Text>,
    )>,
    mut event_writer: EventWriter<NetworkActionEvent>,
) {
    for (interaction, _, mut border_color, _, actions) in queries.p0().iter_mut() {
        // println!("button text: {}", text.0);
        match *interaction {
            Interaction::Pressed => {
                println!("pressed");

                match actions {
                    Some(ButtonAction::Wifi) => {
                        println!("Wifi button pressed");
                        event_writer.write(NetworkActionEvent(NetworkAction::ToggleWifi(true)));
                    }
                    _ => {
                        println!("no action");
                    }
                }
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}
