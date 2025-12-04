use commons::widgets::wing;
use gpui::*;

use crate::utils::GridBounds;

pub mod demo_widget;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub(crate) struct WidgetId(pub usize);

pub(crate) struct WidgetData {
    widget: Box<dyn HomescreenWidget>,
    page_number: usize,
    grid_bounds: GridBounds,
    is_being_dragged: bool,
    bounds: Bounds<Pixels>,
}

impl WidgetData {
    pub fn new(
        widget: Box<dyn HomescreenWidget>,
        page_number: usize,
        grid_bounds: GridBounds,
        bounds: Bounds<Pixels>,
    ) -> Self {
        Self {
            widget,
            page_number,
            grid_bounds,
            is_being_dragged: false,
            bounds,
        }
    }

    // Getter for widget (no setter - immutable after construction)
    pub fn widget(&self) -> &dyn HomescreenWidget {
        &*self.widget
    }

    pub fn widget_mut(&mut self) -> &mut dyn HomescreenWidget {
        &mut *self.widget
    }

    // Getter and setter for page_number
    pub fn page_number(&self) -> usize {
        self.page_number
    }

    pub fn set_page_number(&mut self, page_number: usize) {
        self.page_number = page_number;
    }

    // Getter and setter for bounds
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    pub fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }
}

pub struct WidgetWrapper;
impl WidgetWrapper {
    pub fn render(widget: &dyn HomescreenWidget) -> impl IntoElement {
        let mut w = wing();
        w.border_radius(px(12.0));
        let mut element = w
            .absolute()
            .bg(widget.background_color())
            .p_4()
            .left(widget.get_bounds().origin.x)
            .top(widget.get_bounds().origin.y)
            .w(widget.get_bounds().size.width)
            .h(widget.get_bounds().size.height);

        if widget.has_border() {
            element = element.border_color(widget.border_color()).border(px(2.0));
        }

        element.child(widget.render())
    }
}

pub trait HomescreenWidget {
    fn render(&self) -> AnyElement;
    fn set_bounds(&mut self, bounds: Bounds<Pixels>);
    fn get_bounds(&self) -> Bounds<Pixels>;

    // Default styling methods
    fn background_color(&self) -> Hsla {
        rgb(0x2a2a2a).into()
    }

    fn has_border(&self) -> bool {
        true
    }

    fn border_color(&self) -> Hsla {
        rgb(0x404040).into()
    }
}
