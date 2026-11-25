# T-200 — Fix kittest SIGSEV in Evidence Generation

**Phase:** 1 (CI Reliability) — **Highest Priority**

## Objective
Fix the SIGSEGV (exit code 101) that occurs in the `kittest` CI job when running `LINTEL_GENERATE_EVIDENCE=1 cargo test -p lintel --bin lintel evidence_` on GitHub Actions runners.

## Context
- The evidence tests use egui_kittest's wgpu test renderer to generate PNG screenshots
- The crash is non-deterministic and appears to be related to wgpu/AccessKit initialization on headless CI runners
- AccessKit-only tests (no wgpu) are stable
- The crash happens during `evidence_buttons_dark` test (4th of 8 evidence tests)

## Acceptance Criteria
- [ ] `kittest` CI job passes 5 consecutive runs without SIGSEGV
- [ ] All 8 evidence tests complete and produce PNG outputs
- [ ] AccessKit assertion tests continue to pass
- [ ] No regression in button/form/layout evidence quality

## Investigation Steps
1. Add `RUST_BACKTRACE=1` to CI env to capture stack trace
2. Check if wgpu instance creation fails on specific GPU drivers
3. Test with `wgpu` feature `vulkan` vs `dx12` vs `metal` on different runners
4. Consider adding `#[ignore]` to flaky evidence tests with a follow-up issue
5. Explore egui_kittest version upgrade (currently 0.36.2)

## Files to Modify
- `.github/workflows/ci.yml` — kittest job env/config
- `apps/lintel/src/tests/evidence.rs` — test structure
- `apps/lintel/Cargo.toml` — egui_kittest version/features

## Related
- D-036 (evidence pipeline)
- T-008 (evidence docs)
- kittest CI job in `.ci-required.txt`

## Effort Estimate
Medium (investigation-heavy, may require CI-only workarounds)