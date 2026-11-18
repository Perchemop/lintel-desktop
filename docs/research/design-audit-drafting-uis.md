# Design Audit — 2D Drafting/CAD Application UX (Public Sources)

**Date:** 2026-11-18  
**Public sources — original summaries**  
**Human gate: approval required before implementation depends on this**

---

## 1. LibreCAD (docs.librecad.org)

### 1.1 Main Window Structure
LibreCAD uses a **dock widget system** as the primary UI paradigm. The main window consists of a central drawing canvas surrounded by four dock areas (left, right, top, bottom) plus floating widget support. Users can drag widget title bars to reposition them; widgets can be tabbed together or stacked vertically/horizontally within a dock area. A dedicated "Dock Areas" toolbar toggles visibility of widgets in each of the four dock zones. This is a classic Qt-style MDI layout adapted for CAD — toolbars are separate from dock widgets and provide quick-access icons, while dock widgets provide richer, stateful panels (block list, command line, layer list, library browser, pen wizard).

### 1.2 Drawing Tools — Two-Click Placement & Modifiers
Every drawing tool follows a **two-click (or multi-click) placement pattern**: the user selects a tool, then clicks to define geometric parameters (e.g., "2 Points" line tool: first click = start point, second click = end point). Tools that require additional parameters (length, angle, radius) surface a **Tool Options toolbar** contextually — it appears when the tool is selected and shows relevant input fields. For example, the "Angle" line tool shows length and angle inputs; the "Parallel" tool shows distance and count inputs. The same parameters are also accessible via the **command line** for keyboard-first users. Modifier behaviour is explicit: tools like "Move/Copy" have checkboxes in Tool Options for "Keep original", "Multiple copies", "Alter attributes/layer".

### 1.3 Snapping — Visual Markers, Keyboard Toggles, Status Bar
LibreCAD provides **13 distinct snap modes** plus restriction modes (horizontal/vertical/orthogonal/none). Snap modes can be **exclusive** (only one active) or **combined** (multiple active) — toggled via "Exclusive Snap Mode". Each snap mode has a toolbar icon, a command alias (e.g., `se` for endpoint, `sg` for grid), and appears in the **Snap menu**. Visual feedback: when the cursor approaches a valid snap point, a **marker glyph** appears at the snap location (endpoint marker, midpoint marker, intersection crosshair, centre marker). The command line prompt updates to show the active snap mode. Restriction modes (Restrict Horizontal/Vertical/Orthogonal) constrain cursor movement to axes — visually the cursor moves only along the permitted axis. Keyboard shortcuts: `F7` toggles grid display, `F8` toggles ortho, `F9` toggles grid snap, `F3` toggles object snap (per AutoCAD convention adopted by LibreCAD).

### 1.4 Properties Editing — Selection → Property Panel
LibreCAD uses a **"Properties" tool** (Modify → Properties) that modifies a **single selected entity**. It opens a dialog showing the entity's geometry (type-specific fields: line shows start/end points, circle shows centre/radius) and common attributes (layer, pen colour/width/type). For multi-entity edits, the **"Attributes" tool** modifies common attributes only (layer, pen) across all selected entities — geometry fields are absent. There is no persistent "property panel" dock widget; properties are edited through modal dialogs invoked from the Modify menu or toolbar. Right-click on a layer in the Layer List dock shows a context menu with layer operations (show/hide, lock/unlock, rename, modify attributes).

### 1.5 Layers Panel
The **Layer List Dock** is a dedicated dock widget with three sections: a **filter box** (wildcard-supported text filter), **list operators** (show/hide all, lock/unlock all, add/remove layer), and the **layer list** itself. Each layer row shows: visibility toggle, lock toggle, print toggle, construction layer toggle, and a colour swatch. Clicking the colour swatch opens a colour picker; right-clicking a layer row opens a context menu with the same operations. Layer attributes (name, construction mode, default pen colour/width/type) are edited via a modal dialog opened from the "Modify layer attributes" icon or context menu. Layer **0** is a special protected layer.

### 1.6 Dimensions
LibreCAD offers **9 dimension types**: Aligned, Linear, Horizontal, Vertical, Radial, Diametric, Angular, Leader, and Ordinate (implied). Each is a two-click placement: select start/end points on the entity, then a third click places the dimension text. The **Tool Options toolbar** shows dimension-specific settings: text height, precision, arrow style, offset distance. Dimensions are associative — if the measured entity moves, the dimension updates.

### 1.7 Keyboard-First Workflows
The **command line dock** is central to keyboard-first use. It can be activated 7 ways (type command + Enter/Space, Ctrl+M, Shift, F1, Shift+Space). **Keycode Mode** enables two-letter commands without pressing Enter (e.g., `li` for line). Commands support **relative coordinates** (`@x,y` or `x..y`), **multi-command input** (semicolon-separated: `li;0,0;10..0;0..10;-10..0;c;k`), **variables** (assign command sequences to names, recurse with `\name`), and a **built-in calculator** (`cal` toggles calculator mode). Tab completion works on command names. Coordinates can be arithmetic expressions.

### 1.8 Empty-State / First-Run Patterns
The documentation shows an empty drawing window with dock widgets visible. The "Getting Started" guide walks through: install → configure → drawing setup (units, grid, layers) → first drawing. No explicit onboarding tour or sample file is documented in the public manual; the empty canvas with active dock widgets serves as the initial state.

### 1.9 URLs Used
- Main manual: https://docs.librecad.org/en/latest/
- Dock widgets: https://docs.librecad.org/en/latest/ref/widgets.html
- Drawing tools: https://docs.librecad.org/en/latest/ref/tools.html
- Snapping: https://docs.librecad.org/en/latest/ref/snaps.html
- Command line: https://docs.librecad.org/en/latest/guides/cmdline.html

---

## 2. QCAD (qcad.org)

### 2.1 Main Window Structure
QCAD follows a **traditional CAD menu-bar + toolbar + command line** layout. The manual organizes tools under menus: File, Edit, View, Select, Draw, Dimension, Modify, Snap, Info, Layer, Block, Window, Misc, CAM. Toolbars provide icon access to the same commands. A **command line** at the bottom accepts typed commands with aliases. The manual does not describe dockable panels in detail; the UI appears more toolbar/menu-centric than LibreCAD's dock widget approach.

### 2.2 Drawing Tools
Tools are organized by category in the Draw menu (Line, Arc, Circle, Ellipse, Spline, Polyline, Text, Hatch, Dimension, Image, Points). Each tool has a command alias (short form). The manual emphasizes keyboard shortcuts and command-line equivalents for every menu item.

### 2.3 Snapping
QCAD provides snap modes accessible via the Snap menu and command line. The keyboard shortcut reference (PDF) shows extensive function-key mappings: F3=Object Snap, F7=Grid Display, F8=Ortho, F9=Grid Snap, F10=Polar Tracking, F11=Object Snap Tracking. This mirrors the AutoCAD function-key convention.

### 2.4 Properties & Layers
The Layer menu provides layer management (add, delete, edit, lock, freeze). The Property Editor (implied by Modify → Properties) edits entity properties. The manual's structure suggests a more menu-driven property editing flow than a persistent panel.

### 2.5 Keyboard-First Workflows
QCAD publishes **extensive keyboard shortcut PDFs** (3 pages, 30+ languages). Every menu command has a shortcut. The command line accepts typed commands with aliases. Function keys map to major drawing aids (ortho, grid, snap, polar, object snap). This is a highly keyboard-centric design.

### 2.6 URLs Used
- Manual: https://qcad.org/en/manual/
- Keyboard shortcuts: https://qcad.org/en/shortcuts/
- User manual navigation (menus): https://qcad.org/en/manual/

---

## 3. FreeCAD Sketcher / Draft Workbench (wiki.freecad.org)
*Note: FreeCAD wiki was inaccessible (Anubis challenge). Public knowledge summarised from prior exposure and community documentation patterns.*

### 3.1 Main Window Structure
FreeCAD uses a **workbench paradigm**: the UI completely changes when switching workbenches (Sketcher, Draft, Arch, PartDesign, etc.). Each workbench provides its own toolbar set, panel set, and menu entries. The **Sketcher workbench** provides a dedicated solver-driven sketching environment with a **Tasks panel** (left) showing constraints, solver messages, and a **Combo View** (right) with model tree and property editor. The **Draft workbench** provides traditional 2D drafting tools with a **Draft toolbar** and **Draft snap toolbar**, plus a **Tasks panel** for tool options.

### 3.2 Drawing Tools
Sketcher uses **constraint-based geometry**: user draws approximate geometry, then applies constraints (coincident, horizontal, vertical, tangent, equal, symmetric, etc.) which the solver resolves. Tools are in the Sketcher toolbar: Line, Circle, Arc, Rectangle, Polygon, Slot, etc. Draft workbench uses traditional click-click placement with snap toolbar.

### 3.3 Snapping
Draft workbench has a dedicated **Snap toolbar** with toggle buttons for: Endpoint, Midpoint, Center, Intersection, Extension, Perpendicular, Tangent, Grid, Nearest, Ortho. Snaps can be combined. Visual feedback: snap points highlight when cursor is near.

### 3.4 Properties Editing
The **Property Editor** (in Combo View) shows all properties of the selected object — both geometric (constraint-driven in Sketcher) and visual (view properties: line colour, width, pattern). It is a persistent dockable panel, updating live with selection.

### 3.5 Layers / Groups
FreeCAD uses **Groups** (in model tree) and **Layers** (via Draft Layer Manager or Arch workbench) for organisation. The model tree is the primary navigation/selection structure.

### 3.6 Dimensions
Sketcher has constraint-driven dimensions (distance, angle, radius) that drive geometry. Draft workbench has traditional dimension tools (Linear, Angular, Radius, Diameter) placed with click-click-click.

### 3.7 Keyboard-First Workflows
FreeCAD supports extensive keyboard shortcuts (customisable via Edit → Preferences → Shortcuts). The command line is not central; shortcuts and context menus are primary.

### 3.8 Empty-State / First-Run
FreeCAD starts with the **Start workbench** showing recent files, macros, and a "Create new" button. New documents open in the last-used workbench or Start.

### 3.9 URLs Used
- Draft Workbench: https://wiki.freecad.org/Draft_Workbench (inaccessible at time of research)
- Sketcher Workbench: https://wiki.freecad.org/Sketcher_Workbench (inaccessible at time of research)

---

## 4. Behavioural Patterns Worth Adopting

| Pattern | Why It Works | Source |
|---------|--------------|--------|
| **Dockable, tabbable panels** (LibreCAD) | Users can arrange tools to match their workflow; panels persist across sessions; no floating dialogs lost over canvas | https://docs.librecad.org/en/latest/ref/widgets.html |
| **Contextual Tool Options toolbar** (LibreCAD) | Shows only parameters relevant to the active tool; reduces clutter; same inputs available on command line | https://docs.librecad.org/en/latest/ref/tools.html |
| **Exclusive vs. combined snap modes** (LibreCAD) | Novices use one snap at a time; experts combine; both supported via single toggle | https://docs.librecad.org/en/latest/ref/snaps.html |
| **Visual snap markers at cursor** (LibreCAD, DraftSight, AutoCAD) | Immediate feedback that snap is active and which point will be caught; no guesswork | https://docs.librecad.org/en/latest/ref/snaps.html |
| **Command line with aliases + relative coords + calculator** (LibreCAD) | Power users never leave keyboard; arithmetic in coordinates; variables for macros | https://docs.librecad.org/en/latest/guides/cmdline.html |
| **Function-key mapping for major aids** (QCAD, AutoCAD, BricsCAD) | F7/F8/F9/F10/F11 are muscle memory for CAD users; standard across industry | https://qcad.org/en/shortcuts/; Autodesk help |
| **Right-click context menus on layers/entities** (LibreCAD, BricsCAD) | Fast access to common operations without menu diving; discoverable | https://docs.librecad.org/en/latest/ref/widgets.html |
| **Layer list with inline toggles** (LibreCAD, BricsCAD) | Visibility, lock, print, construction mode all visible and clickable per layer | https://docs.librecad.org/en/latest/ref/widgets.html |
| **Status bar with toggle chips + right-click settings** (AutoCAD, BricsCAD, nanoCAD) | One-click toggle of grid/ortho/snap/polar; right-click opens settings dialog; customizable visibility | https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-E3B34B0A-EA98-45E9-937A-BF5FEF4152DB.htm |
| **Coordinate readout on status bar** (AutoCAD, BricsCAD, Rhino) | Live cursor position (absolute/relative/geographic); click to cycle format; essential for precision | https://help.bricsys.com/en-us/document/bricscad/getting-started-with-bricscad/exploring-the-interface/the-status-bar |
| **Workbench/task-panel paradigm** (FreeCAD) | Mode-specific UI reduces cognitive load; solver-driven sketching separates intent from construction | wiki.freecad.org (general knowledge) |
| **Persistent property editor panel** (FreeCAD, BricsCAD) | Live update on selection; edits apply immediately; no modal dialogs for routine changes | wiki.freecad.org; BricsCAD help |
| **Associative dimensions** (LibreCAD, QCAD, FreeCAD) | Dimensions update when geometry changes; single source of truth | https://docs.librecad.org/en/latest/ref/tools.html |
| **Multi-command input (semicolon-separated)** (LibreCAD) | Scripting without a script; rapid repetitive operations | https://docs.librecad.org/en/latest/guides/cmdline.html |

---

*End of document. Human gate: approval required before implementation depends on this.*