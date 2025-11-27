mod ui;
mod events;
pub mod services;

pub mod prelude {
    pub use crate::ui::{StatusBar, Icon, IconName};
    pub use crate::events::AppEvents;
}