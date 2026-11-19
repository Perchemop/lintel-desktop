# T-008: Rendered Evidence + egui_kittest Assertions + Docs Update

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-001 through T-007
**Estimated scope:** Single PR (final integration)

---

## Objective

Produce rendered evidence per II.30.8, add `egui_kittest` AccessKit assertions, update all tracking docs, bump version to 0.20.0.

---

## Acceptance Criteria

- [ ] Screenshots attached to PR (not committed):
  - Light theme: shell, button variants, form field, empty canvas
  - Dark theme: same views
  - Error state: form with validation errors
  - Dialog: confirmation dialog with focus ring
  - Narrow window (400px): rail collapsed, canvas readable
- [ ] `egui_kittest` assertions in `tests/kittest_integration.rs`:
  - Button roles=button, accessible names, focus ring visible, size ≥ 24pt
  - Form fields: label associated, error announced, Tab order correct
  - Shell: three regions reachable, rail collapsible, Esc dismisses dialogs
  - Keyboard: Tab order, F7/F8/F7/F8 work, Ctrl+K opens palette
- [ ] `ROADMAP.md` updated: v0.20.0 items marked complete
- [ ] `CHANGELOG.md` entry for v0.20.0 (Added/Changed/Fixed)
- [ ] `PROGRESS.md` entry with CI run IDs
- [ ] `docs/tokens.md` ledger row appended (CI workflow step)
- [ ] `RELEASE_NOTES.md` drafted for v0.20.0
- [ ] Version bumped: `apps/lintel/Cargo.toml`, `crates/lintel-core/Cargo.toml` → 0.20.0
- [ ] CI green on all required checks

---

## Files to Modify

- `apps/lintel/src/tests/kittest_integration.rs` — new AccessKit assertions
- `ROADMAP.md`, `CHANGELOG.md`, `PROGRESS.md`, `RELEASE_NOTES.md`
- `apps/lintel/Cargo.toml`, `crates/lintel-core/Cargo.toml` — version 0.20.0
- `.github/workflows/release.yml` — ensure ledger row appended

---

## Notes

- Evidence is rendered from built app (CI artefact) — no pixel baselines (II.30.8)
- `egui_kittest` assertions are the gate, screenshots are for review
- Token ledger row format per §24.1 (underscore-grouped integers)
- Tag `v0.20.0` only after CI green + local install verification