//! Doors and windows, hosted on a wall.
//!
//! An opening is stored as a position **along its host wall** plus a size,
//! never as free-floating geometry. Move the wall and the opening moves with
//! it; delete the wall and the opening goes with it. That host relationship is
//! the whole point of a drafting model over a pile of lines.

use serde::{Deserialize, Serialize};

use crate::ids::{OpeningId, WallId};
use crate::wall::Wall;

pub const MIN_OPENING_WIDTH: f64 = 200.0;
pub const MAX_OPENING_WIDTH: f64 = 6000.0;
pub const MIN_OPENING_HEIGHT: f64 = 200.0;
pub const MAX_OPENING_HEIGHT: f64 = 6000.0;

pub const DEFAULT_DOOR_WIDTH: f64 = 900.0;
pub const DEFAULT_DOOR_HEIGHT: f64 = 2100.0;
pub const DEFAULT_WINDOW_WIDTH: f64 = 1200.0;
pub const DEFAULT_WINDOW_HEIGHT: f64 = 1200.0;
pub const DEFAULT_WINDOW_SILL: f64 = 900.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpeningKind {
    Door,
    Window,
}

impl OpeningKind {
    pub fn label(self) -> &'static str {
        match self {
            OpeningKind::Door => "Door",
            OpeningKind::Window => "Window",
        }
    }

    pub fn default_width(self) -> f64 {
        match self {
            OpeningKind::Door => DEFAULT_DOOR_WIDTH,
            OpeningKind::Window => DEFAULT_WINDOW_WIDTH,
        }
    }

    pub fn default_height(self) -> f64 {
        match self {
            OpeningKind::Door => DEFAULT_DOOR_HEIGHT,
            OpeningKind::Window => DEFAULT_WINDOW_HEIGHT,
        }
    }

    pub fn default_sill(self) -> f64 {
        match self {
            OpeningKind::Door => 0.0,
            OpeningKind::Window => DEFAULT_WINDOW_SILL,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Opening {
    pub id: OpeningId,
    pub kind: OpeningKind,
    pub host: WallId,
    /// Centre of the opening, as a distance from the host wall's start.
    pub offset: f64,
    pub width: f64,
    pub height: f64,
    /// Height of the bottom edge above the floor. Doors sit at 0.
    pub sill: f64,
    /// Swaps the hinge jamb of a door (no effect on a window).
    pub flipped: bool,
}

impl Opening {
    pub fn new(id: OpeningId, kind: OpeningKind, host: WallId, offset: f64) -> Self {
        Self {
            id,
            kind,
            host,
            offset,
            width: kind.default_width(),
            height: kind.default_height(),
            sill: kind.default_sill(),
            flipped: false,
        }
    }

    pub fn start_offset(&self) -> f64 {
        self.offset - self.width * 0.5
    }

    pub fn end_offset(&self) -> f64 {
        self.offset + self.width * 0.5
    }

    /// Does this opening stay inside its host wall and use a sane size?
    pub fn fits_on(&self, wall: &Wall) -> bool {
        if !self.offset.is_finite()
            || !self.width.is_finite()
            || !self.sill.is_finite()
            || !self.height.is_finite()
        {
            return false;
        }
        if !(MIN_OPENING_WIDTH..=MAX_OPENING_WIDTH).contains(&self.width) {
            return false;
        }
        if !(MIN_OPENING_HEIGHT..=MAX_OPENING_HEIGHT).contains(&self.height) {
            return false;
        }
        if self.sill < 0.0 || self.sill + self.height > 20000.0 {
            return false;
        }
        let start = self.start_offset();
        let end = self.end_offset();
        start >= 0.0 && end <= wall.length() && start <= end
    }

    pub fn kind_label(&self) -> &'static str {
        self.kind.label()
    }
}

/// Where an opening of `width` can be centred on a wall of `wall_length`
/// without overhanging either end. Used by the canvas so a click near a jamb
/// places a legal opening instead of being rejected, and by the 3D model later.
pub fn fit_offset_centre(wall_length: f64, width: f64, desired_centre: f64) -> f64 {
    let half = width * 0.5;
    if !wall_length.is_finite() || !desired_centre.is_finite() || wall_length < width {
        return half.max(0.0);
    }
    desired_centre.clamp(half, wall_length - half)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point2;

    fn host() -> Wall {
        Wall::new(
            WallId(1),
            Point2::new(0.0, 0.0),
            Point2::new(4000.0, 0.0),
            crate::wall::DEFAULT_WALL_THICKNESS,
            crate::wall::DEFAULT_WALL_HEIGHT,
        )
    }

    #[test]
    fn door_defaults_are_habitable_sizes() {
        let door = Opening::new(OpeningId(1), OpeningKind::Door, WallId(1), 1800.0);
        assert_eq!(door.width, DEFAULT_DOOR_WIDTH);
        assert_eq!(door.height, DEFAULT_DOOR_HEIGHT);
        assert_eq!(door.sill, 0.0);
        assert!(door.fits_on(&host()));
    }

    #[test]
    fn window_defaults_have_a_sill() {
        let window = Opening::new(OpeningId(2), OpeningKind::Window, WallId(1), 1000.0);
        assert_eq!(window.sill, DEFAULT_WINDOW_SILL);
        assert!(window.fits_on(&host()));
    }

    #[test]
    fn opening_must_stay_inside_the_wall() {
        let mut door = Opening::new(OpeningId(1), OpeningKind::Door, WallId(1), 300.0);
        assert!(!door.fits_on(&host()), "900 mm door centred at 300 mm overhangs the start");
        door.offset = 3600.0;
        assert!(!door.fits_on(&host()), "door overhangs the end");
        door.offset = 2000.0;
        assert!(door.fits_on(&host()));
    }

    #[test]
    fn opening_sizes_are_bounded() {
        let mut door = Opening::new(OpeningId(1), OpeningKind::Door, WallId(1), 2000.0);
        door.width = 100.0;
        assert!(!door.fits_on(&host()));
        door.width = 7000.0;
        assert!(!door.fits_on(&host()));
        door.width = 900.0;
        door.height = 9000.0;
        assert!(!door.fits_on(&host()));
    }

    #[test]
    fn offsets_mark_the_full_span() {
        let door = Opening::new(OpeningId(1), OpeningKind::Door, WallId(1), 2000.0);
        assert_eq!(door.start_offset(), 1550.0);
        assert_eq!(door.end_offset(), 2450.0);
    }

    #[test]
    fn a_click_near_a_jamb_is_pulled_inside_the_wall() {
        // 900 mm door on a 4000 mm wall.
        assert_eq!(fit_offset_centre(4000.0, 900.0, 10.0), 450.0);
        assert_eq!(fit_offset_centre(4000.0, 900.0, 3990.0), 3550.0);
        assert_eq!(fit_offset_centre(4000.0, 900.0, 2000.0), 2000.0);
        // A wall too short for the opening degrades to a centred start rather
        // than a negative offset.
        assert_eq!(fit_offset_centre(500.0, 900.0, 100.0), 450.0);
    }
}
