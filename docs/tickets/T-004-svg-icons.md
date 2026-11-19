# T-004: SVG Icon System

**Spec:** `docs/specs/v0.20.0-design-system-extensions.spec.md`
**Dependencies:** None
**Estimated scope:** Single PR

---

## Objective

Replace any Lucide/inline icon usage with custom SVG icons rasterized at exact pixel size, following `spaceui` pattern.

---

## Acceptance Criteria

- [ ] `assets/icons/` contains original SVG icons (MIT/Apache-2.0 compatible, listed in `ATTRIBUTION.md`)
- [ ] `theme.rs` adds `icon_texture(ctx, icon_name, size_pt) -> TextureHandle`
- [ ] No `lucide` crate in `Cargo.toml` — `grep -r lucide Cargo.toml` empty
- [ ] Icons render crisp at 16, 20, 24, 32pt
- [ ] Icon-only buttons carry AccessKit name via `labelled_by` (II.30.7.4)
- [ ] Unit test: load SVG → texture → size matches requested pt

---

## Files to Create/Modify

- `apps/lintel/assets/icons/` — new directory, SVG files
- `apps/lintel/src/theme.rs` — `icon_texture()` function
- `apps/lintel/src/components/icons.rs` — new file, `Icon` widget
- `apps/lintel/src/components/mod.rs` — export `icons`
- `apps/lintel/src/tests/icons.rs` — unit tests
- `ATTRIBUTION.md` — list icon sources/licences

---

## Notes

- Copy pattern from `spaceui` SVG rasterization at exact size
- Use `epaint::Image::from_texture()` with `fit_to_exact_size()`
- Prefer original icons (not Lucide copies) — clean-room (II.27.2)
- Icon names: semantic (`snap_endpoint`, `layer_visible`, `dimension_aligned`)