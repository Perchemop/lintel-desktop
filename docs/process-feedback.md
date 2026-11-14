# Process feedback

Append-only log (guide Section 16 / Appendix N). Entries are advisory
until the client promotes them into the local guide/addendum.

| Date / session | What I was trying to do | Block or friction | Suggested change | Severity | Workaround | Confirm no safety rule waived |
|---|---|---|---|---|---|---|
| 2026-11-14 v0.14.0 | Ship v0.14.0 | CI on `main` went red immediately after the release: `release_notes` tests asserted the newest version (`"0.13.0"`) and cumulative tokens (`Some(45000)`) as literals, so every new release broke them — twice (v0.13.0, v0.14.0) | When a test reads release-managed data, assert invariants (ordering via `update_check::compare`, monotonic cumulative, oldest-version stability) — never the current newest value. Add this to the test-writer brief template | medium | De-pinned the tests to invariants (PR #36) | yes |
| 2026-11-14 v0.14.0 | Land the v0.14.0 docs/ledger commits | `ci.sh` leaves the working checkout on `opencode/ci`; a follow-up `git commit && git push origin main` from that state put docs commits on the PR branch instead of `main`, needing cherry-picks | `ci.sh` should return to the invoking branch after a run, or print a warning; alternatively the process should `git checkout main` explicitly before any post-run docs commit | low | Manually cherry-picked each stray commit to `main` and reset `opencode/ci` | yes |
