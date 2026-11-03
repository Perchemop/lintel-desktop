//! Length display and parsing.
//!
//! The model is always millimetres; units are a presentation choice. Parsing
//! is deliberately forgiving about separators (`3,600`, `3600`, `3.6 m`) and
//! strict about nonsense (`""`, `abc`, negative lengths) so the inspector can
//! reject input instead of silently storing a wrong dimension.

use serde::{Deserialize, Serialize};

pub const MM_PER_METRE: f64 = 1000.0;
pub const MM_PER_INCH: f64 = 25.4;
pub const MM_PER_FOOT: f64 = 304.8;

/// Which unit family the UI displays. Stored in the project file so a shared
/// file opens in the units the author used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UnitSystem {
    #[default]
    Metric,
    Imperial,
}

impl UnitSystem {
    pub fn label(self) -> &'static str {
        match self {
            UnitSystem::Metric => "Metric (mm / m)",
            UnitSystem::Imperial => "Imperial (ft / in)",
        }
    }

    /// Human-readable length for the status bar and the drawing.
    pub fn format(self, millimetres: f64) -> String {
        match self {
            UnitSystem::Metric => format_metric(millimetres),
            UnitSystem::Imperial => format_imperial(millimetres),
        }
    }
}

/// Metric display: metres above a metre, whole millimetres below it.
pub fn format_metric(millimetres: f64) -> String {
    let value = if millimetres.is_finite() {
        millimetres
    } else {
        0.0
    };
    if value.abs() >= MM_PER_METRE {
        let metres = value / MM_PER_METRE;
        let text = format!("{metres:.2}");
        format!("{} m", trim_trailing_zeros(&text))
    } else {
        format!("{} mm", value.round() as i64)
    }
}

/// Imperial display: feet-and-inches to the nearest sixteenth of an inch.
pub fn format_imperial(millimetres: f64) -> String {
    let value = if millimetres.is_finite() {
        millimetres
    } else {
        0.0
    };
    let sixteenths = (value / MM_PER_INCH * 16.0).round() as i64;
    let sign = if sixteenths < 0 { "-" } else { "" };
    let sixteenths = sixteenths.abs();
    let whole_inches = sixteenths / 16;
    let fraction = sixteenths % 16;
    let feet = whole_inches / 12;
    let inches = whole_inches % 12;
    if fraction == 0 {
        format!("{sign}{feet}'-{inches}\"")
    } else {
        let (num, den) = reduce_sixteenths(fraction);
        format!("{sign}{feet}'-{inches} {num}/{den}\"")
    }
}

fn reduce_sixteenths(fraction: i64) -> (i64, i64) {
    let mut num = fraction;
    let mut den = 16;
    while num % 2 == 0 && den % 2 == 0 {
        num /= 2;
        den /= 2;
    }
    (num, den)
}

fn trim_trailing_zeros(text: &str) -> String {
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        text.to_string()
    }
}

/// Parse a user-typed length into millimetres.
///
/// Accepted: `3600`, `3600 mm`, `3.6 m`, `3,6 m`, `72"`, `6'`, `6'-3"`.
/// Rejected: empty input, unknown units, negative or non-finite results.
pub fn parse_length(input: &str) -> Option<f64> {
    let text = input.trim().to_ascii_lowercase().replace(',', ".");
    if text.is_empty() {
        return None;
    }

    // Feet/inches first: the apostrophe and quote marks are unambiguous.
    if text.contains('\'') || text.contains('"') || text.contains('’') {
        return parse_feet_inches(&text);
    }

    let (number, unit) = split_number_and_unit(&text)?;
    let value: f64 = number.parse().ok()?;
    let millimetres = match unit.as_str() {
        "" | "mm" => value,
        "cm" => value * 10.0,
        "m" => value * MM_PER_METRE,
        "in" | "\"" => value * MM_PER_INCH,
        "ft" | "'" => value * MM_PER_FOOT,
        _ => return None,
    };
    if millimetres.is_finite() && millimetres > 0.0 {
        Some(millimetres)
    } else {
        None
    }
}

fn split_number_and_unit(text: &str) -> Option<(String, String)> {
    let trimmed = text.trim();
    let mut split_at = trimmed.len();
    for (index, ch) in trimmed.char_indices() {
        if !(ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' || ch.is_whitespace()) {
            split_at = index;
            break;
        }
    }
    let number = trimmed[..split_at].trim().to_string();
    let unit = trimmed[split_at..].trim().to_string();
    if number.is_empty() {
        None
    } else {
        Some((number, unit))
    }
}

fn parse_feet_inches(text: &str) -> Option<f64> {
    let cleaned = text.replace('’', "'");
    let mut total_inches = 0.0;
    let mut remainder = cleaned.as_str();

    if let Some(index) = remainder.find('\'') {
        let feet_text = remainder[..index].trim();
        let feet: f64 = feet_text.parse().ok()?;
        total_inches += feet * 12.0;
        remainder = remainder[index + 1..].trim();
    }

    remainder = remainder
        .trim_end_matches('"')
        .trim_end_matches("in")
        .trim();
    if !remainder.is_empty() {
        let inches: f64 = remainder.parse().ok()?;
        total_inches += inches;
    }

    let millimetres = total_inches * MM_PER_INCH;
    if millimetres.is_finite() && millimetres > 0.0 {
        Some(millimetres)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_formatting_switches_at_one_metre() {
        assert_eq!(format_metric(900.0), "900 mm");
        assert_eq!(format_metric(1000.0), "1 m");
        assert_eq!(format_metric(3600.0), "3.6 m");
        assert_eq!(format_metric(0.0), "0 mm");
    }

    #[test]
    fn imperial_formatting_uses_feet_and_inches() {
        assert_eq!(format_imperial(304.8), "1'-0\"");
        assert_eq!(format_imperial(914.4), "3'-0\"");
        assert_eq!(format_imperial(0.0), "0'-0\"");
    }

    #[test]
    fn parses_plain_and_unit_suffixed_numbers() {
        assert_eq!(parse_length("3600"), Some(3600.0));
        assert_eq!(parse_length("3600 mm"), Some(3600.0));
        assert_eq!(parse_length("3.6 m"), Some(3600.0));
        assert_eq!(parse_length("3,6 m"), Some(3600.0));
        assert_eq!(parse_length("360 cm"), Some(3600.0));
    }

    #[test]
    fn parses_feet_and_inches() {
        let mm = parse_length("6'-3\"").expect("length");
        assert!((mm - (6.0 * 304.8 + 3.0 * 25.4)).abs() < 1e-6);
        let mm = parse_length("12'").expect("length");
        assert!((mm - 3657.6).abs() < 1e-6);
    }

    #[test]
    fn rejects_nonsense_and_non_positive_input() {
        assert_eq!(parse_length(""), None);
        assert_eq!(parse_length("abc"), None);
        assert_eq!(parse_length("0"), None);
        assert_eq!(parse_length("-2000"), None);
        assert_eq!(parse_length("12 parsecs"), None);
    }

    #[test]
    fn unit_labels_are_explicit() {
        assert_eq!(UnitSystem::Metric.label(), "Metric (mm / m)");
        assert_eq!(UnitSystem::Imperial.label(), "Imperial (ft / in)");
    }
}
