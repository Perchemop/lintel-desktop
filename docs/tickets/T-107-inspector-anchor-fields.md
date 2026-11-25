# Ticket T-107: Inspector Anchor Fields & Live Preview

**Objective:** Inspector shows witness_gap/overrun inputs, anchor status, Clear buttons with live canvas preview.

**Spec Reference:** REQ-UI-004, REQ-UI-005, CAP-103

**Target Files:**
- `apps/lintel/src/inspector.rs`

**Acceptance Criteria:**
- [ ] Dimension inspector shows `witness_gap` number input (mm, default 2.0) with live preview
- [ ] Dimension inspector shows `witness_overrun` number input (mm, default 2.0) with live preview
- [ ] Anchor status display: "Free" or "Anchored to Wall #N @ X.XX"
- [ ] Clear button next to each anchor → sets anchor to `None` (reverts to Free)
- [ ] Changes emit `SetDimensionWitnessGap`, `SetDimensionWitnessOverrun`, `SetDimensionStartAnchor(None)` commands
- [ ] `kittest` AccessKit assertions: labelled-by for inputs, role=spinbutton, focus order

**Dependencies:** T-105

**Estimated Effort:** 1 session