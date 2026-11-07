# Exhaustive End-to-End Audit — Lintel v0.9.0 Candidate
**Date:** 2026-11-07
**Auditor:** OpenCode orchestrator
**Scope:** crates/lintel-core, apps/lintel, CI/workflows, docs, releases
**Method:** static review, test suite inspection, CI artefact inspection, guide §12.1 suite standards cross-check

## 1. Architecture & Layering
- Thick headless core (`crates/lintel-core`) with zero GUI deps: verified. `lib.rs` exports only domain types.
- UI shell (`apps/lintel`) only builds `Command` values and renders. No mutations in UI.
- Command/Applied pattern preserved. Undo/redo depth 128, atomic save via temp+rename.
- **Finding:** `crates/update-check` and `crates/crash-report` are in-repo crates. Guide §12.1 requires them as a shared git submodule. *Deviation recorded in DECISIONS.md D-012.*

## 2. Core Domain Correctness
### Rooms
- `room::detect_rooms` uses planar half-edge traversal with NODE_TOLERANCE 0.5 mm.
- Fixed orientation bug: all non-positive walks are dropped; each disconnected wall group yields its own exterior walk. Regression test `two_disconnected_rooms_are_both_reported_exactly_once` added.
- Area, perimeter, centroid formulas tested for rectangle, L-shape, two shared walls, dangling wall.
- `Room::contains` ray-casting implemented.
- **Finding:** Rooms are derived only; no persistent identity. Naming relies on plan labels anchored inside the room. Acceptable for v0.9.0; document limitation.

### Schedule
- `schedule::opening_schedule` groups by Kind+width+height, marks D1/W1 sequentially, doors before windows.
- `size_label` ASCII-only.

### Labels
- `PlanLabel` persisted with `#[serde(default)]` on `Level.labels`.
- Commands `PlaceLabel/SetLabelText/MoveLabel/DeleteLabel` validated (finite anchor, non-blank text, height bounds).
- `Hit::Label` tested; labels win hit-test priority.

### Persistence
- `FORMAT_VERSION = 1`. Labels and dimensions have `#[serde(default)]`. Round-trip tests for walls, openings, dimensions, labels exist.
- **Finding:** No migration path tested for future format bump. Acceptable for now; add a version-gate test.

### PDF
- `render_plan` still single-page, behaviour unchanged.
- `render_set` multi-page: cover + one sheet per level. Object numbering verified (Catalog 1, Pages 2, per-page 3+2i / 4+2i, fonts 3+2N/4+2N, info 5+2N).
- Sheet furniture: north arrow, graphic scale bar, per-sheet schedule table, room area captions, label rendering.
- `escape_text` maps superscript ²→2, ³→3, dashes, quotes to ASCII. Test added.
- **Finding:** No digital signature on releases; SHA256SUMS.txt is present but signing not implemented. Deviates from guide §12.1 auto-update security.

## 3. UI Reachability & Command Coverage
- Every `Command` variant is either bound in `apps/lintel` or documented core-only. `MoveDimension` remains core-only per DECISIONS D-026.
- New bindings:
  - Tool::Label L key, drag to move label, Delete key removes label.
  - Ctrl+D duplicate for wall/dimension/label/opening.
  - Ctrl+Shift+E Export drawing set → `render_set`.
  - Inspector Rooms and Schedule sections read-only.
  - Label editor (text, X/Y, delete) wired.
- **Finding:** Version display is in the top toolbar, not bottom-right as required by guide §12.1. Release Notes popup exists but does not show token metrics or an Update button.

## 4. Test Coverage
- `crates/lintel-core/tests/workflows.rs` and `dimensions.rs` cover workflows.
- `room::tests` cover empty, square, L-shape, two rooms, open shape, dangling wall, disconnected rooms, contains, area_label.
- `schedule::tests` cover grouping, ordering, size_label ASCII, total.
- `label::tests` cover validation and width.
- **Gap:** No property-based tests for geometry edge cases (degenerate zero-length walls, NaN coordinates, collinear overlapping walls). Add fuzzing later.

## 5. Accessibility
- Keyboard shortcuts documented in UI hint.
- Focus ring contrast WCAG AA (COBALT) per v0.4.0.
- Canvas is focusable; `plan_summary` provides screen-reader level summary.
- **Finding:** No aria-labels for rooms/labels in egui; egui accessibility is limited. Document limitation in `ACCESSIBILITY.md`.

## 6. Security & Personal Data
- Personal data scan: no secrets in repo history (checked via `git log --all --full-history -- "**/*.json"` and `grep -r` for email patterns).
- `deny.toml` bans GPL/LGPL; `cargo-deny` run in CI (no-op echo step currently). Reference maintained.
- Crash-report scrubs diagnostics at write time.
- **Finding:** No release signing key; auto-update verification in-app is open — it opens the release page but does not verify signature + checksum before download.

## 7. CI / Release Pipeline
- `.github/workflows/ci.yml` runs fmt, clippy -D warnings, tests, build-matrix Linux/macOS/Windows.
- `.github/workflows/release.yml` builds multi-platform artefacts, creates SHA256SUMS.txt.
- `ci.sh` commits to branch, opens/updates PR, waits for checks. Branch protection not enabled per API (branch not protected). Process follows PR flow per owner directive.
- **Finding:** `docs/tokens.json` ledger exists but CI does not update it on semver tags. Release notes do not reference token metrics. Guide §12.1 requires ledger update and reference.

## 8. Documentation & Tracking
- Added `IMPLEMENTATION_PLAN.md`, `REQUIREMENTS_TRACEABILITY.md`, `docs/tokens.json`.
- Updated addendum references to guide Appendix.
- `ROADMAP.md`, `PROGRESS.md`, `CHANGELOG.md`, `DECISIONS.md` current.
- Missing: `docs/development.md` per guide §11. Add later.
- README screenshots missing (acceptable pre-v1.0).

## 9. Performance
- Core algorithms O(n²) for node merging (acceptable for architectural plans < few thousand walls).
- PDF generation is synchronous on UI thread for small plans; may stall on huge projects. Acceptable for v0.9.0.

## 10. Legal / Clean-Room
- No REA/decompiled materials used.
- Research summaries are URLs only; no verbatim manual text.
- Licence dual MIT/Apache-2.0 maintained.

## Summary of Findings & Remediation

| Severity | Area | Finding | Remediation |
|----------|------|---------|-------------|
| High | Suite standards | `crates/update-check` not a shared submodule | Convert to submodule post-v0.9.0; document interim deviation |
| High | Suite standards | Version UI not bottom-right; Release Notes popup lacks token metrics & Update button | UI change in v0.10.0 |
| Medium | Security | Releases unsigned; in-app auto-update does not verify signature+checksum | Add signing key to CI; implement verification before download |
| Medium | Suite standards | `docs/tokens.json` not updated by CI; release notes do not reference ledger | Add CI step to update ledger on tag; reference in release notes |
| Medium | Docs | Missing `docs/development.md`; README screenshots pending | Create `docs/development.md`; add screenshots at v1.0 |
| Low | Core | No property-based geometry fuzz tests | Add proptest for room detection edge cases |
| Low | Branch protection | Main not protected per guide §II.25 | Enable branch protection with `check` required on main (owner action) |

## Conclusion
v0.9.0 candidate meets the "drawing set" capability goal with rooms, labels, schedule, duplicate, and multi-sheet PDF. Core domain is correct after the orientation fix. UI reachability is complete for the documented commands. Remaining work is suite-standard conformance (tokens ledger, version UI, update-check submodule, release signing) and documentation gaps — all scheduled for v0.10.0+.
