use std::collections::{HashMap, HashSet};

use gpui::*;

use crate::{
    animation_manager::AnimationManagerState,
    config::HomescreenConfig,
    input_manager::InputManagerState,
    layout_manager::{LayoutManager, LayoutManagerState},
    utils::{grid_bounds_to_pixels, GridBounds},
    widgets::{HomescreenWidget, WidgetData, WidgetId},
};

pub struct HomescreenState {
    pub config: HomescreenConfig,

    pub(crate) layout_manager_state: LayoutManagerState,
    pub(crate) animation_manager_state: AnimationManagerState,
    pub(crate) input_manager_state: InputManagerState,

    pub(crate) widgets: HashMap<WidgetId, WidgetData>,
    pub(crate) pages: Vec<HashSet<WidgetId>>,
    next_widget_id: usize,

    pub(crate) active_page: usize,
    pub(crate) page_offset: f32,
    pub(crate) is_page_dragging: bool,
    pub(crate) dragging_widget: Option<WidgetId>,
}

impl HomescreenState {
    pub fn new(config: HomescreenConfig) -> Self {
        let layout_manager_state = LayoutManagerState::new(config);
        let animation_manager_state = AnimationManagerState::new(config);
        let input_manager_state = InputManagerState::new(config);

        let widgets = Default::default();
        let pages = Default::default();

        Self {
            config,

            layout_manager_state,
            input_manager_state,
            animation_manager_state,

            widgets,
            pages,
            next_widget_id: 0,

            active_page: 0,
            page_offset: 0.0,
            is_page_dragging: false,
            dragging_widget: None,
        }
    }

    pub fn create_widget<W: HomescreenWidget + 'static>(
        &mut self,
        mut widget: W,
        page_number: usize,
        grid_bounds: GridBounds,
    ) {
        let bounds = grid_bounds_to_pixels(grid_bounds, &self.config);
        widget.set_bounds(bounds);
        let widget_id = WidgetId(self.next_widget_id);
        self.widgets.insert(
            widget_id,
            WidgetData::new(Box::new(widget), page_number, grid_bounds, bounds),
        );
        if page_number >= self.pages.len() {
            self.pages.resize(page_number + 1, Default::default());
        }
        self.pages[page_number].insert(widget_id);
        self.next_widget_id += 1;

        LayoutManager::register_widget(self, widget_id);
    }

    pub fn set_active_page(&mut self, new_page: usize) {
        if new_page >= self.pages.len() {
            return;
        }

        let old_page = self.active_page;
        let offset_adjustment = (old_page as f32 - new_page as f32) * self.config.window.width;

        self.active_page = new_page;
        self.page_offset -= offset_adjustment;
    }

    pub fn find_widget_under_point(&self, point: Point<Pixels>) -> Option<WidgetId> {
        // Only check widgets on the active page
        if let Some(page_widgets) = self.pages.get(self.active_page) {
            for widget_id in page_widgets {
                if let Some(widget_data) = self.widgets.get(widget_id) {
                    let bounds = widget_data.bounds();
                    if point.x >= bounds.origin.x
                        && point.x <= bounds.origin.x + bounds.size.width
                        && point.y >= bounds.origin.y
                        && point.y <= bounds.origin.y + bounds.size.height
                    {
                        return Some(*widget_id);
                    }
                }
            }
        }
        None
    }

    pub fn get_widget_bounds(&self, widget_id: WidgetId) -> Option<Bounds<Pixels>> {
        self.widgets.get(&widget_id).map(|w| w.bounds())
    }

    pub fn set_widget_render_bounds(&mut self, widget_id: WidgetId, bounds: Bounds<Pixels>) {
        if let Some(widget_data) = self.widgets.get_mut(&widget_id) {
            widget_data.widget_mut().set_bounds(bounds);
        }
    }

    pub fn pick_widget(&mut self, widget_id: WidgetId) {
        if let Some(widget_data) = self.widgets.get_mut(&widget_id) {
            widget_data.start_drag();
            self.dragging_widget = Some(widget_id);
        }
    }

    pub fn drop_widget(&mut self) -> bool {
        let Some(widget_id) = self.dragging_widget else {
            return false;
        };

        let widget_bounds = if let Some(widget_data) = self.widgets.get(&widget_id) {
            widget_data.widget().get_bounds()
        } else {
            return false;
        };

        if let Some(widget_data) = self.widgets.get_mut(&widget_id) {
            widget_data.end_drag();
        }

        let success = LayoutManager::try_drop_widget(self, widget_id, widget_bounds);
        self.dragging_widget = None;
        success
    }
}
