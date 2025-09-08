use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

/// A headless scrollbar widget, which can be used to build custom scrollbars. This component emits
/// [`ValueChange`] events when the scrollbar value changes.
///
/// Unlike sliders, scrollbars don't have an [`AccessibilityNode`] component, nor can they have
/// keyboard focus. This is because scrollbars are usually used in conjunction with a scrollable
/// container, which is itself accessible and focusable.
///
/// A scrollbar can have any number of child entities, but one entity must be the scrollbar
/// thumb, which is marked with the [`CoreScrollbarThumb`] component. Other children are ignored.
#[derive(Component, Debug)]
pub struct CoreScrollbar {
    /// Entity being scrolled.
    pub target: Entity,
    /// Whether the scrollbar is vertical or horizontal.
    pub orientation: Orientation,
    /// Minimum size of the scrollbar thumb, in pixel units.
    pub min_thumb_size: f32,
}

/// Marker component to indicate that the entity is a scrollbar thumb. This should be a child
/// of the scrollbar entity.
#[derive(Component, Debug)]
pub struct CoreScrollbarThumb;

#[derive(Component, Debug)]
#[require(ScrollbarDragState)]
pub struct CoreScrollArea;

impl CoreScrollbar {
    pub fn new(target: Entity, orientation: Orientation, min_thumb_size: f32) -> Self {
        Self {
            target,
            orientation,
            min_thumb_size,
        }
    }
}

/// Component used to manage the state of a scrollbar during dragging.
#[derive(Component, Default)]
pub struct ScrollbarDragState {
    /// Whether the scrollbar is currently being dragged.
    dragging: bool,
    /// The value of the scrollbar when dragging started.
    offset: ScrollPosition,
}

pub(crate) fn scrollbar_on_pointer_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    q_thumb: Query<&ChildOf, With<CoreScrollbarThumb>>,
    mut q_scrollbar: Query<(&CoreScrollbar, &GlobalTransform)>,
    mut _q_scroll_pos: Query<(&mut ScrollPosition, &ComputedNode), Without<CoreScrollbar>>,
) {
    if q_thumb.contains(trigger.target()) {
        // If they click on the thumb, do nothing. This will be handled by the drag event.
        trigger.propagate(false);
    } else if let Ok((_scrollbar, _transform)) = q_scrollbar.get_mut(trigger.target()) {
        // If they click on the scrollbar track, page up or down.
        trigger.propagate(false);
        // TODO: Finish this once we figure out how to get the local click coordinates.

        // let Ok((mut scroll_pos, scroll_content)) = q_scroll_pos.get_mut(scrollbar.target) else {
        //     return;
        // };
        // // Rather than scanning for the thumb entity, just calculate where the thumb will be.
        // let visible_size = scroll_content.size() * scroll_content.inverse_scale_factor;
        // let content_size = scroll_content.content_size() * scroll_content.inverse_scale_factor;
        // info!(
        //     "translation: {:?} location {:?}",
        //     transform.translation(),
        //     trigger.event().pointer_location.position
        // );
        // match scrollbar.orientation {
        //     Orientation::Horizontal => {
        //         // let hit_pos = trigger.event().pointer_location.position.x
        //         //     - transform.translation().x * scroll_content.inverse_scale_factor;
        //     }
        //     Orientation::Vertical => {
        //         // let hit_pos = trigger.event().pointer_location.position.y
        //         //     - transform.translation().y * scroll_content.inverse_scale_factor;
        //         // info!("scrollbar_on_pointer_down: {hit_pos}");
        //     }
        // }
    }
}

pub(crate) fn scrollbar_on_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    q_thumb: Query<&ChildOf, With<CoreScrollbarThumb>>,
    mut q_scroll_area: Query<(&ScrollPosition, &mut ScrollbarDragState), With<CoreScrollArea>>,
    mut q_scrollbar: Query<&CoreScrollbar>,
) {
    //if drag srarted on scroll bars
    if let Ok(ChildOf(thumb_parent)) = q_thumb.get(trigger.target()) {
        trigger.propagate(false);
        if let Ok(scrollbar) = q_scrollbar.get_mut(*thumb_parent) {
            if let Ok((scroll_pos, mut drag)) = q_scroll_area.get_mut(scrollbar.target) {
                drag.dragging = true;
                drag.offset = scroll_pos.clone();
            }
        }
    }

    //if drag srarted on scroll area
    if let Ok((scroll_pos, mut drag)) = q_scroll_area.get_mut(trigger.target()) {
        trigger.propagate(false);
        drag.dragging = true;
        drag.offset = scroll_pos.clone();
    };
}

pub(crate) fn scrollbar_on_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_scroll_area: Query<
        (&mut ScrollPosition, &ComputedNode, &mut ScrollbarDragState),
        (Without<CoreScrollbar>, With<CoreScrollArea>),
    >,
    mut q_scrollbar: Query<(&ComputedNode, &CoreScrollbar)>,
) {
    //if drag on scroll bar
    if let Ok((node, scrollbar)) = q_scrollbar.get_mut(trigger.target()) {
        trigger.propagate(false);
        let Ok((mut scroll_pos, scroll_content, drag)) = q_scroll_area.get_mut(scrollbar.target)
        else {
            return;
        };

        if drag.dragging {
            let distance = trigger.event().distance;
            let visible_size = scroll_content.size() * scroll_content.inverse_scale_factor;
            let content_size = scroll_content.content_size() * scroll_content.inverse_scale_factor;
            match scrollbar.orientation {
                Orientation::Horizontal => {
                    let range = (content_size.x - visible_size.x).max(0.);
                    let scrollbar_width = (node.size().x * node.inverse_scale_factor
                        - scrollbar.min_thumb_size)
                        .max(1.0);
                    scroll_pos.offset_x = if range > 0. {
                        (drag.offset.offset_x + (distance.x * content_size.x) / scrollbar_width)
                            .clamp(0., range)
                    } else {
                        0.
                    }
                }
                Orientation::Vertical => {
                    let range = (content_size.y - visible_size.y).max(0.);
                    let scrollbar_height = (node.size().y * node.inverse_scale_factor
                        - scrollbar.min_thumb_size)
                        .max(1.0);
                    scroll_pos.offset_y = if range > 0. {
                        (drag.offset.offset_y + (distance.y * content_size.y) / scrollbar_height)
                            .clamp(0., range)
                    } else {
                        0.
                    }
                }
            };
        }
    }

    //if drag srared on scroll content
    if let Ok((mut scroll_pos, scroll_content, drag)) = q_scroll_area.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            let distance = trigger.event().distance;
            let visible_size = scroll_content.size() * scroll_content.inverse_scale_factor;
            let content_size = scroll_content.content_size() * scroll_content.inverse_scale_factor;
            let x_range = (content_size.x - visible_size.x).max(0.);
            scroll_pos.offset_x = if x_range > 0. {
                (drag.offset.offset_x - distance.x).clamp(0., x_range)
            } else {
                0.
            };

            let y_range = (content_size.y - visible_size.y).max(0.);
            scroll_pos.offset_y = if y_range > 0. {
                (drag.offset.offset_y - distance.y).clamp(0., y_range)
            } else {
                0.
            };
        }
    }
}

pub(crate) fn scrollbar_on_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut q_scrollbar: Query<&mut ScrollbarDragState, (With<CoreScrollbar>, Without<CoreScrollArea>)>,
    mut q_scrollarea: Query<
        &mut ScrollbarDragState,
        (With<CoreScrollArea>, Without<CoreScrollbar>),
    >,
) {
    if let Ok(mut drag) = q_scrollbar.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            drag.dragging = false;
        }
    }

    if let Ok(mut drag) = q_scrollarea.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            drag.dragging = false;
        }
    }
}

fn update_scrollbar_thumb(
    q_scroll_area: Query<(&ScrollPosition, &ComputedNode)>,
    q_scrollbar: Query<(&CoreScrollbar, &ComputedNode, &Children)>,
    mut q_thumb: Query<&mut Node, With<CoreScrollbarThumb>>,
) {
    for (scrollbar, scrollbar_node, children) in q_scrollbar.iter() {
        let Ok(scroll_area) = q_scroll_area.get(scrollbar.target) else {
            continue;
        };

        // Size of the visible scrolling area.
        let visible_size = scroll_area.1.size() * scroll_area.1.inverse_scale_factor;

        // Size of the scrolling content.
        let content_size = scroll_area.1.content_size() * scroll_area.1.inverse_scale_factor;

        // Length of the scrollbar track.
        let track_length = scrollbar_node.size() * scrollbar_node.inverse_scale_factor;

        for child in children {
            if let Ok(mut thumb) = q_thumb.get_mut(*child) {
                match scrollbar.orientation {
                    Orientation::Horizontal => {
                        let thumb_size = if content_size.x > visible_size.x {
                            (track_length.x * visible_size.x / content_size.x)
                                .max(scrollbar.min_thumb_size)
                                .min(track_length.x)
                        } else {
                            track_length.x
                        };

                        let thumb_pos = if content_size.x > visible_size.x {
                            scroll_area.0.offset_x * (track_length.x - thumb_size)
                                / (content_size.x - visible_size.x)
                        } else {
                            0.
                        };

                        thumb.top = Val::Px(0.);
                        thumb.bottom = Val::Px(0.);
                        thumb.left = Val::Px(thumb_pos);
                        thumb.width = Val::Px(thumb_size);
                    }
                    Orientation::Vertical => {
                        let thumb_size = if content_size.y > visible_size.y {
                            (track_length.y * visible_size.y / content_size.y)
                                .max(scrollbar.min_thumb_size)
                                .min(track_length.y)
                        } else {
                            track_length.y
                        };

                        let thumb_pos = if content_size.y > visible_size.y {
                            scroll_area.0.offset_y * (track_length.y - thumb_size)
                                / (content_size.y - visible_size.y)
                        } else {
                            0.
                        };

                        thumb.left = Val::Px(0.);
                        thumb.right = Val::Px(0.);
                        thumb.top = Val::Px(thumb_pos);
                        thumb.height = Val::Px(thumb_size);
                    }
                };
            }
        }
    }
}

pub struct CoreScrollbarPlugin;

impl Plugin for CoreScrollbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(scrollbar_on_pointer_down)
            .add_observer(scrollbar_on_drag_start)
            .add_observer(scrollbar_on_drag_end)
            .add_observer(scrollbar_on_drag)
            // .add_observer(scrollarea_on_drag)
            .add_systems(PostUpdate, update_scrollbar_thumb);
    }
}
