use bevy::color::palettes::basic::RED;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_plugins::pulse_audio::{
    PulseAudioAction, PulseAudioActionEvent, PulseAudioResult, PulseAudioResultEvent,
};
use bevy_plugins::PulseAudioPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PulseAudioPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, wait_action_result))
        .run();
}

fn wait_action_result(mut event_reader: EventReader<PulseAudioResultEvent>) {
    for event in event_reader.read() {
        let actions = &event.0;
        match actions {
            PulseAudioResult::ListSinks(result) => {
                info!("sinks result received: {result:?}");
            }
            PulseAudioResult::Error(error) => {
                error!("error: {error:?}");
            }
            _ => {
                info!("no action");
            }
        }
    }
}

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
        .with_child((Text::new("GetSinks"), TextColor(Color::srgb(0.9, 0.9, 0.9))));
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
        Query<&mut Text>,
    )>,
    mut event_writer: EventWriter<PulseAudioActionEvent>,
) {
    for (interaction, _, mut border_color, _, actions) in queries.p0().iter_mut() {
        // println!("button text: {}", text.0);
        match *interaction {
            Interaction::Pressed => {
                println!("pressed");

                match actions {
                    Some(ButtonAction::Wifi) => {
                        println!("Wifi button pressed");
                        event_writer.write(PulseAudioActionEvent(PulseAudioAction::ListSinks));
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
