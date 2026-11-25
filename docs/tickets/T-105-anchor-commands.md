# Ticket T-105: Anchor & Constraint Commands

**Objective:** Wire all anchor/constraint commands with undo/redo support.

**Spec Reference:** REQ-CMD-001–007, CAP-102

**Target Files:**
- `crates/lintel-core/src/command.rs`
- `crates/lintel-core/src/document.rs`

**Acceptance Criteria:**
- [x] `SetDimensionStartAnchor { id, anchor }` handler: sets anchor, returns `Applied`
- [x] `SetDimensionEndAnchor { id, anchor }` handler: sets anchor, returns `Applied`
- [x] `SetDimensionWitnessGap { id, gap }` handler: validates `gap ≥ 0`
- [x] `SetDimensionWitnessOverrun { id, overrun }` handler: validates `overrun ≥ 0`
- [x] `AddConstraint { constraint }` handler: validates, appends to level.constraints
- [x] `RemoveConstraint { index }` handler: removes at index, bounds check
- [x] All commands support undo/redo via existing `Document` history
- [x] Unit test: each command applies, undoes, redoes correctly
  - Added tests: `a_dimension_start_anchor_can_be_undone_and_redone`
  - Added tests: `a_dimension_end_anchor_can_be_undone_and_redone`
  - Added tests: `witness_margins_can_be_undone_and_redone`
  - Upgraded test: `an_added_constraint_can_be_undone_and_redone`
  - Added test: `a_added_constraint_can_be_redone`
  - Added test: `remove_a_constraint_can_be_undone_and_redone`

**Dependencies:** T-100, T-101, T-103

**Status:** COMPLETE — All acceptance criteria verified via CI.