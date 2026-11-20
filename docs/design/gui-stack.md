# Lintel GUI stack record (guide II.30.2)

Recorded: 2026-11-21 (v0.20.0 design-system slice, ticket T-008).
Every GUI recommendation and pattern in this repo is matched against the
versions below — never against a `Cargo.toml` range or another app's stack.

## Toolkit and resolved versions

| Crate | Manifest pin | **Resolved (`Cargo.lock`)** | Role |
|-------|--------------|------------------------------|------|
| `eframe` | `=0.36` | **0.36.2** | Window/integration shell |
| `egui` | `=0.36` | **0.36.2** | Immediate-mode UI toolkit |
| `egui_kittest` | `0.36` (dev, `wgpu` feature) | 0.36.0 | Headless AccessKit harness + test renderer |
| `resvg` | `0.45` | 0.45.x | Original-SVG icon rasterization (D-034) |
| `image` | `0.25` | 0.25.x | Window icon decode; evidence PNG encode |

The `=0.36` pin means 0.36.2 exactly for the egui family. The manifest range
is never trusted: egui 0.36 removed `Context::run`/`Context::begin_frame`
(use `Context::run_ui`), renamed `Rounding` → `CornerRadius` (`u8`, `From<f32>`
kept), and reshaped `Shadow` to `{ offset: [i8; 2], blur: u8, spread: u8,
color }`. Sibling agents' pre-0.36 habits were corrected against these APIs
in this slice.

## Shell shape

- Pure immediate-mode: everything renders into a plain `&mut Ui` — no
  retained scene graph, no web layer (guide II.30.1: no React/CSS design
  system can apply).
- Shell is the `spaceui`-pattern **Regions** structure:
  `crate::app::shell::regions(ctx, app, now) -> Regions { title_bar, rail,
  canvas, status_bar, inspector, palette }` (spec AC-06). The caller fills
  the regions; the shell owns only layout.
- Panels: one `CentralPanel` with an internal horizontal split (rail |
  canvas); inspector and command palette are overlays. The canvas shrinks
  last on narrow windows (II.30.7.6).

## Theme — one token source, three tiers (II.30.6)

`apps/lintel/src/theme.rs` is the **only** file allowed to construct a
`Color32` (CI grep gate). Layers:

1. **Tier 1 — `BASE_*`**: raw swatches (warm paper `#FCF8F2`, graphite ink,
   cobalt accent `#124E78`, …). No meaning attached.
2. **Tier 2 — semantic roles**: `SURFACE_*`, `TEXT_*`, `ACCENT_*`,
   `DANGER_*`, `SNAP_*`, … — what a colour *means*. Grouped in
   `ThemeSpec`/`SPEC` (`SPEC.semantic.*`), mirrored field-by-field against
   the flat registry by `tests/theme_tiers.rs`.
3. **Tier 3 — component tokens**: `BUTTON_*` (12 role×size specs, six states
   each), `PALETTE_*`, `WALL_*`, spacing (`SPACING_XS..2XL`, 4-pt base),
   radii (`RADIUS_XS/SM/MD/LG` = 2/4/8/12), elevation (`SHADOW_LEVEL_1..3`),
   `BUTTON_CORNER_RADIUS`. Widget code reads **only** tier 3
   (`SPEC.component.<group>.<token>` or the flat alias).

Re-hueing is a tier-1 edit; re-styling one component is a tier-3 edit.

## Component architecture

| Module | Pattern source (copy, never adopt — egui 0.36.2 incompatible with the source crates) | Notes |
|--------|-------------------------------------------------|-------|
| `components/buttons.rs` | shadcn-egui Button + modern-egui `StyledButton` + egui-pop component tokens | 4 roles × 3 sizes, six states, 24/32/40 pt floors, loading keeps size stable |
| `components/forms.rs` | shadcn-egui Field/FieldSet/FieldGroup | label → input via `Response::labelled_by`; error = stroke + painted icon + text; disabled explains why on hover |
| `components/icons.rs` | spaceui exact-size SVG | original SVGs, rasterized at exact physical pixels, AccessKit-labelled |
| `layout/helpers.rs` | shadcn-egui layout | `vstack`/`hstack`/`wrap_row`/`card` (+`card_compact`, `tool_group`), all returning `InnerResponse<R>` |

## Evidence and A11y harness (II.30.7 / II.30.8)

- **Gate, not pictures**: `egui_kittest` AccessKit assertions run headlessly
  in unit tests (`src/tests/kittest_buttons.rs`, `kittest_forms.rs`) and an
  always-on resize/no-overlap test (`src/tests/evidence.rs`). The required
  CI check is `kittest` (`.ci-required.txt`).
- **Pictures**: `LINTEL_GENERATE_EVIDENCE=1 cargo test -p lintel --bin
  lintel evidence_` (run in the `kittest` job) renders
  `docs/qc/{buttons,form,layout}-evidence/*.png` via the harness's wgpu test
  renderer and uploads them as workflow artefacts for the PR. PNGs are
  never committed; there are no pixel baselines.
- **Contrast is a unit test**: `theme_colours_meet_wcag_aa` +
  `tier_*_resolve_*` in `theme.rs` and `tests/theme_tiers.rs` assert 4.5:1
  body text / 3:1 UI graphics against every surface they sit on
  (light drafting palette; a dark Lintel palette is not shipped yet — dark
  evidence shows egui's dark chrome around the same components).
- Keyboard: Tab order follows visual order in buttons and forms; shortcut
  table in `docs/qc/shortcuts.md`.

## CI gates that enforce this record

- `check`: `cargo fmt --check`, `clippy --all-targets --workspace -D
  warnings`, `cargo test --workspace` (headless), version-consistency gate.
- `kittest`: AccessKit harness job + evidence generation/artefact upload
  (swiftshader Vulkan, `DISPLAY=:99`).
- Raw-colour grep: `Color32::from_*` only inside `theme.rs`.
