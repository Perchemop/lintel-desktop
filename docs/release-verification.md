# Verifying Lintel releases

Every release ships a `SHA256SUMS.txt` manifest and a detached OpenPGP
signature `SHA256SUMS.txt.asc`. Releases since 0.11.0 are signed with the
project release key below.

## Release signing key

    pub   rsa3072 2026-11-11 [SC]
          0996 0C90 35E1 6DC7 7F83  6406 7F76 101D E442 D0FE
    uid   Lintel Release Signing <dev@example.com>

The full public key is committed at `resources/lintel-release-signing-public.asc`.

## Verify a download

    gpg --import resources/lintel-release-signing-public.asc
    sha256sum --check --ignore-missing SHA256SUMS.txt
    gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt

`gpg: Good signature from "Lintel Release Signing <dev@example.com>"` plus a
matching checksum means the artefact is intact and came from this project.
Fingerprints should be cross-checked against this file's git history.
