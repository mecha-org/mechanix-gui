use gpui::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidgetSize {
    pub cols: usize,
    pub rows: usize,
}

impl WidgetSize {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols, rows }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridPosition {
    pub col: usize,
    pub row: usize,
}

impl GridPosition {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

#[derive(Clone, Debug)]
pub struct Widget {
    pub id: usize,
    pub name: String,
    pub icon_name: String,
    pub size: WidgetSize,
    pub position: GridPosition,
    pub background_color: Hsla,

    pub current_x: f32,
    pub current_y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub animation_speed: f32,
}

impl Widget {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        icon_name: impl Into<String>,
        size: WidgetSize,
        position: GridPosition,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            icon_name: icon_name.into(),
            size,
            position,
            background_color: rgb(0xFF69B4).into(),
            current_x: 0.0,
            current_y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            animation_speed: 5000.0,
        }
    }

    pub fn with_color(mut self, color: Hsla) -> Self {
        self.background_color = color;
        self
    }

    pub fn occupies(&self, col: usize, row: usize) -> bool {
        col >= self.position.col
            && col < self.position.col + self.size.cols
            && row >= self.position.row
            && row < self.position.row + self.size.rows
    }

    pub fn occupied_cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        for row in self.position.row..(self.position.row + self.size.rows) {
            for col in self.position.col..(self.position.col + self.size.cols) {
                cells.push((col, row));
            }
        }
        cells
    }

    pub fn calculate_target_position(&mut self, cell_size: f32, gap: f32) {
        self.target_x = self.position.col as f32 * (cell_size + gap);
        self.target_y = self.position.row as f32 * (cell_size + gap);
    }

    pub fn set_drag_target(&mut self, cursor_x: f32, cursor_y: f32, cell_size: f32, gap: f32) {
        let widget_width = cell_size * self.size.cols as f32 + gap * (self.size.cols - 1) as f32;
        let widget_height = cell_size * self.size.rows as f32 + gap * (self.size.rows - 1) as f32;
        self.target_x = cursor_x - widget_width / 2.0;
        self.target_y = cursor_y - widget_height / 2.0;
    }

    pub fn is_animating(&self) -> bool {
        let dx = self.target_x - self.current_x;
        let dy = self.target_y - self.current_y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance >= 1.0
    }

    pub fn update_position(&mut self, delta_time: f32) -> bool {
        let dx = self.target_x - self.current_x;
        let dy = self.target_y - self.current_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 1.0 {
            self.current_x = self.target_x;
            self.current_y = self.target_y;
            return false;
        }

        let step = self.animation_speed * delta_time;
        if step >= distance {
            self.current_x = self.target_x;
            self.current_y = self.target_y;
            return false;
        }

        let ratio = step / distance;
        self.current_x += dx * ratio;
        self.current_y += dy * ratio;
        true
    }
}

#[derive(Clone, Debug)]
pub struct Page {
    pub id: usize,
    pub widgets: Vec<Widget>,
    pub grid_cols: usize,
    pub grid_rows: usize,
}

impl Page {
    pub fn new(id: usize, grid_cols: usize, grid_rows: usize) -> Self {
        Self {
            id,
            widgets: Vec::new(),
            grid_cols,
            grid_rows,
        }
    }

    pub fn add_widget(&mut self, widget: Widget) -> Result<(), String> {
        if widget.position.col + widget.size.cols > self.grid_cols {
            return Err("Widget exceeds grid width".to_string());
        }
        if widget.position.row + widget.size.rows > self.grid_rows {
            return Err("Widget exceeds grid height".to_string());
        }

        for existing in &self.widgets {
            for (col, row) in widget.occupied_cells() {
                if existing.occupies(col, row) {
                    return Err(format!("Position ({}, {}) is already occupied", col, row));
                }
            }
        }

        self.widgets.push(widget);
        Ok(())
    }

    pub fn remove_widget(&mut self, widget_id: usize) -> Option<Widget> {
        self.widgets
            .iter()
            .position(|w| w.id == widget_id)
            .map(|idx| self.widgets.remove(idx))
    }

    pub fn move_widget(
        &mut self,
        widget_id: usize,
        new_position: GridPosition,
    ) -> Result<(), String> {
        let widget_idx = self
            .widgets
            .iter()
            .position(|w| w.id == widget_id)
            .ok_or_else(|| "Widget not found".to_string())?;

        let mut widget = self.widgets[widget_idx].clone();

        if new_position.col + widget.size.cols > self.grid_cols {
            return Err("New position exceeds grid width".to_string());
        }
        if new_position.row + widget.size.rows > self.grid_rows {
            return Err("New position exceeds grid height".to_string());
        }

        for (idx, existing) in self.widgets.iter().enumerate() {
            if idx == widget_idx {
                continue;
            }

            let old_position = widget.position;
            widget.position = new_position;

            for (col, row) in widget.occupied_cells() {
                if existing.occupies(col, row) {
                    return Err(format!(
                        "New position ({}, {}) overlaps with widget '{}'",
                        col, row, existing.name
                    ));
                }
            }

            widget.position = old_position;
        }

        self.widgets[widget_idx].position = new_position;
        Ok(())
    }

    pub fn widget_at(&self, col: usize, row: usize) -> Option<&Widget> {
        self.widgets.iter().find(|w| w.occupies(col, row))
    }

    pub fn swap_widgets(&mut self, widget_id1: usize, widget_id2: usize) -> Result<(), String> {
        let idx1 = self
            .widgets
            .iter()
            .position(|w| w.id == widget_id1)
            .ok_or_else(|| "First widget not found".to_string())?;

        let idx2 = self
            .widgets
            .iter()
            .position(|w| w.id == widget_id2)
            .ok_or_else(|| "Second widget not found".to_string())?;

        let pos1 = self.widgets[idx1].position;
        let pos2 = self.widgets[idx2].position;

        self.widgets[idx1].position = pos2;
        self.widgets[idx2].position = pos1;

        Ok(())
    }

    pub fn find_overlapping_widgets(
        &self,
        position: GridPosition,
        size: WidgetSize,
        exclude_id: Option<usize>,
    ) -> Vec<usize> {
        let mut overlapping = Vec::new();

        let occupied_cells: Vec<(usize, usize)> = (position.row..position.row + size.rows)
            .flat_map(|row| (position.col..position.col + size.cols).map(move |col| (col, row)))
            .collect();

        for widget in &self.widgets {
            if let Some(exclude) = exclude_id {
                if widget.id == exclude {
                    continue;
                }
            }

            for (col, row) in &occupied_cells {
                if widget.occupies(*col, *row) {
                    overlapping.push(widget.id);
                    break;
                }
            }
        }

        overlapping
    }

    pub fn can_place_widget_at(
        &self,
        widget_size: WidgetSize,
        position: GridPosition,
        exclude_ids: &[usize],
    ) -> bool {
        if position.col + widget_size.cols > self.grid_cols {
            return false;
        }
        if position.row + widget_size.rows > self.grid_rows {
            return false;
        }

        let occupied_cells: Vec<(usize, usize)> = (position.row..position.row + widget_size.rows)
            .flat_map(|row| {
                (position.col..position.col + widget_size.cols).map(move |col| (col, row))
            })
            .collect();

        for widget in &self.widgets {
            if exclude_ids.contains(&widget.id) {
                continue;
            }

            for (col, row) in &occupied_cells {
                if widget.occupies(*col, *row) {
                    return false;
                }
            }
        }

        true
    }

    pub fn find_closest_available_position(
        &self,
        widget_size: WidgetSize,
        preferred_position: GridPosition,
        exclude_ids: &[usize],
    ) -> Option<GridPosition> {
        let mut best_position: Option<GridPosition> = None;
        let mut best_distance = f32::MAX;

        for row in 0..self.grid_rows {
            for col in 0..self.grid_cols {
                let test_position = GridPosition::new(col, row);

                if self.can_place_widget_at(widget_size, test_position, exclude_ids) {
                    let distance = (((col as i32 - preferred_position.col as i32).pow(2)
                        + (row as i32 - preferred_position.row as i32).pow(2))
                        as f32)
                        .sqrt();

                    if distance < best_distance {
                        best_distance = distance;
                        best_position = Some(test_position);
                    }
                }
            }
        }

        best_position
    }

    pub fn try_rearrange_on_drop(
        &mut self,
        widget_id: usize,
        new_position: GridPosition,
    ) -> Result<(), String> {
        let widget_idx = self
            .widgets
            .iter()
            .position(|w| w.id == widget_id)
            .ok_or_else(|| "Widget not found".to_string())?;

        let widget = &self.widgets[widget_idx];
        let original_position = widget.position;
        let widget_size = widget.size;

        if new_position.col + widget_size.cols > self.grid_cols {
            return Err("New position exceeds grid width".to_string());
        }
        if new_position.row + widget_size.rows > self.grid_rows {
            return Err("New position exceeds grid height".to_string());
        }

        if new_position == original_position {
            return Ok(());
        }

        let backup_positions: Vec<(usize, GridPosition)> =
            self.widgets.iter().map(|w| (w.id, w.position)).collect();

        let overlapping_ids =
            self.find_overlapping_widgets(new_position, widget_size, Some(widget_id));

        let mut displaced_widgets: Vec<(usize, GridPosition, WidgetSize)> = Vec::new();
        for &id in &overlapping_ids {
            if let Some(widget) = self.widgets.iter().find(|w| w.id == id) {
                displaced_widgets.push((id, widget.position, widget.size));
            }
        }

        displaced_widgets.sort_by(|a, b| {
            let dist_a = ((a.1.col as f32).powi(2) + (a.1.row as f32).powi(2)).sqrt();
            let dist_b = ((b.1.col as f32).powi(2) + (b.1.row as f32).powi(2)).sqrt();
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        let widget_idx = self.widgets.iter().position(|w| w.id == widget_id).unwrap();
        self.widgets[widget_idx].position = new_position;

        let mut exclude_ids: Vec<usize> = overlapping_ids.clone();

        let mut new_positions: Vec<(usize, GridPosition)> = Vec::new();

        for (displaced_id, original_pos, size) in displaced_widgets {
            match self.find_closest_available_position(size, original_pos, &exclude_ids) {
                Some(new_pos) => {
                    new_positions.push((displaced_id, new_pos));

                    if let Some(widget) = self.widgets.iter_mut().find(|w| w.id == displaced_id) {
                        widget.position = new_pos;
                    }

                    exclude_ids.retain(|&id| id != displaced_id);
                }
                None => {
                    for (id, pos) in backup_positions {
                        if let Some(widget) = self.widgets.iter_mut().find(|w| w.id == id) {
                            widget.position = pos;
                        }
                    }
                    return Err("Cannot find positions for all widgets".to_string());
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct HomescreenState {
    pub pages: Vec<Page>,
    pub current_page: usize,
    pub hovered_widget: Option<usize>,
    pub selected_widget: Option<usize>,
    pub is_edit_mode: bool,

    pub is_dragging_page: bool,
    pub drag_start_x: f32,
    pub drag_offset_x: f32,
    pub drag_threshold: f32,

    pub is_animating: bool,
    pub current_offset: f32,
    pub target_offset: f32,
    pub animation_velocity: f32,
    pub last_update_time: Option<std::time::Instant>,

    pub dragging_widget: Option<usize>,
    pub dragging_widget_original_page: Option<usize>,
    pub hold_start_time: Option<std::time::Instant>,
    pub hold_start_x: f32,
    pub hold_start_y: f32,
    pub hold_widget: Option<usize>,

    pub edge_hold_start_time: Option<std::time::Instant>,
    pub last_page_switch_time: Option<std::time::Instant>,
}

impl HomescreenState {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            current_page: 0,
            hovered_widget: None,
            selected_widget: None,
            is_edit_mode: false,
            is_dragging_page: false,
            drag_start_x: 0.0,
            drag_offset_x: 0.0,
            drag_threshold: 10.0,
            is_animating: false,
            current_offset: 0.0,
            target_offset: 0.0,
            animation_velocity: 2000.0,
            last_update_time: None,
            dragging_widget: None,
            dragging_widget_original_page: None,
            hold_start_time: None,
            hold_start_x: 0.0,
            hold_start_y: 0.0,
            hold_widget: None,
            edge_hold_start_time: None,
            last_page_switch_time: None,
        }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn current_page(&self) -> Option<&Page> {
        self.pages.get(self.current_page)
    }

    pub fn current_page_mut(&mut self) -> Option<&mut Page> {
        self.pages.get_mut(self.current_page)
    }

    pub fn next_page(&mut self) {
        if self.current_page < self.pages.len().saturating_sub(1) {
            self.current_page += 1;
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn set_hovered(&mut self, widget_id: Option<usize>) {
        self.hovered_widget = widget_id;
    }

    pub fn set_selected(&mut self, widget_id: Option<usize>) {
        self.selected_widget = widget_id;
    }

    pub fn toggle_edit_mode(&mut self) {
        self.is_edit_mode = !self.is_edit_mode;
    }

    pub fn start_hold(&mut self, x: f32, y: f32, widget_id: Option<usize>) {
        self.hold_start_time = Some(std::time::Instant::now());
        self.hold_start_x = x;
        self.hold_start_y = y;
        self.hold_widget = widget_id;
    }

    pub fn check_hold(&mut self, current_x: f32, current_y: f32) -> bool {
        if let (Some(start_time), Some(widget_id)) = (self.hold_start_time, self.hold_widget) {
            let dx = current_x - self.hold_start_x;
            let dy = current_y - self.hold_start_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 10.0 {
                self.hold_start_time = None;
                self.hold_widget = None;
                return false;
            }

            let elapsed = start_time.elapsed().as_secs_f32();
            if elapsed >= 1.0 {
                self.dragging_widget = Some(widget_id);

                for (page_idx, page) in self.pages.iter().enumerate() {
                    if page.widgets.iter().any(|w| w.id == widget_id) {
                        self.dragging_widget_original_page = Some(page_idx);
                        break;
                    }
                }

                self.hold_start_time = None;
                self.hold_widget = None;
                return true;
            }
        }
        false
    }

    pub fn cancel_hold(&mut self) {
        self.hold_start_time = None;
        self.hold_widget = None;
    }

    pub fn end_widget_drag(&mut self) {
        println!("End widget drag");
        self.dragging_widget = None;
        self.dragging_widget_original_page = None;
        self.edge_hold_start_time = None;
        self.last_page_switch_time = None;
    }

    pub fn return_widget_to_original_page(
        &mut self,
        widget_id: usize,
        page_width: f32,
        page_gap: f32,
    ) -> bool {
        if let Some(original_page) = self.dragging_widget_original_page {
            let mut found_page = None;
            for (page_idx, page) in self.pages.iter().enumerate() {
                if page.widgets.iter().any(|w| w.id == widget_id) {
                    found_page = Some(page_idx);
                    break;
                }
            }

            if let Some(current_page) = found_page {
                if current_page != original_page {
                    self.move_widget_to_page_with_offset(
                        widget_id,
                        current_page,
                        original_page,
                        page_width,
                        page_gap,
                    );
                    return true;
                }
            }
        }
        false
    }

    pub fn start_drag(&mut self, x: gpui::Pixels) {
        self.is_animating = false;
        self.last_update_time = None;

        self.current_offset = self.target_offset;

        self.drag_start_x = x.into();
        self.drag_offset_x = 0.0;
        self.is_dragging_page = false;
    }

    pub fn update_drag(&mut self, current_x: gpui::Pixels) {
        let current_x_f32: f32 = current_x.into();
        let delta = current_x_f32 - self.drag_start_x;

        if !self.is_dragging_page && delta.abs() >= self.drag_threshold {
            self.is_dragging_page = true;
        }

        if self.is_dragging_page {
            self.drag_offset_x = delta;
        }
    }

    pub fn end_drag(&mut self, window_width: f32, page_gap: f32) {
        if !self.is_dragging_page {
            return;
        }

        let drag_distance = self.drag_offset_x;
        let threshold = window_width * 0.25;

        if drag_distance < -threshold && self.current_page < self.pages.len() - 1 {
            self.current_page += 1;
        } else if drag_distance > threshold && self.current_page > 0 {
            self.current_page -= 1;
        }

        self.target_offset = -(self.current_page as f32 * (window_width + page_gap));

        self.current_offset = self.current_offset + self.drag_offset_x;
        self.is_animating = true;
        self.is_dragging_page = false;
        self.drag_start_x = 0.0;
        self.drag_offset_x = 0.0;
    }

    pub fn update_animation(&mut self, delta_time: f32) -> bool {
        if !self.is_animating {
            return false;
        }

        let distance = self.target_offset - self.current_offset;

        if distance.abs() < 1.0 {
            self.current_offset = self.target_offset;
            self.is_animating = false;
            return false;
        }

        let direction = if distance > 0.0 { 1.0 } else { -1.0 };
        let step = self.animation_velocity * delta_time * direction;

        if step.abs() > distance.abs() {
            self.current_offset = self.target_offset;
            self.is_animating = false;
            return false;
        }

        self.current_offset += step;
        true
    }

    pub fn get_visual_offset(&self) -> f32 {
        if self.is_dragging_page {
            self.current_offset + self.drag_offset_x
        } else if self.is_animating {
            self.current_offset
        } else {
            self.target_offset
        }
    }

    pub fn check_edge_trigger(
        &mut self,
        cursor_x: f32,
        window_width: f32,
        edge_threshold: f32,
        page_width: f32,
        page_gap: f32,
    ) -> bool {
        let dragging_widget_id = match self.dragging_widget {
            Some(id) => id,
            None => {
                self.edge_hold_start_time = None;
                return false;
            }
        };

        let now = std::time::Instant::now();
        let is_at_edge = (cursor_x < edge_threshold && self.current_page > 0)
            || (cursor_x > window_width - edge_threshold
                && self.current_page < self.pages.len() - 1);

        if !is_at_edge {
            self.edge_hold_start_time = None;
            return false;
        }

        if self.edge_hold_start_time.is_none() {
            self.edge_hold_start_time = Some(now);
            return false;
        }

        let hold_duration = now
            .duration_since(self.edge_hold_start_time.unwrap())
            .as_millis();
        if hold_duration < 500 {
            return false;
        }

        if let Some(last_switch) = self.last_page_switch_time {
            let since_last_switch = now.duration_since(last_switch).as_millis();
            if since_last_switch < 500 {
                return false;
            }
        }

        let old_page = self.current_page;
        let mut changed = false;

        if cursor_x < edge_threshold && self.current_page > 0 {
            self.current_page -= 1;
            changed = true;
        } else if cursor_x > window_width - edge_threshold
            && self.current_page < self.pages.len() - 1
        {
            self.current_page += 1;
            changed = true;
        }

        if changed {
            self.target_offset = -(self.current_page as f32 * (page_width + page_gap));
            self.is_animating = true;

            self.move_widget_to_page(dragging_widget_id, old_page, self.current_page);

            self.last_page_switch_time = Some(now);
            self.edge_hold_start_time = Some(now);
        }

        changed
    }

    fn move_widget_to_page(&mut self, widget_id: usize, from_page: usize, to_page: usize) {
        if from_page >= self.pages.len() || to_page >= self.pages.len() {
            return;
        }

        let widget = {
            let from_page_widgets = &mut self.pages[from_page].widgets;
            let widget_index = from_page_widgets.iter().position(|w| w.id == widget_id);

            match widget_index {
                Some(idx) => from_page_widgets.remove(idx),
                None => return,
            }
        };

        self.pages[to_page].widgets.push(widget);
    }

    pub fn move_widget_to_page_with_offset(
        &mut self,
        widget_id: usize,
        from_page: usize,
        to_page: usize,
        page_width: f32,
        page_gap: f32,
    ) {
        if from_page >= self.pages.len() || to_page >= self.pages.len() || from_page == to_page {
            return;
        }

        let mut widget = {
            let from_page_widgets = &mut self.pages[from_page].widgets;
            let widget_index = from_page_widgets.iter().position(|w| w.id == widget_id);

            match widget_index {
                Some(idx) => from_page_widgets.remove(idx),
                None => return,
            }
        };

        let from_offset = from_page as f32 * (page_width + page_gap);
        let to_offset = to_page as f32 * (page_width + page_gap);
        let position_adjustment = from_offset - to_offset;

        widget.current_x += position_adjustment;
        widget.target_x += position_adjustment;

        self.pages[to_page].widgets.push(widget);
    }
}

impl Default for HomescreenState {
    fn default() -> Self {
        Self::new()
    }
}
