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

    fn find_closest_available_position(
        state: &HomescreenState,
        widget_size: gpui::Size<usize>,
        old_position: Point<usize>,
        page_number: usize,
        excluded_widgets: &[WidgetId],
    ) -> Option<GridBounds> {
        let grid_size = &state.config.grid.grid_size;
        let max_x = grid_size.columns.saturating_sub(widget_size.width);
        let max_y = grid_size.rows.saturating_sub(widget_size.height);

        let mut candidates = Vec::new();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let candidate_bounds = Bounds {
                    origin: point(x, y),
                    size: widget_size,
                };

                let candidate_node = LayoutNode {
                    widget_id: WidgetId(usize::MAX),
                    page_number,
                    grid_bounds: candidate_bounds,
                };

                let mut has_overlap = false;
                for (other_id, other_node) in state.layout_manager_state.layout_nodes.iter() {
                    if !excluded_widgets.contains(other_id)
                        && candidate_node.overlaps_with(other_node)
                    {
                        has_overlap = true;
                        break;
                    }
                }

                if !has_overlap {
                    let dx = x as i32 - old_position.x as i32;
                    let dy = y as i32 - old_position.y as i32;
                    let distance = (dx * dx + dy * dy) as usize;
                    candidates.push((candidate_bounds, distance));
                }
            }
        }

        candidates.sort_by_key(|(_, distance)| *distance);
        candidates.first().map(|(bounds, _)| *bounds)
    }

    fn shuffle_widgets(
        state: &mut HomescreenState,
        widgets_to_shuffle: Vec<WidgetId>,
        page_number: usize,
    ) -> bool {
        let mut widget_infos: Vec<(WidgetId, gpui::Size<usize>, Point<usize>)> = widgets_to_shuffle
            .iter()
            .filter_map(|&widget_id| {
                state.widgets.get(&widget_id).map(|widget_data| {
                    (
                        widget_id,
                        widget_data.grid_bounds.size,
                        widget_data.grid_bounds.origin,
                    )
                })
            })
            .collect();

        widget_infos.sort_by_key(|(_, _, origin)| (origin.y, origin.x));

        let mut placed_widgets = Vec::new();
        let mut new_positions = Vec::new();

        for (widget_id, widget_size, old_position) in widget_infos {
            if let Some(new_bounds) = Self::find_closest_available_position(
                state,
                widget_size,
                old_position,
                page_number,
                &placed_widgets,
            ) {
                placed_widgets.push(widget_id);
                new_positions.push((widget_id, new_bounds));
            } else {
                return false;
            }
        }

        for (widget_id, new_bounds) in new_positions {
            Self::update_widget_layout(state, widget_id, new_bounds, page_number);
        }

        true
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
        let old_page = widget_data.page_number();
        let old_grid_bounds = widget_data.grid_bounds;

        let new_grid_bounds = pixel_bounds_to_closest_grid_bounds(
            widget_bounds,
            widget_data.grid_bounds.size,
            &state.config,
        );

        if !is_valid_grid_position(&new_grid_bounds, &state.config) {
            return false;
        }

        let proposed_node = LayoutNode::new(widget_id, page_number, new_grid_bounds);

        let mut overlapping_widgets = Vec::new();
        for (other_id, other_node) in state.layout_manager_state.layout_nodes.iter() {
            if *other_id != widget_id && proposed_node.overlaps_with(other_node) {
                overlapping_widgets.push(*other_id);
            }
        }

        if !overlapping_widgets.is_empty() {
            Self::update_widget_layout(state, widget_id, new_grid_bounds, page_number);

            if !Self::shuffle_widgets(state, overlapping_widgets, page_number) {
                Self::update_widget_layout(state, widget_id, old_grid_bounds, old_page);
                return false;
            }

            if old_page != page_number {
                if let Some(old_page_widgets) = state.pages.get_mut(old_page) {
                    old_page_widgets.remove(&widget_id);
                }
                if page_number >= state.pages.len() {
                    state.pages.resize(page_number + 1, Default::default());
                }
                state.pages[page_number].insert(widget_id);
            }
        } else {
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
        }

        true
    }
}
