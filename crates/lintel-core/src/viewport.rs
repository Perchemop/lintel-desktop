//! Sheet viewports: a framed window onto one level at its own scale.
//!
//! A viewport is a rectangle of the sheet — millimetres with the origin at the
//! bottom-left, the same convention as [`crate::pdf::SheetSpec`] — that shows
//! one level's geometry at a stated scale. The drawing set draws one sheet per
//! level, so a viewport is bound to a level by id and several viewports may
//! share a sheet. The exporter clips approximately, by scale-fit, rather than
//! with a true PDF clipping path; see `crate::pdf` for the simplification.

use serde::{Deserialize, Serialize};

use crate::geom::Bounds;
use crate::ids::LevelId;

/// Largest dimension of any standard sheet, in millimetres (A1 landscape).
///
/// Used to reject a region that could never sit on a sheet. The sheet's paper
/// is chosen at export time rather than stored with the project, so this is the
/// strongest bound available to validation.
pub const MAX_SHEET_MILLIMETRES: f64 = 841.0;

/// Longest caption kept under a viewport, in characters.
pub const MAX_CAPTION_CHARS: usize = 60;

/// A framed region of a sheet showing one level at its own scale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    /// Level whose geometry this viewport shows.
    pub level: LevelId,
    /// Region of the sheet the viewport occupies, in millimetres.
    pub region: Bounds,
    /// Denominator of the viewport scale: `100` means 1:100.
    pub scale_denominator: u32,
    /// Caption drawn under the frame. Blank = omitted.
    #[serde(default)]
    pub caption: String,
}

impl Viewport {
    /// A viewport with its region corners ordered.
    pub fn new(level: LevelId, region: Bounds, scale_denominator: u32, caption: String) -> Self {
        Self {
            level,
            region: region.normalised(),
            scale_denominator,
            caption,
        }
    }

    /// `"1:100"`, the label printed under the frame.
    pub fn scale_label(&self) -> String {
        format!("1:{}", self.scale_denominator.max(1))
    }

    /// The region with its corners ordered, so an inverted drag still draws.
    pub fn normalised_region(&self) -> Bounds {
        self.region.normalised()
    }

    /// A viewport the sheet can draw: a scale of one or more and a region with
    /// a positive, finite area.
    pub fn is_valid(&self) -> bool {
        let region = self.region.normalised();
        self.scale_denominator >= 1
            && region.min.is_finite()
            && region.max.is_finite()
            && region.width().is_finite()
            && region.height().is_finite()
            && region.width() > 0.0
            && region.height() > 0.0
    }

    /// True when the region could sit on a standard sheet. The check uses the
    /// largest standard paper because the sheet's own size is chosen at export.
    pub fn is_within_sheet(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        let region = self.region.normalised();
        region.min.x >= 0.0
            && region.min.y >= 0.0
            && region.max.x <= MAX_SHEET_MILLIMETRES
            && region.max.y <= MAX_SHEET_MILLIMETRES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point2;

    fn region(x0: f64, y0: f64, x1: f64, y1: f64) -> Bounds {
        Bounds {
            min: Point2::new(x0, y0),
            max: Point2::new(x1, y1),
        }
    }

    #[test]
    fn a_well_formed_viewport_is_valid_and_on_the_sheet() {
        let viewport = Viewport::new(
            LevelId(1),
            region(10.0, 40.0, 150.0, 140.0),
            50,
            String::new(),
        );
        assert!(viewport.is_valid());
        assert!(viewport.is_within_sheet());
        assert_eq!(viewport.scale_label(), "1:50");
    }

    #[test]
    fn an_inverted_region_is_ordered_on_construction() {
        let viewport = Viewport::new(
            LevelId(1),
            region(150.0, 140.0, 10.0, 40.0),
            100,
            String::new(),
        );
        assert_eq!(viewport.region.min, Point2::new(10.0, 40.0));
        assert_eq!(viewport.region.max, Point2::new(150.0, 140.0));
        assert_eq!(viewport.normalised_region(), viewport.region);
    }

    #[test]
    fn a_zero_scale_is_not_valid() {
        let viewport = Viewport::new(
            LevelId(1),
            region(0.0, 0.0, 100.0, 100.0),
            0,
            String::new(),
        );
        assert!(!viewport.is_valid());
        assert!(!viewport.is_within_sheet());
    }

    #[test]
    fn a_flat_region_is_not_valid() {
        let viewport = Viewport::new(
            LevelId(1),
            region(10.0, 10.0, 10.0, 100.0),
            100,
            String::new(),
        );
        assert!(!viewport.is_valid());
    }

    #[test]
    fn a_non_finite_region_is_not_valid() {
        let viewport = Viewport::new(
            LevelId(1),
            region(0.0, 0.0, f64::INFINITY, 100.0),
            100,
            String::new(),
        );
        assert!(!viewport.is_valid());
    }

    #[test]
    fn a_region_beyond_the_largest_sheet_is_rejected() {
        let too_far = Viewport::new(
            LevelId(1),
            region(0.0, 0.0, MAX_SHEET_MILLIMETRES + 1.0, 100.0),
            100,
            String::new(),
        );
        assert!(too_far.is_valid(), "the region is well formed");
        assert!(!too_far.is_within_sheet(), "but it cannot fit a sheet");

        let negative = Viewport::new(
            LevelId(1),
            region(-5.0, 0.0, 100.0, 100.0),
            100,
            String::new(),
        );
        assert!(!negative.is_within_sheet());
    }

    #[test]
    fn a_viewport_round_trips_through_json() {
        let viewport = Viewport::new(
            LevelId(3),
            region(10.0, 40.0, 150.0, 140.0),
            50,
            "Ground floor unit".to_string(),
        );
        let json = serde_json::to_string(&viewport).expect("serialize");
        let back: Viewport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, viewport);
    }

    #[test]
    fn a_viewport_without_a_caption_loads() {
        let json = r#"{
            "level": 1,
            "region": { "min": { "x": 0.0, "y": 0.0 }, "max": { "x": 100.0, "y": 80.0 } },
            "scale_denominator": 50
        }"#;
        let viewport: Viewport = serde_json::from_str(json).expect("viewport");
        assert!(viewport.caption.is_empty());
        assert_eq!(viewport.level, LevelId(1));
    }
}
