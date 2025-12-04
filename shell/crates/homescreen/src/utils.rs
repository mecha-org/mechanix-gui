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

pub fn pixel_position_to_grid_bounds(
    position: Point<Pixels>,
    config: &HomescreenConfig,
    widget_size: gpui::Size<usize>,
) -> GridBounds {
    let grid_config = &config.grid;
    let window_config = &config.window;

    let available_width =
        window_config.width - grid_config.padding.left - grid_config.padding.right;
    let available_height =
        window_config.height - grid_config.padding.top - grid_config.padding.bottom;

    let total_gap_width =
        (grid_config.grid_size.columns - 1) as f32 * grid_config.gap.horizontal;
    let total_gap_height = (grid_config.grid_size.rows - 1) as f32 * grid_config.gap.vertical;

    let cell_width = (available_width - total_gap_width) / grid_config.grid_size.columns as f32;
    let cell_height =
        (available_height - total_gap_height) / grid_config.grid_size.rows as f32;

    let cell_with_gap_h = cell_width + grid_config.gap.horizontal;
    let cell_with_gap_v = cell_height + grid_config.gap.vertical;

    let relative_x: f32 = position.x.into();
    let relative_y: f32 = position.y.into();
    let adjusted_x = (relative_x - grid_config.padding.left).max(0.0);
    let adjusted_y = (relative_y - grid_config.padding.top).max(0.0);

    let grid_x = (adjusted_x / cell_with_gap_h).round() as usize;
    let grid_y = (adjusted_y / cell_with_gap_v).round() as usize;

    Bounds {
        origin: point(grid_x, grid_y),
        size: widget_size,
    }
}

pub fn pixel_bounds_to_closest_grid_bounds(
    pixel_bounds: Bounds<Pixels>,
    widget_grid_size: gpui::Size<usize>,
    config: &HomescreenConfig,
) -> GridBounds {
    let grid_config = &config.grid;
    let window_config = &config.window;

    // Calculate available space and cell dimensions
    let available_width =
        window_config.width - grid_config.padding.left - grid_config.padding.right;
    let available_height =
        window_config.height - grid_config.padding.top - grid_config.padding.bottom;

    let total_gap_width =
        (grid_config.grid_size.columns - 1) as f32 * grid_config.gap.horizontal;
    let total_gap_height = (grid_config.grid_size.rows - 1) as f32 * grid_config.gap.vertical;

    let cell_width = (available_width - total_gap_width) / grid_config.grid_size.columns as f32;
    let cell_height =
        (available_height - total_gap_height) / grid_config.grid_size.rows as f32;

    let cell_with_gap_h = cell_width + grid_config.gap.horizontal;
    let cell_with_gap_v = cell_height + grid_config.gap.vertical;

    // Get widget origin in pixels
    let origin_x: f32 = pixel_bounds.origin.x.into();
    let origin_y: f32 = pixel_bounds.origin.y.into();

    // Remove padding to get position relative to grid
    let adjusted_x = (origin_x - grid_config.padding.left).max(0.0);
    let adjusted_y = (origin_y - grid_config.padding.top).max(0.0);

    // Find which grid cell's center is closest to the widget's origin
    // Each grid cell's center is at (i * cell_with_gap + cell_width/2, j * cell_with_gap + cell_height/2)
    let grid_x = ((adjusted_x + cell_width / 2.0) / cell_with_gap_h).floor() as usize;
    let grid_y = ((adjusted_y + cell_height / 2.0) / cell_with_gap_v).floor() as usize;

    // Clamp to ensure the widget fits within the grid
    let max_grid_x = grid_config
        .grid_size
        .columns
        .saturating_sub(widget_grid_size.width);
    let max_grid_y = grid_config
        .grid_size
        .rows
        .saturating_sub(widget_grid_size.height);

    let grid_x = grid_x.min(max_grid_x);
    let grid_y = grid_y.min(max_grid_y);

    Bounds {
        origin: point(grid_x, grid_y),
        size: widget_grid_size,
    }
}

pub fn is_valid_grid_position(grid_bounds: &GridBounds, config: &HomescreenConfig) -> bool {
    let grid_size = &config.grid.grid_size;
    grid_bounds.origin.x + grid_bounds.size.width <= grid_size.columns
        && grid_bounds.origin.y + grid_bounds.size.height <= grid_size.rows
}
