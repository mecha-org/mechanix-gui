use crate::contexts::camera;
use mctk_core::prelude::*;

#[derive(Debug)]
pub struct Preview;
impl Preview {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Preview {
    fn init(&mut self) {
        camera::Camera::pick_optimal_display_resolution();
        camera::Camera::init();
        camera::Camera::start_fetching();
    }

    fn view(&self) -> Option<Node> {
        let image_buffer = camera::Camera::get_buffer();
        let width = camera::Camera::get_width() as usize;
        let height = camera::Camera::get_height() as usize;
        let aspect_ratio = width as f32 / height as f32;

        Some(node!(
            Image::from_buffer(image_buffer, width, height),
            lay![size: size!(480.0, 480.0 / aspect_ratio)]
        ))
    }
}
