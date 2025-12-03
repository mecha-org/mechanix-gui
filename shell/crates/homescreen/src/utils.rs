use gpui::*;

use crate::config::HomescreenConfig;

pub type GridBounds = Bounds<usize>;

pub fn grid_bounds_to_pixels(
    grid_bounds: GridBounds,
    config: &HomescreenConfig,
) -> Bounds<Pixels> {
    let grid_config = &config.grid;
    let window_config = &config.window;

    // Calculate available space after accounting for padding
    let available_width =
        window_config.width - grid_config.padding.left - grid_config.padding.right;
    let available_height =
        window_config.height - grid_config.padding.top - grid_config.padding.bottom;

    // Calculate total gap space
    let total_gap_width = (grid_config.grid_size.columns - 1) as f32 * grid_config.gap.horizontal;
    let total_gap_height = (grid_config.grid_size.rows - 1) as f32 * grid_config.gap.vertical;

    // Calculate cell size based on available space
    let cell_width = (available_width - total_gap_width) / grid_config.grid_size.columns as f32;
    let cell_height = (available_height - total_gap_height) / grid_config.grid_size.rows as f32;

    // Calculate cell size with gap for positioning
    let cell_with_gap_h = cell_width + grid_config.gap.horizontal;
    let cell_with_gap_v = cell_height + grid_config.gap.vertical;

    // Calculate origin position
    let x = grid_config.padding.left + (grid_bounds.origin.x as f32 * cell_with_gap_h);
    let y = grid_config.padding.top + (grid_bounds.origin.y as f32 * cell_with_gap_v);

    // Calculate widget size (cells * cell_size + (cells - 1) * gap)
    let width = grid_bounds.size.width as f32 * cell_width
        + (grid_bounds.size.width.saturating_sub(1)) as f32 * grid_config.gap.horizontal;

    let height = grid_bounds.size.height as f32 * cell_height
        + (grid_bounds.size.height.saturating_sub(1)) as f32 * grid_config.gap.vertical;

    Bounds {
        origin: point(px(x), px(y)),
        size: size(px(width), px(height)),
    }
}
