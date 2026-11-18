# Requirements Traceability — Lintel

Maps user-facing capabilities to source, tests, and releases. Follows guide §11 tracking requirements.

## Requirement taxonomy
- **C**lean-room — functionality derived from public research only.
- **U**ser workflow — direct user operation.
- **L**egal — licence / clean-room / personal-data gate.

## Traceability

### v0.1.0 Hard Gate
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-001 | Create and open a named project | CAP-001, CAP-017 | `persistence::tests` | v0.1.0 |
| R-002 | Draw and move walls | CAP-002, CAP-003, CAP-004, CAP-005 | `wall::*` | v0.1.0 |
| R-003 | Place doors/windows on walls | CAP-007, CAP-008, CAP-009 | `opening::*` | v0.1.0 |
| R-004 | PDF export of the active level | CAP-020 | `pdf::tests` | v0.1.0 |

### v0.2.0 Joint Resolution & Dimensions
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-005 | Joint fill for unequal thicknesses | CAP-021 | `join::tests` | v0.2.0 |
| R-006 | Linear dimensions with undo/redo | CAP-026…CAP-033 | `dimension::tests` | v0.2.0 |
| R-007 | Suite infra: update-check + crash-report | CAP-040, CAP-041 | Integration smoke | v0.2.0 |

### v0.9.0 Drawing Set Expansion
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-9.1 | Auto-detect enclosed rooms and show area | — | `room::tests::two_disconnected_rooms_are_both_reported_exactly_once` | v0.9.0 |
| R-9.2 | Opening schedule table in PDF and Inspector | — | `schedule::tests` | v0.9.0 |
| R-9.3 | Free text plan labels with CRUD | — | `command::tests::label::*` | v0.9.0 |
| R-9.4 | Multi-sheet drawing set export | — | `pdf::tests::render_set_*` | v0.9.0 |
| R-9.5 | North arrow + graphic scale bar on sheets | — | `pdf::tests` | v0.9.0 |
| R-9.6 | Duplicate selection (Ctrl+D) | — | Manual try-steps | v0.9.0 |

### v0.10.0 Suite Standards Conformance
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-10.1 | CI-managed token ledger | CAP-075 | `release.yml` | v0.10.0 |
| R-10.2 | Release signing step | CAP-076 | `release.yml` | v0.10.0 |
| R-10.3 | Development docs | CAP-077 | — | v0.10.0 |
| R-10.4 | Property-based fuzz tests | CAP-078 | `properties.rs` | v0.10.0 |
| R-10.5 | Version UI bottom-right + Release Notes popup | CAP-079 | `about.rs` | v0.10.0 |
| R-10.6 | Window icon embedded | CAP-080 | `main.rs` | v0.10.0 |

### v0.10.1 Window Icon & Installer Version Fix
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-10.7 | Window icon via `with_icon()` | CAP-081 | `main.rs` | v0.10.1 |
| R-10.8 | SVG icon source + attribution | CAP-082, CAP-084 | `resources/icons` | v0.10.1 |
| R-10.9 | Windows `.iss` version corrected | CAP-083 | `resources/windows/lintel.iss` | v0.10.1 |
| R-10.10 | Command-layer fuzz (never panic/corrupt/undo) | CAP-085 | `properties.rs` | v0.10.1 |

### v0.11.0 Measurements
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-11.1 | Area/perimeter query tool | CAP-086 | `measurements::tests` | v0.11.0 |
| R-11.2 | Wall length totals | CAP-087 | `measurements::tests` | v0.11.0 |
| R-11.3 | `format_area` honours units | CAP-088 | `units::tests` | v0.11.0 |

### v0.12.1 Phase A (Snapping, Selection, Command Palette, Layers, Construction, Status Bar)
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-12.1 | Enhanced entity snapping | CAP-094 | `snap::tests` | v0.12.1 |
| R-12.2 | Window/crossing/fence selection | CAP-095 | `selection::tests` | v0.12.1 |
| R-12.3 | Selection filters by type and layer | CAP-096 | `selection::tests` | v0.12.1 |
| R-12.4 | Command palette (Ctrl+K) | CAP-097 | `command_palette` | v0.12.1 |
| R-12.5 | Snap visual indicators | CAP-098 | `canvas.rs` | v0.12.1 |
| R-12.6 | Layers | CAP-089 | `layer::tests` | v0.12.1 |
| R-12.7 | Construction geometry | CAP-090 | `construction::tests` | v0.12.1 |
| R-12.8 | Status bar | CAP-091–CAP-093 | `status_bar` | v0.12.1 |

### v0.13.0 Anchored Dimensions & Release Governance (unreleased)
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-13.1 | Constraint-driven dimensions core | CAP-100 | `dimension::tests` | v0.13.0 (unreleased) |
| R-13.2 | Constraint solver | CAP-101 | `constraint::tests` | v0.13.0 (unreleased) |
| R-13.3 | Anchor commands | CAP-102 | `command::tests` | v0.13.0 (unreleased) |
| R-13.4 | Anchor UI | CAP-103 | `inspector.rs`, `canvas.rs` | v0.13.0 (unreleased) |
| R-13.5 | Update verification | CAP-104 | `update-check` tests | v0.13.0 (unreleased) |
| R-13.6 | In-app release notes from `RELEASE_NOTES.md` | CAP-105 | `about.rs` | v0.13.0 (unreleased) |
| R-13.7 | Release governance | CAP-106 | `release.yml`, `ci.yml` | v0.13.0 (unreleased) |
| R-13.8 | Documentation parity | CAP-107 | docs | v0.13.0 (unreleased) |
| R-13.9 | Phase A UI completion | CAP-108 | `inspector.rs`, `canvas.rs` | v0.13.0 (unreleased) |

### v0.14.0 Constraint Persistence & Sheet Layout
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-14.1 | Constraint persistence on the level | CAP-111 | `level::tests::constraints_survive_a_save_and_load` | v0.14.0 |
| R-14.2 | Constraint management UI and commands | CAP-112 | `command::tests::adding_a_constraint_pushes_it_onto_the_active_level` | v0.14.0 |
| R-14.3 | Sheet title blocks | CAP-113 | `pdf::tests::the_title_block_prints_the_project_fields` | v0.14.0 |
| R-14.4 | Sheet viewports | CAP-114 | `pdf::tests::a_viewport_is_framed_and_captioned_on_its_sheet` | v0.14.0 |
| R-14.5 | Release QC checklist | CAP-115 | `docs/qc/release-checklist.md` | v0.14.0 |

### v0.15.0 Blocks & Symbols, Print Polish
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-15.1 | Block definitions and instances | CAP-116 | `command::tests::add_block_instance_happy_path` | v0.15.0 |
| R-15.2 | Block instance placement UI | CAP-117 | `apps/lintel/src/main.rs`, `inspector.rs` | v0.15.0 |
| R-15.3 | Viewport print polish | CAP-118 | `pdf::tests::a_viewport_geometry_is_cut_by_a_real_clipping_path` | v0.15.0 |
| R-15.4 | Constraint palette and solver readout | CAP-119 | `command_palette.rs`, `inspector.rs` | v0.15.0 |

### v0.16.0 Block Instance Editing & Canvas Selection
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-16.1 | Move block instances | CAP-120 | `command::tests::a_block_instance_can_be_moved` | v0.16.0 |
| R-16.2 | Rotate block instances | CAP-121 | `command::tests::a_block_instance_can_be_rotated` | v0.16.0 |
| R-16.3 | Flip block instances | CAP-122 | `block::tests::toggle_flip_toggles_the_mirror` | v0.16.0 |
| R-16.4 | Canvas hit-testing and click-to-select | CAP-123 | `Level::hit_test_block_instance`, `apps/lintel/src/main.rs` | v0.16.0 |
| R-16.5 | Editable Inspector instance fields | CAP-124 | `apps/lintel/src/inspector.rs` | v0.16.0 |
| R-16.6 | Auto-select the affected instance | CAP-125 | `command::tests::the_block_instance_edits_can_be_undone_and_redone` | v0.16.0 |

### v0.17.0 Dimension Precision, Keyboard Truth & the Contrast Gate
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-17.1 | Per-dimension label precision (0–4 dp) via Inspector "Precision:" control; old files load at 0 | CAP-126 | `dimension::tests::a_new_dimension_labels_whole_millimetres`, `label_precision_round_trips_and_is_bounded`, `a_dimension_saved_before_label_precision_still_loads`; `command::tests::a_dimension_label_precision_can_be_set`, `a_dimension_label_precision_is_bounded`, `the_new_dimension_commands_round_trip_through_json`; UI `dimension_fields`, `render_dimensions` | v0.17.0 |
| R-17.2 | Delete a single selected block instance, with an honest refusal when it is already gone | CAP-127 | `command::tests::a_block_instance_can_be_deleted_by_id`, `deleting_a_missing_block_instance_reports_it_gone`; UI `delete_selection` in `apps/lintel/src/main.rs` | v0.17.0 |
| R-17.3 | WCAG AA contrast for theme text and meaningful UI | CAP-128 | `theme::tests::theme_colours_meet_wcag_aa` (`relative_luminance`, `contrast_ratio`) | v0.17.0 |

### v0.18.0 The Design System
| Req ID | Description | CAP IDs | Tests | Release |
|--------|-------------|---------|-------|---------|
| R-18.1 | WCAG AA contrast gate: merge-gate test, darkened palette roles and raw colours moved into named theme roles | CAP-128 | `theme::tests::theme_colours_meet_wcag_aa` (`relative_luminance`, `contrast_ratio`) | v0.18.0 |
| R-18.2 | Delete a single selected block instance with an honest refusal; keyboard shortcuts and palette hints are truthful | CAP-127 | `command::tests::a_block_instance_can_be_deleted_by_id`, `deleting_a_missing_block_instance_reports_it_gone`, `palette_shortcuts_are_unique`; UI `delete_selection` | v0.18.0 |
| R-18.3 | `egui_kittest` AccessKit/snapshot gate in CI | — | `main.rs` inline tests: `palette_has_named_actions`, `inspector_sections_labelled`, `canvas_summary_accessible`; `.ci-required.txt` | v0.18.0 |
| R-18.4 | Button role/size patterns with a 24 pt minimum target and visible focus ring | — | `theme.rs` `ButtonRole`/`ButtonSize`/`ButtonRoleStyle`, `button_style()`; inspector/command palette/status bar | v0.18.0 |
| R-18.5 | ThemeSpec three-tier tokens with byte-identical values | — | `theme::tests::tier_two_roles_resolve_to_base_swatches`, `tier_three_components_resolve_to_roles`, `contrast_maths_matches_wcag_reference_points` | v0.18.0 |
| R-18.6 | Token ledger migration and Dev-authorship enforcement | CAP-075 | `docs/tokens.md`, `release.yml`, `pr-author-is-dev`, `ci.sh` | v0.18.0 |
| R-18.7 | Unbound commands documented as core-only (D-032) | — | `DECISIONS.md` D-032 | v0.18.0 |

## Suite Standards Compliance
Guide §12.1 — Suite-wide standards:
- Token ledger `docs/tokens.md` — **implemented** (CI-managed; updated on each tag).
- `crates/update-check` shared submodule — **implemented** (git submodule of `Perchemop/lintel-shared-crates`; see DECISIONS.md D-029).
- Auto-update signature + SHA256SUMS verification — **implemented as of CAP-106** (release.yml fails unsigned; self-verifies signature).
- Version UI bottom-right + Release Notes popup with token metrics + Update button — **implemented** (v0.10.0+).
- README screenshots — **pending** until v1.0.

## Change log
- 2026-11-07: Added v0.9.0 traceability rows.
- 2026-11-13: Extended traceability through v0.13.0; corrected suite-standards status (ledger/submodule/signing/version UI).
- 2026-11-16: Extended traceability through v0.16.0; added the missing v0.14.0 (CAP-111–CAP-115) and v0.15.0 (CAP-116–CAP-119) rows and recorded CAP-120–CAP-125 merged.
- 2026-11-16: CAP-126 (dimension label precision) merged via PR #48.
- 2026-11-17: Added R-17.2 (CAP-127, delete a single block instance) and R-17.3 (CAP-128, WCAG AA contrast gate) for v0.17.0.
- 2026-11-18: Added R-18.1–R-18.7 for the v0.18.0 design-system release (WCAG AA contrast gate, CAP-127/keyboard truth, `egui_kittest`, button patterns, ThemeSpec three-tier tokens, ledger/authorship migration, D-032).