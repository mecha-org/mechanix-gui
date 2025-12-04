#[derive(Default, Debug, Copy, Clone)]
pub struct HomescreenConfig {
    pub window: WindowConfig,
    pub grid: GridConfig,
    pub drag: DragConfig,
    pub animation: AnimationConfig,
}

#[derive(Debug, Copy, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            height: 540.0,
            width: 540.0,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct GridConfig {
    pub grid_size: GridSize,
    pub gap: GridGap,
    pub padding: GridPadding,
}

#[derive(Debug, Copy, Clone)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

impl Default for GridSize {
    fn default() -> Self {
        Self {
            rows: 4,
            columns: 4,
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
            top: 20.0,
            right: 20.0,
            bottom: 20.0,
            left: 20.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DragConfig {
    pub drag_initiation_threshold: f32,
    pub page_switch_threshold: f32,
    pub widget_drag_time_threshold: std::time::Duration,
}

impl Default for DragConfig {
    fn default() -> Self {
        Self {
            drag_initiation_threshold: 5.0,
            page_switch_threshold: 50.0,
            widget_drag_time_threshold: std::time::Duration::from_millis(200),
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
