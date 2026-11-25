# Ticket T-106: Core Headless Tests

**Objective:** Headless workflow tests for anchored dimensions.

**Spec Reference:** AC-001–006, AC-010–013, CAP-100–102

**Target Files:**
- `crates/lintel-core/src/dimension_tests.rs` (new)
- `crates/lintel-core/src/constraint_tests.rs` (extend existing)

**Acceptance Criteria:**
- [ ] Test: wall move → anchored dimension follows (AC-001)
- [ ] Test: PerpendicularDistance constraint moves anchor (AC-002)
- [ ] Test: 500 constraints solve < 50ms (AC-003)
- [ ] Test: save/load round-trip preserves anchors + constraints (AC-004)
- [ ] Test: v0.12.x fixture loads without anchors (AC-005)
- [ ] Test: wall deletion cascade-deletes constraints + clears anchors (AC-006)
- [ ] Test: 501st constraint → LimitExceeded (AC-010)
- [ ] Test: cascade delete on wall removal (AC-011)
- [ ] Test: zero-length clamp warning (AC-012)
- [ ] Test: circular constraint rejection (AC-013)

**Dependencies:** T-100 through T-105

**Estimated Effort:** 2 sessions