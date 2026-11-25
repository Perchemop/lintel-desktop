# Ticket T-109: UI Tests & Design Evidence

**Objective:** kittest AccessKit assertions and rendered design evidence for anchor UI.

**Spec Reference:** REQ-UI-001–006, AC-007–009, CAP-103, guide II.30.8

**Target Files:**
- `apps/lintel/src/tests/evidence.rs` (extend)
- `apps/lintel/src/tests/kittest_forms.rs` (extend or new)

**Acceptance Criteria:**
- [ ] kittest: anchor inputs have labelled-by relations to labels
- [ ] kittest: error/help texts announced as label nodes
- [ ] kittest: Tab order follows visual order (gap → overrun → start anchor → end anchor → Clear)
- [ ] kittest: 400→1200px resize proof — no overlapping controls, buttons ≥24pt
- [ ] Rendered evidence: `docs/qc/anchor-ui-evidence/` — light/dark/hover/click/focus screenshots
- [ ] Evidence folders have README documenting provenance (guide II.30.8)
- [ ] CI `kittest` job generates and uploads evidence artefacts

**Dependencies:** T-107, T-108

**Estimated Effort:** 1 session