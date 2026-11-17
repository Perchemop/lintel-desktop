# Design Audit — egui Ecosystem for Lintel (egui 0.36.2)

**Date:** 2026-11-18  
**Public sources — original summaries**  
**Human gate: approval required before implementation depends on this**

---

## Context
Lintel is a pure Rust + egui/eframe 2D architectural drafting app (resolved egui 0.36.2). It already has its own token module (`apps/lintel/src/theme.rs`) defining semantic colours (`PAPER`, `INK`, `COBALT`, `GREEN`, `WALL_FILL`, `GRID_MINOR/MAJOR`, `PANEL_BG`, `STATUS_BG`, `FOCUS_RING` via `COBALT`, etc.), spacing constants, and a contrast-test function. Per guide II.30.4, **one design path per app** is a hard limit — adopting a full design system crate would replace Lintel's existing token module, not layer on top.

---

## 1. shadcn-egui (oetiker/shadcn-egui)

### What it offers
Complete shadcn/ui v4 port to egui: 6 preset styles (vega, nova, maia, lyra, mira, luma, rhea), OKLCH colour themes, 20+ font families (Geist, JetBrains Mono, Inter, etc. as Cargo features), 4 icon libraries (Lucide, Tabler, Phosphor, Remix, Hugeicons as features), and a full component suite (Button, Badge, Input, Label, Card, Tabs, Switch, Checkbox, Select, Slider, Separator, Tooltip, Popover, Dialog, IconView, Field/FieldSet/FieldGroup for forms, Spinner, Avatar, AvatarGroup, Kbd, Progress, Alert, Textarea). Includes **flexbox-style layout helpers** (`layout::vstack`, `hstack`, `wrap_row`, `card`) that work around egui's single-pass layout. Preset codes from ui.shadcn.com can be applied directly.

### egui version compatibility
**Workspace pins egui 0.37** (see `Cargo.toml` workspace.dependencies: `egui = { version = "0.37", features = ["default_fonts", "monochrome_emoji_fonts"] }`). The 0.8.0 release (2026-11-05) targets egui 0.37. **Not compatible with egui 0.36.2** without upgrading Lintel's egui version.

### Licence
MIT

### Recommendation: **copy pattern only**
Lintel cannot adopt this without an egui version bump (separate slice). However, the **component API patterns** (Button variants, Field/Form composition, layout helpers) and **preset/token architecture** (OKLCH scales, semantic roles, density presets) are worth studying and reimplementing in Lintel's own token module. The flexbox layout helpers solve real egui pain points (wrapping rows, equal-height cards) — copy the pattern, not the crate.

---

## 2. egui-pop / egui-pop-tokens (quazardous/egui-pop)

### What it offers
A full **application framework + design system** for egui desktop apps. Three-tier token system (DTCG 2025.10 format):
- **Reference palette**: warm neutral `gray.0..12` + 5 hue scales (`gold`, `amber`, `red`, `green`, `blue`) with `contrast` step — step semantics documented (0-2 deep backgrounds, 3-5 subtle fills, 6-7 borders, 8 pressed, 9 solid accent, 10 hover, 11 low-contrast text, 12 high-contrast text)
- **System roles**: `surface.{canvas,panel,raised,hover,active,overlay,sunken}`, `text.{default,muted,strong,on-accent}`, `border.{default,strong}`, `accent.{default,hover,active,subtle}`, `danger/warning/success/info`, `focus.ring`, `selection`, `palette.category.0..9`, `palette.log.*`, `palette.axis.*`, `radius.*`, `shadow.*`, `space.*` (intent-based: `inline`, `group`, `section`, `page`), `text-role.*` (title/section/body/muted/meta with size/color/weight), `size.*`, `font.*`, `motion.*`, `opacity.scrim`
- **Component tokens**: per-widget aliases of system tokens (button.*, card.*, slider.*, field.*, progress-*, indicator-*, skeleton.*, dialog.*, palette-window.*, toast.*, scroll-bar.*)

Features: **ThemeSpec composition** (base theme + density overlay + accent seed colour + reduced motion + user overrides + fonts), **WCAG AA contrast tests** (4.5:1 text, 3:1 UI parts, 7:1 high-contrast) run in CI, **YAML/JSON token files** compiled at build time, **theme editor** (live editing in-app), **workbench shell** (activity bar, status bar, splitter, tree view, editor tabs), **docking via egui_stack** (egui_dock successor), **components** (buttons, badges, forms, charts, overlays, navigation), **i18n**, **agents/hooks**, **test macros**.

### egui version compatibility
**Workspace pins egui 0.37** (see `Cargo.toml`: `egui = { version = "0.37", default-features = false }`). The crate is built against egui 0.37. **Not compatible with egui 0.36.2** without upgrading.

### Licence
MIT OR Apache-2.0

### Recommendation: **copy pattern only**
The **DTCG token format**, **three-tier architecture**, **ThemeSpec composition model**, **contrast test methodology**, and **semantic role vocabulary** are excellent and directly applicable to Lintel's existing token module. The workbench shell (activity bar + status bar + docked panels) matches Lintel's planned shell restructure (guide II.27.12). However, egui-pop is a **framework** (host contract, lifecycle hooks, agent system) — adopting it would be a major architectural shift, not a design-system adoption. Copy the token patterns; do not adopt the crate on egui 0.36.

---

## 3. spaceui (lero-aom/spaceui)

### What it offers
A design system with **semantic Theme** (Palette, Metrics, Fonts, Backdrop), **anti-aliased painting** (gradients, polygon fills, glows), **70+ SVG icons** (rasterized at exact pixel size), **widgets** (buttons, pills, tabs, cards, switches, fields, sliders), **shell** (custom title bar, optional icon rail, card/column layout), **chat kit**. Two built-in themes: `carbon_copper` (dark, copper accent) and `deep_space` (indigo/violet with starfield backdrop). Installs via `Theme::install(&ctx)` which sets egui visuals, spacing, text styles, and fonts.

### egui version compatibility
**Minimum supported Rust version: 1.95, the minimum of egui 0.37** (README: "Raising it is a minor-version change"). Targets egui 0.37+. **Not compatible with egui 0.36.2**.

### Licence
Apache-2.0

### Recommendation: **copy pattern only**
The **shell abstraction** (title bar + rail + columns returned as `Regions` to fill) is a clean pattern for Lintel's planned shell restructure. The **SVG icon rasterization at exact size** and **anti-aliased painting helpers** are useful techniques. The semantic `Theme` struct (palette + metrics + fonts + backdrop) mirrors Lintel's token approach. Copy patterns; crate requires egui 0.37.

---

## 4. modern-egui (marc0x71/modern-egui)

### What it offers
Lightweight theme + `Ui` extension traits. **One-line setup**: `theme::apply(&ctx)` installs dark+light palettes, maps to egui `Visuals`, applies typography/spacing metrics. **Semantic palette**: surfaces (`bg`, `surface`, `surface_alt`, `elevated`), borders (`border`, `border_strong`), text (`text`, `text_strong`, `text_muted`), accent (`accent`, `accent_hover`, `accent_active`, `on_accent`), danger, status (`success`, `warning`). **Design tokens** from single `EM = 14.0` base: font sizes (`FONT_XS..2XL`), spacing (`SPACE_1..6`), control sizes, corner radii, layout sizes. **Ui extensions**: `ui.section_title()`, `ui.muted_label()`, `ui.space_group()`, `ui.primary_button()`, `ui.secondary_button()`, `ui.ghost_button()`, `ui.danger_button()`, `ui.text_input_hint()`. **StyledButton** and **StyledText** widgets. Custom palettes via `apply_with()`. Automatic theme switching via `ctx.set_theme()`.

### egui version compatibility
**Targets eframe/egui 0.36** (README: "`modern-egui` currently targets **eframe/egui 0.36** and requires Rust 1.95 or newer (same MSRV as egui 0.36)"). **Compatible with Lintel's egui 0.36.2**.

### Licence
Apache-2.0

### Recommendation: **adopt dependency** (with caveats)
This is the **only crate in the survey that explicitly supports egui 0.36**. It is lightweight (theme + helpers, no framework), respects `ctx.set_theme()`, and its semantic palette closely matches Lintel's existing roles (`PAPER` ≈ `surface`, `INK` ≈ `text`, `COBALT` ≈ `accent`, `GREEN` ≈ `success`, `ERROR` ≈ `danger`, `STATUS_BG` ≈ `surface_alt`). Caveat: Lintel already has a token module — adopting modern-egui would mean **replacing** it (one design path rule). If Lintel wants to keep its custom warm-paper palette and architectural colours (WALL_FILL, GRID_MINOR/MAJOR, DOOR_LEAF), it should **copy modern-egui's patterns** (EM-based token scale, Ui extension traits, `apply`/`apply_with` pattern, semantic palette structure) rather than adopt the crate wholesale. **Recommendation: copy pattern only** to preserve Lintel's domain-specific tokens, but modern-egui is the only crate that could be adopted without an egui upgrade.

---

## 5. egui_sauge (macnjil4/egui_sauge)

### What it offers
Fresh, natural design system: **sage palette + warm neutrals**, **WCAG AA validated** (tests/contrast.rs asserts 4.5:1 for normal text on all surface pairs, 3:1 for large text). **Palette** struct with 20 semantic roles (light + dark): `bg_app`, `bg_surface`, `bg_surface_alt`, `text_primary`, `text_secondary`, `text_tertiary`, `text_on_brand`, `brand_default`, `brand_hover`, `brand_active`, `success`, `warning`, `error`, `info`, `border`, `border_strong`, `focus`, `overlay`, `shadow`. **SPACING** / **RADIUS** / **Elevation** token tables. **Density** preset (`Comfortable` / `Compact`) and **Locale** (`En` / `Fr`). ~80 Phosphor icons (feature-gated). **Components**: Workbench (IDE shell: `Workbench`, `ActivityBar`, `StatusBar`, `Splitter`, `TreeView`, `EditorTabs`), Buttons (Primary/Secondary/Ghost/Danger × Sm/Md/Lg + icons), Badges/Tags/StatusDot/Kbd/Avatar, Feedback (Spinner, ProgressBar, Alert, Toast), Charts (Sparkline, Gauge), Containers (Card, Section, EmptyState, Stat, CodeBlock, Accordion), Forms (InputField, SelectField, Switch, Checkbox, RadioGroup, NumberField), Overlays (Dialog, ConfirmDialog, Drawer, **CommandPalette** ⌘K fuzzy search, MenuItem, SubMenu, Tooltip), Navigation (NavItem, Tabs, Breadcrumb, PageHeader). GUIDE.md documents UX playbook.

### egui version compatibility
**Pins egui 0.34** (Cargo.toml: `egui = "0.34"`, `eframe = "0.34"`). Published 2026-04-26. **Not compatible with egui 0.36.2** — would require downgrade or waiting for egui_sauge to update.

### Licence
Apache-2.0

### Recommendation: **copy pattern only**
Excellent **WCAG AA test methodology** (explicit contrast assertions for every text/background pair in both modes) — Lintel should adopt this pattern in its own token tests. The **CommandPalette component** (fuzzy search, keyboard nav) is directly relevant to Lintel's Ctrl+K palette (CAP-011). The **Workbench shell** (ActivityBar + StatusBar + EditorTabs) matches Lintel's planned chrome. The **semantic palette roles** are well-chosen. But egui 0.34 pin makes adoption impractical. Copy patterns.

---

## 6. egui-elegance / elegance (matrix-research-inc/egui-elegance)

### What it offers
Opinionated widgets for "modern web UI feel": **6-accent rounded buttons** (Primary/Secondary/Ghost/Danger/Outline + 6 accent colours × 3 sizes), **TextInput with focus ring + submit-flash feedback**, **Select**, **Tabs**, **SegmentedButton** (LED toggles), **StatusPill**, **Badge**, **Card**, **Accordion**, **Modal**, **Drawer**, **Toast/Toasts**, **Tooltip**, **Popover**, **ContextMenu**, **MenuBar**, **Dock** (tab docking, drag, floating — wraps egui_stack), **FileDropZone**, **ColorPicker**, **Knob**, **Gauges**, **LogBar**, **ThemeSwitcher**. **Four built-in themes** (paired dark/light: `slate`/`frost`, `charcoal`/`paper`) sharing shape/typography for layout-shift-free toggling. **Theme.install(ctx)** stores theme in context memory AND updates egui `Visuals`/`Style` so built-in widgets inherit palette.

### egui version compatibility
**v0.15.0+ targets egui 0.36** (CHANGELOG: "`egui`, `eframe`, and `egui_kittest` now target `0.36`. Because elegance re-exports egui types in its public API, **your app must move to egui 0.36**"). Latest 0.17.4 (2026-10) works with egui 0.36+. **Compatible with Lintel's egui 0.36.2**.

### Licence
MIT OR Apache-2.0

### Recommendation: **copy pattern only** (or **adopt dependency** if Lintel wants a component library)
The **paired dark/light themes with identical metrics** (no layout shift on toggle) is a strong pattern for Lintel. The **Dock widget** (wrapping egui_stack) solves the docking requirement for Lintel's shell restructure. The **focus ring + submit-flash on TextInput** is a polished UX detail. The **6-accent button system** with consistent sizing (icon buttons same height as text buttons) is a good component recipe. However, elegance is **opinionated** (chunky rounded, web-app aesthetic) — may not match Lintel's warm-paper architectural feel. The crate re-exports egui types, so version lock is tight. **If Lintel wants a component library and can accept the aesthetic, adopt dependency. Otherwise copy patterns** (paired themes, Dock integration, focus-ring TextInput, button sizing rules).

---

## 7. theme-egui (WyattAu/theme-egui)

### What it offers
**Single-file adapter** lowering a `theme-kit` `TokenGraph` into egui `Visuals`. **Zero colour maths** — every output colour is byte-identical to the role the gate approved. Contract: `to_visuals(&theme) -> Result<Visuals, ThemeError>`. Provides `seed_theme()` (midnight palette, gated) and `seed_visuals()` (pre-lowered). The `theme-kit` crate (separate) handles token graphs, contrast auditing, variant derivation. This crate is **only the egui lowering layer**.

### egui version compatibility
**Supports egui >=0.36, <0.38** (Cargo.toml: `egui = { version = ">=0.36, <0.38", default-features = false }`). **Compatible with Lintel's egui 0.36.2**.

### Licence
MIT OR Apache-2.0

### Recommendation: **process only**
This is not a design system — it's a **lowering adapter** for a separate token system (`theme-kit`). Lintel already has its own token module. The **principle** (separate token maths from toolkit lowering; audit contrast in token layer; lowering is pure placement) is sound and matches guide II.30.6. **Adopt the process pattern**: keep Lintel's token crate independent of egui, write a lowering function that produces `Visuals`/`Style` from tokens, run contrast tests on tokens headless.

---

## 8. egui_kittest (emilk/egui, crates/egui_kittest)

### What it offers
**UI testing library based on AccessKit (kittest)**. `Harness::new_ui(app)` runs a headless egui app. **AccessKit queries**: `get_by_label()`, `get_by_role()`, `accesskit_node().toggled()`, `.click()`, `.focus()`. **Accessibility check**: after `Harness::run()`, verifies every input widget has an accessible name (panics if not) — icon-only buttons need alt text, text fields need `labelled_by`. **Snapshot testing** (feature `wgpu` + `snapshot`): `harness.snapshot("name")` renders to PNG, compares against `tests/snapshots/` with per-pixel YIQ threshold (default 0.6) and `max_failed_pixels` (default 0). Configurable via `kittest.toml` (per-OS thresholds). **Recording** (feature `recording`): `KITTEST_RECORD=1 cargo test` writes MP4s. **eframe support** (feature `eframe`): test `eframe::App` directly.

### egui version compatibility
**Version 0.36.2 exists** (crates.io: 0.36.2 published 2026-09-08, 0.36.1 2026-08-07, 0.36.0 2026-08-05). **Compatible with Lintel's egui 0.36.2**.

### Licence
MIT OR Apache-2.0

### Recommendation: **adopt dependency** (process/CI only)
This is **not a design system** — it's a **test harness**. It should be adopted as a **dev-dependency** for accessibility and snapshot testing in CI. It directly implements guide II.30.8 evidence requirements (AccessKit assertions for keyboard focus, dialog dismissal, empty states; snapshot renders for visual regression). No runtime cost; runs only in `#[cfg(test)]`. **Adopt for CI gates**.

---

## Summary Table

| Crate | egui 0.36 compatible? | Type | Recommendation |
|-------|----------------------|------|----------------|
| shadcn-egui | ❌ (0.37) | Full design system + components | Copy patterns (component API, layout helpers, preset architecture) |
| egui-pop / egui-pop-tokens | ❌ (0.37) | Framework + 3-tier DTCG tokens | Copy patterns (DTCG format, ThemeSpec, contrast tests, semantic roles, workbench shell) |
| spaceui | ❌ (0.37+) | Design system + shell + icons | Copy patterns (shell Regions, SVG rasterization, Theme struct) |
| modern-egui | ✅ (0.36) | Lightweight theme + Ui extensions | **Copy patterns** (EM tokens, Ui extensions, apply/apply_with, semantic palette) — only crate that could be adopted, but replaces Lintel's token module |
| egui_sauge | ❌ (0.34) | Design system + components + CommandPalette | Copy patterns (WCAG test methodology, CommandPalette, Workbench shell, semantic palette) |
| egui-elegance | ✅ (0.36+) | Opinionated component library + Dock | Copy patterns (paired themes, Dock, focus-ring TextInput, button sizing) — adopt only if accepting web-app aesthetic |
| theme-egui | ✅ (0.36-0.37) | Lowering adapter only | Process only (separate token maths from lowering) |
| egui_kittest | ✅ (0.36.2) | AccessKit/snapshot testing | **Adopt dependency** (dev-only, CI gates) |

---

## Alignment with Lintel's Existing Token Module (`apps/lintel/src/theme.rs`)

Lintel's token module already defines:
- **Colours**: `PAPER`, `INK`, `COBALT`, `GREEN`, `WALL_FILL`, `WALL_STROKE`, `WALL_DRAFT`, `GRID_MINOR`, `GRID_MAJOR`, `OPENING_GAP`, `DOOR_LEAF`, `WINDOW_LINE`, `PANEL_BG`, `PANEL_BORDER`, `STATUS_BG`, `HINT`, `STATUS_OK`, `ERROR`, `PALETTE_*` (palette-specific), `TRANSPARENT`
- **Contrast maths**: `relative_luminance()`, `contrast_ratio()`, `selection_stroke()`
- **No spacing/typography scale** — these are hard-coded in UI code

**Gaps to fill from ecosystem patterns:**
1. **Spacing scale** (EM-based or 4-pt base: 2/4/8/12/16/24/32/48) — from modern-egui, egui-pop, egui_sauge
2. **Radius scale** (2/4/8/12) — from modern-egui, egui-pop
3. **Typography scale** (display/h1/h2/body/small/mono with ratio 1.125 or 1.2) — from modern-egui, egui-pop, shadcn-egui
4. **Elevation/shadow tokens** — from egui-pop, egui_sauge, spaceui
5. **Density presets** (Comfortable/Compact) — from egui-pop, egui_sauge
6. **Reduced motion token** — from egui-pop
7. **Focus ring token** (already uses COBALT; formalize as `focus.ring`) — from all
8. **Semantic role naming** (surface.base/raised/sunken, text.primary/secondary/muted/disabled, accent/on-accent, danger/warning/success, border.subtle/strong) — from egui-pop, modern-egui, egui_sauge

---

*End of document. Human gate: approval required before implementation depends on this.*