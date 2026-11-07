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

## Suite Standards Compliance
Guide §12.1 — Suite-wide standards:
- Token ledger `docs/tokens.json` — **pending** (file created, CI update step pending).
- `crates/update-check` shared submodule — **deviation**: in-repo for now; see DECISIONS.md D-012.
- Auto-update signature + SHA256SUMS verification — **partial**: SHA256SUMS present, signing not implemented.
- Version UI bottom-right + Release Notes popup with token metrics + Update button — **pending** (version currently top toolbar).
- README screenshots — **pending** until v1.0.

## Change log
- 2026-11-07: Added v0.9.0 traceability rows.
