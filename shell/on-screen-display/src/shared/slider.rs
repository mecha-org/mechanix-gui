use mctk_core::component::{ComponentHasher, Message};
use mctk_core::event::{self, Event};
use mctk_core::layout::{Alignment, Direction};
use mctk_core::{component, msg, state_component_impl, Color, Point, Scale, AABB};
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};
use std::fmt::Debug;
use std::hash::Hash;

use crate::utils::fill_grid_with_true;

#[derive(Debug, Default, Clone)]
pub enum SliderType {
    #[default]
    Box,
    Line,
}
#[derive(Debug, Default)]
pub struct SliderState {
    value: u8,
    grid: Vec<Vec<bool>>,
}
#[component(State = "SliderState")]
#[derive(Default)]
pub struct Slider {
    pub value: u8,
    pub slider_type: SliderType,
    pub height: u8,
    pub active_color: Color,
    pub col_width: f32,
    pub col_spacing: f32,
    pub row_spacing: f32,
    pub on_slide: Option<Box<dyn Fn(u8) -> Message + Send + Sync>>,
    pub on_slide_end: Option<Box<dyn Fn(u8) -> Message + Send + Sync>>,
    pub reset_on_slide_end: bool,
    pub fill_random_on_start: bool,
    pub fill_random_on_slide: bool,
}
impl Debug for Slider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slider")
            .field("value", &self.value)
            .field("slider_type", &self.slider_type)
            .field("height", &self.height)
            .field("active_color", &self.active_color)
            .field("col_width", &self.col_width)
            .field("col_spacing", &self.col_spacing)
            .field("row_spacing", &self.row_spacing)
            .field("state", &self.state)
            .field("dirty", &self.dirty)
            .finish()
    }
}

impl Slider {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn value(mut self, value: u8) -> Self {
        self.value = value;
        self
    }
    pub fn slider_type(mut self, slider_type: SliderType) -> Self {
        self.slider_type = slider_type;
        self
    }
    pub fn height(mut self, height: u8) -> Self {
        self.height = height;
        self
    }
    pub fn col_width(mut self, col_width: f32) -> Self {
        self.col_width = col_width;
        self
    }
    pub fn col_spacing(mut self, col_spacing: f32) -> Self {
        self.col_spacing = col_spacing;
        self
    }
    pub fn row_spacing(mut self, row_spacing: f32) -> Self {
        self.row_spacing = row_spacing;
        self
    }
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }
    pub fn on_slide(mut self, f: Box<dyn Fn(u8) -> Message + Send + Sync>) -> Self {
        self.on_slide = Some(f);
        self
    }
    pub fn on_slide_end(mut self, f: Box<dyn Fn(u8) -> Message + Send + Sync>) -> Self {
        self.on_slide_end = Some(f);
        self
    }
    pub fn reset_on_slide_end(mut self, value: bool) -> Self {
        self.reset_on_slide_end = value;
        self
    }
    pub fn fill_random_on_start(mut self, value: bool) -> Self {
        self.fill_random_on_start = value;
        self
    }
    pub fn fill_random_on_slide(mut self, value: bool) -> Self {
        self.fill_random_on_slide = value;
        self
    }

    pub fn handle_on_drag(
        &mut self,
        relative_logical_position: Point,
        current_logical_aabb: AABB,
    ) -> Option<u8> {
        let dx = relative_logical_position;
        let width = current_logical_aabb.width();
        let value = (dx.x as f32 / width as f32 * 100.) as i8;
        let value = (value).max(0).min(100) as u8;
        let prev_value = self.state_ref().value;
        if prev_value == value {
            return None;
        }

        let prev_grid = self.state_ref().grid.clone();
        let no_of_rows = prev_grid.len();
        let no_of_cols = prev_grid.get(0).unwrap_or(&vec![] as &Vec<bool>).len();
        let mut grid = vec![vec![false; no_of_cols as usize]; no_of_rows as usize];

        // Set specific cells as true based on the current value
        let true_values = (value as f32 * no_of_cols as f32 / 100.) as usize;
        for i in 0..true_values {
            for j in 0..no_of_rows as usize {
                grid[j][i] = true;
            }
        }

        if self.fill_random_on_slide {
            let random_vec =
                fill_grid_with_true(no_of_rows as usize, no_of_cols as usize - true_values, 8);
            for i in 0..(no_of_cols as usize - true_values) {
                for j in 0..no_of_rows as usize {
                    grid[j][i + true_values] = random_vec[j][i];
                }
            }
        }
        self.state_mut().value = value;
        self.state_mut().grid = grid;
        return Some(value);
    }

    pub fn handle_on_drag_end(&mut self) {
        if self.reset_on_slide_end {
            self.state_mut().value = 0;
            let prev_grid = self.state_ref().grid.clone();
            let no_of_rows = prev_grid.len();
            let no_of_cols = prev_grid.get(0).unwrap_or(&vec![] as &Vec<bool>).len();
            let random_vec = fill_grid_with_true(no_of_rows as usize, no_of_cols as usize, 20);
            self.state_mut().grid = random_vec;
        }
    }
}
#[state_component_impl(SliderState)]
impl Component for Slider {
    fn init(&mut self) {
        self.state = Some(SliderState {
            value: self.value,
            grid: Vec::new(),
        })
    }
    fn render_hash(&self, hasher: &mut ComponentHasher) {
        self.state_ref().value.hash(hasher);
        self.state_ref().grid.hash(hasher);
    }
    fn props_hash(&self, hasher: &mut ComponentHasher) {
        self.value.hash(hasher);
    }
    fn new_props(&mut self) {
        self.state_mut().value = self.value;
    }

    fn on_drag_start(&mut self, event: &mut Event<event::DragStart>) {
        println!("Slider::on_drag_start()");
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut Event<event::Drag>) {
        if let Some(value) = self.handle_on_drag(
            event.relative_logical_position(),
            event.current_logical_aabb(),
        ) {
            if let Some(f) = &self.on_slide {
                event.emit(f(value));
            }
        }
    }

    fn on_drag_end(&mut self, event: &mut Event<event::DragEnd>) {
        if let Some(f) = &self.on_slide_end {
            let value = self.state_ref().value;
            event.emit(f(value));
        }

        self.handle_on_drag_end();
    }

    fn on_touch_drag_start(&mut self, event: &mut Event<event::TouchDragStart>) {
        println!("Slider::on_touch_drag_start()");
        event.stop_bubbling();
    }

    fn on_touch_drag(&mut self, event: &mut Event<event::TouchDrag>) {
        if let Some(value) = self.handle_on_drag(
            event.relative_logical_position(),
            event.current_logical_aabb(),
        ) {
            if let Some(f) = &self.on_slide {
                event.emit(f(value));
            }
        }
    }

    fn on_touch_drag_end(&mut self, event: &mut Event<event::TouchDragEnd>) {
        if let Some(f) = &self.on_slide_end {
            let value = self.state_ref().value;
            event.emit(f(value));
        }

        self.handle_on_drag_end();
    }

    fn full_control(&self) -> bool {
        true
    }
    fn set_aabb(
        &mut self,
        aabb: &mut AABB,
        _parent_aabb: AABB,
        _children: Vec<(&mut AABB, Option<Scale>, Option<Point>)>,
        _frame: AABB,
        _scale_factor: f32,
    ) {
        if self.state_ref().grid.len() > 0 {
            return;
        }

        let width = aabb.width();
        let height = aabb.height();
        let line_spacing = self.col_spacing;
        let row_spacing = self.row_spacing;
        let line_width = self.col_width;

        let no_of_cols = ((width - line_width) / (line_width + line_spacing)) as u32 + 1;
        let no_of_rows = ((height - line_width) / (line_width + row_spacing)) as u32 + 1;

        let grid = if self.fill_random_on_start {
            fill_grid_with_true(no_of_rows as usize, no_of_cols as usize, 20)
        } else {
            let mut grid: Vec<Vec<bool>> =
                vec![vec![false; no_of_cols as usize]; no_of_rows as usize];
            //Set specific cells as true based on the current value
            let true_values = (self.state_ref().value as f32 * no_of_cols as f32 / 100.) as usize;
            for i in 0..true_values {
                for j in 0..no_of_rows as usize {
                    grid[j][i] = true;
                }
            }
            grid
        };

        self.state_mut().grid = grid;
    }
    fn view(&self) -> Option<Node> {
        let mut slider = node!(
            Div::new(),
            lay![
                size_pct: [100],
                cross_alignment: Alignment::Stretch
            ]
        );
        let grid = self.state_ref().grid.clone();
        let slider_type = self.slider_type.clone();
        let col_width = self.col_width;
        let col_spacing = self.col_spacing;
        let row_spacing = self.row_spacing;
        for i in 0..grid.get(0).unwrap_or(&vec![] as &Vec<bool>).len() {
            let col = match slider_type {
                SliderType::Box => {
                    let mut col_grid = node!(
                        Div::new(),
                        lay![
                            size:[col_width, Auto],
                            margin: [0., 0., 0., col_spacing],
                            direction: Direction::Column
                        ]
                    )
                    .key(i as u64);
                    for j in 0..grid.len() {
                        let color = if grid[j][i] == true {
                            self.active_color
                        } else {
                            Color::rgb(49., 49., 49.)
                        };
                        col_grid = col_grid.push(
                            node!(
                                Div::new().bg(color),
                                lay![size: size!(col_width, col_width), margin:[0., 0., row_spacing, 0.]]
                            )
                            .key(j as u64),
                        )
                    }

                    col_grid
                }
                SliderType::Line => {
                    let color = if grid[0][i] == true {
                        self.active_color
                    } else {
                        Color::rgb(49., 49., 49.)
                    };
                    let v_line = node!(
                        Div::new().bg(color),
                        lay![
                            size:[col_width, Auto],
                            margin: [0., 0., 0., col_spacing]
                        ]
                    )
                    .key(i as u64);
                    v_line
                }
            };
            slider = slider.push(col);
        }
        Some(slider)
    }
}
