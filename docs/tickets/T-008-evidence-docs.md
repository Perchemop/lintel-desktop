# T-008: Rendered Evidence + egui_kittest Assertions + Docs Update

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-001 through T-007
**Estimated scope:** Single PR (final integration)

---

## Objective

Produce rendered evidence per II.30.8, add `egui_kittest` AccessKit assertions, update all tracking docs, bump version to 0.20.0.

---

## Acceptance Criteria

- [x] Evidence folders created with generator + README (PNGs are CI artefacts,
      attached to the PR — never committed, guide II.30.8 / D-036):
  - `docs/qc/buttons-evidence/` — light, dark, states (disabled/loading/selected), focus ring
  - `docs/qc/form-evidence/` — light, dark, error state
  - `docs/qc/layout-evidence/` — narrow 400 px, wide 1200 px, cards
- [x] `egui_kittest` assertions (in `apps/lintel/src/tests/` — the app is a
      binary crate, so integration tests cannot import its internals; the
      `tests/kittest.rs` file documents this split):
  - Buttons: `src/tests/kittest_buttons.rs` (T-001, already landed) — role=button,
    accessible names, ≥ 24 pt, Tab reaches every variant
  - Forms: `src/tests/kittest_forms.rs` — labelled-by relation on every input,
    error/help texts announced as label nodes, Tab order field → field → submit
    proven by input value identity
  - Layout: `src/tests/evidence.rs::kittest_layout_resizes_without_overlap_or_clipping`
    — resize 400 → 1200 px, every exposed button ≥ 24 pt, no two button rects intersect
  - Keyboard F7/F8/Ctrl+K and shell-region assertions: owned by the shell slice
    (T-005, `src/tests/shell.rs`) — not re-implemented here
- [x] `ROADMAP.md` updated: v0.20.0 section with per-ticket status
- [x] `CHANGELOG.md` entry under Unreleased (Added/Changed) — retitled v0.20.0 at release
- [x] `PROGRESS.md` entry (CI run IDs recorded by the orchestrator's `ci.sh` run)
- [x] `docs/tokens.md` ledger row — appended by the release workflow (CI-owned, unchanged)
- [x] `RELEASE_NOTES.md` drafted for v0.20.0 (marked draft; metrics appended by CI)
- [x] Version bumped: `apps/lintel/Cargo.toml`, `crates/lintel-core/Cargo.toml`,
      both `Cargo.lock` entries → 0.20.0
- [ ] CI green on all required checks (verification pending — orchestrator `ci.sh` run)

---

## Files Created/Modified

- `apps/lintel/src/tests/evidence.rs` — env-gated evidence generator + always-on
  resize/no-overlap assertion
- `apps/lintel/src/tests/forms.rs`, `apps/lintel/src/tests/kittest_forms.rs` —
  form unit + AccessKit tests (T-002 verification)
- `apps/lintel/src/tests/layout.rs` — rewritten for egui 0.36.2 (`run_ui`) with
  honest reflow assertions
- `apps/lintel/src/layout/helpers.rs` — `Frame::new()` (0.36.2), `InnerResponse<R>`
  returns, `card` shadow
- `apps/lintel/src/components/forms.rs` — egui 0.36.2 fixes (no `Response::default`,
  unique per-field child-Ui id) + in-module test harness fix
- `apps/lintel/Cargo.toml` (version 0.20.0, egui_kittest `wgpu` feature),
  `crates/lintel-core/Cargo.toml`, `Cargo.lock`
- `.gitignore` — evidence PNG patterns
- `.github/workflows/ci.yml` — evidence render + `design-evidence` artefact upload
  in the `kittest` job (timeout 10 → 20 min for the wgpu compile)
- `docs/qc/{buttons,form,layout}-evidence/README.md`, `docs/design/gui-stack.md`
- `ROADMAP.md`, `CHANGELOG.md`, `PROGRESS.md`, `RELEASE_NOTES.md`, `DECISIONS.md` (D-036)

---

## Notes

- Evidence is rendered headlessly in CI (`LINTEL_GENERATE_EVIDENCE=1`) — no pixel
  baselines (II.30.8); the AccessKit assertions are the gate, screenshots are for review
- Token ledger row format per §24.1 (underscore-grouped integers) — release workflow owns it
- Tag `v0.20.0` only after CI green + local install verification
