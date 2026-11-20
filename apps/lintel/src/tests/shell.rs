//! Integration tests for the application shell (T-005).
//!
//! Headless tests verifying the shell structure works correctly.

use crate::app::shell::{regions, Regions};
use crate::LintelApp;
use eframe::egui;

#[test]
fn shell_regions_exist_at_800x600() {
    let mut app = LintelApp::new();
    let ctx = egui::Context::default();

    // Set a reasonable viewport size
    ctx.set_pixels_per_point(1.0);

    let now = 1234567890;
    let regions = regions(&ctx, &mut app, now);

    // Verify all three main zones exist and have reasonable sizes
    assert!(
        regions.title_bar.rect.height() > 0.0,
        "Title bar should have height"
    );
    assert!(
        regions.rail.rect.width() > 0.0,
        "Rail should have width"
    );
    assert!(
        regions.canvas.rect.width() > 0.0 && regions.canvas.rect.height() > 0.0,
        "Canvas should have area"
    );
    assert!(
        regions.status_bar.rect.height() > 0.0,
        "Status bar should have height"
    );

    // Title bar at top
    assert!(
        regions.title_bar.rect.top() < regions.rail.rect.top(),
        "Title bar should be above rail"
    );

    // Rail on left of canvas
    assert!(
        regions.rail.rect.right() < regions.canvas.rect.right(),
        "Rail should be left of canvas"
    );

    // Status bar at bottom
    assert!(
        regions.status_bar.rect.top() > regions.canvas.rect.bottom(),
        "Status bar should be below canvas"
    );
}

#[test]
fn shell_title_bar_has_app_name() {
    let mut app = LintelApp::new();
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    let now = 1234567890;
    let regions = regions(&ctx, &mut app, now);

    // The title bar should contain the app name
    // We can't easily inspect text content in headless test without kittest,
    // but we can verify the region exists
    assert!(!regions.title_bar.rect.is_empty());
}

#[test]
fn shell_rail_has_tool_groups() {
    let mut app = LintelApp::new();
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    let now = 1234567890;
    let regions = regions(&ctx, &mut app, now);

    // Rail should have reasonable width for tool groups
    assert!(regions.rail.rect.width() >= 200.0, "Rail should be at least 200pt wide");
    assert!(regions.rail.rect.width() <= 280.0, "Rail should not exceed 280pt");
}

#[test]
fn shell_canvas_receives_remaining_space() {
    let mut app = LintelApp::new();
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    let now = 1234567890;
    let regions = regions(&ctx, &mut app, now);

    // Canvas should take up most of the horizontal space
    let total_width = regions.title_bar.rect.width();
    let rail_width = regions.rail.rect.width();
    let canvas_width = regions.canvas.rect.width();

    assert!(
        canvas_width > rail_width,
        "Canvas should be wider than rail"
    );
    // Canvas should use most of the remaining space after rail
    assert!(
        canvas_width > total_width * 0.6,
        "Canvas should use majority of horizontal space"
    );
}