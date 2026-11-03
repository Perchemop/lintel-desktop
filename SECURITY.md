# Security

**If you discover a security vulnerability in this project, please do NOT file a public issue.**

Instead, please use GitHub's private vulnerability reporting:
1. Go to the Security tab → "Report a vulnerability"
2. Or contact the maintainer directly

All security issues will be acknowledged within seven days. A disclosure date will be agreed upon with the reporter.

## Severity scale

| Severity | Definition | Response |
|----------|-----------|----------|
| Critical | Remote code execution, secret exfiltration, or crash-report data leak without user consent | Fix before next release; private advisory |
| High | Local privilege escalation on the user's machine, or silent data corruption | Fix before next release; private advisory |
| Medium | Information disclosure (logs, paths, crash contents) or weak validation | Fix in next release cycle |
| Low | Hardening opportunity, no direct exploit path | Track in backlog; fix at maintainer discretion |

## Supported platforms

- Linux (x86_64, aarch64)
- macOS (aarch64, x86_64)
- Windows (x86_64)

## Data collection

- Crash reports send only what the user consents to (app version, OS, scrubbed backtrace).
- Update checks query GitHub Releases API; no personal data is transmitted.
- No API keys, tokens, or credentials are sent automatically.