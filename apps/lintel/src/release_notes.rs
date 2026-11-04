//! Embedded release history shown in the in-app "Release notes" window
//! (guide section 9: in-app version and release notes).
//!
//! The data here mirrors `CHANGELOG.md`. It is deliberately plain `&'static`
//! data so the window is cheap to render and works completely offline.

/// One published release, in the shape the release-notes window renders.
pub struct ReleaseNote {
    /// Version string, without a leading `v`.
    pub version: &'static str,
    /// Release date, `YYYY-MM-DD`.
    pub date: &'static str,
    /// One-or-two-sentence honest summary.
    pub summary: &'static str,
    /// What the release added.
    pub added: &'static [&'static str],
    /// What the release changed.
    pub changed: &'static [&'static str],
    /// What the release fixed.
    pub fixed: &'static [&'static str],
    /// Honest known limitations of this release.
    pub known_limitations: &'static [&'static str],
    /// Approximate tokens consumed during this release's development.
    pub tokens_this_release: u64,
}

/// Explains how the token figures below should be read.
pub const TOKEN_METRIC_NOTE: &str = "Token figures are approximate totals recorded at release time, covering \
development and CI-fix sessions for that release. They are reported for transparency, not as an exact accounting.";

/// Approximate total tokens consumed since development began.
pub const TOTAL_TOKENS: u64 = 4_900_000;

/// Every release, newest first.
pub const RELEASES: &[ReleaseNote] = &[
    ReleaseNote {
        version: "0.2.0",
        date: "2026-11-05",
        summary: "Joint resolution for walls of unequal thickness, plus linear dimension lines \
with numeric labels, move/delete editing and persistence.",
        added: &[
            "Dimension lines (aligned, horizontal and vertical) with numeric labels and offset",
            "Place, move, offset and delete a dimension from the canvas and the Inspector",
            "Dimension hit testing that takes priority over the wall beneath it",
            "Dimension save/load round-trip and full undo/redo",
            "Cross-platform CI build matrix (Linux, macOS, Windows)",
            "Shared crash-report and update-check crates",
        ],
        changed: &[
            "Wall joins now extend each wall by half of the other wall's thickness, so outside corners close cleanly",
            "The app version is shown subtly in the toolbar and opens the release-notes window",
        ],
        fixed: &[
            "PDF cross-reference parsing now reads bytes directly, so files with non-UTF-8 content open correctly",
        ],
        known_limitations: &[
            "Dimension label text is drawn horizontally; rotated labels for vertical/aligned dimensions are planned",
            "No Windows/macOS/Linux installers yet; download the zipped binary from the Releases page",
            "Update checking opens the Releases page rather than replacing the running binary in place",
        ],
        tokens_this_release: 2_200_000,
    },
    ReleaseNote {
        version: "0.1.0",
        date: "2026-11-04",
        summary: "First functional test build: draw walls, place doors and windows, load, save \
and export a PDF plan. Joint resolution for unequal thicknesses was a known limitation.",
        added: &[
            "Walls, doors, windows and levels with real geometry and validation",
            "Snapping, pan and zoom on the plan canvas",
            "Undo/redo history (up to 128 edits)",
            "Save and reopen plans as JSON, and export a PDF sheet at true scale",
            "Metric and Imperial unit display",
        ],
        changed: &[],
        fixed: &[],
        known_limitations: &[
            "Joint resolution assumed equal wall thicknesses",
            "No dimension lines in the PDF",
            "No installers and no accessibility features",
        ],
        tokens_this_release: 2_700_000,
    },
];
