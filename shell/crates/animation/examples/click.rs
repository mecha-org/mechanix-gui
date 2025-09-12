use animation::{
    combinator::{event, sequence, tween},
    interpolate::background_color_to,
    prelude::*,
};
use bevy::{color::palettes::css::*, prelude::*};

#[derive(Component)]
struct Bulb;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultTweenPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, read_events)
        .add_observer(on_press)
        .run();
}

fn read_events(mut event_reader: EventReader<TweenEvent<&'static str>>) {
    event_reader.read().for_each(|event| {
        println!("Event: {}", event.data);
    });
}

fn on_press(
    mut trigger: Trigger<Pointer<Pressed>>,
    mut q_bulb: Query<Entity, With<Bulb>>,
    mut commands: Commands,
) {
    if let Ok(entity) = q_bulb.get_mut(trigger.target()) {
        trigger.propagate(false);

        let mut entity_commands = commands.entity(entity);

        let node_target = entity_commands.id().into_target();

        let mut bg_color_state = node_target.state(WHITE.into());

        entity_commands.animation().insert(sequence((
            tween(
                Duration::from_secs(1),
                EaseKind::QuadraticOut,
                bg_color_state.with(background_color_to(RED.into())),
            ),
            event("Press animation completed"),
        )));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        children![(
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            Bulb,
            BackgroundColor(YELLOW.into()),
        )],
    ));
}
