//! Layout helpers: vstack, hstack, wrap_row, card.
//!
//! shadcn-egui layout pattern, copied not adopted (ticket T-003, spec
//! v0.20.0 AC-04). Every helper returns the [`egui::InnerResponse`] of the
//! container so callers can compose responses (hit-testing, tooltips)
//! without losing the layout.
//!
//! All spacing, radii, and colours come from theme tokens — no raw `Color32`
//! (guide II.30.6).

use crate::theme;
use eframe::egui;

/// Vertical stack with equal spacing between children.
///
/// ```ignore
/// vstack(ui, theme::SPACING_MD, |ui| {
///     ui.label("First");
///     ui.label("Second");
/// });
/// ```
pub fn vstack<R>(
    ui: &mut egui::Ui,
    spacing: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = spacing;
        add_contents(ui)
    })
}

/// Horizontal stack with equal spacing between children.
///
/// ```ignore
/// hstack(ui, theme::SPACING_MD, |ui| {
///     ui.button("A");
///     ui.button("B");
/// });
/// ```
pub fn hstack<R>(
    ui: &mut egui::Ui,
    spacing: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = spacing;
        add_contents(ui)
    })
}

/// Wrapping row: lays out children horizontally, wrapping to the next row
/// when the available width is exceeded (II.30.7.6 reflow — at narrow window
/// widths the row wraps instead of clipping).
///
/// ```ignore
/// wrap_row(ui, theme::SPACING_MD, theme::SPACING_SM, |ui| {
///     for item in items {
///         ui.button(item);
///     }
/// });
/// ```
pub fn wrap_row<R>(
    ui: &mut egui::Ui,
    horizontal_spacing: f32,
    vertical_spacing: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    // Clamp each row to the width available right now: hostile input in the
    // form of a tiny (or negative) available width still yields a sane
    // non-negative row width (guide §2.6 — checked, never trusted).
    let available_width = ui.available_width().max(0.0);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(horizontal_spacing, vertical_spacing);
        ui.set_max_width(available_width);
        add_contents(ui)
    })
}

/// Elevated card surface with padding, border, corner radius and a level-1
/// elevation shadow (`SHADOW_LEVEL_1`, ticket T-003).
///
/// ```ignore
/// card(ui, |ui| {
///     ui.heading("Card Title");
///     ui.label("Card content");
/// });
/// ```
pub fn card<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::new()
        .fill(theme::SURFACE_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
        .corner_radius(theme::RADIUS_MD)
        .inner_margin(theme::SPACING_MD)
        .shadow(theme::SHADOW_LEVEL_1)
        .show(ui, |ui| add_contents(ui))
}

/// Smaller card variant for inline use.
pub fn card_compact<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::new()
        .fill(theme::SURFACE_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
        .corner_radius(theme::RADIUS_SM)
        .inner_margin(theme::SPACING_SM)
        .show(ui, |ui| add_contents(ui))
}

/// Tool group card for the rail — a titled card using vstack internally.
pub fn tool_group<R>(
    ui: &mut egui::Ui,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::new()
        .fill(theme::SURFACE_PANEL)
        .stroke(egui::Stroke::new(1.0, theme::BORDER_DEFAULT))
        .corner_radius(theme::RADIUS_MD)
        .inner_margin(theme::SPACING_MD)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = theme::SPACING_SM;
                // Group title: a small, strong, hint-coloured heading.
                ui.label(
                    egui::RichText::new(title)
                        .small()
                        .color(theme::TEXT_HINT)
                        .strong(),
                );
                ui.separator();
                add_contents(ui)
            })
        })
        .inner
}
