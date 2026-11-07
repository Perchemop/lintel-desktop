# Lintel Development Guide

## Overview

Lintel is a free, offline 2D architectural drafting desktop app written in Rust
with egui/eframe. It follows a **thick headless core / thin UI shell** design:

- `crates/lintel-core` — pure Rust domain model, commands, persistence, PDF
  rendering. Zero GUI dependencies. Fully tested headlessly.
- `apps/lintel` — eframe/egui UI shell. Builds `Command` values and renders
  results. No business logic.
- `crates/update-check` — shared suite crate for GitHub Releases update checking.
- `crates/crash-report` — shared suite crate for panic hook and crash capture.

This file documents how to build, test, and contribute. End users should read
`README.md` instead — they do not need Rust installed to run Lintel.

## Repository layout

```
lintel/
├── .github/workflows/    # CI and release workflows
├── apps/lintel/          # UI shell (egui/eframe)
├── crates/
│   ├── lintel-core/      # Thick headless domain core
│   ├── crash-report/     # Shared crash-reporting crate
│   └── update-check/     # Shared update-check crate
├── docs/                 # Development documentation
├── resources/            # Platform icons, desktop files, installers
├── scripts/              # CI helper scripts (scans, etc.)
├── ROADMAP.md            # Public roadmap with readiness %
├── CHANGELOG.md          # Keep a Changelog history
├── RELEASE_NOTES.md      # Per-tag release notes fed to GitHub Releases
├── IMPLEMENTATION_PLAN.md# Vertical-slice plan
├── REQUIREMENTS_TRACEABILITY.md # Requirement → CAP → tests → release mapping
├── DECISIONS.md          # Architectural decisions
├── PROGRESS.md           # Readiness status per release
├── ATTRIBUTION.md        # Third-party licences
└── LICENSE-MIT / LICENSE-APACHE
```

## Prerequisites

You do **not** need Rust installed to *run* Lintel — download a release
artexture from GitHub Releases instead.

To *develop* Lintel:

- Rust 1.85+ (stable channel). Pin via `rust-toolchain.toml`.
- GitHub CLI (`gh`) for CI verification via `ci.sh`.
- On Linux: `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`,
  `libxkbcommon-dev` (for egui/eframe).

## Building and testing

### Do not run cargo locally

This machine is memory-constrained. All Rust builds, tests, clippy, and fmt run
in GitHub Actions via `ci.sh`:

```bash
# Commit, push, open PR, wait for CI green
~/.config/opencode/scripts/ci.sh run "my change description"

# Check current CI state
~/.config/opencode/scripts/ci.sh status

# View failing job logs
~/.config/opencode/scripts/ci.sh logs

# See what would be committed
~/.config/opencode/scripts/ci.sh diff
```

CI runs (in order): `cargo fmt --all -- --check`, `cargo clippy --all-targets
--workspace -- -D warnings`, `cargo test --workspace`, `cargo build --release`,
plus secret scan, personal data scan, and SBOM generation.

### Local development (if you have Rust)

If you want to iterate locally on a machine that can run cargo:

```bash
cargo fmt --all
cargo clippy --all-targets --workspace
cargo test --workspace
cargo build --release -p lintel
```

Core tests run headless — no `DISPLAY`, GPU, or window server required.

## Making a change (Chat → Task → PR)

1. **Chat**: Plan the work. Write a Task brief (goal, acceptance criteria, out
   of scope, try-steps).
2. **Task**: Implement the change. Add or update tests for every user-visible
   behaviour change.
3. **Push**: Branch → `ci.sh run` → CI green → open PR using the template.
4. **Review**: Self-review against the clean-room checklist (Section 2 of
   `guide.md`).
5. **Client try**: Give the human exact numbered try-steps only after CI is green.
6. **Merge**: After review findings are addressed, merge the PR to `main`.
7. **Track**: Update `ROADMAP.md`, `CHANGELOG.md`, `PROGRESS.md` as appropriate.

## Writing tests

- **Workflow tests**: `crates/lintel-core/tests/workflows.rs` and
  `crates/lintel-core/tests/dimensions.rs`. Exercise commands → save → load →
  assert. No GUI, no display.
- **Unit tests**: Inline `#[cfg(test)]` modules in source files.
- **Property-based tests**: `crates/lintel-core/tests/properties.rs` uses
  `proptest` for geometry edge cases (room detection, wall joins).
- **Never** call egui/eframe APIs from `*-core` tests (addendum K.13).
- Use `tempfile` for isolated temp directories (no machine-specific paths).

## Release process

1. Update `RELEASE_NOTES.md` with the new release at the top.
2. Bump versions in `apps/lintel/Cargo.toml` and `crates/lintel-core/Cargo.toml`.
3. Tag with `vX.Y.Z` and push — `release.yml` builds multi-platform artefacts,
   generates `SHA256SUMS.txt`, and publishes the GitHub Release.
4. CI updates `docs/tokens.json` on each semver tag.
5. Release notes must reference `docs/tokens.json` ledger values.

## Clean-room rules

- All implementation must be original — derived from public standards, the
  project's own spec, and capability-level inspiration recorded in
  `docs/inspiration.md`.
- No reverse engineering, decompilation, or proprietary source.
- Third-party assets must be permissively licensed and listed in `ATTRIBUTION.md`.
- Dual licence: MIT OR Apache-2.0.
- No real personal data or secrets anywhere in the tree, history, or artefacts
  (see `guide.md` Part II §29).

## Accessibility

- Every flow must be keyboard-completable. Focus order must be sensible and
  visible.
- egui exposes AccessKit on Windows and macOS. Interactive widgets carry
  meaningful roles, names, values.
- WCAG 2.2 Level AA target where feasible; limitations recorded in
  `ACCESSIBILITY.md`.
- Run the accessibility smoke test in `docs/qc/checklist.md` whenever the UI is
  touched.

## Git identity

Every commit is authored as `"Lintel Dev" <dev@example.com>`. `ci.sh` asserts
this before committing. GitHub PR/merge authorship follows the pushing account
(known platform limitation — see `guide.md` Part II §26.7).

## Further reading

- `guide.md` — the canonical process (local only, never committed).
- `ROADMAP.md` — release matrix and capability tracking.
- `PROGRESS.md` — readiness status per release.
- `DECISIONS.md` — architectural decisions.
- `docs/audit/` — exhaustive end-to-end audit findings and remediation status.