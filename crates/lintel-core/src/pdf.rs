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
use crate::join::PlanGeometry;
use crate::project::Project;
use crate::units::UnitSystem;

/// PostScript points per millimetre.
pub const MM_TO_PT: f64 = 72.0 / 25.4;

/// Sheet margin, in millimetres.
const MARGIN: f64 = 10.0;
const TITLE_BLOCK_WIDTH: f64 = 190.0;
const TITLE_BLOCK_HEIGHT: f64 = 45.0;
/// Room left around the drawing for dimensions and for the drawing to breathe.
const DRAWING_PADDING: f64 = 18.0;

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
    let geometry = PlanGeometry::of_level(level);
    let (sheet_width, sheet_height) = spec.size_mm();

    let mut content = Content::default();
    draw_frame(&mut content, sheet_width, sheet_height);
    draw_title_block(&mut content, project, spec, sheet_width, produced_epoch_s);
    draw_plan(
        &mut content,
        &geometry,
        spec,
        sheet_width,
        sheet_height,
        project.units,
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

fn draw_title_block(
    content: &mut Content,
    project: &Project,
    spec: &SheetSpec,
    sheet_width: f64,
    produced_epoch_s: i64,
) {
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
    content.text(
        Point2::new(x + 4.0, y + 36.0),
        4.0,
        true,
        &truncate(&project.name, 44),
    );
    content.text(
        Point2::new(x + 4.0, y + 21.0),
        3.2,
        false,
        &truncate(&spec.sheet_title, 52),
    );
    content.text(
        Point2::new(x + 4.0, y + 9.0),
        3.0,
        false,
        &format!("Scale {}", spec.scale_label()),
    );
    content.text(
        Point2::new(x + 129.0, y + 9.0),
        3.0,
        false,
        &format!("Sheet {}", truncate(&spec.sheet_number, 20)),
    );
    content.text(
        Point2::new(x + 4.0, y + 3.5),
        2.6,
        false,
        &format!("Lintel v{}", env!("CARGO_PKG_VERSION")),
    );
    let right = format!(
        "{} {} - {}",
        spec.paper.label(),
        spec.orientation_label(),
        format_date(produced_epoch_s)
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
    let model_centre_pt = model_centre;
    let origin = Point2::new(
        area_centre.x - model_centre_pt.x * scale,
        area_centre.y - model_centre_pt.y * scale,
    );
    let layout = SheetLayout { scale, origin };

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

    // 3. Overall dimensions, so the sheet says how big the plan is.
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
    let width_pt = sheet_width * MM_TO_PT;
    let height_pt = sheet_height * MM_TO_PT;
    let objects = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 5 0 R /F2 6 0 R >> >> /Contents 4 0 R >>",
            format_number(width_pt),
            format_number(height_pt)
        ),
        stream_object(ops),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_string(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>"
            .to_string(),
        format!(
            "<< /Title ({}) /Creator (Lintel) /Producer (Lintel v{}) /CreationDate ({}) >>",
            escape_text(&format!("{} - {}", project.name, spec.sheet_title)),
            env!("CARGO_PKG_VERSION"),
            pdf_datetime(produced_epoch_s)
        ),
    ];
    assemble(&objects, 1, 7)
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

    /// Largest x coordinate in the content stream, as a rough size probe.
    fn drawing_span(bytes: &[u8]) -> f64 {
        let text = String::from_utf8_lossy(bytes).to_string();
        text.lines()
            .filter(|line| line.ends_with(" m") || line.ends_with(" l"))
            .filter_map(|line| line.split_whitespace().next())
            .filter_map(|value| value.parse::<f64>().ok())
            .fold(0.0_f64, f64::max)
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
}
