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
| CAP-066 | Shortcuts work while the canvas has focus | pending | awaiting run |
| CAP-067 | Level summary unit-tested (3 cases) | pending | awaiting run |
| CAP-068 | Focus-review outcome recorded | pending | awaiting run |

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
