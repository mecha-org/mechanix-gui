use gpui::*;

mod animation_manager;
mod config;
mod input_manager;
mod layout_manager;
mod state;
mod widgets;

use crate::config::HomescreenConfig;
use crate::state::*;

pub struct Homescreen {
    pub state: HomescreenState,
}

impl Homescreen {
    pub fn new(config: HomescreenConfig) -> Self {
        let state = HomescreenState::new(config);
        Self { state }
    }
}

impl Render for Homescreen {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.state.render()
    }
}

pub mod prelude {
    pub use crate::config::*;
    pub use crate::Homescreen;
}
