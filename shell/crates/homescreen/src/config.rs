use gpui::{Pixels, Size};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HomescreenConfig {
    pub window: WindowConfig,
    pub grid: GridConfig,
    pub drag: DragConfig,
    pub animation: AnimationConfig,
}

impl HomescreenConfig {
    pub fn new(size: Size<Pixels>) -> Self {
        let mut config = Self {
            window: WindowConfig {
                width: size.width.to_f64() as f32,
                height: size.height.to_f64() as f32,
            },
            grid: GridConfig::default(),
            drag: DragConfig::default(),
            animation: AnimationConfig::default(),
        };

        // Set zero gaps for pages 0 and 4 (full-screen widgets)
        config.grid.page_gaps.insert(
            0,
            GridGap {
                horizontal: 0.0,
                vertical: 0.0,
            },
        );
        config.grid.page_gaps.insert(
            2,
            GridGap {
                horizontal: 0.0,
                vertical: 0.0,
            },
        );

        // Set padding for pages 1-3 (non-fullscreen pages)
        let page_padding = GridPadding {
            top: 10.0,
            right: 10.0,
            bottom: 10.0,
            left: 10.0,
        };
        config.grid.page_paddings.insert(1, page_padding);
        // config.grid.page_paddings.insert(2, page_padding);
        // config.grid.page_paddings.insert(3, page_padding);

        config
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct GridConfig {
    pub grid_size: GridSize,
    pub gap: GridGap,
    pub padding: GridPadding,
    pub page_gaps: HashMap<usize, GridGap>,
    pub page_paddings: HashMap<usize, GridPadding>,
}

impl GridConfig {
    pub fn get_gap_for_page(&self, page_number: usize) -> GridGap {
        self.page_gaps
            .get(&page_number)
            .copied()
            .unwrap_or(self.gap)
    }

    pub fn get_padding_for_page(&self, page_number: usize) -> GridPadding {
        self.page_paddings
            .get(&page_number)
            .copied()
            .unwrap_or(self.padding)
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            grid_size: GridSize::default(),
            gap: GridGap::default(),
            padding: GridPadding::default(),
            page_gaps: HashMap::new(),
            page_paddings: HashMap::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

impl Default for GridSize {
    fn default() -> Self {
        Self {
            rows: 3,
            columns: 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GridGap {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for GridGap {
    fn default() -> Self {
        Self {
            horizontal: 10.0,
            vertical: 10.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GridPadding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for GridPadding {
    fn default() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DragConfig {
    pub drag_initiation_threshold: f32,
    pub page_switch_threshold: f32,
    pub widget_drag_time_threshold: std::time::Duration,
    pub edge_hover_threshold: f32,
    pub edge_hover_wait_interval: std::time::Duration,
}

impl Default for DragConfig {
    fn default() -> Self {
        Self {
            drag_initiation_threshold: 5.0,
            page_switch_threshold: 50.0,
            widget_drag_time_threshold: std::time::Duration::from_millis(200),
            edge_hover_threshold: 50.0,
            edge_hover_wait_interval: std::time::Duration::from_millis(500),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AnimationConfig {
    pub page_snap_velocity: f32,
    pub widget_snap_velocity: f32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            page_snap_velocity: 1000.0,
            widget_snap_velocity: 1000.0,
        }
    }
}
