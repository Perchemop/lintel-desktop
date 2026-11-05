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
