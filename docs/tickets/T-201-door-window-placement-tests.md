# T-201 — Door/Window Placement Regression Tests

**Phase:** 2 (Core Fixes)

## Objective
Verify v0.20.1 door/window placement fixes are complete by adding comprehensive regression tests that cover the fixed placement logic.

## Context
- PR #68 fixed: centreline projection, `note_status` error surfacing, empty-canvas hint
- PR #70 fixed: opening placement projected onto wall centreline via `wall_offset_along_centerline`
- Need regression tests to prevent future regressions

## Acceptance Criteria
- [ ] Test `wall_offset_along_centerline` exists in `apps/lintel/src/tests/workflows.rs`
- [ ] Tests cover: placement at wall endpoint, midpoint, off-wall click, near corner
- [ ] Tests verify projection onto centreline (not raw click position)
- [ ] Tests verify no panic on edge cases (zero-length wall, degenerate click)

## Files to Modify
- `apps/lintel/src/tests/workflows.rs` — add regression tests
- `crates/lintel-core/src/opening.rs` — if helper functions need exposure

## Test Cases
1. **Endpoint placement**: Click at wall start/end → door snaps to centreline at that param
2. **Midpoint placement**: Click at wall centre → door centres correctly
3. **Off-wall click**: Click perpendicular to wall → projects to nearest centreline point
4. **Near corner**: Click near wall intersection → picks correct wall, projects correctly
5. **Empty canvas**: Click with no walls → shows hint, no panic

## Related
- PR #68, PR #70
- `wall_offset_along_centerline` function
- `Command::PlaceDoor` / `Command::PlaceWindow`

## Effort Estimate
Small (test-only, logic already fixed)