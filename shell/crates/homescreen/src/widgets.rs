use gpui::*;

struct WidgetId(pub usize);
struct WidgetWrapper<W: HomescreenWidget> {
    widget: W,
}

pub trait HomescreenWidget {
    fn render(&self) -> AnyElement;
}
