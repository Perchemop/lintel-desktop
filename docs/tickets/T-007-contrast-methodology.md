# T-007: Contrast Test Methodology Extension

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** T-006 (three-tier structure for testability)
**Estimated scope:** Single PR

---

## Objective

Extend existing `theme::contrast_test()` to `egui-pop` methodology: three palettes (light, dark, high-contrast), three thresholds (4.5:1 body, 3:1 UI parts, 7:1 high-contrast), CI gate.

---

## Acceptance Criteria

- [ ] `theme.rs` exports:
  ```rust
  pub fn contrast_test() -> Result<(), Vec<ContrastFailure>> { ... }
  pub enum ContrastFailure { Text { role, ratio, required }, Ui { role, ratio, required } }
  ```
- [ ] Tests all three palettes: `Theme::Light`, `Theme::Dark`, `Theme::HighContrast`
- [ ] Thresholds:
  - Body text (roles `text.primary`, `text.secondary`): ≥ 4.5:1
  - Large text (≥24pt or ≥18.66pt bold): ≥ 3:1
  - UI boundaries (`border.strong`, `focus.ring`, `selection.bg`): ≥ 3:1
  - High-contrast variant: ≥ 7:1 for all text
- [ ] CI gate: `cargo test contrast_test` in `.ci-required.txt`
- [ ] Test runs headless, no display (K.13)
- [ ] Failure output names role, measured ratio, required ratio

---

## Files to Modify

- `apps/lintel/src/theme.rs` — extend `contrast_test()`
- `apps/lintel/src/tests/contrast.rs` — test all palettes/thresholds
- `.ci-required.txt` — add `contrast-test` (or ensure `check` job runs it)

---

## Notes

- Copy methodology from `egui-pop` WCAG AA contrast tests
- Luminance math: `L = 0.2126*R + 0.7152*G + 0.0722*B` after sRGB linearisation
- Contrast = `(L1 + 0.05) / (L2 + 0.05)` where L1 > L2
- High-contrast palette: `Visuals::high_contrast()` seed + manual adjustments
- Record reduced-motion limitation in `ACCESSIBILITY.md` (II.21.1)