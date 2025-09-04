use animation::{bevy_time_runner::TimeRunner, prelude::TweenEvent};
use bevy::prelude::*;
use bevy_wayland::prelude::InputRegion;

use crate::{
    components::{BAR_SIZE, Container, ContainerItems, container_items},
    systems::setup::{UniversalSearchWindow, WINDOW_SIZE},
};

pub fn effect_system(
    mut event_reader: EventReader<TweenEvent<&'static str>>,
    q_container: Query<Entity, With<Container>>,
    q_container_items: Query<Entity, With<ContainerItems>>,
    mut commands: Commands,
    mut q_input_region: Query<&mut InputRegion, With<UniversalSearchWindow>>,
) {
    event_reader.read().for_each(|event| match event.data {
        "UniversalSearchOpened" => {
            println!("got UniversalSearchOpened");
            // for entity in q_container.iter() {
            //     commands.entity(entity).remove::<TimeRunner>();
            // }
            //Update input region
            // for mut input_region in q_input_region.iter_mut() {
            //     input_region.0 = Rect::new(0., 0., 540., 576.);
            // }

            // //Spawn container items
            // q_container.iter().for_each(|entity| {
            //     // println!("spawning container items");
            //     commands.entity(entity).with_children(|parent| {
            //         parent.spawn(container_items());
            //     });
            // })
        }
        "UniversalSearchClosed" => {
            println!("got UniversalSearchClosed");
            //Despawn container items
            // q_container_items.iter().for_each(|entity| {
            //     // println!("despawning container items");
            //     commands.entity(entity).despawn();
            // });

            // //Update input region
            // for mut input_region in q_input_region.iter_mut() {
            //     input_region.0 =
            //         Rect::new(0., WINDOW_SIZE.1 - BAR_SIZE.1, BAR_SIZE.0, WINDOW_SIZE.1);
            // }
        }
        _ => {}
    })
}
