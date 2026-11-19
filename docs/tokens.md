# Token usage ledger (guide §24.1)

**Measured 2026-11-19 from the OpenCode session ledger** (`opencode.db`,
`session_v2`), superseding the earlier 5,000-per-release placeholder estimates.
Historical release notes published before this date cite the estimates; the
measured figures below are the authoritative record (guide §19 rule 3:
measured, not assumed).

**Counting definition.** Tokens this release = sum of `tokens_input +
tokens_output + tokens_reasoning` across all agent sessions in the suite
working directory, assigned to the release whose tag date is the first tag
date on/after the session day. Cache-read tokens (billions more) are excluded:
they are repeated prefix reads, not fresh work. Windows before v0.2.0 include
general suite/guide sessions in the same working directory — they cannot be
separated retroactively.

| Version  | Date       | Tokens this release | Tokens cumulative | Notes                                        |
|----------|------------|---------------------|-------------------|----------------------------------------------|
| v0.1.0   | 2026-11-04 | 610,929,565         | 610,929,565       | First functional build (window includes pre-app suite setup) |
| v0.2.0   | 2026-11-05 | 70,213,294          | 681,142,859       | Joint resolution, dimensions, suite infra    |
| v0.3.0   | 2026-11-05 | 0 (same-day window) | 681,142,859       | Error feedback, macOS bundle                 |
| v0.4.0   | 2026-11-05 | 0 (same-day window) | 681,142,859       | Accessibility labels, shortcuts, AppImage    |
| v0.5.0   | 2026-11-06 | 205,358             | 681,348,217       | Windows installer                            |
| v0.6.0   | 2026-11-06 | 0 (same-day window) | 681,348,217       | Keyboard file flows, accessible canvas       |
| v0.7.0   | 2026-11-06 | 0 (same-day window) | 681,348,217       | Every capability reachable                   |
| v0.8.0   | 2026-11-07 | 7,346,937           | 688,695,154       | Keyboard-focus review                        |
| v0.9.0   | 2026-11-07 | 0 (same-day window) | 688,695,154       | Drawing set                                  |
| v0.9.1   | 2026-11-07 | 0 (same-day window) | 688,695,154       | Version fix                                  |
| v0.10.0  | 2026-11-07 | 0 (same-day window) | 688,695,154       | Suite-standard conformance                   |
| v0.10.1  | 2026-11-07 | 0 (same-day window) | 688,695,154       | Window icon, installer version               |
| v0.11.0  | 2026-11-11 | 146,693,132         | 835,388,286       | Constraint-driven dimensions                 |
| v0.12.1  | 2026-11-11 | 0 (same-day window) | 835,388,286       | Constraint persistence renumber              |
| v0.13.0  | 2026-11-13 | 55,187,838          | 890,576,124       | Anchored dimensions, release governance      |
| v0.14.0  | 2026-11-14 | 263,050,570         | 1,153,626,694     | Constraint persistence, sheet layout         |
| v0.15.0  | 2026-11-15 | 22,531,980          | 1,176,158,674     | Blocks & symbols, print polish               |
| v0.15.1  | 2026-11-15 | 0 (same-day window) | 1,176,158,674     | macOS installer fix                          |
| v0.16.0  | 2026-11-16 | 3,096,682           | 1,179,255,356     | Block instance editing & canvas selection    |
| v0.17.0  | 2026-11-17 | 19,269,726          | 1,198,525,082     | Block instance delete, keyboard truth        |
| v0.18.0  | 2026-11-18 | 7,394,424           | 1,205,919,506     | The design system (contrast gate, kittest)   |
| v0.19.0  | 2026-11-19 | 1,353,455           | 1,207,272,961     | Snaps, sheets & full keyboard reach          |

Same-day windows: multiple tags landed on one day, so sessions that day are
assigned to the first tag; later same-day releases show 0 rather than
double-counting.

## Build-time extract for the in-app popup

```json
{"current_version": "0.19.0", "releases": [
  {"version": "0.1.0", "date": "2026-11-04", "tokens_this": 610929565, "tokens_cumulative": 610929565},
  {"version": "0.2.0", "date": "2026-11-05", "tokens_this": 70213294, "tokens_cumulative": 681142859},
  {"version": "0.5.0", "date": "2026-11-06", "tokens_this": 205358, "tokens_cumulative": 681348217},
  {"version": "0.8.0", "date": "2026-11-07", "tokens_this": 7346937, "tokens_cumulative": 688695154},
  {"version": "0.11.0", "date": "2026-11-11", "tokens_this": 146693132, "tokens_cumulative": 835388286},
  {"version": "0.13.0", "date": "2026-11-13", "tokens_this": 55187838, "tokens_cumulative": 890576124},
  {"version": "0.14.0", "date": "2026-11-14", "tokens_this": 263050570, "tokens_cumulative": 1153626694},
  {"version": "0.15.0", "date": "2026-11-15", "tokens_this": 22531980, "tokens_cumulative": 1176158674},
  {"version": "0.16.0", "date": "2026-11-16", "tokens_this": 3096682, "tokens_cumulative": 1179255356},
  {"version": "0.17.0", "date": "2026-11-17", "tokens_this": 19269726, "tokens_cumulative": 1198525082},
  {"version": "0.18.0", "date": "2026-11-18", "tokens_this": 7394424, "tokens_cumulative": 1205919506},
  {"version": "0.19.0", "date": "2026-11-19", "tokens_this": 1353455, "tokens_cumulative": 1207272961}
]}
```
