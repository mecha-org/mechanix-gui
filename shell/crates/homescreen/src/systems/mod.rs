mod setup;
pub use setup::*;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    components::Wing, HomescreenCamera, HomescreenEvent, HomescreenSettings, HomescreenWidget,
    HomescreenWidgetId, HomescreenWidgetInfo,
};

pub fn display_widgets(
    mut commands: Commands,
    settings: Res<HomescreenSettings>,
    render_layer: Single<&RenderLayers, With<HomescreenCamera>>,
    new_widgets: Query<(Entity, &HomescreenWidget), Without<Wing>>,
    mut widgets: Query<(&HomescreenWidget, &mut Wing), With<Wing>>,
) {
    for (entity, widget) in &new_widgets {
        commands.entity(entity).insert_if_new((
            Wing {
                position: widget.location,
                size: widget.size,
                z_index: if widget.is_floating { 1.0 } else { 0.0 },
                background_color: widget.info.color,
                border_color: Color::WHITE,
                upper_wing: 0.0,
                lower_wing: 0.0,
                texture_handle: widget.info.image_handle.clone(),
            },
            render_layer.clone(),
        ));
    }
    let mut upper_winged_vertices = vec![];
    let mut lower_winged_vertices = vec![];
    for (widget, mut wing) in &mut widgets {
        wing.z_index = if widget.is_floating { 1.0 } else { 0.0 };
        wing.upper_wing = 0.0;
        wing.lower_wing = 0.0;
        wing.position = widget.location;
        wing.size = widget.size - 10.0;
        if widget.is_floating {
            continue;
        }
        if widget.grid_size.x == 4 && widget.grid_size.y == 2 {
            if widget.grid_location.y > 0 {
                upper_winged_vertices.push((widget.grid_location.y, 2));
                upper_winged_vertices.push((widget.grid_location.y, 3));
                wing.upper_wing = settings.size.x * 0.4875;
            } else {
                wing.upper_wing = 0.0;
            }
            if widget.grid_location.y < 2 {
                wing.lower_wing = settings.size.x * 0.4875;
                lower_winged_vertices.push((widget.grid_location.y + widget.grid_size.y, 0));
                lower_winged_vertices.push((widget.grid_location.y + widget.grid_size.y, 1));
            } else {
                wing.lower_wing = 0.0;
            }
        }
        if widget.grid_size.x >= 3 && widget.grid_size.y == 3 {
            if widget.grid_location.y > 0 {
                upper_winged_vertices.push((widget.grid_location.y, 2));
                upper_winged_vertices.push((widget.grid_location.y, 3));
                wing.upper_wing = settings.size.x * 0.4875;
            } else {
                wing.upper_wing = 0.0;
            }
            if widget.grid_location.y < 1 {
                lower_winged_vertices.push((widget.grid_location.y + widget.grid_size.y, 0));
                lower_winged_vertices.push((widget.grid_location.y + widget.grid_size.y, 1));
                wing.lower_wing = settings.size.x * 0.4875;
            } else {
                wing.lower_wing = 0.0;
            }
        }
    }
    for (widget, mut wing) in &mut widgets {
        if widget.is_floating {
            continue;
        }
        wing.position = widget.location + settings.size * 0.0125;
        if widget.grid_size.x == 4 {
            continue;
        }
        if upper_winged_vertices.contains(&(
            widget.grid_location.y + widget.grid_size.y,
            widget.grid_location.x + widget.grid_size.x - 1,
        )) {
            wing.size.y = widget.size.y + settings.size.y * 0.0425;
            if widget.grid_location.x < 3 {
                wing.lower_wing =
                    (widget.location.x + widget.size.x - settings.size.x * 0.075).max(0.0);
            }
        }
        if lower_winged_vertices.contains(&(widget.grid_location.y, widget.grid_location.x)) {
            wing.position.y = widget.location.y + settings.size.y * 0.075;
            wing.size.y = widget.size.y + settings.size.y * 0.0425;
            wing.upper_wing = (-widget.location.x - settings.size.y * 0.075).max(0.0);
        }
    }
}

pub fn homescreen_event_handler(
    mut commands: Commands,
    settings: Res<HomescreenSettings>,
    mut event_reader: EventReader<HomescreenEvent>,
    mut widgets: Query<(&mut HomescreenWidget)>,
) {
    let mut next_id = widgets.iter().len();
    for event in event_reader.read() {
        match event {
            HomescreenEvent::CreateWidget { location, info } => {
                let is_empty = is_cell_empty_for_widget(
                    location,
                    info,
                    settings.as_ref(),
                    widgets
                        .iter()
                        .filter_map(|widget| {
                            if widget.page != settings.active_page {
                                None
                            } else {
                                Some(widget)
                            }
                        })
                        .collect(),
                );
                if is_empty {
                    next_id += 1;
                    commands.spawn((HomescreenWidget {
                        id: crate::HomescreenWidgetId(next_id),
                        info: info.clone(),
                        location: settings.grid_to_screen(&settings.screen_to_grid(location)),
                        grid_location: settings.screen_to_grid(location),
                        size: settings.grid_to_screen_size(&info.size).abs(),
                        grid_size: info.size.clone(),
                        is_floating: false,
                        page: settings.active_page,
                    },));
                } else if shuffle_widgets_to_make_space(
                    location,
                    info,
                    settings.as_ref(),
                    widgets
                        .iter_mut()
                        .filter_map(|widget| {
                            if widget.page != settings.active_page {
                                None
                            } else {
                                Some(widget.into_inner())
                            }
                        })
                        .collect(),
                ) {
                    let mut size = info.size;
                    next_id += 1;
                    commands.spawn((HomescreenWidget {
                        id: crate::HomescreenWidgetId(next_id),
                        info: info.clone(),
                        location: settings.grid_to_screen(&settings.screen_to_grid(location)),
                        grid_location: settings.screen_to_grid(location),
                        size: settings.grid_to_screen_size(&info.size),
                        grid_size: info.size.clone(),
                        page: settings.active_page,
                        is_floating: false,
                    },));
                }
            }
            HomescreenEvent::DragWidget { id, location } => {
                widgets
                    .iter_mut()
                    .filter(|widget| widget.id == *id)
                    .for_each(|mut widget| {
                        widget.location.x = location.x - widget.size.x / 2.0;
                        widget.location.y = location.y + widget.size.y / 2.0;
                        widget.is_floating = true;
                    });
            }
            HomescreenEvent::DropWidget { id } => {
                let widget = widgets
                    .iter()
                    .filter(|widget| widget.id == *id)
                    .last()
                    .unwrap();
                let mut location = widget.location;
                location.x = location.x + settings.grid_to_screen_size(&IVec2::new(1, 1)).x / 2.0;
                location.y = location.y - settings.grid_to_screen_size(&IVec2::new(1, 1)).x / 2.0;
                let info = widget.info.clone();
                let is_empty = is_cell_empty_for_widget(
                    &widget.location,
                    &widget.info,
                    settings.as_ref(),
                    widgets
                        .iter()
                        .filter_map(|widget| {
                            if widget.page != settings.active_page {
                                None
                            } else {
                                Some(widget)
                            }
                        })
                        .collect(),
                );

                if is_empty {
                    let mut widget = widgets
                        .iter_mut()
                        .filter(|widget| widget.id == *id)
                        .last()
                        .unwrap();
                    widget.location = settings.grid_to_screen(&settings.screen_to_grid(&location));
                    widget.grid_location = settings.screen_to_grid(&location);
                } else if shuffle_widgets_to_make_space(
                    &location,
                    &info,
                    settings.as_ref(),
                    widgets
                        .iter_mut()
                        .filter_map(|widget| {
                            if widget.page != settings.active_page || widget.id == *id {
                                None
                            } else {
                                Some(widget.into_inner())
                            }
                        })
                        .collect(),
                ) {
                    let mut widget = widgets
                        .iter_mut()
                        .filter(|widget| widget.id == *id)
                        .last()
                        .unwrap();
                    widget.location = settings.grid_to_screen(&settings.screen_to_grid(&location));
                    widget.grid_location = settings.screen_to_grid(&location);
                    widget.is_floating = false;
                } else {
                    let mut widget = widgets
                        .iter_mut()
                        .filter(|widget| widget.id == *id)
                        .last()
                        .unwrap();
                    widget.location = settings.grid_to_screen(&widget.grid_location);
                    widget.is_floating = false;
                }
            }
        }
    }
}

fn is_cell_empty_for_widget(
    location: &Vec2,
    info: &HomescreenWidgetInfo,
    settings: &HomescreenSettings,
    widgets: Vec<&HomescreenWidget>,
) -> bool {
    let grid_location = settings.screen_to_grid(location);
    let grid_size = info.size;

    for widget in widgets {
        for x in 0..widget.grid_size.x {
            for y in 0..widget.grid_size.y {
                let cell = widget.grid_location + IVec2::new(x, y);
                for x in 0..grid_size.x {
                    for y in 0..grid_size.y {
                        let (x, y) = (IVec2::new(x, y) + grid_location).into();
                        if x < 0 || x > settings.columns as i32 {
                            return false;
                        }
                        if y < 0 || y > settings.rows as i32 {
                            return false;
                        }
                        if cell.x == x && cell.y == y {
                            return false;
                        }
                    }
                }
            }
        }
    }
    return true;
}

fn shuffle_widgets_to_make_space(
    location: &Vec2,
    info: &HomescreenWidgetInfo,
    settings: &HomescreenSettings,
    widgets: Vec<&mut HomescreenWidget>,
) -> bool {
    let grid_location = settings.screen_to_grid(location);
    let grid_size = info.size;
    if grid_size.x + grid_location.x > settings.columns as i32 {
        return false;
    }
    if grid_size.y + grid_location.y > settings.rows as i32 {
        return false;
    }

    let mut buffer = vec![];
    let mut removed_widgets_ids = vec![];
    for widget in &widgets {
        buffer.push((
            widget.id.clone(),
            widget.grid_location.clone(),
            widget.grid_size.clone(),
        ));
    }

    for (id, location, size) in &buffer {
        let overlap_x =
            location.x < grid_location.x + grid_size.x && location.x + size.x > grid_location.x;
        let overlap_y =
            location.y < grid_location.y + grid_size.y && location.y + size.y > grid_location.y;

        if overlap_x && overlap_y {
            removed_widgets_ids.push(id.clone());
        }
    }

    let removed_widgets: Vec<(HomescreenWidgetId, IVec2, IVec2)> = buffer
        .clone()
        .into_iter()
        .filter(|(id, _, _)| removed_widgets_ids.contains(id))
        .collect();
    let mut buffer: Vec<(HomescreenWidgetId, IVec2, IVec2)> = buffer
        .clone()
        .into_iter()
        .filter(|(id, _, _)| !removed_widgets_ids.contains(id))
        .collect();
    buffer.push((HomescreenWidgetId(9999), grid_location, grid_size));
    let mut count = 0;
    for (new_id, new_location, new_size) in &removed_widgets {
        let mut possibilities: Vec<IVec2> = vec![];
        for x in 0..settings.columns as i32 {
            for y in 0..settings.rows as i32 {
                let mut flag = true;
                if x + new_size.x > settings.columns as i32 {
                    continue;
                }
                if y + new_size.y > settings.rows as i32 {
                    continue;
                }
                for (_, location, size) in buffer.clone() {
                    let overlap_x = location.x < x + new_size.x && location.x + size.x > x;
                    let overlap_y = location.y < y + new_size.y && location.y + size.y > y;

                    if overlap_x && overlap_y {
                        flag = false;
                    }
                }
                if flag {
                    possibilities.push((x, y).into());
                }
            }
        }
        if possibilities.len() == 0 {
            return false;
        }
        let mut min_distance = 10000;
        let mut output: IVec2 = possibilities.last().unwrap().clone();
        for possibility in possibilities {
            let distance = (possibility - *new_location).abs();
            let distance = distance.x + distance.y;
            if distance < min_distance {
                min_distance = distance;
                output = possibility.clone();
            }
        }
        buffer.push((new_id.clone(), output.clone(), new_size.clone()));
    }
    for widget in widgets {
        let (_, grid_location, _) = buffer
            .iter()
            .filter(|(id, _, _)| *id == widget.id)
            .last()
            .unwrap();

        widget.grid_location = grid_location.clone();
        widget.location = settings.grid_to_screen(grid_location);
    }
    return true;
}
