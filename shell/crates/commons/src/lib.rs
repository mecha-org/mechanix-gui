pub mod assets;
pub mod toml_merge;
pub mod widgets;

pub mod prelude {
    pub use crate::assets::Assets;
    pub use crate::toml_merge::*;
    pub use crate::widgets::Wing;
}
