# Ticket T-104: Backward-Compatible JSON Load

**Objective:** Pre-v0.13.0 project files load without anchor fields, preserving coordinates.

**Spec Reference:** REQ-PERSIST-001–004, AC-004, AC-005

**Target Files:**
- `crates/lintel-core/src/dimension.rs`
- `crates/lintel-core/src/persistence.rs`

**Acceptance Criteria:**
- [ ] `Dimension` serde: `start_anchor`/`end_anchor` optional, default `None`
- [ ] Missing anchor fields in JSON → `None` (equivalent to `Free`)
- [ ] Stored `start`/`end` coordinates preserved when anchors missing
- [ ] Constraints serialize/deserialize round-trip correctly
- [ ] Unit test: load v0.12.x fixture file → dimensions have no anchors, coords unchanged (AC-005)
- [ ] Unit test: save/load project with anchors + constraints → identical (AC-004)

**Dependencies:** T-100

**Estimated Effort:** 1 session