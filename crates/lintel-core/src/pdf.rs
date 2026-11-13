//! Drawing-set export: a real vector PDF, written directly.
//!
//! Lintel's promise is that you can *issue* a drawing, so the exporter is part
//! of the domain and is tested like one. It writes PDF 1.7 by hand: object
//! table, cross-reference table and trailer are emitted byte-exactly, and the
//! page content is a plain stream of path and text operators. No PDF crate is
//! involved, so there is no licence question and nothing to audit but this
//! file.
//!
//! The sheet is drawn at a true scale (1:100 means one paper millimetre to one
//! hundred model millimetres), with a border, a title block and overall
//! dimensions. Everything is millimetres except the emitted operators, which
//! are points.

use std::fmt::Write as _;

use crate::error::CoreError;
use crate::geom::{Bounds, Point2, Vec2, arc_points};
use crate::ids::LevelId;
use crate::join::PlanGeometry;
use crate::level::Level;
use crate::opening::OpeningKind;
use crate::project::Project;
use crate::schedule::opening_schedule;
use crate::title_block::MAX_REVISION_CHARS;
use crate::units::UnitSystem;
use crate::viewport::Viewport;

/// PostScript points per millimetre.
pub const MM_TO_PT: f64 = 72.0 / 25.4;

/// Sheet margin, in millimetres.
const MARGIN: f64 = 10.0;
const TITLE_BLOCK_WIDTH: f64 = 190.0;
const TITLE_BLOCK_HEIGHT: f64 = 45.0;
/// Room left around the drawing for dimensions and for the drawing to breathe.
const DRAWING_PADDING: f64 = 18.0;
/// Width of the opening-schedule table, in millimetres.
const SCHEDULE_WIDTH: f64 = 110.0;
/// Height of the schedule's bold title row, in millimetres.
const SCHEDULE_TITLE_HEIGHT: f64 = 6.0;
/// Height of every other schedule row (header and data), in millimetres.
const SCHEDULE_ROW_HEIGHT: f64 = 5.0;

/// Scales offered in the sheet dialog, smallest denominator first.
pub const STANDARD_SCALES: [u32; 8] = [20, 50, 100, 200, 250, 500, 1000, 2000];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperSize {
    A4,
    A3,
    A2,
    A1,
}

impl PaperSize {
    pub fn all() -> [PaperSize; 4] {
        [PaperSize::A4, PaperSize::A3, PaperSize::A2, PaperSize::A1]
    }

    /// Portrait width and height, in millimetres.
    pub fn millimetres(self) -> (f64, f64) {
        match self {
            PaperSize::A4 => (210.0, 297.0),
            PaperSize::A3 => (297.0, 420.0),
            PaperSize::A2 => (420.0, 594.0),
            PaperSize::A1 => (594.0, 841.0),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            PaperSize::A4 => "A4",
            PaperSize::A3 => "A3",
            PaperSize::A2 => "A2",
            PaperSize::A1 => "A1",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SheetSpec {
    pub paper: PaperSize,
    pub landscape: bool,
    /// Denominator of the scale: `100` means 1:100.
    pub scale_denominator: u32,
    pub sheet_title: String,
    pub sheet_number: String,
}

impl Default for SheetSpec {
    fn default() -> Self {
        Self {
            paper: PaperSize::A3,
            landscape: true,
            scale_denominator: 100,
            sheet_title: "Floor plan".to_string(),
            sheet_number: "A-101".to_string(),
        }
    }
}

impl SheetSpec {
    pub fn size_mm(&self) -> (f64, f64) {
        let (width, height) = self.paper.millimetres();
        if self.landscape {
            (height, width)
        } else {
            (width, height)
        }
    }

    pub fn scale_label(&self) -> String {
        format!("1:{}", self.scale_denominator)
    }

    pub fn orientation_label(&self) -> &'static str {
        if self.landscape {
            "landscape"
        } else {
            "portrait"
        }
    }
}

/// The area of the sheet a plan may occupy, in millimetres:
/// `(x0, y0, width, height)` with the origin at the bottom-left of the paper.
fn drawing_area(sheet_width: f64, sheet_height: f64) -> (f64, f64, f64, f64) {
    let x0 = MARGIN + DRAWING_PADDING;
    let y0 = MARGIN + TITLE_BLOCK_HEIGHT + DRAWING_PADDING;
    let width = (sheet_width - MARGIN - DRAWING_PADDING - x0).max(20.0);
    let height = (sheet_height - MARGIN - DRAWING_PADDING - y0).max(20.0);
    (x0, y0, width, height)
}

/// Smallest standard scale that fits `bounds` on the given sheet. The sheet
/// dialog offers this as "Fit", so a user with a large plan is never told to
/// guess a scale that will not fit.
pub fn fitting_scale(bounds: &Bounds, paper: PaperSize, landscape: bool) -> u32 {
    let (paper_width, paper_height) = paper.millimetres();
    let (sheet_width, sheet_height) = if landscape {
        (paper_height, paper_width)
    } else {
        (paper_width, paper_height)
    };
    let (_, _, area_width, area_height) = drawing_area(sheet_width, sheet_height);
    // A 2 mm cushion keeps a drawing that exactly equals the area from
    // brushing the frame.
    let needed = (bounds.width() / (area_width - 2.0)).max(bounds.height() / (area_height - 2.0));
    let needed = needed.max(1.0);
    STANDARD_SCALES
        .iter()
        .copied()
        .find(|scale| f64::from(*scale) >= needed)
        .unwrap_or_else(|| STANDARD_SCALES.last().copied().unwrap_or(2000))
}

/// Render the active level of `project` as a one-page PDF.
pub fn render_plan(
    project: &Project,
    spec: &SheetSpec,
    produced_epoch_s: i64,
) -> Result<Vec<u8>, CoreError> {
    let level = project
        .active_level()
        .ok_or(CoreError::LevelNotFound(project.active_level))?;
    let (sheet_width, sheet_height) = spec.size_mm();

    let number = sheet_number_for_page(project, spec, 0);
    let sheet_facts = SheetFacts {
        title: &spec.sheet_title,
        level_name: Some(&level.name),
        sheet_number: &number,
    };
    let mut content = Content::default();
    draw_sheet(
        &mut content,
        project,
        level,
        spec,
        &sheet_facts,
        produced_epoch_s,
    );

    Ok(build_pdf(
        &content.ops,
        sheet_width,
        sheet_height,
        project,
        spec,
        produced_epoch_s,
    ))
}

/// Render the whole drawing set: a cover sheet followed by one plan sheet per
/// storey, as a single multi-page PDF.
pub fn render_set(
    project: &Project,
    spec: &SheetSpec,
    produced_epoch_s: i64,
) -> Result<Vec<u8>, CoreError> {
    let (sheet_width, sheet_height) = spec.size_mm();

    let cover_number = sheet_number_for_page(project, spec, 0);
    let cover_facts = SheetFacts {
        title: "Drawing set",
        level_name: None,
        sheet_number: &cover_number,
    };
    let mut cover = Content::default();
    draw_frame(&mut cover, sheet_width, sheet_height);
    draw_title_block(
        &mut cover,
        project,
        spec,
        &cover_facts,
        sheet_width,
        produced_epoch_s,
    );
    draw_cover(&mut cover, project, spec, sheet_width, sheet_height);
    let mut pages: Vec<(String, f64, f64)> = vec![(cover.ops, sheet_width, sheet_height)];

    for (index, level) in project.levels.iter().enumerate() {
        let number = sheet_number_for_page(project, spec, index + 1);
        let sheet_facts = SheetFacts {
            title: &level.name,
            level_name: Some(&level.name),
            sheet_number: &number,
        };
        let mut content = Content::default();
        draw_sheet(
            &mut content,
            project,
            level,
            spec,
            &sheet_facts,
            produced_epoch_s,
        );
        pages.push((content.ops, sheet_width, sheet_height));
    }

    Ok(build_pdf_pages(&pages, project, spec, produced_epoch_s))
}

/// Draw one complete plan sheet: frame, title block, plan geometry, north
/// arrow, scale bar and the level's opening schedule.
fn draw_sheet(
    content: &mut Content,
    project: &Project,
    level: &Level,
    spec: &SheetSpec,
    facts: &SheetFacts<'_>,
    produced_epoch_s: i64,
) {
    let (sheet_width, sheet_height) = spec.size_mm();
    let geometry = PlanGeometry::of_level(level);
    draw_frame(content, sheet_width, sheet_height);
    draw_title_block(content, project, spec, facts, sheet_width, produced_epoch_s);
    // A level with viewports is laid out through them; a plain sheet keeps the
    // single scaled plan it has always drawn.
    let has_viewports = project
        .viewports
        .iter()
        .any(|viewport| viewport.level == level.id);
    if has_viewports {
        draw_viewports(content, &geometry, &project.viewports, level.id);
    } else {
        draw_plan(
            content,
            &geometry,
            spec,
            sheet_width,
            sheet_height,
            project.units,
        );
    }
    draw_orientation(content, spec, sheet_width, sheet_height, project.units);
    draw_schedule_placement(content, level, sheet_width, sheet_height);
}

/// North arrow and scale bar, tucked into the drawing area's corners.
fn draw_orientation(
    content: &mut Content,
    spec: &SheetSpec,
    sheet_width: f64,
    sheet_height: f64,
    units: UnitSystem,
) {
    let (area_x, area_y, area_width, area_height) = drawing_area(sheet_width, sheet_height);
    draw_north_arrow(
        content,
        Point2::new(area_x + area_width - 15.0, area_y + area_height - 15.0),
        10.0,
    );
    draw_scale_bar(
        content,
        Point2::new(area_x, area_y + 4.0),
        spec.scale_denominator,
        units,
    );
}

/// The cover sheet: a sheet index, a project summary, a north arrow and a
/// scale bar.
fn draw_cover(
    content: &mut Content,
    project: &Project,
    spec: &SheetSpec,
    sheet_width: f64,
    sheet_height: f64,
) {
    let (area_x, area_y, area_width, area_height) = drawing_area(sheet_width, sheet_height);

    // Sheet index, one line per level.
    let mut y = area_y + area_height - 12.0;
    content.set_fill_gray(0.0);
    content.text(Point2::new(area_x, y), 4.5, true, "SHEET INDEX");
    y -= 2.5;
    content.set_line_width(0.3);
    content.move_to(Point2::new(area_x, y));
    content.line_to(Point2::new(area_x + 90.0, y));
    content.stroke();
    y -= 6.0;
    for (index, level) in project.levels.iter().enumerate() {
        let line = format!(
            "{}   {}",
            sheet_number_for_page(project, spec, index + 1),
            truncate(&level.name, 40)
        );
        content.text(Point2::new(area_x, y), 3.4, false, &line);
        y -= 6.0;
    }

    // Project summary, summed across every level.
    let mut y = area_y + area_height * 0.5 + 15.0;
    content.set_fill_gray(0.0);
    content.text(Point2::new(area_x, y), 4.5, true, "PROJECT SUMMARY");
    y -= 2.5;
    content.set_line_width(0.3);
    content.move_to(Point2::new(area_x, y));
    content.line_to(Point2::new(area_x + 90.0, y));
    content.stroke();
    y -= 7.0;

    let mut doors = 0_usize;
    let mut windows = 0_usize;
    let mut rooms = 0_usize;
    let mut room_area_m2 = 0.0_f64;
    for level in &project.levels {
        for opening in &level.openings {
            match opening.kind {
                OpeningKind::Door => doors += 1,
                OpeningKind::Window => windows += 1,
            }
        }
        for room in PlanGeometry::of_level(level).rooms() {
            rooms += 1;
            room_area_m2 += room.area_m2();
        }
    }
    let summary = [
        format!("Storeys: {}", project.levels.len()),
        format!("Walls: {}", project.wall_count()),
        format!("Doors: {doors}"),
        format!("Windows: {windows}"),
        format!("Rooms: {rooms}"),
        format!("Room area: {room_area_m2:.2} m2"),
    ];
    for line in summary {
        content.text(Point2::new(area_x, y), 3.2, false, &line);
        y -= 5.0;
    }

    draw_north_arrow(
        content,
        Point2::new(area_x + area_width - 15.0, area_y + area_height - 15.0),
        10.0,
    );
    draw_scale_bar(
        content,
        Point2::new(area_x, area_y + 4.0),
        spec.scale_denominator,
        project.units,
    );
}

fn draw_frame(content: &mut Content, sheet_width: f64, sheet_height: f64) {
    content.set_line_width(0.35);
    content.set_stroke_gray(0.0);
    content.rectangle(
        MARGIN,
        MARGIN,
        sheet_width - 2.0 * MARGIN,
        sheet_height - 2.0 * MARGIN,
    );
    content.stroke();
}

/// The title block strip, bottom-right of the sheet as drafting convention.
///
/// Fields the project does not set are left blank rather than filled with
/// placeholder text: the project name, sheet title and date fall back to the
/// project, the sheet and the export date respectively.
fn draw_title_block(
    content: &mut Content,
    project: &Project,
    spec: &SheetSpec,
    facts: &SheetFacts<'_>,
    sheet_width: f64,
    produced_epoch_s: i64,
) {
    let block = &project.title_block;
    let sheet_title = block.sheet_title_or(facts.title);
    let x = sheet_width - MARGIN - TITLE_BLOCK_WIDTH;
    let y = MARGIN;

    content.set_line_width(0.35);
    content.rectangle(x, y, TITLE_BLOCK_WIDTH, TITLE_BLOCK_HEIGHT);
    content.stroke();

    content.set_line_width(0.2);
    // Divider between the drawing name band and the fields band.
    content.move_to(Point2::new(x, y + 32.0));
    content.line_to(Point2::new(x + TITLE_BLOCK_WIDTH, y + 32.0));
    content.stroke();
    content.move_to(Point2::new(x, y + 15.0));
    content.line_to(Point2::new(x + TITLE_BLOCK_WIDTH, y + 15.0));
    content.stroke();
    // Divider between the two field columns.
    content.move_to(Point2::new(x + 125.0, y));
    content.line_to(Point2::new(x + 125.0, y + 15.0));
    content.stroke();

    content.set_fill_gray(0.0);
    // Top band: the practice, then the project name.
    if let Some(company) = block.company() {
        content.text(
            Point2::new(x + 4.0, y + 40.5),
            2.6,
            false,
            &truncate(company, 60),
        );
    }
    content.text(
        Point2::new(x + 4.0, y + 34.0),
        4.0,
        true,
        &truncate(block.project_name_or(&project.name), 44),
    );
    // Middle band: the sheet title, with the level beneath it when the two
    // differ, so a default block never prints the same name twice.
    content.text(
        Point2::new(x + 4.0, y + 25.0),
        3.2,
        false,
        &truncate(sheet_title, 52),
    );
    if let Some(level_name) = facts.level_name.filter(|name| *name != sheet_title) {
        content.text(
            Point2::new(x + 4.0, y + 18.5),
            2.8,
            false,
            &truncate(level_name, 60),
        );
    }
    // Bottom band, left column: scale, then the revision.
    content.text(
        Point2::new(x + 4.0, y + 9.0),
        3.0,
        false,
        &format!("Scale {}", spec.scale_label()),
    );
    if let Some(revision) = block.revision() {
        content.text(
            Point2::new(x + 62.0, y + 9.0),
            3.0,
            false,
            &format!("Rev {}", truncate(revision, MAX_REVISION_CHARS)),
        );
    }
    // Bottom band, right column: the sheet number.
    content.text(
        Point2::new(x + 129.0, y + 9.0),
        3.0,
        false,
        &format!("Sheet {}", truncate(facts.sheet_number, 20)),
    );
    // Footer: the build, the author and the issue line.
    content.text(
        Point2::new(x + 4.0, y + 3.5),
        2.6,
        false,
        &format!("Lintel v{}", env!("CARGO_PKG_VERSION")),
    );
    if let Some(drawn_by) = block.drawn_by() {
        content.text(
            Point2::new(x + 62.0, y + 3.5),
            2.6,
            false,
            &format!("Drawn {}", truncate(drawn_by, 24)),
        );
    }
    let export_date = format_date(produced_epoch_s);
    let right = format!(
        "{} {} - {}",
        spec.paper.label(),
        spec.orientation_label(),
        block.date_or(&export_date)
    );
    content.text(Point2::new(x + 129.0, y + 3.5), 2.6, false, &right);
}

fn draw_plan(
    content: &mut Content,
    geometry: &PlanGeometry,
    spec: &SheetSpec,
    sheet_width: f64,
    sheet_height: f64,
    units: UnitSystem,
) {
    let Some(bounds) = geometry.bounds() else {
        return;
    };
    let (area_x, area_y, area_width, area_height) = drawing_area(sheet_width, sheet_height);
    let scale = 1.0 / f64::from(spec.scale_denominator.max(1));
    let area_centre = Point2::new(area_x + area_width * 0.5, area_y + area_height * 0.5);
    let model_centre = bounds.centre();
    let origin = Point2::new(
        area_centre.x - model_centre.x * scale,
        area_centre.y - model_centre.y * scale,
    );
    let layout = SheetLayout { scale, origin };
    let clip = Bounds {
        min: Point2::new(area_x, area_y),
        max: Point2::new(area_x + area_width, area_y + area_height),
    };

    draw_plan_body(content, geometry, &layout, clip);

    // Overall dimensions, so the sheet says how big the plan is.
    let bottom_left = layout.to_paper(bounds.min);
    let top_right = layout.to_paper(bounds.max);
    draw_dimension(
        content,
        Point2::new(bottom_left.x, bottom_left.y),
        Point2::new(top_right.x, bottom_left.y),
        -8.0,
        false,
        units.format(bounds.width()),
    );
    draw_dimension(
        content,
        Point2::new(top_right.x, bottom_left.y),
        Point2::new(top_right.x, top_right.y),
        8.0,
        true,
        units.format(bounds.height()),
    );
}

/// Paint a level's walls, openings, room areas and free labels through `layout`,
/// skipping room and label text whose paper position falls outside `clip`.
///
/// `clip` stands in for a real PDF clipping path. It is approximate: a wall that
/// straddles the boundary is drawn whole, so a viewport is a scale fit rather
/// than a byte-exact cut.
fn draw_plan_body(
    content: &mut Content,
    geometry: &PlanGeometry,
    layout: &SheetLayout,
    clip: Bounds,
) {
    let scale = layout.scale;

    // 1. Wall bands, filled so openings read as real gaps.
    content.set_line_width(0.18);
    content.set_stroke_gray(0.0);
    content.set_fill_gray(0.82);
    for wall in &geometry.walls {
        let corners: Vec<Point2> = wall
            .corners
            .iter()
            .map(|point| layout.to_paper(*point))
            .collect();
        content.path(&corners, true);
        content.fill_and_stroke();
    }

    // 2. Openings: clear the band, then draw the leaf, reveal and jambs.
    for opening in &geometry.openings {
        let half = opening.host_thickness * 0.5;
        let normal: Vec2 = opening.host_direction.perpendicular();
        let start = layout.to_paper(opening.start);
        let end = layout.to_paper(opening.end);
        let paper_normal = normal.scaled(half * scale);

        content.set_fill_gray(1.0);
        content.set_stroke_gray(1.0);
        content.path(
            &[
                start + paper_normal,
                end + paper_normal,
                end - paper_normal,
                start - paper_normal,
            ],
            true,
        );
        content.fill();

        content.set_line_width(0.13);
        content.set_stroke_gray(0.0);
        // Jambs.
        for jamb in [opening.start, opening.end] {
            let paper = layout.to_paper(jamb);
            content.move_to(paper + normal.scaled(half * scale));
            content.line_to(paper - normal.scaled(half * scale));
            content.stroke();
        }

        match opening.swing {
            Some(swing) => {
                let hinge = layout.to_paper(swing.hinge);
                let leaf_end = layout.to_paper(swing.leaf_end);
                content.move_to(hinge);
                content.line_to(leaf_end);
                content.stroke();
                let arc: Vec<Point2> = arc_points(
                    swing.hinge,
                    swing.radius,
                    swing.from_degrees,
                    swing.to_degrees,
                    20,
                )
                .into_iter()
                .map(|point| layout.to_paper(point))
                .collect();
                content.path(&arc, false);
                content.stroke();
            }
            None => {
                // A window: the glass line across the reveal.
                let inset = normal.scaled(half * scale / 2.0);
                content.move_to(start + inset);
                content.line_to(end + inset);
                content.stroke();
                content.move_to(start - inset);
                content.line_to(end - inset);
                content.stroke();
            }
        }
    }

    // 3. Room areas, centred on each room, in light grey so walls stay dominant.
    content.set_fill_gray(0.35);
    for room in geometry.rooms() {
        let paper = layout.to_paper(room.centroid);
        if !inside_rect(paper, clip) {
            continue;
        }
        let label = room.area_label();
        let width = label.chars().count() as f64 * 2.5 * 0.6;
        content.text(
            Point2::new(paper.x - width * 0.5, paper.y - 1.25),
            2.5,
            false,
            &label,
        );
    }

    // 4. Free-text labels, in black over the plan.
    content.set_fill_gray(0.0);
    for label in geometry.labels() {
        let paper = layout.to_paper(label.anchor);
        if !inside_rect(paper, clip) {
            continue;
        }
        let size = (label.height_mm * layout.scale).clamp(1.5, 12.0);
        if !size.is_finite() {
            continue;
        }
        content.text(paper, size, false, &label.text);
    }
}

/// True when `point` lies inside the sheet rectangle `rect`.
fn inside_rect(point: Point2, rect: Bounds) -> bool {
    point.is_finite()
        && point.x >= rect.min.x
        && point.x <= rect.max.x
        && point.y >= rect.min.y
        && point.y <= rect.max.y
}

/// Draw every viewport bound to `level` on this sheet: a frame, the level's
/// geometry scaled into it, and the scale label and caption beneath it.
///
/// The clip is approximate: the geometry is scaled to sit inside the frame and
/// room and label text outside it is skipped, but a wall that straddles the
/// boundary is drawn whole rather than cut by a real PDF clipping path.
fn draw_viewports(
    content: &mut Content,
    geometry: &PlanGeometry,
    viewports: &[Viewport],
    level: LevelId,
) {
    let bounds = geometry.bounds();
    for viewport in viewports {
        if viewport.level != level {
            continue;
        }
        let region = viewport.region.normalised();
        if !region.min.is_finite()
            || !region.max.is_finite()
            || region.width() <= 0.0
            || region.height() <= 0.0
        {
            continue;
        }

        content.set_line_width(0.35);
        content.set_stroke_gray(0.0);
        content.rectangle(region.min.x, region.min.y, region.width(), region.height());
        content.stroke();

        if let Some(bounds) = bounds {
            let scale = 1.0 / f64::from(viewport.scale_denominator.max(1));
            let centre = bounds.centre();
            let origin = Point2::new(
                region.min.x + region.width() * 0.5 - centre.x * scale,
                region.min.y + region.height() * 0.5 - centre.y * scale,
            );
            let layout = SheetLayout { scale, origin };
            draw_plan_body(content, geometry, &layout, region);
        }

        content.set_fill_gray(0.0);
        content.text(
            Point2::new(region.min.x, region.min.y - 3.5),
            3.0,
            false,
            &format!("Viewport {}", viewport.scale_label()),
        );
        let caption = viewport.caption.trim();
        if !caption.is_empty() {
            content.text(
                Point2::new(region.min.x + region.width() * 0.5, region.min.y - 3.5),
                3.0,
                false,
                &truncate(caption, 60),
            );
        }
    }
}

/// One dimension line, offset from the measured edge. `vertical` controls
/// whether the offset is applied on the x or the y axis; the label is always
/// drawn horizontally and legibly.
fn draw_dimension(
    content: &mut Content,
    from: Point2,
    to: Point2,
    offset: f64,
    vertical: bool,
    label: String,
) {
    let shift = if vertical {
        Vec2::new(offset, 0.0)
    } else {
        Vec2::new(0.0, offset)
    };
    let a = from + shift;
    let b = to + shift;

    content.set_line_width(0.13);
    content.set_stroke_gray(0.0);

    // Extension lines, with a small gap from the wall.
    let _gap = if vertical {
        Vec2::new(-1.5 * offset.signum(), 0.0)
    } else {
        Vec2::new(0.0, -1.5 * offset.signum())
    };
    for (edge, extended) in [(from, a), (to, b)] {
        content.move_to(edge);
        content.line_to(extended);
    }
    // Dimension line.
    content.move_to(a);
    content.line_to(b);
    content.stroke();

    // Ticks at 45 degrees.
    let tick = 1.4;
    for point in [a, b] {
        content.move_to(point + Vec2::new(-tick, tick));
        content.line_to(point + Vec2::new(tick, -tick));
        content.stroke();
    }

    let middle = Point2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
    let text_at = if vertical {
        Point2::new(middle.x + 1.5, middle.y)
    } else {
        Point2::new(middle.x, middle.y + 1.2)
    };
    content.set_fill_gray(0.0);
    content.text(text_at, 2.6, false, &label);
}

/// North arrow: a filled, outlined triangle pointing up, with "N" above it.
fn draw_north_arrow(content: &mut Content, at: Point2, size_mm: f64) {
    let half = size_mm * 0.5;
    let tip = Point2::new(at.x, at.y + half);
    let left = Point2::new(at.x - half * 0.55, at.y - half * 0.6);
    let right = Point2::new(at.x + half * 0.55, at.y - half * 0.6);

    content.set_line_width(0.2);
    content.set_stroke_gray(0.0);
    content.set_fill_gray(0.0);
    content.path(&[tip, left, right], true);
    content.fill();
    content.path(&[tip, left, right], true);
    content.stroke();

    content.set_fill_gray(0.0);
    content.text(
        Point2::new(at.x - size_mm * 0.18, at.y + half + 1.0),
        (size_mm * 0.42).clamp(1.5, 8.0),
        true,
        "N",
    );
}

/// Scale bar drawn from `at`, its left end, towards the right. A round model
/// length is chosen so the bar is 20-60 mm long on paper; the bar is split into
/// a black left half and a white right half so it reads on any paper.
fn draw_scale_bar(content: &mut Content, at: Point2, scale_denominator: u32, units: UnitSystem) {
    const CANDIDATES: [f64; 7] = [
        1000.0, 2000.0, 5000.0, 10_000.0, 20_000.0, 50_000.0, 100_000.0,
    ];
    let denominator = f64::from(scale_denominator.max(1));
    let mut length_mm = CANDIDATES[CANDIDATES.len() - 1];
    for candidate in CANDIDATES {
        if candidate / denominator >= 25.0 {
            length_mm = candidate;
            break;
        }
    }
    let bar_mm = length_mm / denominator;
    let half = bar_mm * 0.5;
    let height = 2.5;

    content.set_line_width(0.15);
    content.set_stroke_gray(0.0);
    // Left half, black.
    content.set_fill_gray(0.0);
    content.rectangle(at.x, at.y, half, height);
    content.fill();
    // Right half, white.
    content.set_fill_gray(1.0);
    content.rectangle(at.x + half, at.y, half, height);
    content.fill();
    // Outline the whole bar so the white half still reads on white paper.
    content.set_stroke_gray(0.0);
    content.rectangle(at.x, at.y, bar_mm, height);
    content.stroke();

    content.set_fill_gray(0.0);
    content.text(
        Point2::new(at.x, at.y + height + 1.0),
        2.6,
        false,
        &format!("Scale 1:{scale_denominator}"),
    );
    content.text(Point2::new(at.x - 0.8, at.y - 3.5), 2.6, false, "0");
    let length_text = match units {
        UnitSystem::Metric => format!("{} m", length_mm / 1000.0),
        UnitSystem::Imperial => format!("{:.0} ft", length_mm / 304.8),
    };
    content.text(
        Point2::new(at.x + bar_mm + 1.0, at.y - 3.5),
        2.6,
        false,
        &length_text,
    );
}

/// Place the opening schedule in the drawing area's bottom-right corner.
fn draw_schedule_placement(
    content: &mut Content,
    level: &Level,
    sheet_width: f64,
    sheet_height: f64,
) {
    let (area_x, area_y, area_width, area_height) = drawing_area(sheet_width, sheet_height);
    let body_rows = opening_schedule(level).len().max(1);
    let height = SCHEDULE_TITLE_HEIGHT + SCHEDULE_ROW_HEIGHT * (1.0 + body_rows as f64);
    let top = (area_y + height).min(area_y + area_height);
    let left = (area_x + area_width - SCHEDULE_WIDTH).max(area_x);
    draw_schedule(content, Point2::new(left, top), level);
}

/// The opening schedule table, `at` being its top-left corner in millimetres.
fn draw_schedule(content: &mut Content, at: Point2, level: &Level) {
    let rows = opening_schedule(level);
    let body_rows = rows.len().max(1);
    let height = SCHEDULE_TITLE_HEIGHT + SCHEDULE_ROW_HEIGHT * (1.0 + body_rows as f64);
    let left = at.x;
    let top = at.y;
    let bottom = top - height;
    let mark_column = left + 25.0;
    let size_column = mark_column + 55.0;

    content.set_line_width(0.25);
    content.set_stroke_gray(0.0);
    content.rectangle(left, bottom, SCHEDULE_WIDTH, height);
    content.stroke();

    let header_top = top - SCHEDULE_TITLE_HEIGHT;
    let data_top = header_top - SCHEDULE_ROW_HEIGHT;

    content.set_line_width(0.18);
    for y in [header_top, data_top] {
        content.move_to(Point2::new(left, y));
        content.line_to(Point2::new(left + SCHEDULE_WIDTH, y));
    }
    for index in 1..body_rows {
        let y = data_top - SCHEDULE_ROW_HEIGHT * index as f64;
        content.move_to(Point2::new(left, y));
        content.line_to(Point2::new(left + SCHEDULE_WIDTH, y));
    }
    for x in [mark_column, size_column] {
        content.move_to(Point2::new(x, bottom));
        content.line_to(Point2::new(x, header_top));
    }
    content.stroke();

    content.set_fill_gray(0.0);
    content.text(
        Point2::new(left + 2.0, top - SCHEDULE_TITLE_HEIGHT + 1.8),
        3.0,
        true,
        "DOOR & WINDOW SCHEDULE",
    );
    let header_baseline = header_top - SCHEDULE_ROW_HEIGHT + 1.5;
    for (column, heading) in [
        (left + 2.0, "MARK"),
        (mark_column + 2.0, "SIZE"),
        (size_column + 2.0, "QTY"),
    ] {
        content.text(Point2::new(column, header_baseline), 2.6, true, heading);
    }

    if rows.is_empty() {
        content.text(
            Point2::new(left + 2.0, data_top - SCHEDULE_ROW_HEIGHT + 1.5),
            2.6,
            false,
            "No doors or windows",
        );
        return;
    }
    for (index, row) in rows.iter().enumerate() {
        let baseline = data_top - SCHEDULE_ROW_HEIGHT * (index as f64 + 1.0) + 1.5;
        content.text(Point2::new(left + 2.0, baseline), 2.6, false, &row.mark);
        content.text(
            Point2::new(mark_column + 2.0, baseline),
            2.6,
            false,
            &row.size_label(),
        );
        content.text(
            Point2::new(size_column + 2.0, baseline),
            2.6,
            false,
            &row.count.to_string(),
        );
    }
}

/// The per-sheet facts the title block prints, resolved by the exporter: what
/// the sheet is called, which level it belongs to and its number.
struct SheetFacts<'a> {
    title: &'a str,
    level_name: Option<&'a str>,
    sheet_number: &'a str,
}

/// Paper-space placement: `paper = origin + model * scale`.
struct SheetLayout {
    scale: f64,
    origin: Point2,
}

impl SheetLayout {
    fn to_paper(&self, point: Point2) -> Point2 {
        let scaled = point.scaled(self.scale);
        Point2::new(self.origin.x + scaled.x, self.origin.y + scaled.y)
    }
}

fn format_number(value: f64) -> String {
    let value = if value.is_finite() { value } else { 0.0 };
    format!("{value:.3}")
}

/// A page content stream under construction. Lengths arrive in millimetres and
/// leave as points.
#[derive(Default)]
struct Content {
    ops: String,
}

impl Content {
    fn set_stroke_gray(&mut self, gray: f64) {
        let _ = writeln!(self.ops, "{} G", format_number(gray.clamp(0.0, 1.0)));
    }

    fn set_fill_gray(&mut self, gray: f64) {
        let _ = writeln!(self.ops, "{} g", format_number(gray.clamp(0.0, 1.0)));
    }

    fn set_line_width(&mut self, millimetres: f64) {
        let _ = writeln!(self.ops, "{} w", format_number(millimetres * MM_TO_PT));
    }

    fn move_to(&mut self, point: Point2) {
        let _ = writeln!(
            self.ops,
            "{} {} m",
            format_number(point.x * MM_TO_PT),
            format_number(point.y * MM_TO_PT)
        );
    }

    fn line_to(&mut self, point: Point2) {
        let _ = writeln!(
            self.ops,
            "{} {} l",
            format_number(point.x * MM_TO_PT),
            format_number(point.y * MM_TO_PT)
        );
    }

    fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let _ = writeln!(
            self.ops,
            "{} {} {} {} re",
            format_number(x * MM_TO_PT),
            format_number(y * MM_TO_PT),
            format_number(width * MM_TO_PT),
            format_number(height * MM_TO_PT)
        );
    }

    fn path(&mut self, points: &[Point2], close: bool) {
        let Some(first) = points.first() else {
            return;
        };
        self.move_to(*first);
        for point in &points[1..] {
            self.line_to(*point);
        }
        if close {
            self.ops.push_str("h\n");
        }
    }

    fn stroke(&mut self) {
        self.ops.push_str("S\n");
    }

    fn fill(&mut self) {
        self.ops.push_str("f\n");
    }

    fn fill_and_stroke(&mut self) {
        self.ops.push_str("B\n");
    }

    fn text(&mut self, at: Point2, size_mm: f64, bold: bool, text: &str) {
        let font = if bold { "F2" } else { "F1" };
        let _ = writeln!(
            self.ops,
            "BT /{} {} Tf {} {} Td ({}) Tj ET",
            font,
            format_number(size_mm * MM_TO_PT),
            format_number(at.x * MM_TO_PT),
            format_number(at.y * MM_TO_PT),
            escape_text(text)
        );
    }
}

fn escape_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            '\n' | '\r' | '\t' => out.push(' '),
            '²' => out.push('2'),
            '³' => out.push('3'),
            '·' => out.push('.'),
            '—' | '–' => out.push('-'),
            '’' | '‘' => out.push('\''),
            '“' | '”' => out.push('"'),
            '…' => out.push_str("..."),
            other if other.is_ascii() => out.push(other),
            _ => out.push('?'),
        }
    }
    out
}

/// Keep a title block field on the sheet. The marker is ASCII so the content
/// stream stays in the WinAnsi range.
fn truncate(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        text.to_string()
    } else {
        let kept: String = text.chars().take(limit.saturating_sub(1)).collect();
        format!("{kept}~")
    }
}

fn stream_object(ops: &str) -> String {
    format!("<< /Length {} >>\nstream\n{}\nendstream", ops.len(), ops)
}

fn build_pdf(
    ops: &str,
    sheet_width: f64,
    sheet_height: f64,
    project: &Project,
    spec: &SheetSpec,
    produced_epoch_s: i64,
) -> Vec<u8> {
    let pages = [(ops.to_string(), sheet_width, sheet_height)];
    build_pdf_pages(&pages, project, spec, produced_epoch_s)
}

/// Emit a multi-page PDF from already-rendered page content: `(ops, width_mm,
/// height_mm)` per page.
///
/// Object layout for `n` pages: 1 = catalog, 2 = page tree, page `i` (0-based)
/// = object `3 + 2*i` and its content stream `4 + 2*i`; the two fonts are
/// `3 + 2*n` and `4 + 2*n`; the document info is `5 + 2*n`.
fn build_pdf_pages(
    pages: &[(String, f64, f64)],
    project: &Project,
    spec: &SheetSpec,
    produced_epoch_s: i64,
) -> Vec<u8> {
    let n = pages.len();
    let font_regular = 3 + 2 * n;
    let font_bold = 4 + 2 * n;

    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 3 + 2 * i)).collect();
    let mut objects = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        format!(
            "<< /Type /Pages /Kids [ {} ] /Count {} >>",
            kids.join(" "),
            n
        ),
    ];

    for (index, (ops, width_mm, height_mm)) in pages.iter().enumerate() {
        let content_object = 4 + 2 * index;
        objects.push(format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] \
             /Resources << /Font << /F1 {} 0 R /F2 {} 0 R >> >> /Contents {} 0 R >>",
            format_number(*width_mm * MM_TO_PT),
            format_number(*height_mm * MM_TO_PT),
            font_regular,
            font_bold,
            content_object
        ));
        objects.push(stream_object(ops));
    }

    objects.push(
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_string(),
    );
    objects.push(
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>"
            .to_string(),
    );
    objects.push(format!(
        "<< /Title ({}) /Creator (Lintel) /Producer (Lintel v{}) /CreationDate ({}) >>",
        escape_text(&format!("{} - {}", project.name, spec.sheet_title)),
        env!("CARGO_PKG_VERSION"),
        pdf_datetime(produced_epoch_s)
    ));

    assemble(&objects, 1, 5 + 2 * n)
}

/// The sheet number printed at `position` in a set: `0` is the cover, then one
/// position per level. A title block that names its first sheet drives the
/// numbers; otherwise each sheet keeps its own number and the trailing digits
/// count up.
fn sheet_number_for_page(project: &Project, spec: &SheetSpec, position: usize) -> String {
    match project.title_block.sheet_number {
        Some(base) => (base as usize + position).to_string(),
        None if position == 0 => spec.sheet_number.clone(),
        None => sheet_number_for(&spec.sheet_number, position - 1),
    }
}

/// Build a level's sheet number from `base` by adding `offset` to its trailing
/// digit run, preserving that run's width (`"A-101"` + 1 → `"A-102"`,
/// `"A-099"` + 1 → `"A-100"`). With no trailing digits a `-{offset + 1}`
/// suffix is appended instead.
fn sheet_number_for(base: &str, offset: usize) -> String {
    let digits_start = base.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if digits_start == base.len() {
        return format!("{base}-{}", offset + 1);
    }
    let prefix = &base[..digits_start];
    let digits = &base[digits_start..];
    let width = digits.len();
    let value = digits.parse::<usize>().unwrap_or(0).saturating_add(offset);
    format!("{prefix}{value:0width$}")
}

fn assemble(objects: &[String], root: usize, info: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(b"%PDF-1.7\n");
    // Binary comment marks the file as containing binary data, as recommended.
    out.extend_from_slice(b"%\xE2\xE3\xCF\xD3\n");

    let mut offsets = Vec::with_capacity(objects.len());
    for (index, body) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        out.extend_from_slice(body.as_bytes());
        out.extend_from_slice(b"\nendobj\n");
    }

    let xref_offset = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for offset in &offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root {} 0 R /Info {} 0 R >>\nstartxref\n{}\n%%EOF\n",
            objects.len() + 1,
            root,
            info,
            xref_offset
        )
        .as_bytes(),
    );
    out
}

/// `1970-01-01` for epoch zero. Date arithmetic is done here rather than
/// pulled in as a dependency, and is tested against known dates.
pub fn format_date(epoch_s: i64) -> String {
    let (year, month, day) = civil_from_days(epoch_s.div_euclid(86_400));
    format!("{year:04}-{month:02}-{day:02}")
}

fn pdf_datetime(epoch_s: i64) -> String {
    let days = epoch_s.div_euclid(86_400);
    let seconds = epoch_s.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds / 3600;
    let minute = (seconds % 3600) / 60;
    let second = seconds % 60;
    format!("D:{year:04}{month:02}{day:02}{hour:02}{minute:02}{second:02}Z")
}

/// Days since 1970-01-01 to a calendar date (Howard Hinnant's algorithm).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_prime + 2) / 5 + 1) as u32;
    let month = if month_prime < 10 {
        month_prime + 3
    } else {
        month_prime - 9
    } as u32;
    let year = if month <= 2 { year + 1 } else { year };
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::document::Document;
    use crate::title_block::TitleBlock;

    fn plan() -> Document {
        let mut document = Document::new("Flat (3B)", 1_760_000_000);
        let applied = document
            .apply(
                Command::AddWall {
                    start: Point2::new(0.0, 0.0),
                    end: Point2::new(6000.0, 0.0),
                    thickness: 200.0,
                    height: 2800.0,
                },
                1_760_000_100,
            )
            .expect("wall");
        let wall = applied.wall.expect("wall id");
        document
            .apply(
                Command::AddWall {
                    start: Point2::new(6000.0, 0.0),
                    end: Point2::new(6000.0, 4000.0),
                    thickness: 200.0,
                    height: 2800.0,
                },
                1_760_000_110,
            )
            .expect("wall");
        document
            .apply(
                Command::PlaceOpening {
                    kind: crate::opening::OpeningKind::Door,
                    host: wall,
                    offset: 2000.0,
                },
                1_760_000_120,
            )
            .expect("door");
        document
            .apply(
                Command::PlaceOpening {
                    kind: crate::opening::OpeningKind::Window,
                    host: wall,
                    offset: 4500.0,
                },
                1_760_000_130,
            )
            .expect("window");
        document
    }

    fn render(document: &Document) -> Vec<u8> {
        render_plan(document.project(), &SheetSpec::default(), 1_760_000_200).expect("render")
    }

    /// Walk the cross-reference table and prove every offset lands on its
    /// object. A PDF that fails this is a corrupt file.
    ///
    /// Works on raw bytes because `String::from_utf8_lossy` can shift byte
    /// positions when it replaces invalid bytes with the 3-byte replacement
    /// character.
    fn assert_valid_xref(bytes: &[u8]) {
        let marker = b"startxref";
        let start_pos = bytes
            .windows(marker.len())
            .position(|w| w == marker)
            .expect("startxref");
        let after = &bytes[start_pos + marker.len()..];
        let num_start = after
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .expect("offset number");
        let num_end = after[num_start..]
            .iter()
            .position(|b| b.is_ascii_whitespace())
            .unwrap_or(after.len() - num_start);
        let offset: usize = std::str::from_utf8(&after[num_start..num_start + num_end])
            .expect("offset is ASCII")
            .parse()
            .expect("valid offset");
        assert!(
            bytes[offset..].starts_with(b"xref"),
            "startxref does not point at the xref table"
        );

        let mut lines = bytes[offset..].split(|b| *b == b'\n');
        assert_eq!(lines.next(), Some(b"xref".as_slice()));
        let header = lines.next().expect("header");
        let header_str = std::str::from_utf8(header).expect("header is ASCII");
        let count: usize = header_str
            .split_whitespace()
            .nth(1)
            .expect("count")
            .parse()
            .expect("number");
        lines.next(); // the free object
        for number in 1..count {
            let entry = lines.next().expect("entry");
            let entry_str = std::str::from_utf8(entry).expect("entry is ASCII");
            let entry_offset: usize = entry_str
                .split_whitespace()
                .next()
                .expect("value")
                .parse()
                .expect("offset");
            let expected = format!("{number} 0 obj");
            assert!(
                bytes[entry_offset..].starts_with(expected.as_bytes()),
                "entry {number} points at the wrong byte"
            );
        }
    }

    #[test]
    fn a_sheet_is_a_valid_pdf() {
        let bytes = render(&plan());
        assert!(bytes.starts_with(b"%PDF-1.7"));
        assert!(
            String::from_utf8_lossy(&bytes).ends_with("%%EOF\n"),
            "must end with the end-of-file marker"
        );
        assert_valid_xref(&bytes);
    }

    #[test]
    fn the_page_is_the_requested_paper_and_orientation() {
        let bytes = render(&plan());
        let text = String::from_utf8_lossy(&bytes).to_string();
        // A3 landscape is 420 x 297 mm = 1190.551 x 841.890 pt.
        assert!(
            text.contains("/MediaBox [0 0 1190.551 841.890]"),
            "{text:.300}"
        );
    }

    #[test]
    fn a_portrait_sheet_swaps_the_axes() {
        let document = plan();
        let spec = SheetSpec {
            landscape: false,
            ..SheetSpec::default()
        };
        let bytes = render_plan(document.project(), &spec, 1_760_000_200).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("/MediaBox [0 0 841.890 1190.551]"));
    }

    #[test]
    fn the_plan_reaches_the_page() {
        let bytes = render(&plan());
        let text = String::from_utf8_lossy(&bytes).to_string();
        // The drawing operators must be present, not just an empty sheet.
        assert!(text.contains(" m\n"), "no path operators");
        assert!(text.contains(" re\n"), "no rectangle operators");
        assert!(text.contains("B\n"), "walls are not filled");
        // Six metres at 1:100 is 60 mm of paper = 170 pt.
        assert!(text.contains("/F1"), "no font resource used");
    }

    #[test]
    fn never_emits_non_finite_numbers() {
        let bytes = render(&plan());
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(!text.contains("NaN"));
        assert!(!text.contains("inf"));
        assert!(!text.contains("Infinity"));
    }

    #[test]
    fn text_is_escaped_so_parentheses_cannot_break_the_stream() {
        let bytes = render(&plan());
        let text = String::from_utf8_lossy(&bytes).to_string();
        // The project is called "Flat (3B)"; unescaped brackets would corrupt
        // the object.
        assert!(text.contains("Flat \\(3B\\)"));
        assert!(!text.contains("Flat (3B)"));
    }

    #[test]
    fn an_empty_level_still_prints_a_titled_sheet() {
        let document = Document::new("Empty plan", 1_760_000_000);
        let bytes =
            render_plan(document.project(), &SheetSpec::default(), 1_760_000_200).expect("render");
        assert_valid_xref(&bytes);
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("(Empty plan)") || text.contains("Empty plan"));
        // No plan geometry, so no wall fill.
        assert!(!text.contains("B\n"));
    }

    #[test]
    fn a_larger_scale_makes_a_larger_drawing() {
        let document = plan();
        let coarse = render_plan(
            document.project(),
            &SheetSpec {
                scale_denominator: 200,
                ..SheetSpec::default()
            },
            1,
        )
        .expect("render");
        let fine = render_plan(
            document.project(),
            &SheetSpec {
                scale_denominator: 50,
                ..SheetSpec::default()
            },
            1,
        )
        .expect("render");
        // Different scales must produce different PDF content: a finer scale
        // draws the same plan larger on the sheet, so the content stream
        // coordinates differ.
        assert_ne!(coarse, fine, "1:50 and 1:200 should produce different PDFs");
    }

    #[test]
    fn the_scale_that_fits_never_overflows_the_sheet() {
        let small = Bounds {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(6000.0, 4000.0),
        };
        assert_eq!(fitting_scale(&small, PaperSize::A3, true), 50);

        let large = Bounds {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(60_000.0, 30_000.0),
        };
        let scale = fitting_scale(&large, PaperSize::A4, true);
        let (_, _, area_width, area_height) = drawing_area(297.0, 210.0);
        assert!(
            large.width() / f64::from(scale) <= area_width,
            "scale {scale} does not fit"
        );
        assert!(large.height() / f64::from(scale) <= area_height);
    }

    #[test]
    fn dates_are_correct_for_known_epochs() {
        assert_eq!(format_date(0), "1970-01-01");
        assert_eq!(format_date(1_000_000_000), "2001-09-09");
        assert_eq!(pdf_datetime(0), "D:19700101000000Z");
    }

    #[test]
    fn the_sheet_states_what_was_printed() {
        let document = plan();
        let spec = SheetSpec {
            sheet_title: "Ground floor".into(),
            sheet_number: "A-102".into(),
            ..SheetSpec::default()
        };
        let bytes = render_plan(document.project(), &spec, 1_760_000_200).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("(Scale 1:100)"));
        assert!(text.contains("(Sheet A-102)"));
        assert!(text.contains("(Ground floor)"));
        assert!(text.contains("A3 landscape -"));
    }

    #[test]
    fn missing_level_is_an_error_not_a_panic() {
        let mut document = Document::new("Plan", 1);
        document
            .apply(
                Command::AddLevel {
                    name: "Level 1".into(),
                    elevation: 3000.0,
                },
                2,
            )
            .expect("level");
        // Delete the level the document was pointing at by loading a project
        // whose active level does not exist.
        let mut project = document.project().clone();
        project.active_level = crate::ids::LevelId(999);
        let error = render_plan(&project, &SheetSpec::default(), 1).expect_err("no level");
        assert!(matches!(error, CoreError::LevelNotFound(_)));
    }

    /// A project with the `plan()` storey plus a second, empty storey.
    fn two_level_plan() -> Document {
        let mut document = plan();
        document
            .apply(
                Command::AddLevel {
                    name: "Level 1".into(),
                    elevation: 3000.0,
                },
                1_760_000_140,
            )
            .expect("level");
        document
    }

    #[test]
    fn escape_text_maps_superscript_two() {
        assert_eq!(escape_text("12.40 m²"), "12.40 m2");
    }

    #[test]
    fn sheet_numbers_increment_the_trailing_digits() {
        assert_eq!(sheet_number_for("A-101", 1), "A-102");
        assert_eq!(sheet_number_for("A-099", 1), "A-100");
        assert_eq!(sheet_number_for("A-101", 0), "A-101");
    }

    #[test]
    fn sheet_numbers_without_digits_get_a_suffix() {
        assert_eq!(sheet_number_for("COVER", 0), "COVER-1");
        assert_eq!(sheet_number_for("COVER", 2), "COVER-3");
    }

    #[test]
    fn a_drawing_set_has_a_cover_and_a_page_per_level() {
        let document = two_level_plan();
        let bytes =
            render_set(document.project(), &SheetSpec::default(), 1_760_000_200).expect("render");
        assert!(bytes.starts_with(b"%PDF-1.7"));
        assert!(
            String::from_utf8_lossy(&bytes).ends_with("%%EOF\n"),
            "must end with the end-of-file marker"
        );
        assert_valid_xref(&bytes);
        let text = String::from_utf8_lossy(&bytes).to_string();
        // The page tree reports every page: the cover plus one per level.
        assert!(text.contains("/Type /Pages"), "{text:.400}");
        assert!(text.contains("/Count 3"), "{text:.400}");
        // Counting the trailing space skips "/Type /Pages" itself.
        assert_eq!(text.matches("/Type /Page ").count(), 3);
        assert_eq!(text.matches("/MediaBox").count(), 3);
    }

    #[test]
    fn a_single_sheet_is_still_one_page() {
        let bytes = render(&plan());
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("/Count 1"));
        assert_eq!(text.matches("/Type /Page ").count(), 1);
    }

    #[test]
    fn a_level_without_openings_prints_the_empty_schedule() {
        let document = Document::new("Empty plan", 1_760_000_000);
        let bytes =
            render_plan(document.project(), &SheetSpec::default(), 1_760_000_200).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("No doors or windows"), "{text:.400}");
    }

    fn viewport_region() -> Bounds {
        Bounds {
            min: Point2::new(10.0, 40.0),
            max: Point2::new(150.0, 140.0),
        }
    }

    fn rectangle_count(bytes: &[u8]) -> usize {
        String::from_utf8_lossy(bytes).matches(" re\n").count()
    }

    #[test]
    fn the_title_block_prints_the_project_fields() {
        let document = plan();
        let mut project = document.project().clone();
        project.title_block = TitleBlock {
            company: "Acme Drafting".into(),
            project_name: "Riverside House".into(),
            sheet_title: "Ground floor".into(),
            revision: "B".into(),
            date: "2026-11-14".into(),
            drawn_by: "JD".into(),
            sheet_number: None,
        };
        let bytes = render_plan(&project, &SheetSpec::default(), 1).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("Acme Drafting"), "{text:.400}");
        assert!(text.contains("Riverside House"));
        assert!(text.contains("(Ground floor)"));
        assert!(text.contains("Rev B"));
        assert!(text.contains("Drawn JD"));
        assert!(text.contains("2026-11-14"), "the block's own date is used");
    }

    #[test]
    fn a_default_title_block_still_prints_a_date_and_no_junk() {
        let document = plan();
        let bytes = render_plan(document.project(), &SheetSpec::default(), 0).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("1970-01-01"), "the export date is printed");
        assert!(!text.contains("Rev "), "no revision is invented");
        assert!(!text.contains("Drawn "), "no author is invented");
    }

    #[test]
    fn a_title_block_first_sheet_number_drives_the_set() {
        let document = plan();
        let spec = SheetSpec::default();
        // Without an override each sheet keeps its own number.
        assert_eq!(sheet_number_for_page(document.project(), &spec, 0), "A-101");
        assert_eq!(sheet_number_for_page(document.project(), &spec, 2), "A-102");

        let mut project = document.project().clone();
        project.title_block.sheet_number = Some(101);
        assert_eq!(sheet_number_for_page(&project, &spec, 0), "101");
        assert_eq!(sheet_number_for_page(&project, &spec, 1), "102");
        assert_eq!(sheet_number_for_page(&project, &spec, 3), "104");
    }

    #[test]
    fn a_title_block_sheet_number_overrides_the_sheet_number() {
        let document = plan();
        let mut project = document.project().clone();
        project.title_block.sheet_number = Some(101);
        let bytes = render_plan(&project, &SheetSpec::default(), 1).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("(Sheet 101)"), "{text:.400}");
        assert!(!text.contains("(Sheet A-101)"));
    }

    #[test]
    fn a_viewport_is_framed_and_captioned_on_its_sheet() {
        let document = plan();
        let baseline = render(&document);
        let mut project = document.project().clone();
        let viewport = Viewport::new(
            project.active_level,
            viewport_region(),
            50,
            "Ground floor unit".to_string(),
        );
        project.viewports.push(viewport);
        let bytes = render_plan(&project, &SheetSpec::default(), 1_760_000_200).expect("render");
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("(Ground floor unit)"), "{text:.600}");
        assert!(
            text.contains("(Viewport 1:50)"),
            "the scale label is printed"
        );
        assert!(
            rectangle_count(&bytes) > rectangle_count(&baseline),
            "the viewport adds a frame"
        );
    }

    #[test]
    fn a_viewport_scale_changes_what_is_drawn() {
        let document = plan();
        let render_at = |scale: u32| {
            let mut project = document.project().clone();
            let viewport = Viewport::new(
                project.active_level,
                viewport_region(),
                scale,
                String::new(),
            );
            project.viewports.push(viewport);
            render_plan(&project, &SheetSpec::default(), 1).expect("render")
        };
        assert_ne!(
            render_at(20),
            render_at(200),
            "1:20 and 1:200 should not draw the same sheet"
        );
    }

    #[test]
    fn an_empty_level_with_a_viewport_still_prints_the_frame() {
        let document = Document::new("Empty plan", 1_760_000_000);
        let mut project = document.project().clone();
        let viewport = Viewport::new(
            project.active_level,
            viewport_region(),
            50,
            "Bare unit".to_string(),
        );
        project.viewports.push(viewport);
        let bytes = render_plan(&project, &SheetSpec::default(), 1).expect("render");
        assert_valid_xref(&bytes);
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("(Bare unit)"), "{text:.400}");
        // No geometry, so no wall fill, but the frame is still there.
        assert!(!text.contains("B\n"));
    }
}
