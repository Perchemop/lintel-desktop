# Contributing

## Development process

1. **Fork the repository** and create a branch from `main` for your change.
2. **Write or update tests** — headless `cargo test` is the primary QA engine (guide Appendix K/M).
3. **Run the full local check**:
   - `cargo fmt --check --all`
   - `cargo clippy --all-targets -D warnings`
   - `cargo test --workspace`
   - `cargo build --release -p lintel`
4. **Commit** with a descriptive message. Observe the git-identity rule: commits are authored as `<App Name> Dev <dev@example.com>`.
5. **Push** your branch and open a Pull Request using the template.
6. **Wait for CI** to pass (format, clippy, test, build, deny, SBOM, scans).
7. **Address review feedback** — reviewers reject PRs that fail the checklist.
8. **Merge** — only when CI is green and all checklist items are satisfied.

## First-time setup

```bash
# Install Rust toolchain
rustup default stable

# Install prerequisites
rustup component add clippy rustfmt

# Run the initial checks
cargo fmt --check --all
cargo clippy --all-targets -p lintel-core -p lintel -- -D warnings
cargo test --workspace --lib -p lintel-core
```

## Reporting issues

Use the issue templates in `.github/ISSUE_TEMPLATE/` for bug reports, feature requests, and accessibility issues.

## Licence

Contributions are dual-licensed under MIT OR Apache-2.0, unless otherwise stated.