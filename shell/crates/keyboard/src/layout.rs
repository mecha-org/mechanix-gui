use crate::prediction::{PredictedChar, PredictionEngine, PredictionResult, PredictiveConfig};
use crate::types::*;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct KeyButton {
    pub name: String,
    pub label: Label,
    pub size: (f64, f64),
    pub outline_name: String,
    pub action: ActionParsed,
    pub keycodes: Vec<KeyCode>,
}

impl Hash for KeyButton {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub buttons: Vec<(f64, KeyButton)>,
    size: Size,
}

impl Row {
    pub fn new(buttons: Vec<(f64, KeyButton)>) -> Row {
        let width = buttons
            .iter()
            .next_back()
            .map(|(x_offset, button)| button.size.0 + x_offset)
            .unwrap_or(0.0);

        let height =
            crate::utils::find_max_double(buttons.iter(), |(_offset, button)| button.size.1);

        Row {
            buttons,
            size: Size { width, height },
        }
    }

    pub fn get_size(&self) -> Size {
        self.size.clone()
    }

    pub fn get_buttons(&self) -> &Vec<(f64, KeyButton)> {
        &self.buttons
    }

    pub fn find_button_at_position(&self, x: f64) -> Option<(&KeyButton, usize)> {
        if x < 0.0 || self.buttons.is_empty() {
            return None;
        }

        let result = self.buttons.binary_search_by(|(offset, button)| {
            if x < *offset {
                std::cmp::Ordering::Greater
            } else if x >= *offset + button.size.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        match result {
            Ok(index) => Some((&self.buttons[index].1, index)),
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct View {
    pub rows: Vec<(Point, Row)>,
    pub size: Size,
}

impl View {
    pub fn new(rows: Vec<(f64, Row)>) -> View {
        let width = crate::utils::find_max_double(rows.iter(), |(_offset, row)| row.size.width);

        let height = rows
            .iter()
            .next_back()
            .map(|(y_offset, row)| row.size.height + y_offset)
            .unwrap_or(0.0);

        let rows = rows
            .into_iter()
            .map(|(y_offset, row)| {
                (
                    Point {
                        x: (width - row.size.width) / 2.0,
                        y: y_offset,
                    },
                    row,
                )
            })
            .collect::<Vec<_>>();

        View {
            rows,
            size: Size { width, height },
        }
    }

    pub fn find_button_at_position(&self, x: f64, y: f64) -> Option<&KeyButton> {
        if x < 0.0 || y < 0.0 || x >= self.size.width || y >= self.size.height {
            return None;
        }

        for (row_pos, row) in &self.rows {
            if y >= row_pos.y && y < row_pos.y + row.size.height {
                let relative_x = x - row_pos.x;
                return row
                    .find_button_at_position(relative_x)
                    .map(|(button, _)| button);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct ParsedLayout {
    pub views: HashMap<String, View>,
    pub keymaps: Vec<String>,
}

impl ParsedLayout {
    pub fn find_button_at_position(&self, view_name: &str, x: f64, y: f64) -> Option<&KeyButton> {
        self.views.get(view_name)?.find_button_at_position(x, y)
    }

    pub fn find_button_with_prediction(
        &self,
        view_name: &str,
        x: f64,
        y: f64,
        predicted_chars: &[PredictedChar],
        config: &PredictiveConfig,
    ) -> Option<&KeyButton> {
        if let Some(button) = self.find_button_at_position(view_name, x, y) {
            return Some(button);
        }

        let view = self.views.get(view_name)?;
        PredictionEngine::find_predicted_button(view, x, y, predicted_chars, config)
    }

    pub fn get_prediction_candidates(
        &self,
        view_name: &str,
        x: f64,
        y: f64,
        predicted_chars: &[PredictedChar],
        config: &PredictiveConfig,
    ) -> Vec<PredictionResult> {
        let view = match self.views.get(view_name) {
            Some(v) => v,
            None => return Vec::new(),
        };

        PredictionEngine::find_prediction_candidates(view, x, y, predicted_chars, config)
    }

    pub fn find_button_with_simple_prediction(
        &self,
        view_name: &str,
        x: f64,
        y: f64,
        predicted_chars: &[String],
        config: &PredictiveConfig,
    ) -> Option<&KeyButton> {
        let predictions: Vec<PredictedChar> = predicted_chars
            .iter()
            .map(|s| PredictedChar::with_default_confidence(s.clone()))
            .collect();

        self.find_button_with_prediction(view_name, x, y, &predictions, config)
    }

    pub fn find_buttons_near_position(
        &self,
        view_name: &str,
        x: f64,
        y: f64,
        max_distance: f64,
    ) -> Vec<(&KeyButton, f64)> {
        let view = match self.views.get(view_name) {
            Some(v) => v,
            None => return Vec::new(),
        };

        let mut nearby_buttons = Vec::new();

        for (row_pos, row) in &view.rows {
            for (button_x_offset, button) in &row.buttons {
                let button_center_x = row_pos.x + button_x_offset + button.size.0 / 2.0;
                let button_center_y = row_pos.y + button.size.1 / 2.0;

                let distance =
                    ((x - button_center_x).powi(2) + (y - button_center_y).powi(2)).sqrt();

                if distance <= max_distance {
                    nearby_buttons.push((button, distance));
                }
            }
        }

        nearby_buttons.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        nearby_buttons
    }

    pub fn get_view_names(&self) -> Vec<&String> {
        self.views.keys().collect()
    }

    pub fn get_view(&self, view_name: &str) -> Option<&View> {
        self.views.get(view_name)
    }

    pub fn create_predictions_from_stats(
        &self,
        usage_stats: &HashMap<String, f64>,
        max_predictions: usize,
    ) -> Vec<PredictedChar> {
        let mut predictions: Vec<_> = usage_stats
            .iter()
            .map(|(char, &freq)| PredictedChar::new(char.clone(), freq))
            .collect();

        predictions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        predictions.truncate(max_predictions);
        predictions
    }
}

impl ParsedLayout {
    pub fn print_layout(&self, view_name: &str, gap_cfg: &crate::config::GapDirection) {
        use std::cmp::Ordering;

        let view = self.views.get(view_name).unwrap();

        // Recalculate row widths exactly as in create_views
        let row_widths: Vec<f64> = view
            .rows
            .iter()
            .map(|(_row_pos, row)| {
                let last_index = row.buttons.len().saturating_sub(1);
                let mut width = 0.0_f64;
                for (i, (_offset, btn)) in row.buttons.iter().enumerate() {
                    width += btn.size.0;
                    // only add gap if not last button
                    if i < last_index {
                        let gap = gap_cfg
                            .custom
                            .get(btn.name.as_str())
                            .copied()
                            .unwrap_or(gap_cfg.default);
                        width += gap;
                    }
                }
                width
            })
            .collect();

        let max_width = row_widths.iter().cloned().fold(0.0, f64::max);
        println!(
            "Keyboard layout view '{}': (max row width: {:.2})\n",
            view_name, max_width
        );

        for (row_idx, ((_row_pos, row), row_width)) in view.rows.iter().zip(&row_widths).enumerate()
        {
            let start_offset = (max_width - row_width) / 2.0;
            println!(
                "Row {:>2} width: {:.2}, start offset: {:.2}",
                row_idx + 1,
                row_width,
                start_offset
            );

            // Print three console "lines": top frame, labels, offsets, and bottom frame

            // First: print spaces equal to start_offset/5 for all lines to match centering
            let space_unit = 5.0;
            let start_spaces = (start_offset / space_unit).round() as usize;
            let mut top_line = format!("| Row {:>2} {}", row_idx + 1, " ".repeat(start_spaces));
            let mut label_line = format!("|{:width$}", "", width = 8 + start_spaces);
            let mut pos_line = format!("|{:width$}", "", width = 8 + start_spaces);
            let mut bot_line = format!("|{:width$}", "", width = 8 + start_spaces);

            let mut last_x = start_offset;
            for (button_offset, button) in &row.buttons {
                // Print spacing between buttons
                let gap = (*button_offset - last_x).max(0.0);
                let num_spaces = (gap / space_unit).round() as usize;

                top_line.push_str(&" ".repeat(num_spaces));
                label_line.push_str(&" ".repeat(num_spaces));
                pos_line.push_str(&" ".repeat(num_spaces));
                bot_line.push_str(&" ".repeat(num_spaces));

                // Key box
                top_line.push_str("+-------");
                bot_line.push_str("+-------");

                let label_text = match &button.label {
                    crate::layout::Label::Text(text) => text.clone(),
                    crate::layout::Label::Icon(icon) => format!("[{}]", icon),
                };
                let clipped = if label_text.len() > 5 {
                    format!("{:.5}", label_text)
                } else {
                    format!("{:^5}", label_text)
                };
                label_line.push_str(&format!("|{:^7}", clipped));
                let pos_str = format!("{:^5}", format!("{:.0}", button_offset));
                pos_line.push_str(&format!("|{:^7}", pos_str));
                last_x = *button_offset + button.size.0;
            }

            top_line.push('+');
            label_line.push('|');
            pos_line.push('|');
            bot_line.push('+');

            // Print
            println!("{}", top_line);
            println!("{}", label_line);
            println!("{}", pos_line);
            println!("{}", bot_line);
            println!();
        }
    }
}
