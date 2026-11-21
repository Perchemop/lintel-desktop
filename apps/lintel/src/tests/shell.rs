//! Integration tests for the application shell (T-005).
//!
//! Headless tests verifying the shell regions render and lay out at a real
//! window size. No window server, no GPU (guide K.13): egui's immediate-mode
//! passes run entirely in software here.

use crate::LintelApp;
use crate::app::shell::regions;
use eframe::egui;
use std::cell::RefCell;

const WINDOW_SIZE: egui::Vec2 = egui::vec2(800.0, 600.0);

/// Run one 800×600 pass and hand the panel `Ui` to the shell.
fn shell_pass(app: &mut LintelApp, now: i64) -> crate::app::shell::Regions {
    let ctx = egui::Context::default();
    ctx.set_pixels_per_point(1.0);

    let slot = RefCell::new(None);
    let mut input = egui::RawInput::default();
    input.screen_rect = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, WINDOW_SIZE));

    let _ = ctx.run_ui(input, |root| {
        egui::CentralPanel::default().show(root, |ui| {
            *slot.borrow_mut() = Some(regions(ui, app, now));
        });
    });

    slot.into_inner().expect("the shell pass ran")
}

#[test]
fn shell_regions_exist_at_800x600() {
    let mut app = LintelApp::new();
    let regions = shell_pass(&mut app, 1234567890);

    assert!(
        regions.title_bar.rect.height() > 0.0,
        "title bar should have height"
    );
    assert!(regions.rail.rect.width() > 0.0, "rail should have width");
    assert!(
        regions.canvas.rect.width() > 0.0 && regions.canvas.rect.height() > 0.0,
        "canvas should have area"
    );
    assert!(
        regions.status_bar.rect.height() > 0.0,
        "status bar should have height"
    );

    // Title bar at top.
    assert!(
        regions.title_bar.rect.top() < regions.rail.rect.top(),
        "title bar should be above rail"
    );

    // Rail on the left of the canvas.
    assert!(
        regions.rail.rect.right() < regions.canvas.rect.right(),
        "rail should be left of canvas"
    );

    // Status bar below the canvas.
    assert!(
        regions.status_bar.rect.top() > regions.canvas.rect.bottom(),
        "status bar should be below canvas"
    );
}

#[test]
fn shell_title_bar_has_app_name() {
    let mut app = LintelApp::new();
    let regions = shell_pass(&mut app, 1234567890);

    // The AccessKit-visible app name is asserted in kittest_forms.rs; here we
    // prove the region itself is present and non-empty.
    assert!(
        regions.title_bar.rect.area() > 0.0,
        "title bar should occupy real space"
    );
}

#[test]
fn shell_rail_has_tool_groups() {
    let mut app = LintelApp::new();
    let regions = shell_pass(&mut app, 1234567890);

    assert!(
        regions.rail.rect.width() >= 200.0,
        "rail should be at least 200pt wide"
    );
    assert!(
        regions.rail.rect.width() <= 280.0,
        "rail should not exceed 280pt"
    );
}

#[test]
fn shell_canvas_receives_remaining_space() {
    let mut app = LintelApp::new();
    let regions = shell_pass(&mut app, 1234567890);

    let total_width = regions.title_bar.rect.width();
    let rail_width = regions.rail.rect.width();
    let canvas_width = regions.canvas.rect.width();

    assert!(
        canvas_width > rail_width,
        "canvas should be wider than rail"
    );
    assert!(
        canvas_width > total_width * 0.6,
        "canvas should use the majority of horizontal space"
    );
}
