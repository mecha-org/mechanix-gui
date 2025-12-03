#[derive(Debug, Default, Copy, Clone)]
pub struct HomescreenConfig {
    pub window: WindowConfig,
}

#[derive(Debug, Copy, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            height: 540.0,
            width: 540.0,
        }
    }
}
