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
