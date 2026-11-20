# Layout evidence — rendered screenshots (v0.20.0, ticket T-008)

Rendered proof of the flexbox-style layout helpers (ticket T-003):
`vstack`, `hstack`, `wrap_row` and `card` — including the narrow-window
reflow. Spec AC-04/AC-09; guide II.30.8 (evidence), II.30.7.6 (reflow).

## How these are produced

Headlessly in the `kittest` CI job — no display, no GPU on the reviewer's
machine:

```bash
LINTEL_GENERATE_EVIDENCE=1 cargo test -p lintel --bin lintel evidence_
```

The `evidence_layout_narrow_and_wide` test in
`apps/lintel/src/tests/evidence.rs` builds one sheet in an
`egui_kittest` harness, renders it at three window widths, and writes the
PNGs here.

## Files

| File | What it shows |
|------|---------------|
| `layout-narrow-400.png` | The sheet at 400 px: the button row wraps onto several lines inside its card, the compact cards stack, nothing clips |
| `layout-wide-1200.png` | The same sheet at 1200 px: the row spreads onto one line, cards sit side by side |
| `layout-cards.png` | The sheet at 800 px: `card` (padding, border, 8px radius, level-1 shadow) and the rail-style `tool_group` |

## Rules

- The PNGs are **generated artefacts, never committed** (guide II.30.8) —
  `.gitignore` excludes them; CI uploads them and the PR description embeds
  them.
- No pixel baselines: the gate is the always-on assertion
  `kittest_layout_resizes_without_overlap_or_clipping` in
  `apps/lintel/src/tests/evidence.rs` — it resizes 400 → 1200 px and proves
  through the AccessKit tree that every exposed button is ≥ 24 pt and that
  no two button rectangles intersect.
- All spacing/radius/colour values come from the token tiers in
  `apps/lintel/src/theme.rs` (`SPACING_*`, `RADIUS_*`, `SHADOW_LEVEL_1`).
