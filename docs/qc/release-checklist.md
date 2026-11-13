# Release QC checklist

Run top to bottom before cutting a release tag. Every item is either an
executable command (run it, paste the outcome into the release PR) or a named
manual procedure with a sign-off line. A release with an unsigned-off box is
not cut — guide §12.1 makes the pipeline refuse most of these automatically;
this checklist covers what the pipeline cannot see.

## 1. Pipeline gates (automated — confirm green)

- [ ] CI green on `main` at the release commit — including the
      version-consistency step:

      ```bash
      gh run list --repo Perchemop/lintel-desktop -L 1
      ```

- [ ] Both crate versions equal the intended tag (CI enforces; verify
      locally):

      ```bash
      grep -m1 '^version' apps/lintel/Cargo.toml crates/lintel-core/Cargo.toml
      ```

- [ ] `RELEASE_NOTES.md` has a section for the release version, quoting the
      `docs/tokens.json` ledger values (this release + cumulative).
- [ ] `CHANGELOG.md` entry dated for the release.

## 2. Signed artefacts (automated by `release.yml`, verify after publish)

- [ ] The release lists `SHA256SUMS.txt` **and** `SHA256SUMS.txt.asc`:

      ```bash
      gh release view v<TAG> --repo Perchemop/lintel-desktop --json assets \
        --jq '[.assets[].name]'
      ```

- [ ] Checksums and signature verify on a clean directory:

      ```bash
      mkdir -p /tmp/opencode/qc && cd /tmp/opencode/qc
      gh release download v<TAG> --repo Perchemop/lintel-desktop
      gpg --import ../../lintel/resources/lintel-release-signing-public.asc
      sha256sum --check --ignore-missing SHA256SUMS.txt
      gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt
      ```

      Expect `sha256sum: OK` per artefact and `gpg: Good signature from
      "Lintel Release Signing <dev@example.com>"`.

## 3. AppImage execution smoke (manual — Linux host)

- [ ] Download `lintel-appimage-x86_64.AppImage`, `chmod +x`, launch. Confirm:
      window opens with the Lintel icon, a new plan can be created, one wall
      drawn, the plan saved, the app quits cleanly.

      Sign-off: ____________ date: ____________

## 4. Screen-reader pass (manual — Windows Narrator and/or macOS VoiceOver)

- [ ] Tab reaches: canvas, toolbar controls, level summary, inspector fields,
      command palette (Ctrl+K), release-notes popup (bottom-right version
      link).
- [ ] Narrator/VoiceOver announces each control's label; no unnamed buttons.
- [ ] Keyboard-only flow: create plan → add wall → add door → add dimension →
      save — all reachable without a pointer.

      Sign-off: ____________ date: ____________

## 5. Ledger and notes consistency

- [ ] After the tag push, `docs/tokens.json` on `main` shows the new
      `release_version` and incremented cumulative figure (CI-owned; do not
      hand-edit):

      ```bash
      gh api repos/Perchemop/lintel-desktop/contents/docs/tokens.json \
        --jq .content | base64 -d
      ```

- [ ] The published release body's "Token metrics" section matches the
      ledger.
- [ ] `RELEASE_TOKENS` was set for the release if the true figure is known
      (the workflow warns when it records the 5,000 default).

## 6. Post-release

- [ ] `ROADMAP.md` / `PROGRESS.md` updated: release marked shipped, next
      version's first slice named.
- [ ] Any QC defects found above are filed as checklist-amendment commits
      before the next release, so the checklist stays the source of truth.
