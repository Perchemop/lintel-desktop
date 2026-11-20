//! Layout helpers: vstack, hstack, wrap_row, card.
//!
//! All spacing, radii, and colours from theme tokens — no raw Color32.

use crate::theme;
use eframe::egui;

/// Vertical stack with equal spacing between children.
///
/// ```rust
/// vstack(ui, theme::SPACING_MD, |ui| {
///     ui.label("First");
///     ui.label("Second");
///     ui.label("Third");
/// });
/// ```
pub fn vstack(ui: &mut egui::Ui, spacing: f32, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = spacing;
        add_contents(ui);
    });
}

/// Horizontal stack with equal spacing between children.
///
/// ```rust
/// hstack(ui, theme::SPACING_MD, |ui| {
///     ui.button("A");
///     ui.button("B");
/// });
/// ```
pub fn hstack(ui: &mut egui::Ui, spacing: f32, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = spacing;
        add_contents(ui);
    });
}

/// Wrapping row: lays out children horizontally, wrapping to the next row
/// when the available width is exceeded.
///
/// ```rust
/// wrap_row(ui, theme::SPACING_MD, theme::SPACING_SM, |ui| {
///     for item in items {
///         ui.button(item);
///     }
/// });
/// ```
pub fn wrap_row(
    ui: &mut egui::Ui,
    horizontal_spacing: f32,
    vertical_spacing: f32,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    // Store the available width for wrapping logic
    let available_width = ui.available_width();
    
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(horizontal_spacing, vertical_spacing);
        // Limit each row to available width minus a small margin
        ui.set_max_width(available_width);
        add_contents(ui);
    });
}

/// Elevated card surface with padding, border, and corner radius.
///
/// ```rust
/// card(ui, |ui| {
///     ui.heading("Card Title");
///     ui.label("Card content");
/// });
/// ```
pub fn card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let frame = egui::Frame::new()
        .fill(theme::SURFACE_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
        .corner_radius(theme::RADIUS_MD)
        .inner_margin(theme::SPACING_MD);
    
    frame.show(ui, |ui| {
        add_contents(ui);
    });
}

/// Smaller card variant for inline use.
pub fn card_compact(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let frame = egui::Frame::new()
        .fill(theme::SURFACE_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
        .corner_radius(theme::RADIUS_SM)
        .inner_margin(theme::SPACING_SM);
    
    frame.show(ui, |ui| {
        add_contents(ui);
    });
}

/// Tool group card for the rail — uses vstack internally.
pub fn tool_group(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    let frame = egui::Frame::new()
        .fill(theme::SURFACE_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
        .corner_radius(theme::RADIUS_MD)
        .inner_margin(theme::SPACING_MD);
    
    frame.show(ui, |ui| {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme::SPACING_SM;
            // Group title
            ui.label(
                egui::RichText::new(title)
                    .small()
                    .color(theme::TEXT_HINT)
                    .strong(),
            );
            ui.separator();
            add_contents(ui);
        });
    });
}