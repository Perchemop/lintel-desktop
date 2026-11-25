# Ticket T-108: Canvas Click-to-Place Witness Lines

**Objective:** Hover preview, click-to-place, and drag-from-grip interactions for dimension anchors.

**Spec Reference:** REQ-UI-001–003, REQ-UI-006, CAP-103

**Target Files:**
- `apps/lintel/src/main.rs` (canvas interaction)
- `apps/lintel/src/command_palette.rs` (constraint actions)

**Acceptance Criteria:**
- [ ] Dimension tool active + placing start point: hover wall → preview anchor at cursor param
- [ ] Click wall → creates anchor at click param, perpendicular=true, opens Inspector
- [ ] Drag from existing dimension endpoint grip → release over wall → sets anchor at release param
- [ ] Command Palette: "Add Perpendicular Constraint" action for active level
- [ ] Command Palette: "Remove Constraint" action for active level
- [ ] Design evidence: light/dark screenshots of hover preview, click placement, drag interaction

**Dependencies:** T-105, T-107

**Estimated Effort:** 2 sessions