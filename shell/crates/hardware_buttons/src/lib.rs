use gpui::App;

mod handle;
mod signal;

pub fn init(cx: &mut App) {
	signal::init(cx);
}
