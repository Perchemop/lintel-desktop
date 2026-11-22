//! Structural tests for the three-tier token system (ticket T-006, spec
//! v0.20.0 AC-07).
//!
//! Headless (guide K.13): pure constant maths, no display, no egui context.
//! What is verified here, beyond the per-token pairs in `theme.rs`:
//!
//! 1. [`ThemeSpec::LIGHT`] mirrors the flat token registry **field by
//!    field** — the spec is a grouping, never a second copy of a value.
//! 2. Tier-1 swatches are pinned byte-identical (the restructure is
//!    behaviour-preserving).
//! 3. Every tier-2 role resolves to a tier-1 swatch — one hop, no invented
//!    colours.
//! 4. Every tier-3 component token (and every button-state field) resolves
//!    to a tier-2 role — widget code never invents colours either.
//! 5. The button group covers all twelve role × size variants with the 24pt
//!    interactive-target floor per size (guide II.30.7.5).
//!
//! No `Color32` constructor appears in this file: pinned values are compared
//! through `theme::rgb_of` / channel accessors so the raw-colour grep gate
//! (`Color32::from_` only in `theme.rs`) stays clean.

use crate::theme::{self, SPEC, ThemeSpec};
use eframe::egui::Color32;

#[test]
fn themespec_mirrors_the_flat_registry_field_by_field() {
    // Tier 1 — grouped spec fields must equal the flat BASE_* constants.
    let base: [(&str, Color32, Color32); 35] = [
        ("base.paper", SPEC.base.paper, theme::BASE_PAPER),
        ("base.ink", SPEC.base.ink, theme::BASE_INK),
        ("base.black", SPEC.base.black, theme::BASE_BLACK),
        (
            "base.graphite_muted",
            SPEC.base.graphite_muted,
            theme::BASE_GRAPHITE_MUTED,
        ),
        (
            "base.graphite_hint",
            SPEC.base.graphite_hint,
            theme::BASE_GRAPHITE_HINT,
        ),
        (
            "base.graphite_disabled",
            SPEC.base.graphite_disabled,
            theme::BASE_GRAPHITE_DISABLED,
        ),
        (
            "base.graphite_secondary",
            SPEC.base.graphite_secondary,
            theme::BASE_GRAPHITE_SECONDARY,
        ),
        (
            "base.match_blue",
            SPEC.base.match_blue,
            theme::BASE_MATCH_BLUE,
        ),
        ("base.cobalt", SPEC.base.cobalt, theme::BASE_COBALT),
        (
            "base.cobalt_hover",
            SPEC.base.cobalt_hover,
            theme::BASE_COBALT_HOVER,
        ),
        ("base.green", SPEC.base.green, theme::BASE_GREEN),
        ("base.valid", SPEC.base.valid, theme::BASE_VALID),
        ("base.error", SPEC.base.error, theme::BASE_ERROR),
        ("base.danger", SPEC.base.danger, theme::BASE_DANGER),
        (
            "base.danger_hover",
            SPEC.base.danger_hover,
            theme::BASE_DANGER_HOVER,
        ),
        ("base.warning", SPEC.base.warning, theme::BASE_WARNING),
        ("base.panel_bg", SPEC.base.panel_bg, theme::BASE_PANEL_BG),
        (
            "base.panel_border",
            SPEC.base.panel_border,
            theme::BASE_PANEL_BORDER,
        ),
        ("base.status_bg", SPEC.base.status_bg, theme::BASE_STATUS_BG),
        ("base.wall_fill", SPEC.base.wall_fill, theme::BASE_WALL_FILL),
        (
            "base.wall_stroke",
            SPEC.base.wall_stroke,
            theme::BASE_WALL_STROKE,
        ),
        (
            "base.wall_pending",
            SPEC.base.wall_pending,
            theme::BASE_WALL_PENDING,
        ),
        (
            "base.grid_minor",
            SPEC.base.grid_minor,
            theme::BASE_GRID_MINOR,
        ),
        (
            "base.grid_major",
            SPEC.base.grid_major,
            theme::BASE_GRID_MAJOR,
        ),
        (
            "base.window_line",
            SPEC.base.window_line,
            theme::BASE_WINDOW_LINE,
        ),
        (
            "base.selection_wash",
            SPEC.base.selection_wash,
            theme::BASE_SELECTION_WASH,
        ),
        (
            "base.selection_edge",
            SPEC.base.selection_edge,
            theme::BASE_SELECTION_EDGE,
        ),
        (
            "base.transparent",
            SPEC.base.transparent,
            theme::BASE_TRANSPARENT,
        ),
        (
            "base.snap_midpoint",
            SPEC.base.snap_midpoint,
            theme::BASE_SNAP_MIDPOINT,
        ),
        (
            "base.snap_perpendicular",
            SPEC.base.snap_perpendicular,
            theme::BASE_SNAP_PERPENDICULAR,
        ),
        (
            "base.snap_tangent",
            SPEC.base.snap_tangent,
            theme::BASE_SNAP_TANGENT,
        ),
        (
            "base.snap_ortho",
            SPEC.base.snap_ortho,
            theme::BASE_SNAP_ORTHO,
        ),
        ("base.snap_grid", SPEC.base.snap_grid, theme::BASE_SNAP_GRID),
        (
            "base.construction",
            SPEC.base.construction,
            theme::BASE_CONSTRUCTION,
        ),
        (
            "base.block_highlight",
            SPEC.base.block_highlight,
            theme::BASE_BLOCK_HIGHLIGHT,
        ),
    ];
    for (name, grouped, flat) in base {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }

    // Tier 2 — grouped spec fields must equal the flat semantic constants.
    let semantic: [(&str, Color32, Color32); 42] = [
        (
            "semantic.surface_base",
            SPEC.semantic.surface_base,
            theme::SURFACE_BASE,
        ),
        (
            "semantic.surface_panel",
            SPEC.semantic.surface_panel,
            theme::SURFACE_PANEL,
        ),
        (
            "semantic.surface_status",
            SPEC.semantic.surface_status,
            theme::SURFACE_STATUS,
        ),
        (
            "semantic.surface_selection",
            SPEC.semantic.surface_selection,
            theme::SURFACE_SELECTION,
        ),
        (
            "semantic.surface_transparent",
            SPEC.semantic.surface_transparent,
            theme::SURFACE_TRANSPARENT,
        ),
        (
            "semantic.border_default",
            SPEC.semantic.border_default,
            theme::BORDER_DEFAULT,
        ),
        (
            "semantic.border_selection",
            SPEC.semantic.border_selection,
            theme::BORDER_SELECTION,
        ),
        (
            "semantic.text_primary",
            SPEC.semantic.text_primary,
            theme::TEXT_PRIMARY,
        ),
        (
            "semantic.text_secondary",
            SPEC.semantic.text_secondary,
            theme::TEXT_SECONDARY,
        ),
        (
            "semantic.text_muted",
            SPEC.semantic.text_muted,
            theme::TEXT_MUTED,
        ),
        (
            "semantic.text_hint",
            SPEC.semantic.text_hint,
            theme::TEXT_HINT,
        ),
        (
            "semantic.text_strong",
            SPEC.semantic.text_strong,
            theme::TEXT_STRONG,
        ),
        (
            "semantic.text_disabled",
            SPEC.semantic.text_disabled,
            theme::TEXT_DISABLED,
        ),
        (
            "semantic.text_match",
            SPEC.semantic.text_match,
            theme::TEXT_MATCH,
        ),
        (
            "semantic.text_success",
            SPEC.semantic.text_success,
            theme::TEXT_SUCCESS,
        ),
        (
            "semantic.text_warning",
            SPEC.semantic.text_warning,
            theme::TEXT_WARNING,
        ),
        (
            "semantic.text_error",
            SPEC.semantic.text_error,
            theme::TEXT_ERROR,
        ),
        (
            "semantic.text_danger",
            SPEC.semantic.text_danger,
            theme::TEXT_DANGER,
        ),
        (
            "semantic.text_on_accent",
            SPEC.semantic.text_on_accent,
            theme::TEXT_ON_ACCENT,
        ),
        (
            "semantic.accent_default",
            SPEC.semantic.accent_default,
            theme::ACCENT_DEFAULT,
        ),
        (
            "semantic.accent_hover",
            SPEC.semantic.accent_hover,
            theme::ACCENT_HOVER,
        ),
        (
            "semantic.accent_active",
            SPEC.semantic.accent_active,
            theme::ACCENT_ACTIVE,
        ),
        (
            "semantic.focus_ring",
            SPEC.semantic.focus_ring,
            theme::FOCUS_RING,
        ),
        (
            "semantic.success_default",
            SPEC.semantic.success_default,
            theme::SUCCESS_DEFAULT,
        ),
        (
            "semantic.danger_default",
            SPEC.semantic.danger_default,
            theme::DANGER_DEFAULT,
        ),
        (
            "semantic.danger_hover",
            SPEC.semantic.danger_hover,
            theme::DANGER_HOVER,
        ),
        (
            "semantic.stroke_strong",
            SPEC.semantic.stroke_strong,
            theme::STROKE_STRONG,
        ),
        (
            "semantic.block_highlight",
            SPEC.semantic.block_highlight,
            theme::BLOCK_HIGHLIGHT,
        ),
        (
            "semantic.snap_midpoint",
            SPEC.semantic.snap_midpoint,
            theme::SNAP_MIDPOINT,
        ),
        (
            "semantic.snap_perpendicular",
            SPEC.semantic.snap_perpendicular,
            theme::SNAP_PERPENDICULAR,
        ),
        (
            "semantic.snap_tangent",
            SPEC.semantic.snap_tangent,
            theme::SNAP_TANGENT,
        ),
        (
            "semantic.snap_ortho",
            SPEC.semantic.snap_ortho,
            theme::SNAP_ORTHO,
        ),
        (
            "semantic.snap_grid",
            SPEC.semantic.snap_grid,
            theme::SNAP_GRID,
        ),
        (
            "semantic.construction_stroke",
            SPEC.semantic.construction_stroke,
            theme::CONSTRUCTION_STROKE,
        ),
        (
            "semantic.draft_fill",
            SPEC.semantic.draft_fill,
            theme::DRAFT_FILL,
        ),
        (
            "semantic.draft_stroke",
            SPEC.semantic.draft_stroke,
            theme::DRAFT_STROKE,
        ),
        (
            "semantic.draft_pending",
            SPEC.semantic.draft_pending,
            theme::DRAFT_PENDING,
        ),
        (
            "semantic.grid_line_minor",
            SPEC.semantic.grid_line_minor,
            theme::GRID_LINE_MINOR,
        ),
        (
            "semantic.grid_line_major",
            SPEC.semantic.grid_line_major,
            theme::GRID_LINE_MAJOR,
        ),
        (
            "semantic.opening_cut",
            SPEC.semantic.opening_cut,
            theme::OPENING_CUT,
        ),
        (
            "semantic.opening_leaf",
            SPEC.semantic.opening_leaf,
            theme::OPENING_LEAF,
        ),
        (
            "semantic.opening_glazing",
            SPEC.semantic.opening_glazing,
            theme::OPENING_GLAZING,
        ),
    ];
    for (name, grouped, flat) in semantic {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }

    // Tier 3 — grouped spec fields must equal the flat component constants.
    let canvas: [(&str, Color32, Color32); 17] = [
        (
            "canvas.sheet",
            SPEC.component.canvas.sheet,
            theme::CANVAS_SHEET,
        ),
        (
            "canvas.text",
            SPEC.component.canvas.text,
            theme::CANVAS_TEXT,
        ),
        (
            "canvas.wall_fill",
            SPEC.component.canvas.wall_fill,
            theme::WALL_FILL,
        ),
        (
            "canvas.wall_stroke",
            SPEC.component.canvas.wall_stroke,
            theme::WALL_STROKE,
        ),
        (
            "canvas.wall_pending",
            SPEC.component.canvas.wall_pending,
            theme::WALL_PENDING,
        ),
        (
            "canvas.grid_minor",
            SPEC.component.canvas.grid_minor,
            theme::GRID_MINOR,
        ),
        (
            "canvas.grid_major",
            SPEC.component.canvas.grid_major,
            theme::GRID_MAJOR,
        ),
        (
            "canvas.opening_gap",
            SPEC.component.canvas.opening_gap,
            theme::OPENING_GAP,
        ),
        (
            "canvas.door_leaf",
            SPEC.component.canvas.door_leaf,
            theme::DOOR_LEAF,
        ),
        (
            "canvas.window_line",
            SPEC.component.canvas.window_line,
            theme::WINDOW_LINE,
        ),
        (
            "canvas.construction_stroke",
            SPEC.component.canvas.construction_stroke,
            theme::CANVAS_CONSTRUCTION,
        ),
        (
            "canvas.stroke_none",
            SPEC.component.canvas.stroke_none,
            theme::STROKE_NONE,
        ),
        (
            "canvas.selection",
            SPEC.component.canvas.selection,
            theme::SELECTION_HIGHLIGHT,
        ),
        (
            "canvas.selection_window",
            SPEC.component.canvas.selection_window,
            theme::SELECTION_WINDOW,
        ),
        (
            "canvas.selection_crossing",
            SPEC.component.canvas.selection_crossing,
            theme::SELECTION_CROSSING,
        ),
        (
            "canvas.dimension_stroke",
            SPEC.component.canvas.dimension_stroke,
            theme::DIMENSION_STROKE,
        ),
        (
            "canvas.block_instance_stroke",
            SPEC.component.canvas.block_instance_stroke,
            theme::BLOCK_INSTANCE_STROKE,
        ),
    ];
    for (name, grouped, flat) in canvas {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }

    let panel: [(&str, Color32, Color32); 3] = [
        (
            "panel.surface",
            SPEC.component.panel.surface,
            theme::PANEL_SURFACE,
        ),
        (
            "panel.status",
            SPEC.component.panel.status,
            theme::STATUS_SURFACE,
        ),
        (
            "panel.border",
            SPEC.component.panel.border,
            theme::PANEL_BORDER,
        ),
    ];
    for (name, grouped, flat) in panel {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }

    let text: [(&str, Color32, Color32); 6] = [
        (
            "text.primary",
            SPEC.component.text.primary,
            theme::BODY_TEXT,
        ),
        ("text.hint", SPEC.component.text.hint, theme::HINT_TEXT),
        ("text.muted", SPEC.component.text.muted, theme::MUTED_TEXT),
        (
            "text.success",
            SPEC.component.text.success,
            theme::SUCCESS_TEXT,
        ),
        ("text.error", SPEC.component.text.error, theme::ERROR_TEXT),
        (
            "text.danger",
            SPEC.component.text.danger,
            theme::DANGER_TEXT,
        ),
    ];
    for (name, grouped, flat) in text {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }

    let palette: [(&str, Color32, Color32); 9] = [
        (
            "palette.selected_fill",
            SPEC.component.palette.selected_fill,
            theme::PALETTE_SELECTED_FILL,
        ),
        (
            "palette.unselected_fill",
            SPEC.component.palette.unselected_fill,
            theme::PALETTE_UNSELECTED_FILL,
        ),
        (
            "palette.selected_stroke",
            SPEC.component.palette.selected_stroke,
            theme::PALETTE_SELECTED_STROKE,
        ),
        (
            "palette.hit",
            SPEC.component.palette.hit,
            theme::PALETTE_HIT,
        ),
        (
            "palette.label",
            SPEC.component.palette.label,
            theme::PALETTE_LABEL,
        ),
        (
            "palette.disabled",
            SPEC.component.palette.disabled,
            theme::PALETTE_DISABLED,
        ),
        (
            "palette.recent",
            SPEC.component.palette.recent,
            theme::PALETTE_RECENT,
        ),
        (
            "palette.shortcut",
            SPEC.component.palette.shortcut,
            theme::PALETTE_SHORTCUT,
        ),
        (
            "palette.category",
            SPEC.component.palette.category,
            theme::PALETTE_CATEGORY,
        ),
    ];
    for (name, grouped, flat) in palette {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }

    let buttons = [
        (
            "button.primary_sm",
            SPEC.component.button.primary_sm,
            theme::BUTTON_PRIMARY_S,
        ),
        (
            "button.primary_md",
            SPEC.component.button.primary_md,
            theme::BUTTON_PRIMARY_M,
        ),
        (
            "button.primary_lg",
            SPEC.component.button.primary_lg,
            theme::BUTTON_PRIMARY_L,
        ),
        (
            "button.secondary_sm",
            SPEC.component.button.secondary_sm,
            theme::BUTTON_SECONDARY_S,
        ),
        (
            "button.secondary_md",
            SPEC.component.button.secondary_md,
            theme::BUTTON_SECONDARY_M,
        ),
        (
            "button.secondary_lg",
            SPEC.component.button.secondary_lg,
            theme::BUTTON_SECONDARY_L,
        ),
        (
            "button.ghost_sm",
            SPEC.component.button.ghost_sm,
            theme::BUTTON_GHOST_S,
        ),
        (
            "button.ghost_md",
            SPEC.component.button.ghost_md,
            theme::BUTTON_GHOST_M,
        ),
        (
            "button.ghost_lg",
            SPEC.component.button.ghost_lg,
            theme::BUTTON_GHOST_L,
        ),
        (
            "button.danger_sm",
            SPEC.component.button.danger_sm,
            theme::BUTTON_DANGER_S,
        ),
        (
            "button.danger_md",
            SPEC.component.button.danger_md,
            theme::BUTTON_DANGER_M,
        ),
        (
            "button.danger_lg",
            SPEC.component.button.danger_lg,
            theme::BUTTON_DANGER_L,
        ),
    ];
    for (name, grouped, flat) in buttons {
        assert_eq!(grouped, flat, "{name} must mirror the flat registry");
    }
    assert_eq!(
        SPEC.component.button.corner_radius,
        theme::BUTTON_CORNER_RADIUS,
        "button.corner_radius must mirror the flat registry"
    );

    // And the whole spec is exactly its const definition.
    assert_eq!(SPEC, ThemeSpec::LIGHT);
}

#[test]
fn tier_one_swatches_are_pinned() {
    // Behaviour-preservation pin (T-006): the three-tier restructure must not
    // move a single base value. If a swatch changes deliberately, change it
    // here AND in the release notes.
    let pins: [(&str, [u8; 3], [u8; 3]); 32] = [
        (
            "BASE_PAPER",
            theme::rgb_of(theme::BASE_PAPER),
            [252, 248, 242],
        ),
        ("BASE_INK", theme::rgb_of(theme::BASE_INK), [26, 24, 22]),
        (
            "BASE_GRAPHITE_MUTED",
            theme::rgb_of(theme::BASE_GRAPHITE_MUTED),
            [102, 102, 102],
        ),
        (
            "BASE_GRAPHITE_HINT",
            theme::rgb_of(theme::BASE_GRAPHITE_HINT),
            [100, 100, 100],
        ),
        (
            "BASE_GRAPHITE_DISABLED",
            theme::rgb_of(theme::BASE_GRAPHITE_DISABLED),
            [105, 105, 105],
        ),
        (
            "BASE_GRAPHITE_SECONDARY",
            theme::rgb_of(theme::BASE_GRAPHITE_SECONDARY),
            [95, 95, 95],
        ),
        ("BASE_BLACK", theme::rgb_of(theme::BASE_BLACK), [0, 0, 0]),
        (
            "BASE_MATCH_BLUE",
            theme::rgb_of(theme::BASE_MATCH_BLUE),
            [20, 90, 160],
        ),
        (
            "BASE_COBALT",
            theme::rgb_of(theme::BASE_COBALT),
            [18, 78, 120],
        ),
        (
            "BASE_COBALT_HOVER",
            theme::rgb_of(theme::BASE_COBALT_HOVER),
            [24, 94, 140],
        ),
        (
            "BASE_GREEN",
            theme::rgb_of(theme::BASE_GREEN),
            [34, 158, 96],
        ),
        (
            "BASE_VALID",
            theme::rgb_of(theme::BASE_VALID),
            [26, 112, 62],
        ),
        ("BASE_ERROR", theme::rgb_of(theme::BASE_ERROR), [139, 0, 0]),
        (
            "BASE_DANGER",
            theme::rgb_of(theme::BASE_DANGER),
            [180, 0, 0],
        ),
        (
            "BASE_DANGER_HOVER",
            theme::rgb_of(theme::BASE_DANGER_HOVER),
            [120, 0, 0],
        ),
        (
            "BASE_WARNING",
            theme::rgb_of(theme::BASE_WARNING),
            [140, 84, 14],
        ),
        (
            "BASE_PANEL_BG",
            theme::rgb_of(theme::BASE_PANEL_BG),
            [245, 240, 232],
        ),
        (
            "BASE_PANEL_BORDER",
            theme::rgb_of(theme::BASE_PANEL_BORDER),
            [212, 204, 192],
        ),
        (
            "BASE_STATUS_BG",
            theme::rgb_of(theme::BASE_STATUS_BG),
            [239, 232, 220],
        ),
        (
            "BASE_WALL_FILL",
            theme::rgb_of(theme::BASE_WALL_FILL),
            [226, 222, 214],
        ),
        (
            "BASE_WALL_STROKE",
            theme::rgb_of(theme::BASE_WALL_STROKE),
            [84, 78, 70],
        ),
        (
            "BASE_WALL_PENDING",
            theme::rgb_of(theme::BASE_WALL_PENDING),
            [200, 194, 184],
        ),
        (
            "BASE_GRID_MINOR",
            theme::rgb_of(theme::BASE_GRID_MINOR),
            [233, 226, 216],
        ),
        (
            "BASE_GRID_MAJOR",
            theme::rgb_of(theme::BASE_GRID_MAJOR),
            [200, 190, 176],
        ),
        (
            "BASE_WINDOW_LINE",
            theme::rgb_of(theme::BASE_WINDOW_LINE),
            [90, 82, 74],
        ),
        (
            "BASE_SNAP_MIDPOINT",
            theme::rgb_of(theme::BASE_SNAP_MIDPOINT),
            [176, 106, 22],
        ),
        (
            "BASE_SNAP_PERPENDICULAR",
            theme::rgb_of(theme::BASE_SNAP_PERPENDICULAR),
            [120, 74, 160],
        ),
        (
            "BASE_SNAP_TANGENT",
            theme::rgb_of(theme::BASE_SNAP_TANGENT),
            [24, 128, 140],
        ),
        (
            "BASE_SNAP_ORTHO",
            theme::rgb_of(theme::BASE_SNAP_ORTHO),
            [90, 82, 74],
        ),
        (
            "BASE_SNAP_GRID",
            theme::rgb_of(theme::BASE_SNAP_GRID),
            [130, 130, 130],
        ),
        (
            "BASE_CONSTRUCTION",
            theme::rgb_of(theme::BASE_CONSTRUCTION),
            [140, 120, 90],
        ),
        (
            "BASE_BLOCK_HIGHLIGHT",
            theme::rgb_of(theme::BASE_BLOCK_HIGHLIGHT),
            [220, 60, 60],
        ),
    ];
    for (name, actual, pinned) in pins {
        assert_eq!(actual, pinned, "{name} moved — base values are pinned");
    }

    // The alpha-bearing swatches: pinned channels including alpha.
    let wash = theme::BASE_SELECTION_WASH;
    assert_eq!(
        (wash.r(), wash.g(), wash.b(), wash.a()),
        (0, 0, 0, 18),
        "BASE_SELECTION_WASH moved"
    );
    let edge = theme::BASE_SELECTION_EDGE;
    assert_eq!(
        (edge.r(), edge.g(), edge.b(), edge.a()),
        (0, 0, 0, 140),
        "BASE_SELECTION_EDGE moved"
    );
    let transparent = theme::BASE_TRANSPARENT;
    assert_eq!(
        (
            transparent.r(),
            transparent.g(),
            transparent.b(),
            transparent.a()
        ),
        (0, 0, 0, 0),
        "BASE_TRANSPARENT moved"
    );
}

/// All tier-1 swatches as opaque RGB triples (alpha dropped — only used for
/// membership checks; the exact pin lives in [`tier_one_swatches_are_pinned`]).
fn base_rgbs() -> Vec<[u8; 3]> {
    vec![
        theme::rgb_of(theme::BASE_PAPER),
        theme::rgb_of(theme::BASE_INK),
        theme::rgb_of(theme::BASE_BLACK),
        theme::rgb_of(theme::BASE_GRAPHITE_MUTED),
        theme::rgb_of(theme::BASE_GRAPHITE_HINT),
        theme::rgb_of(theme::BASE_GRAPHITE_DISABLED),
        theme::rgb_of(theme::BASE_GRAPHITE_SECONDARY),
        theme::rgb_of(theme::BASE_MATCH_BLUE),
        theme::rgb_of(theme::BASE_COBALT),
        theme::rgb_of(theme::BASE_COBALT_HOVER),
        theme::rgb_of(theme::BASE_GREEN),
        theme::rgb_of(theme::BASE_VALID),
        theme::rgb_of(theme::BASE_ERROR),
        theme::rgb_of(theme::BASE_DANGER),
        theme::rgb_of(theme::BASE_DANGER_HOVER),
        theme::rgb_of(theme::BASE_WARNING),
        theme::rgb_of(theme::BASE_PANEL_BG),
        theme::rgb_of(theme::BASE_PANEL_BORDER),
        theme::rgb_of(theme::BASE_STATUS_BG),
        theme::rgb_of(theme::BASE_WALL_FILL),
        theme::rgb_of(theme::BASE_WALL_STROKE),
        theme::rgb_of(theme::BASE_WALL_PENDING),
        theme::rgb_of(theme::BASE_GRID_MINOR),
        theme::rgb_of(theme::BASE_GRID_MAJOR),
        theme::rgb_of(theme::BASE_WINDOW_LINE),
        theme::rgb_of(theme::BASE_SELECTION_WASH),
        theme::rgb_of(theme::BASE_SELECTION_EDGE),
        theme::rgb_of(theme::BASE_TRANSPARENT),
        theme::rgb_of(theme::BASE_SNAP_MIDPOINT),
        theme::rgb_of(theme::BASE_SNAP_PERPENDICULAR),
        theme::rgb_of(theme::BASE_SNAP_TANGENT),
        theme::rgb_of(theme::BASE_SNAP_ORTHO),
        theme::rgb_of(theme::BASE_SNAP_GRID),
        theme::rgb_of(theme::BASE_CONSTRUCTION),
        theme::rgb_of(theme::BASE_BLOCK_HIGHLIGHT),
    ]
}

/// All tier-2 roles in [`SPEC`], as RGB triples.
fn semantic_rgbs() -> Vec<[u8; 3]> {
    let s = &SPEC.semantic;
    vec![
        theme::rgb_of(s.surface_base),
        theme::rgb_of(s.surface_panel),
        theme::rgb_of(s.surface_status),
        theme::rgb_of(s.surface_selection),
        theme::rgb_of(s.surface_transparent),
        theme::rgb_of(s.border_default),
        theme::rgb_of(s.border_selection),
        theme::rgb_of(s.text_primary),
        theme::rgb_of(s.text_secondary),
        theme::rgb_of(s.text_muted),
        theme::rgb_of(s.text_hint),
        theme::rgb_of(s.text_strong),
        theme::rgb_of(s.text_disabled),
        theme::rgb_of(s.text_match),
        theme::rgb_of(s.text_success),
        theme::rgb_of(s.text_warning),
        theme::rgb_of(s.text_error),
        theme::rgb_of(s.text_danger),
        theme::rgb_of(s.text_on_accent),
        theme::rgb_of(s.accent_default),
        theme::rgb_of(s.accent_hover),
        theme::rgb_of(s.accent_active),
        theme::rgb_of(s.focus_ring),
        theme::rgb_of(s.success_default),
        theme::rgb_of(s.danger_default),
        theme::rgb_of(s.danger_hover),
        theme::rgb_of(s.stroke_strong),
        theme::rgb_of(s.block_highlight),
        theme::rgb_of(s.snap_midpoint),
        theme::rgb_of(s.snap_perpendicular),
        theme::rgb_of(s.snap_tangent),
        theme::rgb_of(s.snap_ortho),
        theme::rgb_of(s.snap_grid),
        theme::rgb_of(s.construction_stroke),
        theme::rgb_of(s.draft_fill),
        theme::rgb_of(s.draft_stroke),
        theme::rgb_of(s.draft_pending),
        theme::rgb_of(s.grid_line_minor),
        theme::rgb_of(s.grid_line_major),
        theme::rgb_of(s.opening_cut),
        theme::rgb_of(s.opening_leaf),
        theme::rgb_of(s.opening_glazing),
    ]
}

#[test]
fn every_semantic_role_resolves_to_a_base_swatch() {
    // One-hop rule (guide II.30.6): tier 2 never invents a colour — each of
    // its 41 fields must equal some BASE_* swatch. A new role that does not
    // resolve fails here even though no per-token pair covers it yet.
    let base = base_rgbs();
    let roles: [(&str, Color32); 42] = [
        ("surface_base", SPEC.semantic.surface_base),
        ("surface_panel", SPEC.semantic.surface_panel),
        ("surface_status", SPEC.semantic.surface_status),
        ("surface_selection", SPEC.semantic.surface_selection),
        ("surface_transparent", SPEC.semantic.surface_transparent),
        ("border_default", SPEC.semantic.border_default),
        ("border_selection", SPEC.semantic.border_selection),
        ("text_primary", SPEC.semantic.text_primary),
        ("text_secondary", SPEC.semantic.text_secondary),
        ("text_muted", SPEC.semantic.text_muted),
        ("text_hint", SPEC.semantic.text_hint),
        ("text_strong", SPEC.semantic.text_strong),
        ("text_disabled", SPEC.semantic.text_disabled),
        ("text_match", SPEC.semantic.text_match),
        ("text_success", SPEC.semantic.text_success),
        ("text_warning", SPEC.semantic.text_warning),
        ("text_error", SPEC.semantic.text_error),
        ("text_danger", SPEC.semantic.text_danger),
        ("text_on_accent", SPEC.semantic.text_on_accent),
        ("accent_default", SPEC.semantic.accent_default),
        ("accent_hover", SPEC.semantic.accent_hover),
        ("accent_active", SPEC.semantic.accent_active),
        ("focus_ring", SPEC.semantic.focus_ring),
        ("success_default", SPEC.semantic.success_default),
        ("danger_default", SPEC.semantic.danger_default),
        ("danger_hover", SPEC.semantic.danger_hover),
        ("stroke_strong", SPEC.semantic.stroke_strong),
        ("block_highlight", SPEC.semantic.block_highlight),
        ("snap_midpoint", SPEC.semantic.snap_midpoint),
        ("snap_perpendicular", SPEC.semantic.snap_perpendicular),
        ("snap_tangent", SPEC.semantic.snap_tangent),
        ("snap_ortho", SPEC.semantic.snap_ortho),
        ("snap_grid", SPEC.semantic.snap_grid),
        ("construction_stroke", SPEC.semantic.construction_stroke),
        ("draft_fill", SPEC.semantic.draft_fill),
        ("draft_stroke", SPEC.semantic.draft_stroke),
        ("draft_pending", SPEC.semantic.draft_pending),
        ("grid_line_minor", SPEC.semantic.grid_line_minor),
        ("grid_line_major", SPEC.semantic.grid_line_major),
        ("opening_cut", SPEC.semantic.opening_cut),
        ("opening_leaf", SPEC.semantic.opening_leaf),
        ("opening_glazing", SPEC.semantic.opening_glazing),
    ];
    assert_eq!(
        roles.len(),
        semantic_rgbs().len(),
        "a semantic role was added without extending this completeness check"
    );
    for (name, colour) in roles {
        let rgb = theme::rgb_of(colour);
        assert!(
            base.contains(&rgb),
            "{name} ({rgb:?}) does not resolve to any BASE_* swatch"
        );
    }
}

#[test]
fn every_component_token_resolves_to_a_semantic_role() {
    // One-hop rule for tier 3: widget-facing tokens compose the semantic
    // roles above them, never a raw swatch.
    let semantic = semantic_rgbs();
    let tokens: [(&str, Color32); 35] = [
        ("canvas.sheet", SPEC.component.canvas.sheet),
        ("canvas.text", SPEC.component.canvas.text),
        ("canvas.wall_fill", SPEC.component.canvas.wall_fill),
        ("canvas.wall_stroke", SPEC.component.canvas.wall_stroke),
        ("canvas.wall_pending", SPEC.component.canvas.wall_pending),
        ("canvas.grid_minor", SPEC.component.canvas.grid_minor),
        ("canvas.grid_major", SPEC.component.canvas.grid_major),
        ("canvas.opening_gap", SPEC.component.canvas.opening_gap),
        ("canvas.door_leaf", SPEC.component.canvas.door_leaf),
        ("canvas.window_line", SPEC.component.canvas.window_line),
        (
            "canvas.construction_stroke",
            SPEC.component.canvas.construction_stroke,
        ),
        ("canvas.stroke_none", SPEC.component.canvas.stroke_none),
        ("canvas.selection", SPEC.component.canvas.selection),
        (
            "canvas.selection_window",
            SPEC.component.canvas.selection_window,
        ),
        (
            "canvas.selection_crossing",
            SPEC.component.canvas.selection_crossing,
        ),
        (
            "canvas.dimension_stroke",
            SPEC.component.canvas.dimension_stroke,
        ),
        (
            "canvas.block_instance_stroke",
            SPEC.component.canvas.block_instance_stroke,
        ),
        ("panel.surface", SPEC.component.panel.surface),
        ("panel.status", SPEC.component.panel.status),
        ("panel.border", SPEC.component.panel.border),
        ("text.primary", SPEC.component.text.primary),
        ("text.hint", SPEC.component.text.hint),
        ("text.muted", SPEC.component.text.muted),
        ("text.success", SPEC.component.text.success),
        ("text.error", SPEC.component.text.error),
        ("text.danger", SPEC.component.text.danger),
        (
            "palette.selected_fill",
            SPEC.component.palette.selected_fill,
        ),
        (
            "palette.unselected_fill",
            SPEC.component.palette.unselected_fill,
        ),
        (
            "palette.selected_stroke",
            SPEC.component.palette.selected_stroke,
        ),
        ("palette.hit", SPEC.component.palette.hit),
        ("palette.label", SPEC.component.palette.label),
        ("palette.disabled", SPEC.component.palette.disabled),
        ("palette.recent", SPEC.component.palette.recent),
        ("palette.shortcut", SPEC.component.palette.shortcut),
        ("palette.category", SPEC.component.palette.category),
    ];
    for (name, colour) in tokens {
        let rgb = theme::rgb_of(colour);
        assert!(
            semantic.contains(&rgb),
            "{name} ({rgb:?}) does not resolve to any semantic role"
        );
    }

    // Every button-state field is a semantic role too.
    let variants: [(&str, theme::ButtonRoleStyle); 12] = [
        ("primary_sm", SPEC.component.button.primary_sm),
        ("primary_md", SPEC.component.button.primary_md),
        ("primary_lg", SPEC.component.button.primary_lg),
        ("secondary_sm", SPEC.component.button.secondary_sm),
        ("secondary_md", SPEC.component.button.secondary_md),
        ("secondary_lg", SPEC.component.button.secondary_lg),
        ("ghost_sm", SPEC.component.button.ghost_sm),
        ("ghost_md", SPEC.component.button.ghost_md),
        ("ghost_lg", SPEC.component.button.ghost_lg),
        ("danger_sm", SPEC.component.button.danger_sm),
        ("danger_md", SPEC.component.button.danger_md),
        ("danger_lg", SPEC.component.button.danger_lg),
    ];
    for (name, style) in variants {
        for (field, colour) in [
            ("fill", style.fill),
            ("text", style.text),
            ("hover", style.hover),
            ("active", style.active),
            ("text_active", style.text_active),
            ("focus", style.focus),
            ("disabled", style.disabled),
            ("text_disabled", style.text_disabled),
        ] {
            let rgb = theme::rgb_of(colour);
            assert!(
                semantic.contains(&rgb),
                "{name}.{field} ({rgb:?}) does not resolve to any semantic role"
            );
        }
    }
}

#[test]
fn button_group_covers_all_twelve_variants_with_size_floors() {
    // The grouped button tokens must carry every role × size variant, with
    // the 24pt interactive-target floor honoured at every size
    // (guide II.30.7.5): sm 24, md 32, lg 40.
    let variants: [(&str, theme::ButtonRoleStyle, f32); 12] = [
        ("primary_sm", SPEC.component.button.primary_sm, 24.0),
        ("primary_md", SPEC.component.button.primary_md, 32.0),
        ("primary_lg", SPEC.component.button.primary_lg, 40.0),
        ("secondary_sm", SPEC.component.button.secondary_sm, 24.0),
        ("secondary_md", SPEC.component.button.secondary_md, 32.0),
        ("secondary_lg", SPEC.component.button.secondary_lg, 40.0),
        ("ghost_sm", SPEC.component.button.ghost_sm, 24.0),
        ("ghost_md", SPEC.component.button.ghost_md, 32.0),
        ("ghost_lg", SPEC.component.button.ghost_lg, 40.0),
        ("danger_sm", SPEC.component.button.danger_sm, 24.0),
        ("danger_md", SPEC.component.button.danger_md, 32.0),
        ("danger_lg", SPEC.component.button.danger_lg, 40.0),
    ];
    for (name, style, floor) in variants {
        assert!(floor >= 24.0, "{name}: floor below the 24pt target minimum");
        assert_eq!(
            style.min_size.x, floor,
            "{name}.min_size.x must honour its size floor"
        );
        assert_eq!(
            style.min_size.y, floor,
            "{name}.min_size.y must honour its size floor"
        );
    }
    assert_eq!(
        SPEC.component.button.corner_radius,
        theme::BUTTON_CORNER_RADIUS,
        "grouped corner radius must mirror the flat token"
    );
}
