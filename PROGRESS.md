# Progress

## v0.11.0 Status — RELEASED

- [x] `crates/lintel-core/src/measurements.rs` added: `Measurements`,
      `LevelTotals`, `ProjectTotals` (room area/perimeter, level and project
      wall/opening totals)
- [x] `format_area` honours `UnitSystem` (m² / ft² / in²) in
      `crates/lintel-core/src/units.rs`
- [x] Unit tests in `measurements::tests` (area, perimeter, totals, missing
      level, out-of-range room, JSON round-trip)
- [x] UI: `Tool::Measurements` with Shift+M; Inspector → Measurements shows the
      selected room, per-level totals and per-room list, and project totals
- [x] Release prep: version bumps landed in commit `803b590`
      (2026-11-11 rebuild; the original tag shipped with a stale 0.10.1 stamp —
      audit F1)
- [x] CI green, tag `v0.11.0` on `main` (commit `803b590`), GitHub Release
      published with all assets and `SHA256SUMS.txt`
- [x] `docs/tokens.md` ledger updated by CI (CI-owned; hand commits discarded)
- [ ] Guide §12.1 release gate: signed artefacts — `SHA256SUMS.txt.asc` still
      absent (audit F3: GPG step silently skips when `GPG_PRIVATE_KEY` unset);
      release notes for 0.11.0 added in `803b590`

Readiness % (v0.11.0): **95% — released; signing gate open (audit F3).**

## v0.11.1 Status — RELEASED AS 0.12.1

**Phase A continued: Snapping + Selection + Command Palette**, continuing the
2026-11-08 elite CAD reports. No `v0.11.1` tag ever existed; this content was
published under tag `v0.12.1`.

- [x] CAP-094 — Enhanced entity snapping (endpoint, midpoint, intersection,
      perpendicular, tangent, quadrant, grid, ortho) with priority ordering
- [x] CAP-095 — Window / crossing / fence selection modes
- [x] CAP-096 — Selection filter by entity type and layer
- [x] CAP-097 — Command palette (Ctrl+K, fuzzy match, recent commands)
- [x] CAP-098 — Snap visual indicators on canvas
- [x] Tests: snap candidate generation and priority ordering in `snap.rs`;
      window/crossing/fence, add/remove sets and type/layer filters in
      `selection.rs`
- [x] Release prep: both `Cargo.toml` files bumped to 0.12.1 in commit
      `803b590` (tag originally shipped with a stale 0.10.1 stamp — audit F1)
- [x] CI green, tag `v0.12.1` on `main` (commit `803b590`), GitHub Release
      published with all assets and `SHA256SUMS.txt`
- [x] Guide §12.1 gate: `docs/tokens.md` ledger updated by CI (30,000
      cumulative after the 2026-11-10 rebuild runs)
- [x] Signing gate (audit F3) — capability in place 2026-11-11: repo secret
      `GPG_PRIVATE_KEY` set (project key `09960C90…E442D0FE`, `Lintel Release
      Signing <dev@example.com>`); public key + verification guide committed in
      `6fdcc7d` (`resources/lintel-release-signing-public.asc`,
      `docs/release-verification.md`). Applies to releases built from now on;
      backfilling signatures onto the already-published 0.11.0/0.12.1 artefacts
      was declined — re-running the workflow would inflate the CI token ledger
      further (audit F8). First signed release: v0.13.0.

Phase A foundation already on the branch (core):

- [x] `layer.rs`: `Layer`, protected default layer and per-entity assignment
      with serde-default migration
- [x] `construction.rs`: line/ray/circle/arc construction geometry, snap-enabled
      and never printed
- [x] `status_bar.rs`: cursor coordinates, snap/ortho/grid chips, selection
      count and zoom %
- [ ] UI wiring for the foundation: layers palette and construction tools
      (→ v0.13.0 CAP-108)
- [x] `snap.rs` candidate foundation (midpoint/intersection/perpendicular source
      and priority order) — folded into the CAP-094 enhanced-snapping work

Readiness % (v0.11.1): **100% of the Phase A continued slice (CAP-094–CAP-098)
— published as 0.12.1, rebuilt 2026-11-10 (runs `42902103302` / `42902103890`).**

## v0.13.0 Status — RELEASED

**Anchored Dimensions & Release Governance** (CAP-100–CAP-108). Full scope in
`ROADMAP.md`.

- [x] CAP-100 — Constraint-driven dimensions core: `DimensionAnchor`,
      `start_anchor`/`end_anchor`/`witness_gap`/`witness_overrun`, anchor
      resolution, backward-compatible JSON load — PR #22 merged (`2b07f58`),
      CI run green (check + 3-platform build-matrix), 24 new tests
- [x] CAP-101 — Constraint solver: `Constraint` enum, `solve_constraints`,
      `apply_constraints_to_anchors` (now also updates the stored fallback
      point) — PR #22
- [x] CAP-102 — Anchor commands: `SetDimensionStartAnchor`/`EndAnchor`,
      `SetDimensionWitnessGap`/`Overrun` with validation
      (`DimensionNotFound`, param-in-[0,1], margins ≥ 0); **undo/redo tests**
      added: `a_dimension_start_anchor_can_be_undone_and_redone`,
      `a_dimension_end_anchor_can_be_undone_and_redone`,
      `witness_margins_can_be_undone_and_redone`,
      `an_added_constraint_can_be_undone_and_redone`,
      `a_added_constraint_can_be_redone`,
      `remove_a_constraint_can_be_undone_and_redone` — T-105
- [x] CAP-103 — Anchor UI: inspector fields, click-to-place witness lines,
      grip editing (renderer adopts `dimension_line_at`/`witness_line` +
      active level plumbing) — PR #23 merged (`7d88830`), CI green; inspector
      anchor rows + witness margin controls, canvas witness lines + cobalt
      anchor grips; canvas click-to-place deferred to CAP-108 as noted
- [x] CAP-104 — Update verification: `update-check` gains asset/digest
      metadata (`ReleaseAsset`, `checksum_asset`, `signature_asset`,
      `sha256`), app honours the cache TTL (single hoisted `UpdateChecker`),
      and the update copy states plainly that Lintel never auto-installs and
      the user verifies against `SHA256SUMS.txt`/`.asc` — PR #24 merged
      (`61cf97a`); `update-check` is a git submodule (`lintel-shared-crates`
      @ `4f7045f`)
- [x] CAP-105 — In-app release notes parsed at runtime from
      `include_str!("../../../RELEASE_NOTES.md")` — single source of truth,
      drift impossible; ledger-era figures surfaced, pre-ledger releases
      marked "predates the token ledger"; PR #26 merged
- [x] CAP-106 — Release governance: `release.yml` now fails when the tag does
      not match both crate versions, refuses to publish unsigned (and
      self-verifies the signature), anchors releases to the real git tag with
      `--verify-tag` (single `v`-convention), and warns on the default
      `RELEASE_TOKENS`; `ci.yml` gains a version-consistency gate — PR #25
      merged (`d248b8b`)
- [x] CAP-107 — Documentation parity: capability-inventory through CAP-099,
      parity rows, CHANGELOG/ROADMAP/PROGRESS reconciled (this slice)
- 2026-11-13: CAP-107 complete — capability-inventory, parity, traceability,
  architecture, decisions, README and implementation plan reconciled to published
  tags; v0.13.0 release gate remains.
- [x] CAP-108 — Phase A UI completion: Layers panel in the inspector
      (visible/locked toggles, honest empty state), four construction tools
      (line/ray/circle/arc) with two-click canvas flows, command-palette
      entries for every construction action, wired shortcuts (C/Shift+C/Q/X,
      Shift+V toggles selected-construction visibility, Del deletes), and
      canvas click-to-place dimension anchors — PR #27 merged

**Landed:** the core slice merged to `main` at `b4a7df1` (merge of PR #22,
commit `2b07f58`, 2026-11-11); anchor UI `7d88830` (PR #23); update
verification `61cf97a` (PR #24); release governance `d248b8b` (PR #25);
release-notes sync `1a43bea` (PR #26); Phase A UI completion `e86f697`
(PR #27); documentation parity `4a590fd` (PR #28). Constraint persistence on
`Level` is still an open design point (solver takes an explicit
`&[Constraint]`) — next planning target alongside CAP-099 sheet layout.

Readiness % (v0.13.0): **100% — CAP-100–108 all landed; release gate executed
(both crates at 0.13.0, notes quoting the ledger, tag `v0.13.0` cut after
merge — first signed release; all 8 artefacts including
      `SHA256SUMS.txt.asc`, workflow now uploads the signature (PR #30,
      `76a774b`)).**

**2026-11-25: T-106 Core Headless Tests CI fix (PR #73)** — Fixed remaining CI
infrastructure issues for v0.13.0 test suite: fixed `highest_id()` method on
`Project` for proper ID tracking on load, corrected constraint.rs imports
(`crate::dimension` vs `crate::dim`), resolved test formatting per rustfmt,
and fixed two failing tests (`dimension_offsets_must_be_positive` used wrong
offset value, `project_with_anchored_dimensions_and_constraints_round_trips`
expected pre-solve anchor values). All 442 tests now pass; all required CI
checks green (check, contrast-test, kittest, build-matrix ×3, pr-author-is-dev).

**2026-11-25: T-207 Inspector empty states (PR #81)** — Improved empty-state
messages in the Inspector for Constraints, Viewports, Block Definitions, and
Block Instances sections. All empty states now include an actionable description
and (where applicable) a primary action button to add the relevant entity. CI
green on all required checks (check, contrast-test, kittest, build-matrix ×3,
pr-author-is-dev).

**2026-11-26: T-202 + T-203 constraint tests + T-206 solver perf (PR #83)** —
Added seven headless tests to `constraint.rs`: three 2-node EqualLength cycle
stress tests (single pair, five independent cycles, mixed with
PerpendicularDistance/FixedAngle) and four anchor + constraint round-trip tests
(PerpendicularDistance, FixedAngle, EqualLength, mixed). Optimized
`solve_constraints` with a `HashMap` wall lookup (`resolve_on_wall` on
`DimensionAnchor`) so 500 constraints resolve well under the 50 ms budget.
`Project` import hoisted to the test module; clippy-clean. CI green on all
required checks (check, contrast-test, kittest, build-matrix ×3,
pr-author-is-dev).

## v0.14.0 Status — RELEASED

**Constraint Persistence & Sheet Layout** (CAP-111–115). Full scope in
`ROADMAP.md`.

- [x] CAP-111 — Constraint persistence: `constraints: Vec<Constraint>` on
      `Level` (serde default), `Project::solve_constraints()` re-solves after
      every command and on file load, `Project::validate` rejects dangling
      walls/dimensions, out-of-range params, self-equal-length and duplicate
      pins — PR #31 merged; old project files load with an empty list

- [x] CAP-112 — Constraint management UI: Inspector "Constraints" panel
      (list + Remove per row, add-form with kind/dimension/wall/param/degrees,
      honest empty state); core `AddConstraint`/`RemoveConstraint` commands
      with a shared validator now living in `constraint.rs` — PR #32 merged
      (`ef21549`); palette entry skipped (inspector is the discoverable home)

- [x] CAP-113 — Sheet title blocks: `TitleBlock` (company, project, sheet
      title, revision, date, drawn-by, sheet-number base), rendered as the
      bottom-right strip on every exported sheet, `SetTitleBlock` command,
      old files load with defaults — PR #33 merged (`cb9f5cf`)
- [x] CAP-114 — Sheet viewports: `Viewport` (level-bound region + 1:N scale +
      caption), multiple per level, framed and scale-fit in the PDF,
      `AddViewport`/`RemoveViewport` commands, validate rejects dangling
      levels/non-positive scale/off-sheet regions — PR #33
      (simplification: walls straddling a viewport edge draw whole, not
      clipped)

- [x] CAP-115 — QC codified: `docs/qc/release-checklist.md` — six sections
      covering pipeline gates, signed-artefact verification (executable
      commands), AppImage smoke, screen-reader pass, ledger consistency and
      post-release duties, each with sign-off lines

- [x] Release gate: both crates bumped 0.13.0 → 0.14.0; `RELEASE_NOTES.md`
      0.14.0 section; CHANGELOG dated; CI green; tag `v0.14.0` cut — release
      run `43529660411` green, all 8 artefacts published **including
      `SHA256SUMS.txt.asc` signed by the pipeline on the first pass**;
      ledger at 50,000 cumulative

**Landed:** all five capabilities (CAP-111–115) and the release.

Readiness % (v0.14.0): **100% — shipped:**
https://github.com/Perchemop/lintel-desktop/releases/tag/v0.14.0

## v0.15.0 Status — RELEASED

**Blocks & Symbols, Print Polish** (CAP-116–119). All four capabilities landed and the release gate executed.

- [x] CAP-116 — Block definition core: `BlockDef`/`BlockId`/`BlockInstance` types, `AddBlockDef`/`RemoveBlockDef`/`AddBlockInstance`/`RemoveBlockInstance` commands with validation, undo/redo, JSON round-trip — PR #37 merged, CI green (check + 3-platform build-matrix), 10 new tests
- [x] CAP-117 — Block instance UI: `Tool::Block` click-to-place on canvas, inspector "Blocks" section (def listing + instance fields) — PR #38 merged, CI green
- [x] CAP-118 — Viewport print polish: real PDF clipping path, viewport-anchored north arrow + scale bar, placement guards — PR #39 merged, CI green, 9 new tests
- [x] CAP-119 — Constraint palette entries + solver readout in inspector — PR #40 merged, CI green

- [x] Release gate: both crates bumped 0.14.0 → 0.15.0; `RELEASE_NOTES.md` 0.15.0 section quoting ledger; CHANGELOG dated; CI green; tag `v0.15.0` cut

**Landed:** all four capabilities (CAP-116–119) and the release.

Readiness % (v0.15.0): **100% — shipped:**
https://github.com/Perchemop/lintel-desktop/releases/tag/v0.15.0

## v0.15.1 Status — RELEASED

**macOS Installer Fix.** Patch release: macOS now ships a signed `.dmg` installer alongside the `.zip` archive, completing the installable-artefacts requirement for every platform. No feature changes from 0.15.0.

- [x] Release pipeline builds and attaches `lintel-macos-aarch64.dmg` for macOS — release.yml updated (PR #42 merged, CI green)
- [x] Both crates bumped 0.15.0 → 0.15.1; `RELEASE_NOTES.md` 0.15.1 section quoting ledger; CHANGELOG dated; CI green; tag `v0.15.1` cut

**Landed:** macOS .dmg installer fix.

Readiness % (v0.15.1): **100% — shipped:**
https://github.com/Perchemop/lintel-desktop/releases/tag/v0.15.1

## v0.20.1 Status — RELEASE PREP

**Patch release: door & window placement fixes (PRs #68, #70) that merged
after the v0.20.0 tag.** Fixes are regression-tested and merged green; this
release makes them installable.

- [x] Fixes landed on `main`: PR #68 (placement bugs: centreline projection,
      `note_status` error surfacing, empty-canvas hint) and PR #70 (opening
      placement projected onto the wall centreline,
      `wall_offset_along_centerline` + regression test in
      `apps/lintel/src/tests/workflows.rs`) — both merged with CI green
      (check, kittest, contrast-test, build-matrix ×3, pr-author-is-dev)
- [x] Drift fix: CHANGELOG `Unreleased` content that shipped in v0.20.0
      renamed to `## v0.20.0 — 2026-11-22`; v0.20.1 section added
- [x] Version bumps 0.20.0 → 0.20.1 (both crates + Cargo.lock)
- [x] RELEASE_NOTES.md drafted for v0.20.1; first-reader gate: `lost: 0`
- [x] scope-creep-check: `scope: clean` (docs + version bumps only)
- [ ] Tag `v0.20.1` after this change merges, then confirm the release run,

## v0.22.0 Status — RELEASE PREP

**Workbench shell & theme release**, v0.22.0 adds the workbench shell with
activity bar, dockable panels (Inspector, Layers, Constraints, Output with
visibility toggles T-226/T-230/T-238/T-239/T-240), status-bar theme selector
(light/dark/high-contrast T-224), contextual per-tool status-bar hints
T-230, runtime themes Light/Dark/High-Contrast T-224, settings persistence
(theme mode + panel visibility) in `lintel-settings.json` via serde/serde_json
T-225, SPEC→spec() migration enabling runtime theme switching.

CI evidence: PR #90 merge commit `e6020a2` (`check` + `build-matrix` on
Linux/macOS/Windows), run ID 46141818194 (all checks green: check,
contrast-test, kittest, pr-author-is-dev, build-matrix).

- [x] Version bumps 0.21.0 → 0.22.0 (both crates + Cargo.lock)
- [x] CHANGELOG.md drafted for v0.22.0; first-reader gate: `lost: 0`
- [x] RELEASE_NOTES.md drafted for v0.22.0; first-reader gate: `lost: 0`
- [x] T-220..T-240 all implemented and verified green in CI
- [x] ROADMAP.md updated with v0.22.0 scope and CI run ID evidence
- [x] PROGRESS.md drafted for v0.22.0; first-reader gate: `lost: 0`
- [ ] Tag `v0.22.0` after this change merges, then confirm the release run, installers and signed `SHA256SUMS.txt.asc` (dependency-doctor gate before the tag)

## v0.20.0 Status — RELEASED

**Design-system extensions (spec `docs/specs/v0.20.0-design-system-extensions.spec.md`).**

- [x] T-006 — ThemeSpec three-tier restructure (domain-dev session, 2026-11-21):
      `theme.rs` carries explicit tier banners (`=== TIER 1: BASE PALETTE ===`,
      `=== TIER 2: SEMANTIC ROLES ===`, `=== TIER 3: COMPONENT TOKENS ===`) and
      a `ThemeSpec { base, semantic, component }` grouping (`ThemeSpec::LIGHT`,
      plus `SPEC` as the one handle widget code reads). Component tokens added:
      `WALL_PENDING` (replaces the dead `WALL_DRAFT`), `CANVAS_SHEET`,
      `CANVAS_TEXT`, `CANVAS_CONSTRUCTION`, `STROKE_NONE`,
      `SELECTION_HIGHLIGHT/WINDOW/CROSSING`, `DIMENSION_STROKE`,
      `BLOCK_INSTANCE_STROKE`, `PANEL_SURFACE`, `STATUS_SURFACE`,
      `PANEL_BORDER`, `BODY_TEXT`, `HINT_TEXT`, `MUTED_TEXT`, `SUCCESS_TEXT`,
      `ERROR_TEXT`, `DANGER_TEXT`, `PALETTE_UNSELECTED_FILL`, `BUTTON_RING`;
      semantic roles `TEXT_ERROR` (split from the old destructive-text role)
      and `TEXT_WARNING` (on T-007's `BASE_WARNING` swatch). Widget code
      (canvas, dimension_ui, status_bar, inspector, command_palette, about,
      app/shell, layout/helpers, components/forms) now reads
      `SPEC.component.<group>.<token>` — no direct tier-1/2 colour reads and
      no raw `Color32` outside `theme.rs` (grep gate clean).
- [x] T-006 — structural tests `apps/lintel/src/tests/theme_tiers.rs`:
      `ThemeSpec::LIGHT` mirrors the flat registry field-by-field, tier-1
      swatches pinned byte-identical (incl. alpha channels), every semantic
      role resolves to a base swatch, every component token (and every button
      state field) resolves to a semantic role, and the button group covers
      all 12 role × size variants with the 24pt floors.
- [x] T-002 — form components `apps/lintel/src/components/forms.rs`
      (T-008 session filled the gap after the component landed with its
      in-module tests): `Field` (label + input + error/help, `labelled_by`
      association, disabled-reason tooltip, painted error icon + 2px danger
      stroke), `FieldSet` (framed legend, collapsible) and `FieldGroup`;
      all colours/spacing from `SPEC.component.field` (D-035 pairing:
      `BORDER_STRONG` boundary, `TEXT_ERROR` messages). Fixed against egui
      0.36.2: `Response` has no `Default` (zero-size hover allocation
      instead) and the in-module harness uses `Context::run_ui` (0.36
      removed `Context::run`/`begin_frame`).
- [x] T-003 — layout helpers rewritten in
      `apps/lintel/src/layout/helpers.rs`: every helper (`vstack`, `hstack`,
      `wrap_row`, `card`, `card_compact`, `tool_group`) returns its
      `InnerResponse<R>` so callers can compose responses; `Frame::none()`
      (nonexistent in 0.36.2) → `Frame::new()`; `card` gained the
      `SHADOW_LEVEL_1` elevation the ticket required; `wrap_row` clamps to
      the available width (checked, never trusted).
- [x] T-008 — rendered evidence: env-gated `evidence_*` tests in
      `apps/lintel/src/tests/evidence.rs` render
      `docs/qc/buttons-evidence/{buttons-light,dark,states,focus}.png`,
      `docs/qc/form-evidence/{form-light,dark,error}.png` and
      `docs/qc/layout-evidence/{layout-narrow-400,wide-1200,cards}.png`
      headlessly through the egui_kittest wgpu renderer; the `kittest` CI
      job runs them (`LINTEL_GENERATE_EVIDENCE=1 cargo test -p lintel --bin
      lintel evidence_`) and uploads the folders as the `design-evidence`
      artefact. PNGs are never committed (`.gitignore` + II.30.8, no pixel
      baselines); each folder's README documents provenance and contents.
- [x] T-008 — always-on egui_kittest assertions:
      `tests/kittest_forms.rs` (inputs carry labelled-by relations, error
      and help texts are announced as label nodes, Tab order field → field
      → submit by value identity) and
      `tests/evidence.rs::kittest_layout_resizes_without_overlap_or_clipping`
      (400 → 1200 px: every exposed button ≥ 24 pt, no two button rects
      intersect).
- [x] T-008 — docs: `docs/design/gui-stack.md` (II.30.2 stack record:
      egui/eframe 0.36.2 resolved, three-tier tokens, component
      architecture, evidence pipeline, 0.36 API breaks), evidence-folder
      READMEs, version bumps 0.19.0 → 0.20.0 (both crates + Cargo.lock
      entries), DECISIONS D-036.
- [x] Verification: the slice landed as PRs #69/#70 after the tag, each green
      on every required check (pr-author-is-dev, check, contrast-test,
      kittest, build-matrix ubuntu/macos/windows); tag `v0.20.0` published
      2026-11-22 with installers (setup.exe, dmg, AppImage, zips) and signed
      `SHA256SUMS.txt.asc`.

## v0.19.0 Status — RELEASE PREP

**Snaps, Sheets & Full Keyboard Reach.** All five items merged to `main` with
CI green. Version bumps are staged in the working tree; the tag and release
are the remaining gate.

- [x] SNAP_* semantic tokens — five snap-marker colours (Midpoint,
      Perpendicular, Tangent, Ortho, Grid) moved from raw `Color32` in
      `canvas.rs` into tier-1 `BASE_SNAP_*` + tier-2 `SNAP_*` roles; tier-3
      `snap_marker_colour(kind)` covers all eight snap kinds;
      `CONSTRUCTION_STROKE` added as a token and no raw `Color32` remains in
      `canvas.rs` — PR #59 merged, CI green
- [x] CONSTRUCTION_STROKE contrast fix (guide II.30.7.1) — darkened
      (168,148,116) → (140,120,90): 2.77:1 → 4.0:1 on PAPER (was below the
      3:1 UI-graphics floor); 3:1 assertion added to the contrast gate —
      PR #59 merged, CI green
- [x] D-032 command bindings — Inspector "Title block" section (company/sheet
      title/revision/date/drawn-by → `SetTitleBlock`), "Viewports" section
      (list with Remove → `RemoveViewport`; Add form with level id, scale
      denominator and caption, default A4 landscape region → `AddViewport`)
      and "Add definition" form in Blocks (name → `AddBlockDef`, ids assigned
      UI-side as max+1); D-032 updated: only `MoveDimension` remains core-only
      by design (D-026), D-027 reachability fully satisfied — PR #60 merged,
      CI green
- [x] F7/F8 snap aliases (QCAD/AutoCAD convention from approved research) —
      F7 toggles grid and F8 toggles ortho alongside G/O;
      `docs/qc/shortcuts.md` updated — PR #61 merged, CI green
- [x] Keyboard block-instance selection — new "Select block instance" palette
      action (Ctrl+K) with a pick form listing every active-level instance
      (raw id + definition name + position); Select sets the instance and
      clears entity selection as canvas-click does; the last pointer-only
      a11y flow is gone and the `ACCESSIBILITY.md` limitation was replaced
      with the keyboard path — PR #61 merged, CI green
- [ ] Tag `v0.19.0` after this change merges, then confirm the release run
      and the signed artefacts

**Landed:** snap-marker tokens and the construction-stroke contrast fix
(PR #59); D-032 Inspector bindings (PR #60); F7/F8 aliases and keyboard
block-instance selection (PR #61).

Readiness % (v0.19.0): **100% of the v0.19.0 scope — merged with CI green;
release tag pending after this change.**

## v0.18.0 Status — RELEASED

**The Design System.** Post-v0.17.0 a11y/design audit fixes, all merged to
`main` with CI green. Version bumps are staged in the working tree; the tag
and release are the remaining gate.

- [x] WCAG AA contrast gate (guide II.30.7.1) — `relative_luminance` /
      `contrast_ratio` and the `theme_colours_meet_wcag_aa` merge-gate test;
      darkened palette roles, raw `Color32` in status_bar/command_palette/
      main.rs toolbar moved into named theme roles — PR #53 merged, CI green
- [x] CAP-127 — delete a single block instance: `Command::DeleteBlockInstance
      { block_instance_id }`, `BlockInstanceNotFound` refusal, status-bar
      report and three domain tests; keyboard truth (plain Z/Y, R, Ctrl+U,
      truthful palette hints, `palette_shortcuts_are_unique`, dead "View
      Constraints" entry removed, matched-of-total footer) — PR #50 merged,
      CI green
- [x] `egui_kittest` CI job — AccessKit + snapshot tests
      (`palette_has_named_actions`, `inspector_sections_labelled`,
      `canvas_summary_accessible`), `kittest` added to `.ci-required.txt`;
      egui-elegance button patterns (`ButtonRole`/`ButtonSize`/
      `ButtonRoleStyle`, 12 constants, 24/32/40 pt floors, 2 px COBALT
      focus ring) applied to inspector, command palette and status bar — PR
      #55 merged, CI green (runs `44255343149`, `44263943707`)
- [x] ThemeSpec three-tier composition — 24 `BASE_*` → 31 semantic roles →
      component tokens (`WALL_*`, `PALETTE_*`, `BUTTON_*`), every raw literal
      in tier 1, three new tests, byte-identical values — PR #57 merged,
      CI green (commit `db0f3733`)
- [x] Ledger migration — `docs/tokens.md` replaces `docs/tokens.json`,
      release workflow appends the new row after publishing;
      `pr-author-is-dev` required check, Dev identity asserted by `ci.sh`,
      Dev account recorded as D-031 — PR #52 merged, CI green; D-032
      (unbound commands are core-only) — PR #56 merged, CI green
- [x] Tag `v0.18.0` after this change merges, then confirm the release run
      and the signed artefacts

**Landed:** WCAG AA contrast gate (PR #53); CAP-127, keyboard truth and
accessibility corrections (PR #50); `egui_kittest` gate and button patterns
(PR #55, runs `44255343149` / `44263943707`); ThemeSpec three-tier tokens
(PR #57, `db0f3733`); token-ledger migration and authorship enforcement
(PR #52); D-032 (PR #56).

Readiness % (v0.18.0): **100% of the design-system scope — merged with CI
green; release tag pending after this change.**

## v0.17.0 Status — RELEASED

**Dimension Label Precision, Keyboard Truth & the Contrast Gate** (CAP-126–
CAP-128). All scope merged to `main`; tag `v0.17.0` pushed and release
published with signed artefacts.

- [x] CAP-126 — Per-dimension label precision: `Dimension.label_precision`
      (0–4 decimal places, serde default 0 so old files load unchanged),
      `Command::SetDimensionPrecision` with domain validation (rejected values
      leave the dimension untouched), Inspector "Precision:" control and a
      canvas/PDF label honouring the setting — PR #48 merged, CI green
      (run `43847513509`)
- [x] CAP-127 — Delete a single block instance: `Command::DeleteBlockInstance
      { block_instance_id }` with a `BlockInstanceNotFound` refusal,
      `delete_selection` deletes the selected canvas instance and the refusal
      is reported in the status bar; three domain tests — PR #50 merged,
      CI green
- [x] Keyboard truth — plain Z/Y undo-redo (plus Ctrl+Z/Ctrl+Y), R opens release
      notes, Ctrl+U checks for updates; palette shortcut hints wired or
      honestly blank (`palette_shortcuts_are_unique`), dead "View Constraints"
      entry removed, footer shows matched of total — PR #50 merged
- [x] Accessibility statement corrections — undo/redo keys, softened focus
      claim, block-instance delete, pointer-only instance selection limitation
      and Inspector section list in `ACCESSIBILITY.md`; `docs/qc/shortcuts.md`
      created (guide II.30.7.3) — PR #50 merged
- [x] WCAG AA contrast gate (guide II.30.7.1) — `relative_luminance` /
      `contrast_ratio` and the `theme_colours_meet_wcag_aa` merge-gate test;
      the darkened palette roles (TEXT_MUTED, HINT, STATUS_OK,
      PALETTE_DISABLED, PALETTE_RECENT, PALETTE_CATEGORY) and raw `Color32`
      values moved into named theme roles — PR #53 merged, CI green
- [x] Ledger migration — `docs/tokens.md` replaces `docs/tokens.json`
      (19-row reconstructed table), release workflow appends the new row after
      publishing; `pr-author-is-dev` required check and Dev account recorded
      as D-031 — PR #52 merged, CI green
- [x] Tag `v0.17.0` pushed; release published with signed `SHA256SUMS.txt` +
      `.asc`; ledger row appended by CI workflow

**Landed:** CAP-126 (PR #48, run `43847513509`, merge commit `e5b808d`);
CAP-127, keyboard truth and accessibility corrections (PR #50); ledger
migration and authorship enforcement (PR #52); WCAG AA contrast gate (PR #53);
egui_kittest CI gate + button patterns (PR #55, merge commit `dba868a`).

Readiness % (v0.17.0): **100% — shipped.**

## v0.16.0 Status — RELEASE PREP

**Block Instance Editing & Canvas Selection** (CAP-120–125). All six capabilities landed and are merged to `main`; the release gate (notes, changelog, version bumps, tag) is this change.

- [x] CAP-120 — Move block instances: `Command::MoveBlockInstance` +
      `Level::move_block_instance` with finite-position validation, undo/redo and
      JSON round-trip — PR #45 merged, CI green (run `43708368391`)
- [x] CAP-121 — Rotate block instances: `Command::RotateBlockInstance` +
      `Level::rotate_block_instance` with finite-rotation validation — PR #45
      merged, CI green (run `43708368391`)
- [x] CAP-122 — Flip block instances: `Command::FlipBlockInstance` +
      `BlockInstance::toggle_flip` — PR #45 merged, CI green (run `43708368391`)
- [x] CAP-123 — Canvas hit-testing and click-to-select:
      `Level::hit_test_block_instance` (transformed definition bounds, topmost
      instance wins, non-finite point rejected), `BlockDef::local_points` +
      `BlockInstance::transform_point`; `Tool::Select` prefers instances, red
      outline, Esc clears — PR #46 (`74e02c7`), CI green (run `43822843607`)
- [x] CAP-124 — Editable instance fields in the Inspector: "Block instance"
      section (X/Y position, rotation degrees, Flipped checkbox) applying
      `MoveBlockInstance`/`RotateBlockInstance`/`FlipBlockInstance`, honest
      empty states — PR #46 (run `43822843607`)
- [x] CAP-125 — `Applied.block_instance` auto-selects the affected instance
      after add/move/rotate/flip — PR #46 (run `43822843607`)
- [x] Release gate prep: both crates bumped 0.15.1 → 0.16.0; `RELEASE_NOTES.md`
      0.16.0 section quoting the ledger; CHANGELOG dated; inventory, parity and
      traceability extended through CAP-125
- [ ] Tag `v0.16.0` after this change merges, then confirm the release run and
      the signed artefacts

**Landed:** all six capabilities (CAP-120–125), merged in PR #45 (run
`43708368391`) and PR #46 (run `43822843607`, merge commit `d318a96`). Resolves
both limitations published in v0.15.0: read-only instance fields and no canvas
hit-testing.

Readiness % (v0.16.0): **100% of CAP-120–125 — merged; release tag pending
after this change.**

## v0.10.1 Status — RELEASED

- [x] Window icon embedded via `with_icon()` + `image` crate decode
- [x] `include_bytes!` path fixed to three levels to repo root
- [x] `resources/icons/lintel.svg` created as canonical source
- [x] Windows `.iss` version bumped 0.8.0 → 0.10.1
- [x] `ATTRIBUTION.md` icon attribution added
- [x] Command-layer fuzz tests added (3 proptest cases in `crates/lintel-core/tests/properties.rs`)
- [x] CI green (PR #16 audit fixes merged, PR #17 v0.10.1 bump)
- [x] v0.10.1 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.1 (release run `42372245530`; all seven assets present, `SHA256SUMS.txt` verified)
- [x] Token ledger `docs/tokens.md` updated by CI; cumulative 10,000 tokens (ledger accumulation bug fixed)

Readiness % (v0.10.1): **100% — shipped.**

## v0.10.0 Status — RELEASED

- [x] Suite-standard conformance: `docs/tokens.md` CI-managed ledger, release signing step added to `release.yml`, `docs/development.md` added
- [x] Property-based geometry and command-layer fuzz tests added (`crates/lintel-core/tests/properties.rs`)
- [x] Version UI moved to bottom-right; Release Notes popup gains Update button
- [x] Window icon embedded via `include_bytes!` + `image` crate
- [x] CI green, PR merged, tag `v0.10.0` pushed, release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.0

Readiness % (v0.10.0): **100% — shipped.**

## v0.9.0 Status — RELEASED

- [x] Drawing Set shipped: auto-detected rooms, free text plan labels, opening schedule, multi-sheet PDF with cover, north arrow + scale bar
- [x] Room detection orientation bug fixed; non-positive walks dropped
- [x] `LabelNotFound` error message normalized to `"{0} does not exist"`
- [x] Room area captions, label rendering, `Tool::Label` L key, drag-move, Delete, Ctrl+D duplicate, Ctrl+Shift+E Export drawing set
- [x] `PlanGeometry.labels/rooms` wired, IDs/error/command/join plumbing complete
- [x] CI green, PR `opencode/v0.9.0-rooms` merged, tag `v0.9.0` pushed, release published

Readiness % (v0.9.0): **100% — shipped.**

## v0.8.0 Status — RELEASED

- [x] Focus review found a regression: tabbing to the canvas disabled every shortcut
- [x] Shortcuts now apply when nothing is focused **or** the canvas is focused; still suppressed in text fields
- [x] Unit tests for `plan_summary` (empty, singular, multi-storey) — verified in CI
- [x] `ACCESSIBILITY.md` records the focus-review outcome
- [x] CI green (PR #11 run `42294376007`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.8.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.8.0
      (release run `42295310359`; all six assets present)

Readiness % (v0.8.0): **100% — shipped.**

### Still planned (v0.9.0)

- [ ] Screen-reader output spot-check on Windows/macOS (manual)
- [ ] Linux AppImage execution check on an x86_64 host

## v0.7.0 Status — RELEASED

- [x] Audit: grepped the UI for every `Command` variant; found 4 unreachable capabilities
- [x] Inspector → Project: rename the project (`RenameProject`) and switch Metric/Imperial (`SetUnits`)
- [x] Inspector → Storeys: delete a storey (`DeleteLevel`, disabled for the last) and add one (`AddLevel`)
- [x] Dragging a selected dimension changes its offset (`SetDimensionOffset`)
- [x] Fmt/clippy-safe: removed the now-unused `LevelId` import; rustfmt clean
- [x] CI green (PR #10 run `42211261713`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.7.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.7.0
      (release run `42212698044`; all six assets present)

Readiness % (v0.7.0): **100% — shipped.**

### Known, deliberate exception

`MoveDimension` (move a dimension's measured points) stays core-only: dragging
binds to the offset instead, so a dimension never detaches from what it
measures. Recorded in DECISIONS.md D-026.

### Still planned (v0.8.0)

- [ ] Screen-reader output spot-check on Windows/macOS (manual)
- [ ] Linux AppImage execution check on an x86_64 host
- [ ] Keyboard-focus management review

## v0.6.0 Status — RELEASED

- [x] File keyboard shortcuts: Ctrl+N/O/S/Shift+S/E
- [x] Focusable canvas with a screen-reader label naming the active level and its contents
- [x] Level summary shown in the toolbar
- [x] Plain tool shortcuts guarded so Ctrl+S cannot also switch tools
- [x] **Fixed:** PDF export dialog was unreachable (nothing set `export_open`); now File → Export sheet… and Ctrl+E
- [x] Linux AppImage statically verified (ELF + SquashFS magic + SHA-256); execution needs an x86_64 host
- [x] `ACCESSIBILITY.md` and `docs/qc/checklist.md` updated
- [x] CI green (PR #9 run `42192427679`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.6.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.6.0
      (release run `42193835276`; all six assets present)

Readiness % (v0.6.0): **100% — shipped.**

### Still planned (v0.7.0)

- [ ] Screen-reader output spot-check on Windows/macOS (manual)
- [ ] Linux AppImage execution check on an x86_64 host
- [ ] Keyboard-focus management review

## v0.5.0 Status — RELEASED

- [x] Windows installer step restored using Chocolatey (`choco install innosetup`)
- [x] `.iss` `DefaultDirName` typo fixed (`{autpf}` → `{autopf}`)
- [x] `.iss` AppId literal brace escaped; `SetupIconFile` uses a real Windows `.ico`
- [x] `.iss` version bumped to 0.5.0; app + core bumped to 0.5.0
- [x] `*.exe` re-added to the release collect pattern so the installer ships with checksums
- [x] CI green (PR #8 run `42099183213`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.5.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.5.0
      (release run `42103705669`; `lintel-windows-x86_64-setup.exe` verified as a valid PE,
      SHA256SUMS recomputed and matching)

Readiness % (v0.5.0): **100% — shipped.**

## v0.4.0 Status — RELEASED

- [x] Keyboard shortcut hints + hover labels on all controls (W=Wall S=Select D=Door N=Window M=Dimension · Z=Undo Y=Redo F=Zoom-to-fit Esc=Cancel Del=Delete)
- [x] Focus-ring contrast WCAG AA (COBALT verified ~8.3:1 on PAPER, ~7.8:1 on PANEL_BG)
- [x] Escape dismissal on all dialogs (About, Release notes, Update, Export sheet)
- [x] Linux AppImage release artefact (`lintel-appimage-x86_64.AppImage`, verified ELF)
- [x] ACCESSIBILITY.md (honest statement incl. canvas limitation) and docs/qc/checklist.md
- [x] CI green (PR #7 run `42013256872`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.4.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.4.0
      (release run `42024620827`; AppImage + SHA256SUMS verified)

Readiness % (v0.4.0): **100% — shipped.**

### Still planned (v0.5.0)

- [ ] Windows installer (MSI or setup .exe) — Inno Setup download hangs on CI; deferred
- [ ] Screen-reader labels beyond egui's built-in accessibility
- [ ] Keyboard-focus management review

## v0.3.0 Status — RELEASED

- [x] Rejected edits and file errors surfaced in the toolbar (with a Clear message item)
- [x] macOS release archive packages `Lintel.app` with a real `Info.plist`
- [x] Empty-state hint when a level has no walls (landed with the v0.2.0 polish)
- [x] CI green (PR #6 run `41998649019`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.3.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.3.0
      (release run `41999766800`; `Lintel.app` verified inside the published asset)

Readiness % (v0.3.0): **100% — shipped.**

## v0.2.0 Status — RELEASED

- [x] Dimension lines (aligned/horizontal/vertical) with numeric labels
- [x] Dimension place / move / offset / delete commands and Inspector panel
- [x] Dimension hit testing ahead of walls; save/load and undo/redo
- [x] Joint resolution for unequal wall thicknesses
- [x] Shared `update-check` crate (GitHub Releases API, fail-soft offline)
- [x] Shared `crash-report` crate (panic hook, scrubbed diagnostics, issue URL)
- [x] In-app release-notes window with per-release and cumulative token metrics
- [x] In-app update check with a non-intrusive "update available" prompt
- [x] Cross-platform CI build matrix (Linux, macOS, Windows)
- [x] Multi-platform release matrix (Linux x86_64/aarch64, macOS aarch64, Windows x86_64)
- [x] README rewritten for non-technical users
- [x] CI green (PR #4 run `41907396871`: `check` + `build-matrix` Linux/macOS/Windows)
- [x] v0.2.0 tag and GitHub Release published — https://github.com/Perchemop/lintel-desktop/releases/tag/0.2.0

Readiness % (v0.2.0): **100% — shipped.**

### Still planned (v0.2.x / v0.3.0)

- [x] Dimension kind selector + extension-line toggle (landed v0.2.x polish, PR #5)
- [x] Rotated dimension labels for vertical/aligned dimensions (landed v0.2.x polish, PR #5)
- [x] Label precision (CAP-126, v0.17.0 work — PR #48)
- [x] Installers (AppImage, signed Windows setup .exe, macOS .dmg — shipped since v0.15.1; MSI/.app bundle formats remain optional)
- [ ] Accessibility, empty states and keyboard-focus management

## v0.1.0 Status — RELEASED

- [x] Core domain implemented (`crates/lintel-core`) with headless workflow tests
- [x] GUI shell (`apps/lintel`) with eframe/egui UI wired to core commands
- [x] Documentation set (project-brief, scope, inspiration, architecture, capability-inventory, parity, …)
- [x] ROADMAP / CHANGELOG / DECISIONS / ATTRIBUTION maintained
- [x] `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- [x] CI green on push (`check` job)
- [x] v0.1.0 tag and release — https://github.com/Perchemop/lintel-desktop/releases/tag/0.1.0

Readiness % (v0.1.0): **100% — shipped.**

## Research

- [x] No large-scale research notes needed for v0.1.0–v0.10.x; public
      egui/eframe/PDF/ureq sources only
- [x] Elite CAD feasibility reports filed 2026-11-08:
      `ELITE_CAD_RESEARCH_SUMMARY.md`, `DOMAIN_INVESTIGATION_ELITE_CAD.md`,
      `DOMAIN_DESIGN_SKETCHES.md`, and `docs/research/elite-*.md`
- [x] `docs/research/` reserved for gated research before larger Tasks

## Process feedback

- No friction logged this session.