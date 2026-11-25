# Ticket T-100: DimensionAnchor Free Variant & Resolution

**Objective:** Add `DimensionAnchor::Free` variant and `resolve_on` method; integrate with `Dimension.effective_start/end`.

**Spec Reference:** REQ-ANCH-001–005, CAP-100

**Target Files:**
- `crates/lintel-core/src/dimension.rs`

**Acceptance Criteria:**
- [ ] `DimensionAnchor` enum has variants `Anchored { wall_id, param, perpendicular }` and `Free`
- [ ] `resolve_on(&self, level: &Level) -> Option<Point2>` returns point for `Anchored`, `None` for `Free`
- [ ] `Dimension::effective_start()` returns `start_anchor.resolve_on().unwrap_or(start)`
- [ ] `Dimension::effective_end()` returns `end_anchor.resolve_on().unwrap_or(end)`
- [ ] Unit tests: `Free` anchor returns `None`, `Anchored` resolves correctly

**Dependencies:** None

**Estimated Effort:** 1 session