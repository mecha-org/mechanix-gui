use mctk_core::component::Component;
use mctk_core::event::Event;
use mctk_core::layout::{Direction, PositionType, ScrollPosition, Size};
use mctk_core::widgets::{Div, RoundedRect};
use mctk_core::{lay, rect, size};
use mctk_core::{msg, types::*};
use mctk_core::{node, node::Node};
use mctk_macros::{component, state_component_impl};
use std::collections::VecDeque;
use std::hash::Hash;
use std::ops::Neg;

use crate::gui::Message;
use crate::utils::{find_t_for_y, get_translations};

#[derive(Debug, Default)]
pub struct ScrollableState {
    //Current scroll position
    scroll_position: Point,

    //Position of scrollable when drag was started
    drag_start_position: Point,

    aabb: Option<AABB>,

    translations: VecDeque<f64>,
}

#[component(State = "ScrollableState")]
#[derive(Debug, Default)]
pub struct Scrollable {
    size: Size,
}

impl Scrollable {
    pub fn new(s: Size, scroll_position: f32) -> Self {
        Self {
            state: Some(ScrollableState {
                scroll_position: Point::new(scroll_position, 0.),
                ..Default::default()
            }),
            dirty: false,
            size: s,
        }
    }
}

#[state_component_impl(ScrollableState)]
impl Component for Scrollable {
    fn on_tick(&mut self, _event: &mut mctk_core::event::Event<mctk_core::event::Tick>) {
        let mut translations = self.state_ref().translations.clone();

        if translations.len() == 0 {
            return;
        }

        if let Some(t) = translations.pop_front() {
            self.state_mut().scroll_position = Point::new(t as f32, 0.);
        };

        self.state_mut().translations = translations;
    }

    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.state_ref().translations.len().hash(hasher);

        // if self.state.is_some() {
        //     self.state_ref().scroll_position.hash(hasher);
        // }
        // println!("Scrollable::render_hash() {:?}", hasher.finish());
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        event.stop_bubbling();
        println!("Scrollable::on_drag_start()");
        //Current scroll position will become drag start position when drag is started
        let drag_start = self.state_ref().scroll_position;
        self.state_mut().drag_start_position = drag_start;
        event.emit(msg!(Message::Scrolling { status: true }));
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        event.stop_bubbling();
        //Current scroll position will become drag start position when drag is started
        let drag_start = self.state_ref().scroll_position;
        self.state_mut().drag_start_position = drag_start;
        event.emit(msg!(Message::Scrolling { status: true }));
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        println!("Scrollable::on_drag()");
        //on drag we will update scroll position
        let start_position = self.state_ref().drag_start_position;
        let size = event.current_physical_aabb().size();
        let inner_scale = event.current_inner_scale().unwrap();
        let mut scroll_position = self.state_ref().scroll_position;
        let drag = event.physical_delta().x.neg();
        let delta_position = drag * (inner_scale.width / size.width);
        let max_position = inner_scale.width - size.width;
        scroll_position.x = (start_position.x + delta_position)
            .round()
            .min(max_position)
            .max(0.0);
        self.state_mut().scroll_position = scroll_position;
    }

    fn on_touch_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDrag>) {
        //on drag we will update scroll position
        let start_position = self.state_ref().drag_start_position;
        let size = event.current_physical_aabb().size();
        let inner_scale = event.current_inner_scale().unwrap();
        let mut scroll_position = self.state_ref().scroll_position;
        let drag = event.physical_delta().x.neg();
        let delta_position = drag * (inner_scale.width / size.width);
        let max_position = inner_scale.width - size.width;
        scroll_position.x = (start_position.x + delta_position)
            .round()
            .min(max_position)
            .max(0.0);
        self.state_mut().scroll_position = scroll_position;
        // println!("scroll_position {:?}", scroll_position);
    }
    fn on_drag_end(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragEnd>) {
        let scroll_position = self.state_ref().scroll_position.x;
        let start_position = self.state_ref().drag_start_position.x;

        // Define snapping points
        let snap_points = [0.0, 66.0, 286.0];
        let mut next_scroll_position = scroll_position;

        let drag_right = event.logical_delta().x.is_sign_negative();
        if drag_right {
            for snap_point in snap_points.into_iter() {
                if scroll_position <= snap_point {
                    next_scroll_position = snap_point;
                    break;
                }
            }
        } else {
            for snap_point in snap_points.into_iter().rev() {
                if scroll_position >= snap_point {
                    next_scroll_position = snap_point;
                    break;
                }
            }
        }

        let bezier_points: [f64; 4] = if drag_right {
            [
                scroll_position as f64,
                scroll_position as f64,
                next_scroll_position as f64,
                next_scroll_position as f64,
            ]
        } else {
            [
                next_scroll_position as f64,
                next_scroll_position as f64,
                scroll_position as f64,
                scroll_position as f64,
            ]
        };

        let step = 1.0 / 12.0;
        let t = find_t_for_y(bezier_points, scroll_position as f64).unwrap_or(0.0);

        let start_time = if drag_right {
            if (scroll_position as f64 - bezier_points[3]).abs() < 0.1 {
                1.0
            } else {
                t
            }
        } else {
            if (scroll_position as f64 - bezier_points[0]).abs() < 0.1 {
                0.0
            } else {
                t
            }
        };
        let translations = get_translations(bezier_points, start_time, step, !drag_right);
        self.state_mut().translations = translations.into();
        event.emit(msg!(Message::Scrolling { status: false }));
    }

    fn on_touch_drag_end(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragEnd>,
    ) {
        let scroll_position = self.state_ref().scroll_position.x;
        let start_position = self.state_ref().drag_start_position.x;

        // Define snapping points
        let snap_points = [0.0, 66.0, 286.0];
        let mut next_scroll_position = scroll_position;

        let drag_right = event.logical_delta().x.is_sign_negative();
        if drag_right {
            for snap_point in snap_points.into_iter() {
                if scroll_position <= snap_point {
                    next_scroll_position = snap_point;
                    break;
                }
            }
        } else {
            for snap_point in snap_points.into_iter().rev() {
                if scroll_position >= snap_point {
                    next_scroll_position = snap_point;
                    break;
                }
            }
        }

        let bezier_points: [f64; 4] = if drag_right {
            [
                scroll_position as f64,
                scroll_position as f64,
                next_scroll_position as f64,
                next_scroll_position as f64,
            ]
        } else {
            [
                next_scroll_position as f64,
                next_scroll_position as f64,
                scroll_position as f64,
                scroll_position as f64,
            ]
        };

        let step = 1.0 / 12.0;
        let t = find_t_for_y(bezier_points, scroll_position as f64).unwrap_or(0.0);

        let start_time = if drag_right {
            if (scroll_position as f64 - bezier_points[3]).abs() < 0.1 {
                1.0
            } else {
                t
            }
        } else {
            if (scroll_position as f64 - bezier_points[0]).abs() < 0.1 {
                0.0
            } else {
                t
            }
        };
        let translations = get_translations(bezier_points, start_time, step, !drag_right);
        self.state_mut().translations = translations.into();
        event.emit(msg!(Message::Scrolling { status: false }));
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0, 1])
    }

    fn scroll_position(&self) -> Option<ScrollPosition> {
        let p = self.state_ref().scroll_position;
        Some(ScrollPosition {
            x: Some(p.x),
            y: None,
        })
    }

    fn full_control(&self) -> bool {
        true
    }

    fn set_aabb(
        &mut self,
        aabb: &mut AABB,
        _parent_aabb: AABB,
        _children: Vec<(&mut AABB, Option<Scale>, Option<Point>)>,
        _frame: AABB,
        _scale_factor: f32,
    ) {
        if self.state_ref().aabb == Some(*aabb) {
            return;
        }

        self.state_mut().aabb = Some(*aabb);
        // aabb.set_scale(self.size.width.into(), self.size.height.into());
    }

    fn view(&self) -> Option<Node> {
        let size = self.state_ref().aabb.clone().unwrap_or_default().size();
        println!("size is {:?}", size);
        let scroll_x = self.state_ref().scroll_position.x;

        Some(
            node!(
                Div::new(),
                lay![
                    size: [Auto]
                ]
            )
            .key(scroll_x as u64)
            .push(node!(
                RoundedRect {
                    scissor: Some(false),
                    background_color: Color::TRANSPARENT,
                    border_color: Color::TRANSPARENT,
                    border_width: (0., 0., 0., 0.),
                    radius: (0., 0., 0., 0.),
                    swipe: 0
                },
                lay![
                    size: [size.width, size.height],
                    position_type: PositionType::Absolute,
                    position: [0., 0., 0., 0.]
                ]
            ))
            .push(node!(
                Div::new(),
                lay![
                    direction: Direction::Column,
                ]
            ))
            .push(node!(
                RoundedRect {
                    scissor: Some(true),
                    background_color: Color::TRANSPARENT,
                    border_color: Color::TRANSPARENT,
                    border_width: (0., 0., 0., 0.),
                    radius: (0., 0., 0., 0.),
                    swipe: 0
                },
                lay![
                    size: [size.width, size.height],
                    position_type: PositionType::Absolute,
                    position: [0., 0., 0., 0.]
                ]
            )),
        )
    }
}
