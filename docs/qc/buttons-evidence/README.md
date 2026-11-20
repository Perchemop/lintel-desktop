# Button evidence — rendered screenshots (v0.20.0, ticket T-008)

Rendered proof of the `StyledButton` system (ticket T-001): 4 roles × 3 sizes
= 12 variants, plus the disabled/loading/selected states and the keyboard
focus ring. Spec AC-02/AC-09; guide II.30.8 (evidence), II.30.10 (recipe).

## How these are produced

Headlessly in the `kittest` CI job — no display, no GPU on the reviewer's
machine:

```bash
LINTEL_GENERATE_EVIDENCE=1 cargo test -p lintel --bin lintel evidence_
```

Each `evidence_*` test in `apps/lintel/src/tests/evidence.rs` builds its
sheet in an `egui_kittest` harness, renders the frame with the harness's
wgpu test renderer, and writes the PNG here.

## Files

| File | What it shows |
|------|---------------|
| `buttons-light.png` | All 12 variants (Primary/Secondary/Ghost/Danger × sm/md/lg) on the light drafting theme, grouped in role cards |
| `buttons-dark.png` | The same sheet with egui's dark visuals — Lintel's component tokens stay light-tuned; the dark sheet shows how the components sit on egui's dark chrome |
| `buttons-states.png` | Disabled (hover explains why), disabled+reason, loading (spinner, size stable), selected, danger-loading, and a normal reference button |
| `buttons-focus.png` | The sheet after one Tab press: the first button holds keyboard focus and paints its 2px cobalt focus ring |

## Rules

- The PNGs are **generated artefacts, never committed** (guide II.30.8) —
  `.gitignore` excludes them; CI uploads them and the PR description embeds
  them.
- There are **no pixel baselines** and no snapshot comparisons: the gate is
  the `egui_kittest` AccessKit assertions in
  `apps/lintel/src/tests/kittest_buttons.rs` (role=button, accessible name,
  ≥24pt target, Tab reaches every variant); the pictures are for human
  review.
- Every colour in the sheets resolves through the three token tiers in
  `apps/lintel/src/theme.rs`; no raw `Color32` outside the theme module.
