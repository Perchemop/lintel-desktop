# Decisions

## Recorded decisions for this project.

| Decision ID | Area | Decision | Rationale | Date |
|---|---|---|---|---|
| D-001 | Product name | Chose "Lintel" over alternatives | Structural clarity; no trademark conflicts; handle `@perchemop/lintel-desktop` available | 2026-11-03 |
| D-002 | License | Dual licence MIT OR Apache-2.0 | Permissive; allows commercial and open-source use; no copyleft viral risk | 2026-11-03 |
| D-003 | Persistence format | JSON over SQLite | Human-readable; no native dependency; round-trip editable; part of clean-room rules | 2026-11-03 |
| D-004 | Unit system default | Metric (mm) | Developer's local platform; metric is universal for architectural drafting | 2026-11-03 |
| D-005 | CLI/gate strategy | Headless `cargo test` as primary QA; client visual pass on green CI | Matches addendum K/M rules; no display required in CI | 2026-11-03 |
| D-006 | v0.1.0 gate | All Section 6.1 criteria must be satisfied before tagging | Hard gate prevents feature creep; ensures shipable foundation | 2026-11-03 |
| D-007 | Shared suite crates | `crash-report` + `update-check` from day one | Suite readiness; consistent crash/update behaviour across apps | 2026-11-03 |
| D-008 | CI from day one | `.github/workflows/ci.yml` and `release.yml` from foundation stage | Learn release path early; avoid broken tags/artefacts at the worst time | 2026-11-03 |