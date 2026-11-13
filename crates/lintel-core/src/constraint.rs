//! Geometric constraints: rules that keep a dimension true to the walls it
//! measures.
//!
//! A constraint is *not* solved on every edit. The solver here is the small,
//! explicit pass the model needs today: it resolves the constraints it is
//! given against the current level and pushes the result into the anchored
//! ends of the dimensions on that level. Anything whose end cannot be pinned to
//! a single point is left for a later pass rather than guessed at, so the
//! drawing never moves in a way the user cannot follow.
//!
//! The whole module is pure: it reads a [`Level`], mutates the dimensions on it
//! and returns what changed. No constraint can panic on bad geometry; an
//! impossible one simply resolves to nothing.

use serde::{Deserialize, Serialize};

use crate::dimension::DimensionAnchor;
use crate::error::CoreError;
use crate::geom::Point2;
use crate::ids::{DimensionId, WallId};
use crate::level::Level;

use std::collections::HashSet;

/// A rule that ties a dimension to the geometry it measures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constraint {
    /// An anchored dimension measures across a wall, keeping its measured
    /// point `param` of the way along `wall_id`.
    PerpendicularDistance {
        /// The dimension the rule belongs to.
        dimension: DimensionId,
        /// The wall the measured point sits on.
        wall_id: WallId,
        /// Position along the wall as a fraction of its length.
        param: f64,
    },
    /// Two measured spans keep a fixed angle between them.
    FixedAngle {
        /// The dimension the rule belongs to.
        dimension: DimensionId,
        /// The angle between the two spans, in degrees.
        degrees: f64,
    },
    /// Two dimensions keep measuring the same length.
    EqualLength {
        /// One of the two dimensions.
        a: DimensionId,
        /// The other one.
        b: DimensionId,
    },
}

/// A constraint evaluated against a level: the rule plus the point it pins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedConstraint {
    /// The rule that produced this resolution.
    pub constraint: Constraint,
    /// The point on the wall the constrained measured point is pinned to.
    pub point: Point2,
}

impl ResolvedConstraint {
    /// The wall and the fraction along it this resolution pins, when it pins a
    /// wall point at all.
    ///
    /// [`Constraint::FixedAngle`] and [`Constraint::EqualLength`] relate two
    /// dimensions and pin no single point, so they resolve to `None`.
    pub fn perpendicular_target(&self) -> Option<(WallId, f64)> {
        match &self.constraint {
            Constraint::PerpendicularDistance { wall_id, param, .. } => Some((*wall_id, *param)),
            Constraint::FixedAngle { .. } | Constraint::EqualLength { .. } => None,
        }
    }
}

/// Evaluate `constraints` against `level`.
///
/// Only constraints that pin a single point are returned: a
/// [`Constraint::PerpendicularDistance`] resolves to the point on its wall. A
/// constraint whose wall is missing, whose parameter is not a fraction of the
/// wall, or whose wall has no usable geometry resolves to nothing.
///
/// `FixedAngle` and `EqualLength` relate two dimensions and carry no point yet,
/// so they are skipped until the solver grows a pass that moves both ends of a
/// pair. They are accepted here so a project can store them now.
pub fn solve_constraints(level: &Level, constraints: &[Constraint]) -> Vec<ResolvedConstraint> {
    constraints
        .iter()
        .filter_map(|constraint| resolve(level, constraint))
        .collect()
}

/// Push every resolved perpendicular-distance constraint into the dimensions
/// anchored to that wall, so a stored fallback point never drifts away from the
/// wall it was measured against.
///
/// Each anchored end on the constrained wall takes the constraint's `param` and
/// is flagged [`DimensionAnchor::perpendicular`]; the dimension's stored
/// `start`/`end` moves onto the resolved point. Dimensions with no anchor on
/// that wall are left untouched. Returns how many dimensions were re-pointed,
/// counting a dimension once per constraint that moved it.
pub fn apply_constraints_to_anchors(level: &mut Level, constraints: &[Constraint]) -> usize {
    let resolved = solve_constraints(level, constraints);
    let mut moved = 0;
    for resolution in resolved {
        let Some((wall_id, param)) = resolution.perpendicular_target() else {
            continue;
        };
        for dimension in &mut level.dimensions {
            let on_start = retarget(&mut dimension.start_anchor, wall_id, param);
            let on_end = retarget(&mut dimension.end_anchor, wall_id, param);
            if on_start {
                dimension.start = resolution.point;
            }
            if on_end {
                dimension.end = resolution.point;
            }
            if on_start || on_end {
                moved += 1;
            }
        }
    }
    moved
}

/// Point `anchor` at `param` of the way along `wall_id`, reporting whether it
/// sat on that wall.
fn retarget(anchor: &mut Option<DimensionAnchor>, wall_id: WallId, param: f64) -> bool {
    let Some(anchor) = anchor.as_mut() else {
        return false;
    };
    if anchor.wall_id != wall_id {
        return false;
    }
    anchor.param = param;
    anchor.perpendicular = true;
    true
}

fn resolve(level: &Level, constraint: &Constraint) -> Option<ResolvedConstraint> {
    let Constraint::PerpendicularDistance { wall_id, param, .. } = constraint else {
        return None;
    };
    let wall = level.wall(*wall_id)?;
    let point = DimensionAnchor::new(*wall_id, *param).resolve_on(wall)?;
    Some(ResolvedConstraint {
        constraint: constraint.clone(),
        point,
    })
}

/// Validate a single constraint against `level`: every referenced wall and
/// dimension must exist on the level, and the rule's own parameters must be
/// usable. Cross-constraint duplicate checks live in [`validate_constraints`].
pub fn validate_constraint(level: &Level, constraint: &Constraint) -> Result<(), CoreError> {
    match constraint {
        Constraint::PerpendicularDistance {
            dimension,
            wall_id,
            param,
        } => {
            if level.wall(*wall_id).is_none() {
                return Err(CoreError::InvalidProject(format!(
                    "constraint pins {wall_id}, which this level does not have"
                )));
            }
            ensure_dimension(level, *dimension)?;
            if !param.is_finite() || !(0.0..=1.0).contains(param) {
                return Err(CoreError::InvalidProject(format!(
                    "{dimension} has a constraint parameter outside 0..1"
                )));
            }
        }
        Constraint::FixedAngle { dimension, degrees } => {
            ensure_dimension(level, *dimension)?;
            if !degrees.is_finite() {
                return Err(CoreError::InvalidProject(format!(
                    "{dimension} has a fixed angle that is not a finite number"
                )));
            }
        }
        Constraint::EqualLength { a, b } => {
            ensure_dimension(level, *a)?;
            ensure_dimension(level, *b)?;
            if a == b {
                return Err(CoreError::InvalidProject(format!(
                    "{a} cannot be constrained equal to itself"
                )));
            }
        }
    }
    Ok(())
}

/// Validate every constraint on `level`, including cross-constraint duplicate
/// rules: no dimension may be pinned by more than one perpendicular
/// constraint, fixed-angle constraint, or equal-length pair.
pub fn validate_constraints(level: &Level) -> Result<(), CoreError> {
    let mut pinned: HashSet<DimensionId> = HashSet::new();
    let mut angled: HashSet<DimensionId> = HashSet::new();
    let mut paired: HashSet<(DimensionId, DimensionId)> = HashSet::new();

    for constraint in &level.constraints {
        validate_constraint(level, constraint)?;
        match constraint {
            Constraint::PerpendicularDistance { dimension, .. } => {
                if !pinned.insert(*dimension) {
                    return Err(CoreError::InvalidProject(format!(
                        "{dimension} is pinned by more than one constraint"
                    )));
                }
            }
            Constraint::FixedAngle { dimension, .. } => {
                if !angled.insert(*dimension) {
                    return Err(CoreError::InvalidProject(format!(
                        "{dimension} has more than one fixed angle"
                    )));
                }
            }
            Constraint::EqualLength { a, b } => {
                let pair = if a < b { (*a, *b) } else { (*b, *a) };
                if !paired.insert(pair) {
                    return Err(CoreError::InvalidProject(format!(
                        "{a} and {b} already share an equal-length constraint"
                    )));
                }
            }
        }
    }
    Ok(())
}

fn ensure_dimension(level: &Level, id: DimensionId) -> Result<(), CoreError> {
    if level.dimension(id).is_none() {
        return Err(CoreError::InvalidProject(format!(
            "constraint refers to {id}, which this level does not have"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimension::{Dimension, DimensionKind};
    use crate::ids::LevelId;
    use crate::wall::Wall;

    const WALL: WallId = WallId(2);
    const DIMENSION: DimensionId = DimensionId(3);

    /// A 4 m wall along the x-axis with a 0..3000 horizontal dimension on it.
    fn level_with_wall() -> Level {
        let mut level = Level::new(LevelId(1), "Level 0", 0.0);
        level.walls.push(Wall::new(
            WALL,
            Point2::new(0.0, 0.0),
            Point2::new(4000.0, 0.0),
            200.0,
            2800.0,
        ));
        level.dimensions.push(Dimension::new(
            DIMENSION,
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));
        level
    }

    fn perpendicular(param: f64) -> Constraint {
        Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param,
        }
    }

    #[test]
    fn a_perpendicular_constraint_resolves_to_its_wall_point() {
        let level = level_with_wall();
        let constraint = perpendicular(0.25);
        let resolved = solve_constraints(&level, std::slice::from_ref(&constraint));
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].point, Point2::new(1000.0, 0.0));
        assert_eq!(resolved[0].constraint, constraint);
        assert_eq!(
            resolved[0].perpendicular_target(),
            Some((WALL, 0.25)),
            "the resolution reports the wall it pins"
        );
    }

    #[test]
    fn constraints_that_pin_no_point_are_skipped() {
        let level = level_with_wall();
        let constraints = [
            Constraint::FixedAngle {
                dimension: DIMENSION,
                degrees: 90.0,
            },
            Constraint::EqualLength {
                a: DIMENSION,
                b: DimensionId(4),
            },
            Constraint::PerpendicularDistance {
                dimension: DIMENSION,
                wall_id: WallId(9),
                param: 0.5,
            },
            perpendicular(2.0),
        ];
        assert!(
            solve_constraints(&level, &constraints).is_empty(),
            "no wall point, no resolution"
        );
    }

    #[test]
    fn applying_a_constraint_moves_the_anchored_end_and_its_anchor() {
        let mut level = level_with_wall();
        level.dimensions[0].start_anchor = Some(DimensionAnchor::new(WALL, 0.1));

        let moved = apply_constraints_to_anchors(&mut level, &[perpendicular(0.6)]);
        assert_eq!(moved, 1);

        let dimension = &level.dimensions[0];
        assert_eq!(dimension.start, Point2::new(2400.0, 0.0));
        let anchor = dimension.start_anchor.expect("start anchor");
        assert_eq!(anchor.param, 0.6);
        assert!(anchor.perpendicular);
        assert_eq!(
            dimension.end,
            Point2::new(3000.0, 0.0),
            "an unanchored end stays where it was"
        );
    }

    #[test]
    fn a_dimension_with_no_anchor_is_untouched() {
        let mut level = level_with_wall();
        let moved = apply_constraints_to_anchors(&mut level, &[perpendicular(0.6)]);
        assert_eq!(moved, 0);
        assert_eq!(level.dimensions[0].start, Point2::new(0.0, 0.0));
        assert_eq!(level.dimensions[0].end, Point2::new(3000.0, 0.0));
    }

    #[test]
    fn a_stored_constraint_repoints_the_anchored_dimension_on_solve() {
        let mut level = level_with_wall();
        level.dimensions[0].start_anchor = Some(DimensionAnchor::new(WALL, 0.1));
        let constraint = perpendicular(0.75);
        level.constraints.push(constraint.clone());

        let constraints = level.constraints.clone();
        let moved = apply_constraints_to_anchors(&mut level, &constraints);

        assert_eq!(moved, 1);
        assert_eq!(
            level.dimensions[0].start,
            Point2::new(3000.0, 0.0),
            "the anchored end lands on the wall point the constraint names"
        );
        assert_eq!(level.constraints, vec![constraint]);
    }

    #[test]
    fn constraints_round_trip_through_json() {
        let constraint = perpendicular(0.6);
        let json = serde_json::to_string(&constraint).expect("serialize");
        let back: Constraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, constraint);

        let resolved = ResolvedConstraint {
            constraint: back,
            point: Point2::new(2400.0, 0.0),
        };
        let json = serde_json::to_string(&resolved).expect("serialize");
        let back: ResolvedConstraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, resolved);
        assert_eq!(back.perpendicular_target(), Some((WALL, 0.6)));
    }
}
