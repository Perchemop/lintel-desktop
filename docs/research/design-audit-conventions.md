# Design Audit — Drafting App Interaction Conventions (Public Standards)

**Date:** 2026-11-18  
**Public sources — original summaries**  
**Human gate: approval required before implementation depends on this**

---

## 1. ISO / ASME Drawing Conventions Relevant to UI

### 1.1 Title Blocks (ISO 7200, ISO 9431, ASME Y14.100)
- **Position**: Bottom-right corner of the drawing sheet (ISO 7200 does not fix position; ISO 9431 for construction drawings places it bottom-right; ASME Y14.100 places it across the bottom). Rationale: when a folded drawing is filed, the bottom-right corner is face-up — the title block is read without unfolding.
- **Mandatory fields (ISO 7200)**: Identification number, title, document type, date of issue, sheet number, creator, approval person, legal owner. Revision and status are optional.
- **Scale & projection symbol**: NOT in the title block per ISO 7200 — they are properties of a *view*, not the document. Convention: "scales as shown" in title block; actual scale bars and projection symbols (first-angle/third-angle) placed adjacent to each view. ASME/ANSI practice often puts both in the block.
- **Sheet sizes**: ISO 5457 / ISO 216 (A4–A0); ASME Y14.1 (ANSI A–E).
- **Border**: Minimum 10 mm blank outer border for printer margins.

### 1.2 Scale Bars
- **Placement**: Adjacent to the view they apply to, not in the title block (ISO 128-1 §5b: "values for dimensions shall not be determined or scaled directly from the technical drawing" — but scale bars enable visual estimation).
- **Multiple scales**: When a sheet has multiple views at different scales, each view shows its own scale bar. The title block may state the "main scale" with "other scales as shown".
- **Recommended scales (ISO 5455 / EN ISO 5455)**: 1:1, 1:2, 1:5, 1:10, 1:20, 1:50, 1:100, 1:200, 1:500, 1:1000, 1:2000, 1:5000 (and reciprocals for enlargement). Construction often uses 1:50, 1:100, 1:200.
- **Visual form**: Graphic bar scale (not just numeric ratio) — survives reproduction/copying better.

### 1.3 Dimension Text Placement (ISO 129-1, ASME Y14.5)
- **Text position**: Above the dimension line, parallel to it, centred horizontally. Gap between dimension line and text: ~1 mm (≈ 1/25 inch).
- **Orientation**: For horizontal dimensions, text reads left-to-right (horizontal). For vertical dimensions, text reads from right side (rotated 90° CCW) so it's readable from the right. For angled dimensions, text is parallel to the dimension line, readable from the right or bottom.
- **Arrowheads**: Filled, 1:3 length:width ratio (ISO 129-1). Touch the extension lines.
- **Extension lines**: Gap of ~1 mm from the object outline; extend ~2-3 mm past the dimension line.
- **Units**: SI (mm) — no unit symbol on dimension text if mm; if metres, show "m". Imperial: feet-inches with fractional inches (1/16" precision typical).
- **Reference dimensions**: In brackets/parentheses — for info only, not inspected.
- **Theoretically exact / basic dimensions**: Boxed (ASME Y14.5) or noted per ISO 1101.

### 1.4 Line Conventions (ISO 128-2)
- **Visible outline**: Thick continuous (0.7 mm)
- **Hidden outline**: Thin dashed (0.25 mm)
- **Centre lines**: Thin long-dash short-dash (0.25 mm)
- **Dimension / extension / leader lines**: Thin continuous (0.25 mm)
- **Hatching / section lines**: Thin continuous (0.25 mm) at 45°
- **Cutting plane lines**: Thick long-dash double-short-dash (0.7 mm)
- **Minimum line weight**: 0.25 mm for reproduction without reduction
- **Thickness ratio**: Each thickness ≥ 2× the next thinner
- **Minimum gap between parallel lines**: 0.7 mm

### 1.5 Lettering (ISO 3098-1)
- **Minimum height**: 2.5 mm (drawings reproduced without reduction), 3.5 mm (reduced/microfilmed)
- **Standard fonts**: ISO 3098-1 defines "isocpeur" (CAD) and similar; in practice, monospace or technical sans-serif (e.g., ISOCPEUR, Tekton, Arial Narrow Condensed).
- **All caps** for notes and dimensions; sentence case for title block fields.

### 1.6 URLs Used
- ISO 128-1:2020: https://www.iso.org/standard/65296.html
- ISO 128-1 preview (iteh.ai): https://cdn.standards.iteh.ai/samples/65296/94d88cd9fbd74672bdc3c842779c1232/ISO-128-1-2020.pdf
- ISO 128 Wikipedia: https://en.wikipedia.org/wiki/ISO_128
- ISO 7200:2004: https://www.iso.org/standard/35446.html
- Title block explained (RoyMech): https://www.roymech.co.uk/Useful_Tables/Drawing/Title_blocks.html
- Drawing sheet layout & title block (blog.draftsperson.net): https://blog.draftsperson.net/drawing-sheet-layout-metric
- How to read engineering drawing (calibr.pro): https://calibr.pro/guides/how-to-read-engineering-drawing/
- VIRSTEM standards overview: https://sites.google.com/tktk.ee/virstem/fundamentals-of-modelling-and-engineering-graphics/5-standards/5-1-overview-of-standards

---

## 2. Colour / Accessibility Conventions for CAD

### 2.1 Canvas Background: Dark vs. Paper-White
- **Traditional CAD (AutoCAD, BricsCAD, nanoCAD, QCAD)**: **Dark canvas** (near-black, dark grey) by default. Rationale: reduces eye strain during long sessions; coloured entity lines (cyan, magenta, yellow, green, white) pop on dark.
- **Architectural / AEC (Revit, ArchiCAD, LibreCAD default)**: **Paper-white / warm off-white** canvas. Rationale: mimics paper drawings; line weights and colours read as they will print.
- **Modern trend**: **Both modes supported**, user-preference persisted. AutoCAD 2020+ has dark/light theme; BricsCAD has dark mode; LibreCAD supports themes.
- **Lintel's current choice**: Warm off-white (`PAPER` = `rgb(252,248,242)`) — aligns with architectural tradition. **Must support dark mode** for accessibility and user preference (guide II.30.6).

### 2.2 Semantic Colour Roles (guide II.30.6 + CAD conventions)
| Role | CAD Convention | WCAG AA Target |
|------|----------------|----------------|
| `surface.base` / `.raised` / `.sunken` | Panel backgrounds, dock widgets, canvas | Distinguishable without borders alone |
| `text.primary` / `.secondary` / `.muted` / `.disabled` | Entity labels, dimension text, UI labels | 4.5:1 on surface |
| `accent` / `.hover` / `.on` | Active tool, selection highlight, primary action | 4.5:1 text on accent; 3:1 accent on surface |
| `danger` / `.on` | Delete, destructive actions, errors | 4.5:1 text on danger; never only colour signal |
| `warning` / `.on` | Validation warnings, near-limit values | 3:1 on surface |
| `success` / `.on` | Valid input, connected status | 3:1 on surface |
| `border.subtle` / `.strong` | Panel dividers, input outlines | 3:1 strong on surface |
| `focus.ring` | Keyboard focus indicator | 3:1 on control AND surface |
| `overlay.scrim` | Modal backdrop | Keeps dialog text readable |
| `selection.bg` / `.text` | Selected entities, text selection | Contrast as text-on-fill |
| `grid.minor` / `.major` | Construction grid lines | Subtle on canvas; major ≥ 3:1 on canvas |

**CAD-specific entity colours** (layer-based, user-configurable):
- **ByLayer** (default): entity inherits layer colour
- **Standard layer colours** (AutoCAD ACI): 1=Red, 2=Yellow, 3=Green, 4=Cyan, 5=Blue, 6=Magenta, 7=White/Black, 8=Dark Grey, 9=Light Grey...
- **Lintel's current**: `WALL_FILL` (warm beige), `WALL_STROKE` (dark brown), `DOOR_LEAF` (ink), `WINDOW_LINE` (muted brown), `GRID_MINOR/MAJOR` (subtle warm greys), `COBALT` (selection/active), `GREEN` (endpoint snap)

### 2.3 Colour-Never-Alone Rule (guide II.30.7.2, WCAG 1.4.1)
- **Selection**: Fill + marker (grip) + status bar text
- **Error**: Red border + icon + text message + focus
- **Snap active**: Cursor glyph + status chip + tooltip
- **Layer locked**: Lock icon + muted text + cursor change

### 2.4 URLs Used
- AutoCAD status bar (colour-coded toggles): https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-E3B34B0A-EA98-45E9-937A-BF5FEF4152DB.htm
- AutoCAD dark/light theme: https://help.autodesk.com/view/ACD/2027/ENU/?guid=GUID-0E4DE630-C2D9-4C0B-889B-16774D8AB1EE
- BricsCAD status bar: https://help.bricsys.com/en-us/document/bricscad/getting-started-with-bricscad/exploring-the-interface/the-status-bar
- nanoCAD snap/grid: https://nanocad.com/learning/online-help/nanocad-platform/snap-and-grid-mode
- Rhino status bar: https://docs.mcneel.com/rhino/8mac/help/en-us/user_interface/rhino_window.htm

---

## 3. Command Palette UX Best Practices (Public Sources)

### 3.1 Invocation
- **Shortcut**: `Ctrl+K` (Windows/Linux), `Cmd+K` (macOS) — de facto standard (VS Code, Sublime Text, Raycast, Superhuman, Linear, GitHub, Figma, Notion). Alternative: `Ctrl+/` or `Cmd+Shift+P` (VS Code command palette).
- **Visible trigger**: Search/icon button in header with shortcut printed (e.g., "⌘K") — essential for discoverability and touch.
- **Focus on open**: Input focused immediately, caret visible, no extra click.

### 3.2 Empty Query State (Before User Types)
- **Show**: Recent commands (last 5–10), frequent commands, contextual actions for current view/selection, common creation commands.
- **Hint line**: "Search commands, tools, and settings" or capability statement.
- **Do not show**: Full command list (overwhelming), dashboard, promotional content.

### 3.3 Fuzzy Matching & Ranking
- **Algorithm**: Fuzzy subsequence (letters in order, gaps allowed) with scoring:
  - +8 for match at word boundary (start, after space/dash/underscore, camelCase boundary)
  - +5 for consecutive matched letters
  - Penalty for gaps
- **Exact/prefix matches** rank above fuzzy matches.
- **Aliases/synonyms**: "archive" finds "Mark Done"; "dark" finds "Theme"; "inv" finds "Invoices".
- **Recency/frequency (frecency)**: Boost recently/often used items — cap influence so list doesn't freeze.
- **Context boost**: Actions relevant to current page/selection rank higher.
- **Stable ordering**: Don't reshuffle results as async results arrive; append new groups.

### 3.4 Result Structure & Grouping
- **Three result types** (mixed in one list, grouped):
  1. **Navigation** (Go to Settings, Open Reports) — short label, page icon, route shortcut
  2. **Records/Entities** (Projects, Contacts) — secondary line (owner, status)
  3. **Actions** (Create Project, Toggle Dark Mode) — verb-first, show shortcut, confirm destructive
- **Contextual actions** only shown when relevant (e.g., "Assign to me" on a ticket).
- **Help/fallback** at bottom (Search docs, Keyboard shortcuts).
- **Nested commands**: "Change status ›" opens sub-list; breadcrumb in input; Backspace goes back.

### 3.5 Keyboard Behavior
- `ArrowUp/Down`: Navigate (no page scroll)
- `Enter`: Execute highlighted item
- `Esc`: Close palette, return focus to previous element
- `Tab`: Stay in palette (don't jump to page)
- `Backspace` on empty input: Go back (nested) or close
- Focus management: On open → input; on close → return to previous focus (critical for keyboard-only users)

### 3.6 Accessibility
- Single focus trap; input gets initial focus
- ARIA live regions announce result counts and selection
- Colour contrast ≥ WCAG AA; selection not colour-only
- Mouse/touch support (click to select)
- Screen reader: each item has accessible name; groups announced

### 3.7 URLs Used
- Design a command palette (completefrontend.com): https://completefrontend.com/system-design/problems/command-palette
- How to build a remarkable command palette (superhuman.com): https://blog.superhuman.com/how-to-build-a-remarkable-command-palette
- Designing a command palette for web apps (yeshbe.com): https://yeshbe.com/blog/designing-a-command-palette-for-web-apps
- Command palettes in SaaS (designify.design): http://designify.design/blog/command-palette-design
- Command palette UI design (setproduct.com): https://www.setproduct.com/blog/command-palette-ui-design-guide
- How to design a command palette users will reach for (137foundry.com): https://137foundry.com/articles/command-palette-interface-users-actually-reach-for
- Designing a command palette (destiner.io): https://destiner.io/blog/post/designing-a-command-palette
- Command palette UX (mockitt.com): https://mockitt.com/en/guide/command-palette-ux.html
- Good design patterns: command palette (averyv.me): https://averyv.me/blog/command-palette
- 5 SaaS command palette examples (saasframe.io): https://www.saasframe.io/patterns/command-palette

---

## 4. Status Bar Conventions in CAD Applications

### 4.1 Standard Layout (AutoCAD, BricsCAD, nanoCAD, Rhino, DraftSight)
**Position**: Lower-right corner of application window (AutoCAD, BricsCAD) or full-width bottom bar (Rhino, LibreCAD).

**Core toggles (always present)**:
| Control | Function | Visual | Keyboard |
|---------|----------|--------|----------|
| **Grid Display** | Toggle grid visibility | Icon + blue background when on | F7 |
| **Snap Mode** | Toggle snap-to-grid | Icon + blue when on | F9 |
| **Ortho Mode** | Constrain cursor to horizontal/vertical | Icon + blue when on | F8 |
| **Polar Tracking** | Guide cursor at specified angles | Icon + blue when on | F10 |
| **Object Snap (OSnap)** | Snap to entity geometry (endpoint, mid, center...) | Icon + dropdown arrow; blue when on | F3 |
| **Object Snap Tracking** | Track from OSnap points | Icon + blue when on | F11 |

**Coordinate readout**:
- Shows cursor position (absolute, relative, geographic, or off)
- Click to cycle format; right-click for settings menu
- Updates live as cursor moves

**Layer / Style indicators** (BricsCAD, nanoCAD):
- Current layer name (click → layer list; right-click → select layer)
- Current colour, linetype, text style, dimension style
- Click to change; right-click for menu

**Model/Paper Space toggle** (AutoCAD, BricsCAD):
- "Model" / "Layout" / "Tile" — click to switch; right-click for layout list

**Customization menu** (far right, three-line/hamburger icon):
- Check/uncheck any status bar tool to show/hide
- Tools not applicable in current space (e.g., grid in paper space) auto-hide

### 4.2 Visual Language
- **Toggle state**: Blue/accent background = ON; default background = OFF (AutoCAD, BricsCAD, nanoCAD)
- **Dropdown arrow**: Indicates a menu with settings/options (OSnap, Grid, Polar, Layer)
- **Right-click on any tool**: Opens settings dialog for that tool (Grid Settings, OSnap Settings, etc.)
- **Hover + F1**: Context-sensitive help for that tool

### 4.3 Function Key Mapping (Industry Standard)
| Key | Feature | Mutual Exclusion |
|-----|---------|------------------|
| F3 | Object Snap | — |
| F4 | 3D Object Snap | — |
| F7 | Grid Display | — |
| F8 | Ortho Mode | F8 ↔ F10 (Ortho vs Polar) |
| F9 | Grid Snap | — |
| F10 | Polar Tracking | F10 ↔ F8 |
| F11 | Object Snap Tracking | — |
| F12 | Dynamic Input | — |

### 4.4 Dynamic Input / Heads-Up Display
- **AutoCAD Dynamic Input (DYN)**: Tooltip near cursor showing distance/angle fields, accepts Tab navigation between fields, Enter to confirm. Toggled via status bar or F12.
- **BricsCAD Quad Cursor**: Context-sensitive menu at cursor with entity-specific commands.
- **DraftSight**: Similar heads-up input.

### 4.5 URLs Used
- AutoCAD status bar quick reference: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-E3B34B0A-EA98-45E9-937A-BF5FEF4152DB.htm
- AutoCAD status bar help: https://help.autodesk.com/view/ACD/2027/ENU/?guid=GUID-0E4DE630-C2D9-4C0B-889B-16774D8AB1EE
- BricsCAD status bar: https://help.bricsys.com/en-us/document/bricscad/getting-started-with-bricscad/exploring-the-interface/the-status-bar
- BricsCAD status bar (V24): https://help.bricsys.com/en-us/document/bricscad/getting-started-with-bricscad/exploring-the-interface/the-status-bar?version=V24
- nanoCAD snap and grid: https://nanocad.com/learning/online-help/nanocad-platform/snap-and-grid-mode
- Rhino status bar: https://docs.mcneel.com/rhino/8mac/help/en-us/user_interface/rhino_window.htm
- DraftSight status bar blog: https://blog.draftsight.com/2019/11/25/draftsight-status-bar-star-pt-1/
- AutoCAD function key reference: https://help.autodesk.com/view/ACD/2025/ENU?guid=GUID-ACAA0279-047D-458E-889F-60BBFDD40489

---

## 5. Key Conventions for Lintel Implementation

| Area | Convention to Follow | Rationale |
|------|---------------------|-----------|
| **Title block** | Bottom-right; mandatory fields per ISO 7200; scale bar adjacent to viewport, not in title block | Standard compliance; folded-drawing readability |
| **Scale bars** | Graphic bar per viewport; ISO 5455 recommended scales | Survives reproduction; visual estimation |
| **Dimension text** | Above line, parallel, centred; gap ~1 mm; vertical dims readable from right | ISO 129-1 / ASME Y14.5 |
| **Canvas** | Warm paper-white default (`PAPER`); dark mode option persisted | Architectural tradition + accessibility |
| **Semantic colours** | II.30.6 roles + CAD entity colours by layer | Consistency; accessibility; layer-based workflow |
| **Command palette** | Ctrl+K; empty state = recent + contextual; fuzzy subsequence scoring; grouped results; stable ordering; full keyboard + a11y | Industry best practice; power-user efficiency |
| **Status bar** | Lower-right; toggle chips (blue=on); coordinate readout; layer/style indicators; model/level switch; customize menu; F7–F11 standard keys | 30+ years of CAD muscle memory |
| **Snapping** | Visual glyph at cursor; status chips for active snaps; F3/F9/F8/F10/F11 keys; combined + exclusive modes | LibreCAD + AutoCAD conventions |
| **Layers** | Dockable list with inline toggles (vis, lock, print, construction); colour swatch; right-click context menu | LibreCAD + BricsCAD |
| **Properties** | Persistent dockable panel (not modal); live update on selection; multi-entity common attributes | FreeCAD + BricsCAD |

---

*End of document. Human gate: approval required before implementation depends on this.*