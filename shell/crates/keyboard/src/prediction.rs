use crate::layout::KeyButton;
use crate::types::{ActionParsed, Label};

/// Predicted character with confidence score
#[derive(Debug, Clone, PartialEq)]
pub struct PredictedChar {
    pub character: String,
    pub confidence: f64, // 0.0 to 1.0, where 1.0 is highest confidence
}

impl PredictedChar {
    pub fn new(character: String, confidence: f64) -> Self {
        Self {
            character,
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    pub fn with_max_confidence(character: String) -> Self {
        Self::new(character, 1.0)
    }

    pub fn with_default_confidence(character: String) -> Self {
        Self::new(character, 0.5)
    }
}

/// Configuration for predictive touch area expansion
#[derive(Debug, Clone)]
pub struct PredictiveConfig {
    /// Base multiplier for expanding touch areas
    pub base_expansion_factor: f64,
    /// Maximum distance from button center to consider a hit
    pub max_expansion_distance: f64,
    /// Weight factor for confidence scoring
    pub confidence_weight: f64,
    /// Minimum confidence threshold for prediction
    pub min_confidence_threshold: f64,
}

impl Default for PredictiveConfig {
    fn default() -> Self {
        Self {
            base_expansion_factor: 1.3,
            max_expansion_distance: 20.0,
            confidence_weight: 2.0,
            min_confidence_threshold: 0.1,
        }
    }
}

impl PredictiveConfig {
    pub fn conservative() -> Self {
        Self {
            base_expansion_factor: 1.2,
            max_expansion_distance: 15.0,
            confidence_weight: 1.5,
            min_confidence_threshold: 0.2,
        }
    }

    pub fn aggressive() -> Self {
        Self {
            base_expansion_factor: 1.5,
            max_expansion_distance: 30.0,
            confidence_weight: 3.0,
            min_confidence_threshold: 0.05,
        }
    }
}

/// Result of button prediction with scoring details
#[derive(Debug, Clone)]
pub struct PredictionResult<'a> {
    pub button: &'a KeyButton,
    pub confidence: f64,
    pub distance: f64,
    pub weighted_score: f64,
}

/// Engine for handling predictive touch logic
pub struct PredictionEngine;

impl PredictionEngine {
    pub fn find_predicted_button<'a>(
        view: &'a crate::layout::View,
        x: f64,
        y: f64,
        predictions: &[PredictedChar],
        config: &PredictiveConfig,
    ) -> Option<&'a KeyButton> {
        let candidates = Self::find_prediction_candidates(view, x, y, predictions, config);

        candidates
            .into_iter()
            .min_by(|a, b| {
                a.weighted_score
                    .partial_cmp(&b.weighted_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|result| result.button)
    }

    pub fn find_prediction_candidates<'a>(
        view: &'a crate::layout::View,
        x: f64,
        y: f64,
        predictions: &[PredictedChar],
        config: &PredictiveConfig,
    ) -> Vec<PredictionResult<'a>> {
        let mut candidates = Vec::new();

        for (row_pos, row) in &view.rows {
            for (button_x_offset, button) in &row.buttons {
                if let Some(confidence) = Self::get_button_confidence(button, predictions) {
                    if confidence < config.min_confidence_threshold {
                        continue;
                    }

                    let button_center =
                        Self::calculate_button_center(row_pos, button_x_offset, button);

                    if Self::is_within_expanded_area(
                        x,
                        y,
                        &button_center,
                        button,
                        confidence,
                        config,
                    ) {
                        let distance = Self::calculate_distance(x, y, &button_center);

                        if distance <= config.max_expansion_distance {
                            let weighted_score =
                                Self::calculate_weighted_score(distance, confidence, config);

                            candidates.push(PredictionResult {
                                button,
                                confidence,
                                distance,
                                weighted_score,
                            });
                        }
                    }
                }
            }
        }

        candidates
    }

    fn get_button_confidence(button: &KeyButton, predictions: &[PredictedChar]) -> Option<f64> {
        match &button.action {
            ActionParsed::Submit { text, keysym } => {
                let mut max_confidence = 0.0f64;

                if let Some(button_text) = text {
                    for predicted in predictions {
                        if predicted.character == *button_text {
                            max_confidence = max_confidence.max(predicted.confidence);
                        }
                    }
                }

                // for key in keys {
                //     for predicted in predictions {
                //         if predicted.character == key.0 {
                //             max_confidence = max_confidence.max(predicted.confidence);
                //         }
                //     }
                // }

                if let Label::Text(label_text) = &button.label {
                    for predicted in predictions {
                        if predicted.character == *label_text {
                            max_confidence = max_confidence.max(predicted.confidence);
                        }
                    }
                }

                if max_confidence > 0.0 {
                    Some(max_confidence)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn calculate_button_center(
        row_pos: &crate::types::Point,
        button_x_offset: &f64,
        button: &KeyButton,
    ) -> (f64, f64) {
        (
            row_pos.x + button_x_offset + button.size.0 / 2.0,
            row_pos.y + button.size.1 / 2.0,
        )
    }

    fn calculate_distance(x: f64, y: f64, center: &(f64, f64)) -> f64 {
        ((x - center.0).powi(2) + (y - center.1).powi(2)).sqrt()
    }

    fn is_within_expanded_area(
        x: f64,
        y: f64,
        button_center: &(f64, f64),
        button: &KeyButton,
        confidence: f64,
        config: &PredictiveConfig,
    ) -> bool {
        let expansion_factor = Self::calculate_expansion_factor(confidence, config);
        let expanded_width = button.size.0 * expansion_factor;
        let expanded_height = button.size.1 * expansion_factor;

        let half_width = expanded_width / 2.0;
        let half_height = expanded_height / 2.0;

        x >= button_center.0 - half_width
            && x <= button_center.0 + half_width
            && y >= button_center.1 - half_height
            && y <= button_center.1 + half_height
    }

    fn calculate_expansion_factor(confidence: f64, config: &PredictiveConfig) -> f64 {
        config.base_expansion_factor * (0.5 + confidence * 0.5)
    }

    fn calculate_weighted_score(distance: f64, confidence: f64, config: &PredictiveConfig) -> f64 {
        distance / (1.0 + confidence * config.confidence_weight)
    }
}
