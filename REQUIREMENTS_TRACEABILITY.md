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

## Suite Standards Compliance
Guide §12.1 — Suite-wide standards:
- Token ledger `docs/tokens.json` — **implemented** (CI-managed; updated on each tag).
- `crates/update-check` shared submodule — **implemented** (git submodule of `Perchemop/lintel-shared-crates`; see DECISIONS.md D-029).
- Auto-update signature + SHA256SUMS verification — **implemented as of CAP-106** (release.yml fails unsigned; self-verifies signature).
- Version UI bottom-right + Release Notes popup with token metrics + Update button — **implemented** (v0.10.0+).
- README screenshots — **pending** until v1.0.

## Change log
- 2026-11-07: Added v0.9.0 traceability rows.
- 2026-11-13: Extended traceability through v0.13.0; corrected suite-standards status (ledger/submodule/signing/version UI).
