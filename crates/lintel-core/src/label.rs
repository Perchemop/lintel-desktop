//! Free text annotation placed on a plan.
//!
//! A label stores its anchor, its text and a cap height; the drawn width is
//! estimated from the text, so the UI never has to measure a font and the PDF
//! can lay the same label out without a font metric table.

use serde::{Deserialize, Serialize};

use crate::geom::Point2;
use crate::ids::{LabelId, LayerId};
use crate::layer::EntityStyle;

/// Cap height a new label starts with, in millimetres.
pub const DEFAULT_LABEL_HEIGHT_MM: f64 = 300.0;

/// Smallest cap height a label may use, in millimetres.
pub const MIN_LABEL_HEIGHT_MM: f64 = 50.0;

/// Largest cap height a label may use, in millimetres.
pub const MAX_LABEL_HEIGHT_MM: f64 = 5000.0;

/// Free text annotation placed on a plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanLabel {
    pub id: LabelId,
    /// Layer this label is drawn on. Kept in step with `style.layer_id`.
    #[serde(default)]
    pub layer_id: LayerId,
    /// Per-label overrides of the layer's appearance.
    #[serde(default)]
    pub style: EntityStyle,
    /// Point the text starts from (bottom-left of the first character).
    pub anchor: Point2,
    pub text: String,
    /// Cap height in millimetres.
    pub height_mm: f64,
}

impl PlanLabel {
    /// Create a label with [`DEFAULT_LABEL_HEIGHT_MM`].
    pub fn new(id: LabelId, anchor: Point2, text: impl Into<String>) -> Self {
        Self {
            id,
            layer_id: LayerId::DEFAULT,
            style: EntityStyle::default(),
            anchor,
            text: text.into(),
            height_mm: DEFAULT_LABEL_HEIGHT_MM,
        }
    }

    /// Move this label onto `layer_id`, keeping its style mirror in step.
    pub fn on_layer(mut self, layer_id: LayerId) -> Self {
        self.layer_id = layer_id;
        self.style.layer_id = layer_id;
        self
    }

    /// Finite anchor, non-blank text, height within the bounds above.
    pub fn is_valid(&self) -> bool {
        self.anchor.is_finite()
            && !self.text.trim().is_empty()
            && self.height_mm.is_finite()
            && (MIN_LABEL_HEIGHT_MM..=MAX_LABEL_HEIGHT_MM).contains(&self.height_mm)
    }

    /// Estimated drawn width in millimetres (roughly `height * 0.6 * chars`).
    pub fn width_mm(&self) -> f64 {
        let characters = self.text.chars().count() as f64;
        self.height_mm * 0.6 * characters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(text: &str) -> PlanLabel {
        PlanLabel::new(LabelId(1), Point2::new(1000.0, 500.0), text)
    }

    #[test]
    fn new_uses_the_default_height() {
        let label = label("Living room");
        assert_eq!(label.height_mm, DEFAULT_LABEL_HEIGHT_MM);
        assert_eq!(label.id, LabelId(1));
        assert_eq!(label.anchor, Point2::new(1000.0, 500.0));
        assert!(label.is_valid());
    }

    #[test]
    fn blank_and_whitespace_text_is_invalid() {
        assert!(!label("").is_valid());
        assert!(!label("   ").is_valid());
        assert!(!label("\t\n").is_valid());
    }

    #[test]
    fn a_non_finite_anchor_is_invalid() {
        let mut label = label("Kitchen");
        label.anchor.x = f64::NAN;
        assert!(!label.is_valid());
        label.anchor.x = 0.0;
        label.anchor.y = f64::INFINITY;
        assert!(!label.is_valid());
    }

    #[test]
    fn heights_outside_the_bounds_are_invalid() {
        let mut label = label("Hall");
        label.height_mm = MIN_LABEL_HEIGHT_MM - 1.0;
        assert!(!label.is_valid());
        label.height_mm = MAX_LABEL_HEIGHT_MM + 1.0;
        assert!(!label.is_valid());
        label.height_mm = f64::NAN;
        assert!(!label.is_valid());
        label.height_mm = MIN_LABEL_HEIGHT_MM;
        assert!(label.is_valid());
        label.height_mm = MAX_LABEL_HEIGHT_MM;
        assert!(label.is_valid());
    }

    #[test]
    fn width_grows_with_text_length_and_height() {
        let short = label("W");
        let long = label("Living room");
        assert!(long.width_mm() > short.width_mm());

        let mut tall = label("W");
        tall.height_mm = DEFAULT_LABEL_HEIGHT_MM * 2.0;
        assert!(tall.width_mm() > short.width_mm());
        assert!((short.width_mm() - DEFAULT_LABEL_HEIGHT_MM * 0.6).abs() < 1e-9);
    }

    #[test]
    fn label_round_trips_through_json() {
        let original = label("Bedroom");
        let json = serde_json::to_string(&original).expect("serialize");
        let back: PlanLabel = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    fn a_new_label_starts_on_the_default_layer() {
        let label = label("Hall");
        assert_eq!(label.layer_id, LayerId::DEFAULT);
        assert_eq!(label.style.layer_id, LayerId::DEFAULT);
    }

    #[test]
    fn a_label_can_be_placed_on_another_layer() {
        let label = label("Hall").on_layer(LayerId(7));
        assert_eq!(label.layer_id, LayerId(7));
        assert_eq!(label.style.layer_id, LayerId(7));
    }

    #[test]
    fn a_label_saved_before_layers_still_loads() {
        let json = r#"{
            "id": 2,
            "anchor": { "x": 1000.0, "y": 500.0 },
            "text": "Hall",
            "height_mm": 300.0
        }"#;
        let label: PlanLabel = serde_json::from_str(json).expect("old label");
        assert_eq!(label.layer_id, LayerId::DEFAULT);
        assert_eq!(label.style, EntityStyle::default());
        assert!(label.is_valid());
    }
}
