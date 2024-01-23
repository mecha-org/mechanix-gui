use wayland_client::protocol::wl_output::Transform;

#[derive(Debug, Clone)]
pub struct Orientation {
    pub vector: (f32, f32),
    pub wayland_state: Transform,
    pub x_state: &'static str,
    pub matrix: [&'static str; 9],
}

pub trait DisplayManager: Send {
    /// Change the orientation of the target display.
    fn change_rotation_state(&mut self, new_state: &Orientation);

    /// Get the current transformation of the target display.
    fn get_rotation_state(&mut self) -> Result<Transform, String>;
}

pub mod sway;
pub mod wlroots;
pub mod xorg;
