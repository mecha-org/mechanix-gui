use commons::widgets::wing;
use gpui::*;
use theme::prelude::AlphaExt;

use crate::utils::GridBounds;

pub mod app_drawer;
pub mod demo_widget;
pub mod universal_search;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub(crate) struct WidgetId(pub usize);

pub(crate) struct WidgetData {
    widget: Box<dyn HomescreenWidget>,
    page_number: usize,
    pub(crate) grid_bounds: GridBounds,
    is_being_dragged: bool,
    dragged_page: Option<usize>,
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
            dragged_page: None,
            bounds,
        }
    }

    pub fn is_being_dragged(&self) -> bool {
        self.is_being_dragged
    }

    pub fn start_drag(&mut self) {
        self.is_being_dragged = true;
        self.dragged_page = Some(self.page_number);
    }

    pub fn end_drag(&mut self, page_number: usize) {
        self.is_being_dragged = false;
        self.dragged_page = Some(page_number)
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

    // Getter and setter for dragged_page
    pub fn dragged_page(&self) -> Option<usize> {
        self.dragged_page
    }

    pub fn set_dragged_page(&mut self, dragged_page: Option<usize>) {
        self.dragged_page = dragged_page;
    }

    // Getter and setter for bounds
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    pub fn set_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.bounds = bounds;
    }
}

#[derive(Debug)]
pub struct TouchingWidgetInfo {
    pub widget_id: WidgetId,
    pub grid_bounds: GridBounds,
}

pub struct WidgetWrapper;
impl WidgetWrapper {
    fn is_touching(current: &GridBounds, other: &GridBounds) -> bool {
        // Check if widgets are adjacent (touching edges)
        let current_right = current.origin.x + current.size.width;
        let current_bottom = current.origin.y + current.size.height;
        let other_right = other.origin.x + other.size.width;
        let other_bottom = other.origin.y + other.size.height;

        // Check if they share any edge
        // Touching horizontally (left or right)
        let touching_horizontally = (current_right == other.origin.x || other_right == current.origin.x)
            && !(current_bottom <= other.origin.y || current.origin.y >= other_bottom);

        // Touching vertically (top or bottom)
        let touching_vertically = (current_bottom == other.origin.y || other_bottom == current.origin.y)
            && !(current_right <= other.origin.x || current.origin.x >= other_right);

        touching_horizontally || touching_vertically
    }

    pub fn get_touching_widgets(
        widget_id: WidgetId,
        state: &crate::state::HomescreenState,
    ) -> Vec<TouchingWidgetInfo> {
        let mut touching_widgets = Vec::new();

        let current_node = match state.layout_manager_state.get_layout_node(&widget_id) {
            Some(node) => node,
            None => return touching_widgets,
        };

        for (other_id, other_node) in state.layout_manager_state.iter_layout_nodes() {
            if other_id != &widget_id
                && current_node.page_number == other_node.page_number
                && Self::is_touching(&current_node.grid_bounds, &other_node.grid_bounds)
            {
                touching_widgets.push(TouchingWidgetInfo {
                    widget_id: *other_id,
                    grid_bounds: other_node.grid_bounds,
                });
            }
        }

        touching_widgets
    }

    pub fn render(
        widget_id: WidgetId,
        widget_data: &WidgetData,
        state: &crate::state::HomescreenState,
        cx: &gpui::App,
    ) -> impl IntoElement {
        let widget = widget_data.widget();
        let colors = theme::ActiveTheme::theme(cx).colors.clone();
        let mut w = wing();
        w.border_radius(px(8.0));
        w.border_width(px(1.0));
        let mut upper_wing_height = 0.0;
        let mut lower_wing_height = 0.0;

        // Check if this widget should have wings
        let grid_columns = state.config.grid.grid_size.columns;
        let grid_rows = state.config.grid.grid_size.rows;
        let center_column = grid_columns / 2;
        let right_edge = widget_data.grid_bounds.origin.x + widget_data.grid_bounds.size.width;
        let bottom_edge = widget_data.grid_bounds.origin.y + widget_data.grid_bounds.size.height;

        // Check if widget is touching top or bottom edges of homescreen
        let touching_top_edge = widget_data.grid_bounds.origin.y == 0;
        let touching_bottom_edge = bottom_edge == grid_rows;

        // Get touching widgets
        let touching_widgets = Self::get_touching_widgets(widget_id, state);

        // Check if right edge is at center (touching from left) and touching a widget above with width 4
        let right_edge_at_center = right_edge == center_column;
        let left_edge_at_center = widget_data.grid_bounds.origin.x == center_column;

        let touching_wide_widget_above = touching_widgets.iter().any(|info| {
            info.grid_bounds.size.width == 4 && info.grid_bounds.origin.y < widget_data.grid_bounds.origin.y
        });

        // Check if touching a full-width widget below
        let touching_wide_widget_below = touching_widgets.iter().any(|info| {
            info.grid_bounds.size.width == 4 && info.grid_bounds.origin.y > widget_data.grid_bounds.origin.y
        });

        let half_width = state.config.window.width / 2.0;
        let wing_size_adjustment = 30.0;
        let widget_width: f32 = widget.get_bounds().size.width.into();

        // Check if widget should have reduced size (touching from left center with widget below)
        let should_reduce_size = right_edge_at_center && touching_wide_widget_below;

        // Check if widget should have increased size (touching from right center with widget below)
        let should_increase_size = left_edge_at_center && touching_wide_widget_below;

        let size_adjustment = if should_reduce_size {
            -wing_size_adjustment
        } else if should_increase_size {
            wing_size_adjustment
        } else {
            0.0
        };

        // Check if widget is touching or crossing the center line
        let widget_touches_or_crosses_center = widget_data.grid_bounds.origin.x <= center_column && right_edge >= center_column;

        // Apply wing logic based on position
        if widget_data.grid_bounds.size.width == 4 {
            // Add upper wing only if not touching top edge
            if !touching_top_edge {
                upper_wing_height = 30.0;
                let upper_wing_width = (half_width - 4.0 * upper_wing_height / 3.0 - 5.0).min(widget_width);
                w.upper_wing_size(size(px(upper_wing_width), px(upper_wing_height)));
            }

            // Add lower wing only if not touching bottom edge
            if !touching_bottom_edge {
                lower_wing_height = 30.0;
                let lower_wing_width = (half_width - lower_wing_height / 2.0 + 5.0).min(widget_width);
                w.lower_wing_size(size(px(lower_wing_width), px(lower_wing_height)));
            }
        } else if widget_touches_or_crosses_center && touching_wide_widget_above && !touching_top_edge {
            // Widget touching/crossing center and touching full-width widget above
            let widget_left: f32 = widget.get_bounds().origin.x.into();
            // Only add upper wing (left of center) if widget has portion to the left of center
            if widget_left < half_width {
                upper_wing_height = 30.0;
                let widget_right = widget_left + widget_width;
                let portion_left_of_center = (half_width - widget_left).min(widget_right - widget_left).max(0.0);
                let upper_wing_width = portion_left_of_center - upper_wing_height;
                w.upper_wing_size(size(px(upper_wing_width), px(upper_wing_height)));
            }
        } else if widget_touches_or_crosses_center && touching_wide_widget_below && !touching_bottom_edge {
            // Widget touching/crossing center and touching full-width widget below
            let widget_left: f32 = widget.get_bounds().origin.x.into();
            let widget_right = widget_left + widget_width;
            // Only add lower wing (right of center) if widget has portion to the right of center
            if widget_right > half_width {
                lower_wing_height = 30.0;
                let portion_right_of_center = widget_right - half_width;
                let lower_wing_width = portion_right_of_center.min(widget_width);
                w.lower_wing_size(size(px(lower_wing_width), px(lower_wing_height)));
            }
        }

        // Calculate horizontal position adjustment (move left when increasing size)
        let horizontal_adjustment = if should_increase_size {
            -wing_size_adjustment
        } else {
            0.0
        };

        // Height adjustment: only reduce height when shrinking, not when growing
        let height_adjustment = if should_reduce_size {
            -wing_size_adjustment
        } else {
            0.0
        };

        let element = w
            .absolute()
            .bg(colors.background_1000)
            .border_color(colors.accent_200.with_alpha(0.4))
            .left(widget.get_bounds().origin.x + px(horizontal_adjustment))
            .top(widget.get_bounds().origin.y - px(upper_wing_height))
            .w(widget.get_bounds().size.width + px(size_adjustment))
            .h(widget.get_bounds().size.height + px(upper_wing_height) + px(height_adjustment));

        element.child(
            div()
                .size_full()
                .p_3()
                .child(widget.render())
        )
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
