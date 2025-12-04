use std::collections::HashMap;

use gpui::*;

use crate::{
    config::HomescreenConfig,
    state::HomescreenState,
    utils::{grid_bounds_to_pixels, is_valid_grid_position, pixel_bounds_to_closest_grid_bounds, GridBounds},
    widgets::WidgetId,
};

pub struct LayoutManagerState {
    layout_nodes: HashMap<WidgetId, LayoutNode>,
}

impl LayoutManagerState {
    pub fn new(_config: HomescreenConfig) -> Self {
        Self {
            layout_nodes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutNode {
    pub widget_id: WidgetId,
    pub page_number: usize,
    pub grid_bounds: GridBounds,
}

impl LayoutNode {
    pub fn new(widget_id: WidgetId, page_number: usize, grid_bounds: GridBounds) -> Self {
        Self {
            widget_id,
            page_number,
            grid_bounds,
        }
    }

    pub fn overlaps_with(&self, other: &LayoutNode) -> bool {
        if self.page_number != other.page_number {
            return false;
        }

        let self_right = self.grid_bounds.origin.x + self.grid_bounds.size.width;
        let self_bottom = self.grid_bounds.origin.y + self.grid_bounds.size.height;
        let other_right = other.grid_bounds.origin.x + other.grid_bounds.size.width;
        let other_bottom = other.grid_bounds.origin.y + other.grid_bounds.size.height;

        !(self_right <= other.grid_bounds.origin.x
            || self.grid_bounds.origin.x >= other_right
            || self_bottom <= other.grid_bounds.origin.y
            || self.grid_bounds.origin.y >= other_bottom)
    }
}

pub struct LayoutManager;

impl LayoutManager {
    pub fn register_widget(state: &mut HomescreenState, widget_id: WidgetId) {
        if let Some(widget_data) = state.widgets.get(&widget_id) {
            let layout_node = LayoutNode::new(
                widget_id,
                widget_data.page_number(),
                widget_data.grid_bounds,
            );
            state
                .layout_manager_state
                .layout_nodes
                .insert(widget_id, layout_node);
        }
    }

    pub fn update_widget_layout(
        state: &mut HomescreenState,
        widget_id: WidgetId,
        new_grid_bounds: GridBounds,
        new_page_number: usize,
    ) {
        if let Some(layout_node) = state.layout_manager_state.layout_nodes.get_mut(&widget_id) {
            layout_node.grid_bounds = new_grid_bounds;
            layout_node.page_number = new_page_number;
        }

        if let Some(widget_data) = state.widgets.get_mut(&widget_id) {
            widget_data.grid_bounds = new_grid_bounds;
            widget_data.set_page_number(new_page_number);
            let new_bounds = grid_bounds_to_pixels(new_grid_bounds, &state.config);
            widget_data.set_bounds(new_bounds);
        }
    }

    pub fn try_drop_widget(
        state: &mut HomescreenState,
        widget_id: WidgetId,
        widget_bounds: Bounds<Pixels>,
    ) -> bool {
        let Some(widget_data) = state.widgets.get(&widget_id) else {
            return false;
        };

        let page_number = state.active_page;
        let new_grid_bounds = pixel_bounds_to_closest_grid_bounds(
            widget_bounds,
            widget_data.grid_bounds.size,
            &state.config,
        );

        if !is_valid_grid_position(&new_grid_bounds, &state.config) {
            return false;
        }

        let proposed_node = LayoutNode::new(widget_id, page_number, new_grid_bounds);

        for (other_id, other_node) in state.layout_manager_state.layout_nodes.iter() {
            if *other_id != widget_id && proposed_node.overlaps_with(other_node) {
                return false;
            }
        }

        let old_page = widget_data.page_number();
        if old_page != page_number {
            if let Some(old_page_widgets) = state.pages.get_mut(old_page) {
                old_page_widgets.remove(&widget_id);
            }
            if page_number >= state.pages.len() {
                state.pages.resize(page_number + 1, Default::default());
            }
            state.pages[page_number].insert(widget_id);
        }

        Self::update_widget_layout(state, widget_id, new_grid_bounds, page_number);
        true
    }
}
