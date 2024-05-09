use std::ops::Neg;

use mctk_core::component::Component;
use mctk_macros::{component, state_component_impl};

#[derive(Debug, Clone)]
pub enum CarouselMessage {
    ChildDragX(f32),
}

#[derive(Debug, Default)]
pub struct CarouselState {
    drag_x: f32,
}

#[component(State = "CarouselState", Internal)]
#[derive(Debug, Default)]
pub struct Carousel {}

impl Carousel {
    pub fn new() -> Self {
        Self {
            state: Some(CarouselState::default()),
            dirty: Default::default(),
        }
    }
}

#[state_component_impl(CarouselState)]
impl Component for Carousel {
    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        match msg.downcast_ref::<CarouselMessage>() {
            Some(CarouselMessage::ChildDragX(drag_x)) => {
                self.state_mut().drag_x = *drag_x;
            }
            _ => (),
        }

        vec![]
    }

    fn full_control(&self) -> bool {
        true
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        let delta_x = event.bounded_logical_delta().x.neg();
        if delta_x > 0. {
            self.state_mut().drag_x = delta_x;
        }
    }

    fn set_aabb(
        &mut self,
        aabb: &mut mctk_core::AABB,
        _parent_aabb: mctk_core::AABB,
        _children: Vec<(
            &mut mctk_core::AABB,
            Option<mctk_core::Scale>,
            Option<mctk_core::Point>,
        )>,
        _frame: mctk_core::AABB,
        _scale_factor: f32,
    ) {
        let drag_x = self.state_ref().drag_x;
        let up_ab = aabb.translate(drag_x, 0.);

        aabb.pos = up_ab.pos;
        println!("Carousel darg_x {:?}", drag_x);
        println!("Carousel aabb {:?}", aabb.pos);
    }
}
