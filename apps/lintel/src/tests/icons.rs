// Unit tests for the SVG icon system (ticket T-004, spec v0.20.0 AC-05),
// wired as `crate::tests::icons` from src/tests.rs. Headless: textures are
// CPU-side epaint images, no display or GPU needed (addendum K.13).

use crate::components::icons::{Icon, label_of};
use crate::theme;

#[test]
fn every_registered_icon_rasterizes_at_the_requested_point_sizes() {
    let ctx = eframe::egui::Context::default();
    // The crisp set from the spec: 16, 20, 24 and 32 pt. A bare context has
    // pixels_per_point() == 1.0, so one point is one physical pixel.
    for &name in theme::ICON_NAMES {
        for &size_pt in &[16.0f32, 20.0, 24.0, 32.0] {
            let handle = theme::icon_texture(&ctx, name, size_pt);
            let expected = size_pt as usize;
            assert_eq!(
                handle.size(),
                [expected, expected],
                "icon {name} at {size_pt} pt must rasterize to {expected}x{expected} physical pixels",
            );
        }
    }
}

#[test]
fn the_texture_cache_returns_the_same_handle_for_the_same_name_and_size() {
    let ctx = eframe::egui::Context::default();
    let first = theme::icon_texture(&ctx, "snap_grid", 24.0);
    let second = theme::icon_texture(&ctx, "snap_grid", 24.0);
    assert_eq!(
        first.id(),
        second.id(),
        "a repeated (name, size) request must hit the cache, not re-upload"
    );
    let larger = theme::icon_texture(&ctx, "snap_grid", 32.0);
    assert_ne!(
        first.id(),
        larger.id(),
        "a different size must rasterize its own texture"
    );
    let other = theme::icon_texture(&ctx, "snap_ortho", 24.0);
    assert_ne!(
        first.id(),
        other.id(),
        "a different name must rasterize its own texture"
    );
}

#[test]
fn rasterization_follows_pixels_per_point() {
    let ctx = eframe::egui::Context::default();
    ctx.set_pixels_per_point(2.0);
    // A new scale takes effect at the start of the next pass (egui applies
    // it as the zoom factor), so run one warm-up pass before rasterizing.
    let _ = ctx.run_ui(Default::default(), |_ui| {});
    assert_eq!(ctx.pixels_per_point(), 2.0, "warm-up pass must apply the scale");

    let handle = theme::icon_texture(&ctx, "snap_endpoint", 24.0);
    assert_eq!(
        handle.size(),
        [48, 48],
        "24 pt at 2x display scale must be 48 physical pixels"
    );
}

#[test]
fn an_unknown_icon_name_falls_back_to_the_placeholder_without_panicking() {
    let ctx = eframe::egui::Context::default();
    let handle = theme::icon_texture(&ctx, "not_an_icon", 24.0);
    assert_eq!(
        handle.size(),
        [24, 24],
        "the placeholder must come back at the requested size"
    );
}

#[test]
fn degenerate_sizes_clamp_to_one_pixel() {
    let ctx = eframe::egui::Context::default();
    let zero = theme::icon_texture(&ctx, "snap_grid", 0.0);
    assert_eq!(zero.size(), [1, 1], "a zero size still yields one pixel");
    let negative = theme::icon_texture(&ctx, "snap_grid", -5.0);
    assert_eq!(negative.size(), [1, 1], "a negative size still yields one pixel");
}

#[test]
fn every_icon_rasterizes_inked_pixels() {
    // A malformed SVG degrades to a blank image with a log line; this test
    // pins every registered asset to actually paint ink on a transparent
    // sheet, so a broken file cannot slip through silently.
    for &name in theme::ICON_NAMES {
        let image = theme::rasterize_icon(name, 24);
        let inked = image.pixels.iter().filter(|c| c.a() > 0).count();
        assert!(
            inked > 0,
            "icon {name} painted no ink — check assets/icons/{name}.svg"
        );
        assert!(
            inked < image.pixels.len(),
            "icon {name} filled the whole sheet — the background must stay transparent"
        );
    }
}

#[test]
fn the_registry_holds_the_full_cad_set() {
    // The twelve CAD-workflow icons the ticket requires.
    assert_eq!(theme::ICON_NAMES.len(), 12);
    for name in [
        "snap_endpoint",
        "snap_midpoint",
        "snap_center",
        "snap_intersection",
        "snap_grid",
        "snap_ortho",
        "layer_visible",
        "layer_lock",
        "layer_print",
        "dimension_aligned",
        "dimension_radial",
        "block_insert",
    ] {
        assert!(
            theme::ICON_NAMES.contains(&name),
            "{name} must be registered in theme::ICON_NAMES"
        );
    }
}

#[test]
fn every_icon_has_a_human_readable_accesskit_label() {
    for &name in theme::ICON_NAMES {
        let label = label_of(name);
        assert!(!label.is_empty(), "icon {name} must carry an AccessKit label");
        assert_ne!(
            label, "Unknown icon",
            "icon {name} must map to a real label, not the fallback"
        );
    }
}

// --- AccessKit assertions through the egui_kittest harness (guide II.30.8) ---

#[test]
fn the_icon_widget_carries_its_accesskit_label_at_its_exact_size() {
    use egui_kittest::kittest::Queryable as _;

    let harness = egui_kittest::Harness::new_ui(|ui| {
        Icon::new("snap_endpoint").size(32.0).show(ui);
    });

    let node = harness
        .query_by_role_and_label(egui::accesskit::Role::Image, "Snap to endpoint")
        .expect("the icon must be an AccessKit image node named 'Snap to endpoint'");
    let rect = node.rect();
    assert!(
        (rect.width() - 32.0).abs() < 0.5 && (rect.height() - 32.0).abs() < 0.5,
        "the icon must occupy exactly its requested point size, got {rect:?}"
    );
}

#[test]
fn an_icon_only_button_is_named_via_labelled_by() {
    use egui_kittest::kittest::Queryable as _;

    let harness = egui_kittest::Harness::new_ui(|ui| {
        Icon::new("layer_lock").size(16.0).icon_button(ui);
    });

    // The button has no text of its own: its AccessKit name must come from
    // the icon's label through Response::labelled_by (guide II.30.7.4).
    let button = harness.query_by_role_and_label(egui::accesskit::Role::Button, "Layer lock");
    assert!(
        button.is_some(),
        "the icon-only button must be named for screen readers via labelled_by"
    );
}

#[test]
fn the_icon_button_meets_the_24pt_target_floor() {
    use egui_kittest::kittest::Queryable as _;

    let harness = egui_kittest::Harness::new_ui(|ui| {
        Icon::new("snap_grid").size(16.0).icon_button(ui);
    });

    // The icon's label resolves to the button (kittest excludes the node
    // acting as the label), so the query returns the button node itself.
    let button = harness
        .query_by_label("Snap to grid")
        .expect("the icon-only button must be found by its resolved label");
    let rect = button.rect();
    assert!(
        rect.width() >= 24.0 && rect.height() >= 24.0,
        "icon-only buttons must meet the 24 pt target floor (guide II.30.7.5), got {rect:?}"
    );
}
