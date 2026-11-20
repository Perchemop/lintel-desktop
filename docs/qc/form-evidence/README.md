# Form evidence — rendered screenshots (v0.20.0, ticket T-008)

Rendered proof of the `Field` / `FieldSet` / `FieldGroup` form components
(ticket T-002): label + input + error + help slots, the framed field set, and
the error state. Spec AC-03/AC-09; guide II.30.8 (evidence), II.30.7 (a11y).

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
| `form-light.png` | A "Storey" `FieldSet` (Name + Elevation with help hints) plus a wall-length `Field` with an error, and the Save/Cancel action row — light theme |
| `form-dark.png` | The same form with egui's dark visuals |
| `form-error.png` | The error state close-up: 2px danger stroke on the input, painted warning icon, and the message "Length must be positive" — never colour alone (II.30.7.2) |

## Rules

- The PNGs are **generated artefacts, never committed** (guide II.30.8) —
  `.gitignore` excludes them; CI uploads them and the PR description embeds
  them.
- No pixel baselines: the gate is the AccessKit assertions in
  `apps/lintel/src/tests/kittest_forms.rs` — every input carries a labelled-by
  relation to its label, the error and help texts are announced as label
  nodes, and Tab follows visual order (Name → Elevation → submit).
- Disabled fields explain why on hover (`Field::disabled_reason`,
  II.30.11); all colours/spacing come from the token tiers in
  `apps/lintel/src/theme.rs`.
