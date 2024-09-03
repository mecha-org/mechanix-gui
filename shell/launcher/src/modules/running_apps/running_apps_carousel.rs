use std::{
    hash::{Hash, Hasher},
    ops::Neg,
};

use mctk_core::{
    component::{Component, ComponentHasher},
    lay,
    layout::ScrollPosition,
    msg, node,
    renderables::{rect::InstanceBuilder, Rect, Renderable},
    size_pct,
    widgets::{Div, TransitionPositions},
    Color, Point, Scale, AABB,
};
use mctk_macros::{component, state_component_impl};

use crate::gui::Message;

#[derive(Debug, Clone)]
pub enum CarouselMessage {
    ChildDragX(f32),
    ChildDragXSlow(f32, f32),
}

#[derive(Debug, Default)]
pub struct CarouselState {
    scroll_position: Point,
    drag_start_position: Point,
    transition: Option<TransitionPositions>,
    aabb: Option<AABB>,
    inner_scale: Option<Scale>,
    init_scroll_position_set: bool,
}

#[component(State = "CarouselState", Internal)]
#[derive(Debug, Default)]
pub struct Carousel {
    children: Vec<(AABB, Option<Scale>, Option<Point>)>,
}

impl Carousel {
    pub fn new() -> Self {
        // println!("Carousel::new()");
        Self {
            children: vec![],
            state: Some(CarouselState::default()),
            dirty: Default::default(),
        }
    }

    fn x_scrollable(&self) -> bool {
        true
    }

    fn y_scrollable(&self) -> bool {
        true
    }

    fn scrollable(&self) -> bool {
        self.x_scrollable() || self.y_scrollable()
    }

    fn handle_input_down(
        &mut self,
        current_physical_aabb: AABB,
        current_inner_scale: Option<Scale>,
    ) {
        self.state_mut().aabb = Some(current_physical_aabb);
        self.state_mut().inner_scale = current_inner_scale;
        let drag_start = self.state_ref().scroll_position;
        self.state_mut().drag_start_position = drag_start;
    }

    fn handle_drag_start(&mut self) {
        let drag_start = self.state_ref().scroll_position;
        self.state_mut().drag_start_position = drag_start;
    }

    fn handle_drag(
        &mut self,
        physical_delta: Point,
        current_physical_aabb: AABB,
        current_inner_scale: Option<Scale>,
    ) {
        let start_position = self.state_ref().drag_start_position;
        let mut scroll_position = self.state_ref().scroll_position;
        let delta_position = physical_delta.x.neg();
        let max_position = current_inner_scale.unwrap().width - current_physical_aabb.size().width;
        scroll_position.x = (start_position.x + delta_position)
            .round()
            .min(max_position)
            .max(0.0);
        self.state_mut().scroll_position = scroll_position;
    }

    fn handle_drag_end(&mut self, logical_delta: Point) {
        let children = &self.children;
        if children.is_empty() {
            println!("no children");
            return;
        }

        let (child_aabb, ..) = children.get(0).unwrap();
        let dx = logical_delta.x.neg();
        let w: f32 = child_aabb.width();
        self.update(msg!(CarouselMessage::ChildDragXSlow(dx, w)));
    }
}

#[state_component_impl(CarouselState)]
impl Component for Carousel {
    // fn new_props(&mut self) {
    //     let children_len = self.children.len();
    //     println!("children_len {:?}", children_len);
    //     if children_len >= 3 {
    //         let x = (children_len - 2) as f32 * 180. + (children_len - 2 - 1) as f32 * 180.;
    //         println!("x is {:?}", x);
    //         self.state_mut().scroll_position = Point::new(x, 0.0);
    //     }
    // }

    // fn props_hash(&self, hasher: &mut ComponentHasher) {
    //     for (aabb, ..) in &self.children {
    //         (if aabb.pos.x < 0. {
    //             (aabb.pos.x * -1.) as u32
    //         } else {
    //             aabb.pos.x as u32
    //         })
    //         .hash(hasher);
    //     }
    //     println!("props hash {:?}", hasher.finish());
    // }

    fn full_control(&self) -> bool {
        true
    }

    fn spacing(&self) -> Scale {
        Scale::new(0., 0.)
    }

    fn set_aabb(
        &mut self,
        aabb: &mut AABB,
        _parent_aabb: AABB,
        children: Vec<(&mut AABB, Option<Scale>, Option<Point>)>,
        _frame: AABB,
        _scale_factor: f32,
    ) {
        let mut updated_children = vec![];
        let mut total_width = 0.;
        for (i, (aabb, scale, point)) in children.into_iter().enumerate() {
            // let spacing = self.spacing();
            // let mut pos = aabb.pos;
            // pos.x += spacing.width * i as f32; // change here
            total_width += aabb.width();
            // aabb.set_top_left_mut(pos.x, pos.y);
            updated_children.push((aabb.clone(), scale, point));
        }

        // println!("updated_children {:?}", updated_children.len());
        let children_len = updated_children.len();
        // println!("children_len {:?}", children_len);
        if self.state.is_some() {
            if !self.state_ref().init_scroll_position_set {
                // if children_len >= 3 {
                // let x = (children_len - 2) as f32 * 180. + (children_len - 2 - 1) as f32 * 180.;
                // + self.spacing().width / 2.;
                // println!("total_width is {:?}", total_width);
                self.state_mut().scroll_position = Point::new(total_width - 480., 0.0);
                // }
                self.state_mut().init_scroll_position_set = true;
            }
        }
        self.children = updated_children;
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        if self.state.is_some() {
            self.state_ref().scroll_position.hash(hasher);
        }
    }

    fn on_tick(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Tick>) {
        //Update scroll position based on velocity and frames per seconds
        if let Some(TransitionPositions { from, to, velocity }) = self.state_ref().transition {
            let mut scroll_position = self.state_ref().scroll_position;
            let distance = from.dist(to).round();
            let distance_scrolled = scroll_position.dist(from).round();

            if distance != 0. && scroll_position != to && distance_scrolled != distance {
                if to.x >= from.x {
                    scroll_position.x += distance * velocity;
                } else {
                    scroll_position.x -= distance * velocity;
                }

                self.state_mut().scroll_position = scroll_position;
            } else {
                self.state_mut().transition = None;
            }
        }
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        match msg.downcast_ref::<CarouselMessage>() {
            Some(CarouselMessage::ChildDragX(drag_x)) => {
                if self.state_ref().transition.is_some() {
                    return vec![];
                }

                let start_position = self.state_ref().drag_start_position;
                let mut scroll_position = self.state_ref().scroll_position;
                let delta_position = *drag_x;
                let max_position = self.state_ref().inner_scale.unwrap().width
                    - self.state_ref().aabb.unwrap().size().width;
                scroll_position.x = (start_position.x + delta_position)
                    .round()
                    .min(max_position)
                    .max(0.0);
                self.state_mut().scroll_position = scroll_position;
            }
            Some(CarouselMessage::ChildDragXSlow(dx, w)) => {
                if self.state_ref().transition.is_some() {
                    return vec![];
                }
                let drag_x = if *dx > 0. {
                    w + ((dx / w).floor() * w) + (48. * ((dx / w).floor() + 1.))
                } else {
                    w.neg()
                        + ((dx / w).abs().floor().neg() * w)
                        + (48. * ((dx / w).abs().floor().neg() - 1.))
                };

                let start_position = self.state_ref().drag_start_position;
                let mut scroll_position = self.state_ref().scroll_position;
                let delta_position = drag_x;
                let max_position = self.state_ref().inner_scale.unwrap().width
                    - self.state_ref().aabb.unwrap().size().width;
                scroll_position.x = (start_position.x + delta_position)
                    .round()
                    .min(max_position)
                    .max(0.0);
                self.state_mut().transition = Some(TransitionPositions {
                    from: self.state_ref().scroll_position,
                    to: scroll_position,
                    velocity: 0.02,
                });
            }
            _ => (),
        }

        let mut pm: Vec<mctk_core::component::Message> = vec![];

        if let Some(x) = msg.downcast_ref::<Message>() {
            pm.push(Box::new(x.clone()));
        }

        pm
    }

    fn scroll_position(&self) -> Option<ScrollPosition> {
        if self.scrollable() {
            let p = self.state_ref().scroll_position;
            Some(ScrollPosition {
                x: if self.x_scrollable() { Some(p.x) } else { None },
                y: if self.y_scrollable() { Some(p.y) } else { None },
            })
        } else {
            None
        }
    }

    fn on_mouse_down(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::MouseDown>) {
        self.handle_input_down(event.current_physical_aabb(), event.current_inner_scale())
    }

    fn on_touch_down(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDown>) {
        self.handle_input_down(event.current_physical_aabb(), event.current_inner_scale())
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        if self.state_ref().transition.is_some() {
            return;
        }
        self.handle_drag_start();
        event.stop_bubbling();
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        if self.state_ref().transition.is_some() {
            return;
        }
        self.handle_drag_start();
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        if self.state_ref().transition.is_some() {
            return;
        }
        self.handle_drag(
            event.physical_delta(),
            event.current_physical_aabb(),
            event.current_inner_scale(),
        );
    }

    fn on_touch_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDrag>) {
        if self.state_ref().transition.is_some() {
            return;
        }
        self.handle_drag(
            event.physical_delta(),
            event.current_physical_aabb(),
            event.current_inner_scale(),
        );
    }

    fn on_drag_end(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragEnd>) {
        self.handle_drag_end(event.logical_delta());
    }

    fn on_touch_drag_end(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragEnd>,
    ) {
        self.handle_drag_end(event.logical_delta());
    }

    // fn render(
    //     &mut self,
    //     context: mctk_core::component::RenderContext,
    // ) -> Option<Vec<mctk_core::renderables::Renderable>> {
    //     println!(
    //         "scroll position {:?} drag position {:?}",
    //         self.state_ref().scroll_position,
    //         self.state_ref().drag_start_position
    //     );
    //     let width = context.aabb.width();
    //     let height = context.aabb.height();
    //     let mut pos = context.aabb.pos;
    //     let mut rs = vec![];

    //     //Background
    //     let background = InstanceBuilder::default()
    //         .pos(pos)
    //         .scale(Scale { width, height })
    //         .color(Color::RED)
    //         .build()
    //         .unwrap();

    //     rs.push(Renderable::Rect(Rect::from_instance_data(background)));

    //     Some(rs)
    // }
}
