# Parity evidence

Capability ID → implementation evidence (test name, CI run). Updated after
each green CI run; percentages move only from evidence.

## v0.1.0 — shipped

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-001 … CAP-025 | `crates/lintel-core/tests/workflows.rs` + unit tests in each module | `41792206941` (PR #1), `41793150063` (main) | verified |

## v0.2.0 — joint resolution, dimensions, suite infrastructure

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-021 | `join.rs` unequal-thickness tests | `41796458537` (PR #2, main) | verified |
| CAP-026 … CAP-033 | `crates/lintel-core/tests/dimensions.rs` (6 headless tests) + `dimension.rs` unit tests | `41819328412` (main) | verified |
| CAP-040 | `crash-report` unit tests (scrub, URL, JSON round-trip) | `41907396871` (PR #4) | verified |
| CAP-041 | `update-check` unit tests (version compare, `pick_latest`, `evaluate`) | `41907396871` (PR #4) | verified |
| CAP-042 | In-app Release notes window (`apps/lintel/src/about.rs`) | `41907396871` (PR #4) | verified |
| CAP-043 | `build-matrix` job in `.github/workflows/ci.yml` (Linux/macOS/Windows) | `41907396871` (PR #4) | verified |
| CAP-044 | `release.yml` multi-target matrix | v0.2.0 release run | published |
| CAP-045 | README rewrite (guide §9) | `41907396871` (PR #4) | verified |
| CAP-046 | Empty-state hint on an empty level | `41992132501` (PR #5) | verified |
| CAP-047 | Rejected edits / file errors shown in the app | `41998649019` (PR #6) | verified |
| CAP-048 | macOS release ships a `Lintel.app` bundle | `41999766800` (v0.3.0 release) | verified |

## v0.4.0 — accessibility labels, shortcuts, AppImage

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-049 | Keyboard shortcut hints + hover labels on all controls | `42013256872` (PR #7) | verified |
| CAP-050 | Focus-ring contrast WCAG AA (COBALT) | `42013256872` (PR #7) | verified |
| CAP-051 | Linux AppImage release artefact | `42024620827` (v0.4.0 release) | verified |
| CAP-052 | Windows installer (Inno Setup via Chocolatey) | pending | awaiting run |
| CAP-053 | `.iss` DefaultDirName typo fixed | `42099183213` (PR #8) | verified |
| CAP-054 | `.iss` AppId brace + Windows `.ico` icon | `42103705669` (v0.5.0 release) | verified |

## v0.6.0 — keyboard-complete flows, accessible canvas

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-055 | File keyboard shortcuts (Ctrl+N/O/S/Shift+S/E) | `42192427679` (PR #9) | verified |
| CAP-056 | Focusable canvas + screen-reader level summary | `42192427679` (PR #9) | verified |
| CAP-057 | Toolbar level summary | `42192427679` (PR #9) | verified |
| CAP-058 | Tool shortcuts guarded against modifiers | `42192427679` (PR #9) | verified |
| CAP-059 | PDF export dialog reachable (bug fix) | `42192427679` (PR #9) | verified |
| CAP-060 | Linux AppImage static verification | `42193835276` (v0.6.0 release) | verified |

## v0.7.0 — reachable capabilities

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-061 | Add / delete a storey from the Inspector | `42211261713` (PR #10) | verified |
| CAP-062 | Rename the project from the Inspector | `42211261713` (PR #10) | verified |
| CAP-063 | Switch Metric / Imperial from the Inspector | `42211261713` (PR #10) | verified |
| CAP-064 | Drag a dimension to change its offset | `42211261713` (PR #10) | verified |
| CAP-065 | Command-reachability audit clean (only `MoveDimension` core-only, documented) | `42211261713` (PR #10) | verified |

## v0.8.0 — keyboard-focus review

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-066 | Shortcuts work while the canvas has focus | `42294376007` (PR #11) | verified |
| CAP-067 | Level summary unit-tested (3 cases) | `42294376007` (PR #11) | verified |
| CAP-068 | Focus-review outcome recorded | `42294376007` (PR #11) | verified |

## v0.9.0 — Drawing Set

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-069 | `crates/lintel-core/src/room.rs` planar face traversal | PR `opencode/v0.9.0-rooms` merged | verified |
| CAP-070 | `label.rs` + `PlaceLabel`/`SetLabelText`/`MoveLabel`/`DeleteLabel` | PR `opencode/v0.9.0-rooms` merged | verified |
| CAP-071 | `schedule.rs` door/window schedule rows | PR `opencode/v0.9.0-rooms` merged | verified |
| CAP-072 | `pdf.rs` `render_set`, north arrow, scale bar | PR `opencode/v0.9.0-rooms` merged | verified |
| CAP-073 | Room area captions on canvas | PR `opencode/v0.9.0-rooms` merged | verified |
| CAP-074 | `Tool::Label`, L key, Ctrl+D, Ctrl+Shift+E export | PR `opencode/v0.9.0-rooms` merged | verified |

## v0.10.0 — Suite Standards Conformance

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-075 | `docs/tokens.json` CI-managed ledger | release run `42365552699` | verified |
| CAP-076 | Release signing step in `release.yml` | release run `42365552699` | verified |
| CAP-077 | `docs/development.md` added | release run `42365552699` | verified |
| CAP-078 | Property-based geometry + command-layer fuzz tests | release run `42365552699` | verified |
| CAP-079 | Version UI bottom-right + Release Notes popup + Update button | release run `42365552699` | verified |
| CAP-080 | Window icon embedded | release run `42365552699` | verified |

## v0.10.1 — Window Icon & Installer Version Fix

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-081 | Window icon via `with_icon()` + `image` crate | PR #16/#17, release run `42372245530` | verified |
| CAP-082 | `resources/icons/lintel.svg` SVG source | PR #16/#17, release run `42372245530` | verified |
| CAP-083 | Windows `.iss` version corrected 0.8.0 → 0.10.1 | PR #16/#17, release run `42372245530` | verified |
| CAP-084 | Icon attribution in `ATTRIBUTION.md` | PR #16/#17, release run `42372245530` | verified |
| CAP-085 | Command-layer fuzz tests (never panic/corrupt/undo) | PR #16/#17, release run `42372245530` | verified |

## v0.11.0 — Measurements

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-086 | Area/perimeter query tool (Shift+M) | tag `v0.11.0`, commit `803b590` | verified |
| CAP-087 | Wall length totals per level and project | tag `v0.11.0`, commit `803b590` | verified |
| CAP-088 | `format_area` honours `UnitSystem` | tag `v0.11.0`, commit `803b590` | verified |

## v0.12.1 — Phase A (Snapping, Selection, Command Palette, Layers, Construction, Status Bar)

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-089 | Layers: `Layer`, protected default, per-entity assignment | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-090 | Construction geometry (line/ray/circle/arc) | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-091 | Status bar cursor coordinates | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-092 | Status bar snap/ortho/grid chips | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-093 | Status bar selection count + zoom % | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-094 | Enhanced entity snapping | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-095 | Window/crossing/fence selection modes | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-096 | Selection filters by type and layer | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-097 | Command palette (Ctrl+K) | PR #21 `18a5bbf`, tag `v0.12.1` | verified |
| CAP-098 | Snap visual indicators on canvas | PR #21 `18a5bbf`, tag `v0.12.1` | verified |

## v0.13.0 — Anchored Dimensions & Release Governance (unreleased)

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-100 | Constraint-driven dimensions core | PR #22 `2b07f58` | implemented (unreleased) |
| CAP-101 | Constraint solver | PR #22 `2b07f58` | implemented (unreleased) |
| CAP-102 | Anchor commands | PR #22 `2b07f58` | implemented (unreleased) |
| CAP-103 | Anchor UI | PR #23 `7d88830` | implemented (unreleased) |
| CAP-104 | Update verification | PR #24 `61cf97a` | implemented (unreleased) |
| CAP-105 | In-app release notes from `RELEASE_NOTES.md` | PR #26 `1a43bea` | implemented (unreleased) |
| CAP-106 | Release governance | PR #25 `d248b8b` | implemented (unreleased) |
| CAP-107 | Documentation parity | this slice | implemented (unreleased) |
| CAP-108 | Phase A UI completion | PR #27 `e86f697` | implemented (unreleased) |

## v0.14.0 — Constraint Persistence & Sheet Layout

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-111 | `Level.constraints` (serde default), `Project::validate`, `level::tests::constraints_survive_a_save_and_load` | PR #31 | verified |
| CAP-112 | `AddConstraint`/`RemoveConstraint` + `inspector.rs` Constraints panel; `command::tests::adding_a_constraint_pushes_it_onto_the_active_level` | PR #32 `ef21549` | verified |
| CAP-113 | `TitleBlock` + `SetTitleBlock`; `pdf::tests::the_title_block_prints_the_project_fields` | PR #33 `cb9f5cf` | verified |
| CAP-114 | `Viewport` + framed scale-fit rendering; `pdf::tests::a_viewport_is_framed_and_captioned_on_its_sheet` | PR #33 | verified |
| CAP-115 | `docs/qc/release-checklist.md` (six sections with sign-off lines) | release run `43529660411` | verified |

## v0.15.0 — Blocks & Symbols, Print Polish

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-116 | `BlockDef`/`BlockInstance` + add/remove commands; `command::tests::add_block_instance_happy_path`, `block_commands_round_trip_through_json` | PR #37 | verified |
| CAP-117 | `Tool::Block` placement and inspector "Blocks" section (`apps/lintel/src/main.rs`, `inspector.rs`) | PR #38 | verified |
| CAP-118 | `pdf::tests::a_viewport_geometry_is_cut_by_a_real_clipping_path`, `each_viewport_carries_its_own_north_arrow_and_scale_bar` | PR #39 | verified |
| CAP-119 | `AddConstraint` palette entries + read-only inspector "Constraint solver" readout | PR #40 | verified |

## v0.16.0 — Block Instance Editing & Canvas Selection

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-120 | `command::tests::a_block_instance_can_be_moved`, `a_block_instance_position_must_be_finite`; `level::tests::block_instances_can_be_moved_rotated_and_flipped` | `43708368391` (PR #45) | verified |
| CAP-121 | `command::tests::a_block_instance_can_be_rotated`, `a_block_instance_rotation_must_be_finite` | `43708368391` (PR #45) | verified |
| CAP-122 | `command::tests::a_block_instance_can_be_flipped`; `block::tests::toggle_flip_toggles_the_mirror` | `43708368391` (PR #45) | verified |
| CAP-123 | `Level::hit_test_block_instance` with five unit tests (`a_click_inside_an_instances_bounds_finds_it`, `hit_test_respects_rotation_and_flip`, `hit_test_rejects_non_finite_points`, `hit_test_prefers_the_topmost_instance`, `hit_test_skips_instances_with_missing_definitions`); UI path `apps/lintel/src/main.rs` `hit_block_instance` | `43822843607` (PR #46), `302f1dcb` (PR #47) | verified |
| CAP-124 | `apps/lintel/src/inspector.rs` `block_instance_fields` (X/Y, rotation, flip, empty states) | `43822843607` (PR #46) | verified |
| CAP-125 | `Applied.block_instance` + auto-select in `apps/lintel/src/main.rs`; `command::tests::the_block_instance_edits_can_be_undone_and_redone` | `43822843607` (PR #46) | verified |

## v0.17.0 — Dimension Label Precision

| ID | Evidence | CI run | Status |
|---|---|---|---|
| CAP-126 | `Dimension::label_precision` + `SetDimensionPrecision` validator + six domain tests (`a_new_dimension_labels_whole_millimetres`, `label_precision_round_trips_and_is_bounded`, `a_dimension_saved_before_label_precision_still_loads`, `a_dimension_label_precision_can_be_set`, `a_dimension_label_precision_is_bounded`, extended `the_new_dimension_commands_round_trip_through_json`); UI `dimension_fields` precision control + `render_dimensions` label format | `43847513509` (PR #48) | verified |

## v0.8.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.8.0 —
release workflow run `42295310359`, all six assets present.

## v0.7.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.7.0 —
release workflow run `42212698044`, all six assets present.

## v0.6.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.6.0 —
release workflow run `42193835276`, all six assets present
(`lintel-windows-x86_64-setup.exe`, the Windows/Linux/macOS zips, the AppImage
and `SHA256SUMS.txt`).

## v0.5.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.5.0 —
release workflow run `42103705669`. `lintel-windows-x86_64-setup.exe` verified
as a valid PE binary; SHA256SUMS.txt recomputed and matching.

## v0.4.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.4.0 —
release workflow run `42024620827`. The macOS archive contains `Lintel.app`,
and the release includes `lintel-appimage-x86_64.AppImage` (valid ELF) plus
`SHA256SUMS.txt` (recomputed hashes match). Notes came from `RELEASE_NOTES.md`.

## v0.3.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.3.0 —
release workflow run `41999766800`. The macOS archive contains
`Lintel.app/Contents/MacOS/lintel` and an `Info.plist` whose
`CFBundleShortVersionString` expands to the tag version (verified by
downloading the published asset and inspecting the archive).

## v0.2.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.2.0 —
release workflow run `41993262778` built all four targets (Linux x86_64,
Linux aarch64, macOS aarch64, Windows x86_64) and attached
`SHA256SUMS.txt`. Notes came from `RELEASE_NOTES.md`.

## v0.9.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.9.0 —
PR `opencode/v0.9.0-rooms` merged; release workflow run published all assets.

## v0.10.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.0 —
release workflow run `42365552699`.

## v0.10.1 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.10.1 —
release run `42372245530`; all seven assets present, `SHA256SUMS.txt` verified.

## v0.11.0 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.11.0 —
tag on commit `803b590` (rebuilt 2026-11-10); signed artefacts pending (audit F3).

## v0.12.1 release

Published at https://github.com/Perchemop/lintel-desktop/releases/tag/0.12.1 —
tag on commit `803b590` (rebuilt 2026-11-10); Phase A content; first signing
gate applies to releases built after 2026-11-11.

## v0.13.0 release

Not yet released — CAP-100–CAP-108 landed on `main`; release gate pending.
