# Ticket T-101: PerpendicularDistance Constraint Implementation

**Objective:** Complete `Constraint::PerpendicularDistance` validation, resolution, and application.

**Spec Reference:** REQ-CONST-001–002, REQ-SOLVE-001–003, CAP-101

**Target Files:**
- `crates/lintel-core/src/constraint.rs`
- `crates/lintel-core/src/dimension.rs`

**Acceptance Criteria:**
- [ ] `Constraint::PerpendicularDistance { wall_id, param, dimension }` validates: wall exists, param ∈ [0,1], dimension exists on level
- [ ] `solve_constraints` returns `ResolvedConstraint` with correct point for `PerpendicularDistance`
- [ ] `apply_constraints_to_anchors` updates dimension's anchor `param` to resolved value, returns moved count
- [ ] Auto-solve in `MoveWall` command handler (calls `apply_constraints_to_anchors` after move)
- [ ] Unit test: wall move → anchored dimension follows (AC-001)
- [ ] Unit test: constraint round-trip JSON

**Dependencies:** T-100 (DimensionAnchor::Free)

**Estimated Effort:** 2 sessions