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

use std::collections::{HashMap, HashSet};

/// Maximum number of constraints allowed per level (REQ-CONST-004).
pub const MAX_CONSTRAINTS: usize = 500;

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
    if anchor.wall_id() != Some(wall_id) {
        return false;
    }
    if let DimensionAnchor::Anchored {
        param: p,
        perpendicular,
        ..
    } = anchor
    {
        *p = param;
        *perpendicular = true;
        true
    } else {
        false
    }
}

fn resolve(level: &Level, constraint: &Constraint) -> Option<ResolvedConstraint> {
    let Constraint::PerpendicularDistance { wall_id, param, .. } = constraint else {
        return None;
    };
    let anchor = DimensionAnchor::anchored(*wall_id, *param, false);
    let point = anchor.resolve_on(level)?;
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
            if !degrees.is_finite() || !(0.0..360.0).contains(degrees) {
                return Err(CoreError::InvalidProject(format!(
                    "{dimension} has a fixed angle outside 0..360 degrees"
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
/// Also enforces the 500-constraint limit per level.
pub fn validate_constraints(level: &Level) -> Result<(), CoreError> {
    // REQ-CONST-004: Maximum 500 constraints per level
    if level.constraints.len() > MAX_CONSTRAINTS {
        return Err(CoreError::LimitExceeded);
    }

    let mut pinned: HashSet<DimensionId> = HashSet::new();
    let mut angled: HashSet<DimensionId> = HashSet::new();
    let mut paired: HashSet<(DimensionId, DimensionId)> = HashSet::new();

    // Build a dependency graph for circular reference detection
    // FixedAngle and EqualLength create bidirectional dependencies
    let mut dependency_graph: HashMap<DimensionId, HashSet<DimensionId>> = HashMap::new();

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
                // FixedAngle alone doesn't create a cycle on its own
                // (it's a self-constraint that doesn't relate dimensions)
            }
            Constraint::EqualLength { a, b } => {
                let pair = if a < b { (*a, *b) } else { (*b, *a) };
                if !paired.insert(pair) {
                    return Err(CoreError::InvalidProject(format!(
                        "{a} and {b} already share an equal-length constraint"
                    )));
                }
                // Add bidirectional dependency for cycle detection
                dependency_graph.entry(*a).or_default().insert(*b);
                dependency_graph.entry(*b).or_default().insert(*a);
            }
        }
    }

    // Check for circular references in the dependency graph
    // This detects A->B->A cycles (and longer cycles)
    if has_circular_reference(&dependency_graph) {
        return Err(CoreError::CircularConstraint);
    }

    Ok(())
}

/// Detect circular references in a dependency graph.
/// A cycle exists if any node can reach itself through the graph.
fn has_circular_reference(graph: &HashMap<DimensionId, HashSet<DimensionId>>) -> bool {
    let mut visited: HashSet<DimensionId> = HashSet::new();
    let mut rec_stack: HashSet<DimensionId> = HashSet::new();

    for &node in graph.keys() {
        if !visited.contains(&node) && has_cycle_dfs(node, graph, &mut visited, &mut rec_stack) {
            return true;
        }
    }
    false
}

/// Depth-first search to detect cycles in a directed graph.
fn has_cycle_dfs(
    node: DimensionId,
    graph: &HashMap<DimensionId, HashSet<DimensionId>>,
    visited: &mut HashSet<DimensionId>,
    rec_stack: &mut HashSet<DimensionId>,
) -> bool {
    visited.insert(node);
    rec_stack.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                if has_cycle_dfs(neighbor, graph, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                // Found a back edge - cycle detected
                return true;
            }
        }
    }

    rec_stack.remove(&node);
    false
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
        level.dimensions[0].start_anchor = Some(DimensionAnchor::anchored(WALL, 0.1, true));

        let moved = apply_constraints_to_anchors(&mut level, &[perpendicular(0.6)]);
        assert_eq!(moved, 1);

        let dimension = &level.dimensions[0];
        assert_eq!(dimension.start, Point2::new(2400.0, 0.0));
        let anchor = dimension.start_anchor.expect("start anchor");
        assert_eq!(anchor.param().unwrap(), 0.6);
        assert!(anchor.perpendicular());
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
        level.dimensions[0].start_anchor = Some(DimensionAnchor::anchored(WALL, 0.1, true));
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

    // T-103 tests for constraint validation & limits

    #[test]
    fn validate_constraint_checks_wall_exists() {
        let level = level_with_wall();
        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WallId(99), // Non-existent wall
            param: 0.5,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not have"));
    }

    #[test]
    fn validate_constraint_checks_param_in_range() {
        let level = level_with_wall();

        // param < 0
        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param: -0.1,
        };
        assert!(validate_constraint(&level, &constraint).is_err());

        // param > 1
        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param: 1.5,
        };
        assert!(validate_constraint(&level, &constraint).is_err());

        // param = 0 and param = 1 are valid
        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param: 0.0,
        };
        assert!(validate_constraint(&level, &constraint).is_ok());

        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param: 1.0,
        };
        assert!(validate_constraint(&level, &constraint).is_ok());
    }

    #[test]
    fn validate_constraint_checks_dimension_exists() {
        let level = level_with_wall();
        let constraint = Constraint::PerpendicularDistance {
            dimension: DimensionId(99), // Non-existent dimension
            wall_id: WALL,
            param: 0.5,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not have"));
    }

    #[test]
    fn validate_constraints_checks_max_500_limit() {
        use crate::error::CoreError;

        // Create a level with exactly 500 constraints (at the limit)
        let mut level = level_with_wall();
        for i in 0..500 {
            let constraint = Constraint::PerpendicularDistance {
                dimension: DIMENSION,
                wall_id: WALL,
                param: (i % 100) as f64 / 100.0,
            };
            level.constraints.push(constraint);
        }

        // At the limit, valid constraints should pass
        let result = validate_constraints(&level);
        assert!(result.is_ok(), "500 constraints should be valid");

        // Add one more constraint (501st)
        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param: 0.5,
        };
        level.constraints.push(constraint);

        let result = validate_constraints(&level);
        assert!(
            matches!(result, Err(CoreError::LimitExceeded)),
            "501 constraints should return LimitExceeded"
        );
    }

    #[test]
    fn validate_constraints_checks_duplicate_constraints() {
        let mut level = level_with_wall();

        // Add a perpendicular constraint
        let constraint1 = perpendicular(0.5);
        level.constraints.push(constraint1.clone());

        // Adding duplicate perpendicular constraint on same dimension
        let constraint2 = perpendicular(0.6);
        level.constraints.push(constraint2);

        let result = validate_constraints(&level);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("pinned by more than one constraint")
        );
    }

    // AC-010: 501st constraint returns LimitExceeded via AddConstraint

    #[test]
    fn adding_501st_constraint_returns_limit_exceeded() {
        use crate::error::CoreError;
        use crate::project::Project;

        let mut project = Project::new("Test plan", 1_760_000_000);

        // Create a wall and dimension for constraints
        let wall = WallId(1);
        let dim = DimensionId(1);

        // Add 500 constraints first
        for i in 0..500 {
            let constraint = Constraint::PerpendicularDistance {
                dimension: dim,
                wall_id: wall,
                param: (i % 100) as f64 / 100.0,
            };
            // Manually add to level to bypass validation (we're testing the limit)
            let level = project.active_level_mut().expect("level");
            level.constraints.push(constraint);
        }

        // Now try to add one more via AddConstraint command
        let new_constraint = Constraint::PerpendicularDistance {
            dimension: dim,
            wall_id: wall,
            param: 0.5,
        };

        let result = apply_command(
            &mut project,
            Command::AddConstraint {
                constraint: new_constraint,
            },
        );

        assert!(
            matches!(result, Err(CoreError::LimitExceeded)),
            "501st constraint should return LimitExceeded"
        );

        // Verify the constraint was NOT added
        let level = project.active_level().expect("level");
        assert_eq!(
            level.constraints.len(),
            500,
            "constraint should not have been added"
        );
    }

    // AC-013: Circular constraint rejection tests

    #[test]
    fn circular_equal_length_constraint_is_rejected() {
        // Test that a full EqualLength cycle A -> B -> C -> A is detected
        let mut level = level_with_wall();

        // Add dimensions for testing
        let dim_a = DimensionId(100);
        let dim_b = DimensionId(101);
        let dim_c = DimensionId(102);

        level.dimensions.push(Dimension::new(
            dim_a,
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            500.0,
        ));
        level.dimensions.push(Dimension::new(
            dim_b,
            DimensionKind::LinearHorizontal,
            Point2::new(2000.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));
        level.dimensions.push(Dimension::new(
            dim_c,
            DimensionKind::LinearHorizontal,
            Point2::new(4000.0, 0.0),
            Point2::new(5000.0, 0.0),
            500.0,
        ));

        // A -> B -> C -> A would be a cycle
        // EqualLength { a, b }, EqualLength { b, c }, EqualLength { c, a }
        level
            .constraints
            .push(Constraint::EqualLength { a: dim_a, b: dim_b });
        level
            .constraints
            .push(Constraint::EqualLength { a: dim_b, b: dim_c });

        // Adding EqualLength { c, a } creates a cycle, which should be rejected by validate_constraints
        let circular_constraint = Constraint::EqualLength { a: dim_c, b: dim_a };
        level.constraints.push(circular_constraint);

        // validate_constraints should detect the cycle
        let result = validate_constraints(&level);
        assert!(
            matches!(result, Err(CoreError::CircularConstraint)),
            "Cycle A->B->C->A should be detected"
        );
    }

    #[test]
    fn equal_length_creates_dependency_cycle_with_three_constraints() {
        // Test that a full EqualLength cycle A -> B -> C -> A is detected
        let mut level = Level::new(LevelId(1), "test", 0.0);

        let dim_a = DimensionId(1);
        let dim_b = DimensionId(2);
        let dim_c = DimensionId(3);

        level.dimensions.push(Dimension::new(
            dim_a,
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            500.0,
        ));
        level.dimensions.push(Dimension::new(
            dim_b,
            DimensionKind::LinearHorizontal,
            Point2::new(2000.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));
        level.dimensions.push(Dimension::new(
            dim_c,
            DimensionKind::LinearHorizontal,
            Point2::new(4000.0, 0.0),
            Point2::new(5000.0, 0.0),
            500.0,
        ));

        // Create A -> B -> C -> A cycle
        level
            .constraints
            .push(Constraint::EqualLength { a: dim_a, b: dim_b });
        level
            .constraints
            .push(Constraint::EqualLength { a: dim_b, b: dim_c });
        level
            .constraints
            .push(Constraint::EqualLength { a: dim_c, b: dim_a });

        // validate_constraints should detect the cycle
        let result = validate_constraints(&level);
        assert!(
            matches!(result, Err(CoreError::CircularConstraint)),
            "Cycle A->B->C->A should be detected"
        );
    }

    #[test]
    fn single_equal_length_constraint_is_valid() {
        let mut level = level_with_wall();

        let dim_a = DimensionId(100);
        let dim_b = DimensionId(101);

        level.dimensions.push(Dimension::new(
            dim_a,
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(1000.0, 0.0),
            500.0,
        ));
        level.dimensions.push(Dimension::new(
            dim_b,
            DimensionKind::LinearHorizontal,
            Point2::new(2000.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        ));

        level
            .constraints
            .push(Constraint::EqualLength { a: dim_a, b: dim_b });
        let result = validate_constraints(&level);
        assert!(
            result.is_ok(),
            "Single EqualLength constraint should be valid"
        );
    }

    #[test]
    fn fixed_angle_degrees_out_of_range_is_invalid() {
        let level = level_with_wall();

        // Degrees < 0
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: -10.0,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_err());

        // Degrees = 360
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: 360.0,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_err());

        // Degrees > 360
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: 400.0,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_err());

        // Degrees = 0 (valid)
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: 0.0,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_ok());

        // Degrees = 359.999 (valid)
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: 359.999,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_ok());
    }

    #[test]
    fn equal_length_same_dimension_is_invalid() {
        let level = level_with_wall();

        // EqualLength with same dimension on both sides
        let constraint = Constraint::EqualLength {
            a: DIMENSION,
            b: DIMENSION,
        };
        let result = validate_constraint(&level, &constraint);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot be constrained equal to itself")
        );
    }

    // Serde roundtrip tests for constraints

    #[test]
    fn fixed_angle_constraint_round_trips() {
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: 45.0,
        };
        let json = serde_json::to_string(&constraint).expect("serialize");
        let back: Constraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, constraint);
    }

    #[test]
    fn equal_length_constraint_round_trips() {
        let constraint = Constraint::EqualLength {
            a: DIMENSION,
            b: DimensionId(4),
        };
        let json = serde_json::to_string(&constraint).expect("serialize");
        let back: Constraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, constraint);
    }

    #[test]
    fn all_constraint_variants_round_trip() {
        // PerpendicularDistance
        let constraint = Constraint::PerpendicularDistance {
            dimension: DIMENSION,
            wall_id: WALL,
            param: 0.5,
        };
        let json = serde_json::to_string(&constraint).expect("serialize");
        let back: Constraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, constraint);

        // FixedAngle
        let constraint = Constraint::FixedAngle {
            dimension: DIMENSION,
            degrees: 90.0,
        };
        let json = serde_json::to_string(&constraint).expect("serialize");
        let back: Constraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, constraint);

        // EqualLength
        let constraint = Constraint::EqualLength {
            a: DIMENSION,
            b: DimensionId(4),
        };
        let json = serde_json::to_string(&constraint).expect("serialize");
        let back: Constraint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, constraint);
    }

    #[test]
    fn circular_constraint_error_has_correct_message() {
        let error = CoreError::CircularConstraint;
        assert!(
            error.to_string().contains("circular"),
            "error message should mention 'circular'"
        );
    }

    // AC-002: PerpendicularDistance constraint moves anchor
    #[test]
    fn perpendicular_constraint_moves_anchor() {
        let mut level = level_with_wall();
        level.dimensions[0].start_anchor = Some(DimensionAnchor::anchored(WALL, 0.5, true));

        let constraint = perpendicular(0.75);
        level.constraints.push(constraint.clone());

        let constraints: Vec<_> = level.constraints.clone();
        let moved = apply_constraints_to_anchors(&mut level, &constraints);

        assert_eq!(moved, 1);
        let dimension = &level.dimensions[0];
        let start_anchor = dimension.start_anchor.expect("should have start anchor");
        assert_eq!(start_anchor.param().unwrap(), 0.75);
        assert_eq!(dimension.start, Point2::new(3000.0, 0.0));
    }

    // AC-003: 500 constraints solve < 50ms
    #[test]
    fn five_hundred_constraints_solve_quickly() {
        fn create_test_level() -> Level {
            let mut level = Level::new(LevelId(1), "test", 0.0);
            level.walls.push(Wall::new(
                WallId(2),
                Point2::new(0.0, 0.0),
                Point2::new(3000.0, 0.0),
                200.0,
                2800.0,
            ));
            level.dimensions.push(Dimension::new(
                DimensionId(3),
                DimensionKind::LinearHorizontal,
                Point2::new(0.0, 0.0),
                Point2::new(3000.0, 0.0),
                500.0,
            ));

            for i in 0..500 {
                let param = (i % 100) as f64 / 100.0;
                level.constraints.push(Constraint::PerpendicularDistance {
                    dimension: DIMENSION,
                    wall_id: WALL,
                    param,
                });
            }
            level
        }

        let level = create_test_level();
        let start = std::time::Instant::now();
        let constraints: Vec<_> = level.constraints.clone();
        let resolved = solve_constraints(&level, &constraints);
        let elapsed = start.elapsed();

        assert_eq!(resolved.len(), 500);
        assert!(
            elapsed.as_millis() < 50,
            "solve took {} ms",
            elapsed.as_millis()
        );
    }

    // AC-006, AC-011: Wall deletion cascade
    #[test]
    fn wall_deletion_cascades_constraint_and_anchor_clear() {
        let mut level = level_with_wall();
        level.dimensions[0].start_anchor = Some(DimensionAnchor::anchored(WALL, 0.25, true));
        level.constraints.push(perpendicular(0.5));

        assert!(level.wall(WALL).is_some());
        assert_eq!(level.constraints.len(), 1);

        level.walls.retain(|w| w.id != WALL);
        assert!(level.wall(WALL).is_none());
        assert_eq!(level.constraints.len(), 0);
        assert!(
            level.dimensions[0].start_anchor.is_none()
                || level.dimensions[0]
                    .start_anchor
                    .unwrap()
                    .wall_id()
                    .is_none()
        );
    }

    // AC-012: Zero-length dimension warning
    #[test]
    fn zero_length_dimension_is_invalid_at_level() {
        let level = level_with_wall();

        let mut dimension = Dimension::new(
            DimensionId(99),
            DimensionKind::LinearHorizontal,
            Point2::new(0.0, 0.0),
            Point2::new(3000.0, 0.0),
            500.0,
        );
        dimension.start_anchor = Some(DimensionAnchor::anchored(WALL, 0.5, false));
        dimension.end_anchor = Some(DimensionAnchor::anchored(WALL, 0.5, false));

        assert!(dimension.is_valid(), "stored points are valid");
        assert!(
            !dimension.is_valid_at(Some(&level)),
            "resolved points are zero-length at level"
        );
        assert!(dimension.dimension_line_at(Some(&level)).is_none());
    }
}
