#![deny(clippy::all)]

mod errors;
pub use errors::{DisplayError, DisplayErrorCodes};

mod display;
pub use display::Display;