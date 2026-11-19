# T-006: ThemeSpec Three-Tier Token Restructure

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-001 (establishes COMPONENT_* tier)
**Estimated scope:** Single PR

---

## Objective

Restructure `theme.rs` into explicit three tiers per `egui-pop` DTCG model:
1. **BASE_*** — raw palette values (OKLCH/sRGB, no semantics)
2. **SEMANTIC_*** — system roles (surface, text, border, accent, focus, selection, danger/warning/success, palette categories)
3. **COMPONENT_*** — widget-specific aliases (button.*, field.*, card.*, etc.)

---

## Acceptance Criteria

- [ ] `theme.rs` has three clearly separated sections with comments:
  ```rust
  // === TIER 1: BASE PALETTE ===
  pub const BASE_PAPER: Color32 = ...;
  pub const BASE_INK: Color32 = ...;
  // ...
  
  // === TIER 2: SEMANTIC ROLES ===
  pub const SEMANTIC_SURFACE_BASE: Color32 = BASE_PAPER;
  pub const SEMANTIC_TEXT_PRIMARY: Color32 = BASE_INK;
  // ...
  
  // === TIER 3: COMPONENT TOKENS ===
  pub const COMPONENT_BUTTON_PRIMARY_BG: Color32 = SEMANTIC_ACCENT;
  // ...
  ```
- [ ] `Tokens` struct reads from COMPONENT_* only (widget code unchanged)
- [ ] `visuals()` and `style()` map from SEMANTIC_* → egui Visuals/Style
- [ ] Contrast test (T-007) asserts all three tiers in light/dark/high-contrast
- [ ] CI grep gate: `grep -rn "Color32::from_" apps/ crates/ | grep -v theme.rs` empty
- [ ] No behavioural change — only restructuring

---

## Files to Modify

- `apps/lintel/src/theme.rs` — restructure into three tiers
- `apps/lintel/src/tests/theme_tiers.rs` — test tier structure

---

## Notes

- Copy pattern from `egui-pop` DTCG three-tier architecture
- BASE_* values stay byte-identical (no visual change)
- SEMANTIC_* aliases BASE_* with intent names
- COMPONENT_* aliases SEMANTIC_* for widget consumption
- Density variants (compact/standard/comfortable) at SEMANTIC_* level