# Ticket T-102: FixedAngle & EqualLength Constraints

**Objective:** Implement `Constraint::FixedAngle` and `Constraint::EqualLength` with validation, resolution, and application.

**Spec Reference:** REQ-CONST-001, REQ-CONST-003, REQ-SOLVE-001–003, CAP-101

**Target Files:**
- `crates/lintel-core/src/constraint.rs`

**Acceptance Criteria:**
- [ ] `FixedAngle { dimension, degrees }` validates: dimension exists, degrees ∈ [0, 360)
- [ ] `EqualLength { a, b }` validates: both dimensions exist on level, a ≠ b
- [ ] Circular reference detection: reject if adding creates A→B→A chain (REQ-CONST-003)
- [ ] `solve_constraints` resolves `FixedAngle` (angle from horizontal) and `EqualLength` (match length of reference dimension)
- [ ] `apply_constraints_to_anchors` applies resolved points to affected dimensions
- [ ] Unit test: circular constraint rejection (AC-013)
- [ ] Unit test: FixedAngle resolution, EqualLength matching

**Dependencies:** T-101

**Estimated Effort:** 2 sessions