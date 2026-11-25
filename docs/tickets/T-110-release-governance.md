# Ticket T-110: Release Governance (CAP-104–107)

**Objective:** Update verification, notes sync, signed releases, docs parity.

**Spec Reference:** CAP-104–107, guide §12.1

**Target Files:**
- `crates/update-check/src/lib.rs`
- `crates/lintel-core/src/lib.rs` (build-time notes)
- `.github/workflows/release.yml`
- `docs/capability-inventory.md`
- `docs/parity.md`

**Acceptance Criteria:**
- [ ] **CAP-104** `update-check` verifies `SHA256SUMS.txt` + `.asc` signature before surfacing update
- [ ] **CAP-104** Cache TTL honoured (no more `check_uncached`)
- [ ] **CAP-105** In-app release notes generated from `RELEASE_NOTES.md` at build time (embed via `include_str!`)
- [ ] **CAP-106** `release.yml` fails if tag ≠ `CARGO_PKG_VERSION`
- [ ] **CAP-106** `release.yml` fails if `GPG_PRIVATE_KEY` unset (no silent unsigned publish)
- [ ] **CAP-107** `docs/capability-inventory.md` extended through CAP-108 with status + evidence
- [ ] **CAP-107** `docs/parity.md` rows for v0.13.0 capabilities
- [ ] **CAP-107** CHANGELOG/ROADMAP/PROGRESS reconciled to published tags

**Dependencies:** None (parallelizable)

**Estimated Effort:** 2 sessions