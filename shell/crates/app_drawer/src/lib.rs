mod models;
pub mod ui;
pub mod prelude {
    pub use crate::models::*;
    pub use crate::ui::AppDrawer;
    pub use crate::ui::icon::*;
    pub use crate::ui::input::*;
}
