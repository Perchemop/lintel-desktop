# Ticket T-103: Constraint Validation & Limits

**Objective:** Enforce all validation rules and the 500-constraints-per-level limit.

**Spec Reference:** REQ-CONST-002, REQ-CONST-004, REQ-CONST-005, AC-010, AC-011

**Target Files:**
- `crates/lintel-core/src/constraint.rs`
- `crates/lintel-core/src/command.rs`

**Acceptance Criteria:**
- [ ] `validate_constraint` checks: wall exists, param ∈ [0,1], dimension exists
- [ ] `validate_constraints` checks: no duplicate constraints, no cycles, count ≤ 500
- [ ] `AddConstraint` returns `CoreError::LimitExceeded` when >500 constraints
- [ ] `DeleteWall` cascade-deletes constraints referencing deleted wall (REQ-CONST-005)
- [ ] Cascade delete also clears corresponding anchors on affected dimensions
- [ ] Unit test: 501st constraint → LimitExceeded (AC-010)
- [ ] Unit test: wall deletion removes constraints + clears anchors (AC-011)

**Dependencies:** T-101

**Estimated Effort:** 1 session