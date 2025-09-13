use bevy::{
    picking::pointer::PointerInteraction,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

mod components;
mod systems;
mod ui;

use components::WingWidgetPlugin;
use systems::*;

pub struct HomescreenPlugin;
impl Plugin for HomescreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WingWidgetPlugin, MeshPickingPlugin));
        app.add_systems(Startup, (setup, spawn_widgets).chain());
        app.add_systems(
            Update,
            (
                exit_on_esc,
                homescreen_event_handler,
                display_widgets,
                update_time,
                update_clock_node_size, // test_system,
            ),
        );
        app.add_observer(on_drag);
        app.add_observer(on_drop);
        app.init_resource::<HomescreenSettings>();
        app.add_event::<HomescreenEvent>();
    }
}

fn on_drop(
    released: Trigger<Pointer<DragEnd>>,
    mut event_writer: EventWriter<HomescreenEvent>,
    widgets: Query<&HomescreenWidget>,
) {
    let entity = released.target;
    if let Ok(widget) = widgets.get(entity) {
        event_writer.write(HomescreenEvent::DropWidget { id: widget.id });
    }
}

fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut event_writer: EventWriter<HomescreenEvent>,
    camera: Single<(&Camera, &GlobalTransform), With<HomescreenCamera>>,
    widgets: Query<&HomescreenWidget>,
) {
    let entity = drag.target;
    if let Ok(widget) = widgets.get(entity) {
        event_writer.write(HomescreenEvent::DragWidget {
            id: widget.id,
            location: camera
                .0
                .viewport_to_world_2d(camera.1, drag.pointer_location.position)
                .unwrap(),
        });
    }
}

// fn test_system(
//     keys: Res<ButtonInput<KeyCode>>,
//     window: Single<&Window, With<HomescreenWindow>>,
//     camera: Single<(&Camera, &GlobalTransform), With<HomescreenCamera>>,
//     mut event_writer: EventWriter<HomescreenEvent>,
//     mut local: Local<usize>,
//     mut size: Local<IVec2>,
// ) {
//     if keys.just_pressed(KeyCode::KeyA) {
//         size.x -= 1;
//     }
//     if keys.just_pressed(KeyCode::KeyD) {
//         size.x += 1;
//     }
//     if keys.just_pressed(KeyCode::KeyW) {
//         size.y -= 1;
//     }
//     if keys.just_pressed(KeyCode::KeyS) {
//         size.y += 1;
//     }

//     if keys.just_pressed(KeyCode::KeyC) {
//         *local += 1;
//         let colors: Vec<Color> = vec![
//             Color::srgb(1.0, 0.2, 0.2), // Red
//             Color::srgb(0.2, 0.8, 0.2), // Green
//             Color::srgb(0.2, 0.4, 1.0), // Blue
//             Color::srgb(1.0, 1.0, 0.2), // Yellow
//             Color::srgb(1.0, 0.5, 0.0), // Orange
//             Color::srgb(0.7, 0.2, 1.0), // Purple
//             Color::srgb(0.2, 1.0, 1.0), // Cyan
//             Color::srgb(1.0, 0.4, 0.8), // Pink
//         ];
//         *local %= colors.len();
//         if let Some(widget_position) = window.cursor_position() {
//             let widget_position = camera
//                 .0
//                 .viewport_to_world_2d(camera.1, widget_position)
//                 .unwrap();
//             event_writer.write(HomescreenEvent::CreateWidget {
//                 location: widget_position,
//                 info: HomescreenWidgetInfo {
//                     size: size.abs(),
//                     name: "Test Widget".into(),
//                     color: colors[*local],
//                 },
//             });
//         }
//     }
// }

#[derive(Component, Debug)]
pub struct HomescreenCamera;
#[derive(Component, Debug)]
pub struct HomescreenWindow;

#[derive(Debug, Clone)]
pub struct HomescreenWidgetInfo {
    pub size: IVec2,
    pub name: String,
    pub color: Color,
    pub image_handle: Option<Handle<Image>>,
}
#[derive(Event, Debug)]
pub enum HomescreenEvent {
    CreateWidget {
        location: Vec2,
        info: HomescreenWidgetInfo,
    },
    DragWidget {
        id: HomescreenWidgetId,
        location: Vec2,
    },
    DropWidget {
        id: HomescreenWidgetId,
    },
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct HomescreenWidgetId(pub usize);

#[derive(Component, Debug, Clone)]
pub struct HomescreenWidget {
    pub id: HomescreenWidgetId,
    pub info: HomescreenWidgetInfo,

    pub location: Vec2,
    pub grid_location: IVec2,

    pub size: Vec2,
    pub grid_size: IVec2,

    pub is_floating: bool,
    pub page: usize,
}

#[derive(Resource, Debug)]
pub struct HomescreenSettings {
    pub rows: usize,
    pub columns: usize,

    pub size: Vec2,
    pub position: Vec2,

    pub number_of_pages: usize,
    pub active_page: usize,
}

impl Default for HomescreenSettings {
    fn default() -> Self {
        Self {
            rows: 4,
            columns: 4,
            size: (520., 520.).into(),
            position: (-260.0, 260.0).into(),
            number_of_pages: 1,
            active_page: Default::default(),
        }
    }
}

impl HomescreenSettings {
    pub fn screen_to_grid(&self, screen_coordinates: &Vec2) -> IVec2 {
        let relative_coords = screen_coordinates - self.position;
        let cell_size = self.size / Vec2::new(self.columns as f32, self.rows as f32);

        let fractional_x = relative_coords.x / cell_size.x;
        let fractional_y = -relative_coords.y / cell_size.y;

        let clamped_x = fractional_x.clamp(0.0, self.columns as f32 - 1.0);
        let clamped_y = fractional_y.clamp(0.0, self.rows as f32 - 1.0);

        IVec2::new(clamped_x as i32, clamped_y as i32)
    }

    pub fn grid_to_screen(&self, grid_coordinate: &IVec2) -> Vec2 {
        let cell_size = self.size / Vec2::new(self.columns as f32, self.rows as f32);

        let relative_pos = Vec2::new(
            (grid_coordinate.x as f32) * cell_size.x,
            -(grid_coordinate.y as f32) * cell_size.y,
        );

        self.position + relative_pos
    }

    pub fn grid_to_screen_size(&self, grid_coordinate: &IVec2) -> Vec2 {
        let cell_size = self.size / Vec2::new(self.columns as f32, self.rows as f32);

        (
            cell_size.x * grid_coordinate.x as f32,
            cell_size.y * grid_coordinate.y as f32,
        )
            .into()
    }
}
