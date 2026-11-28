# QCAD & LibreCAD — Deep Research for Lintel (Clean-Room Behavioural Study)

**Date:** 2026-11-28 · **Method:** public documentation only (QCAD.org manual/tutorials/features/shortcuts PDF; docs.librecad.org manuals; LibreCAD wiki FAQ; project READMEs). All wording below is an original behavioural summary; every claim carries a source URL. No icons, no verbatim text, no proprietary internals. QCAD and LibreCAD are both GPL/open-source with public documentation; both are Qt-based 2D DXF drafters — the closest open-source peers to Lintel's class.

---

## 1. Behaviours per app (grouped, with source URLs)

### 1.1 QCAD — main window layout

QCAD's application window is organised around a central drawing area with four stable satellite zones. On the **left** sits the "CAD toolbar", a single stacked icon column that is *context-adaptive*: it always shows the tool families that make sense right now — when a line tool is waiting for its start point, the same toolbar switches to showing snap-mode buttons instead of drawing buttons. Menus remain the full reference; the toolbar is the working surface. On the **right** sit the layer list and block list docks (toggle via View → Layer List / View → Block List). Just above the status bar is the **command widget** (command line + history); at the bottom, the **status bar** carries three dedicated widgets: (a) a *coordinate widget* showing the cursor's absolute and relative position in both cartesian and polar form, (b) a *mouse widget* explaining what the left and right mouse buttons currently do, and (c) a *selection widget* counting selected entities. A fifth zone, the **options toolbar** at the top, appears whenever an active tool has parameters. QCAD also has a property editor window, a selection filter widget, a view list, and (Pro) layout tabs under the drawing area for switching between model space and print layouts.

- Source: https://qcad.org/doc/qcad/2.2/reference/en/chapter06.html (User Interface chapter)
- Source: https://qcad.org/en/manual/scripts/widgets/commandline/doc/commandline
- Source: https://qcad.org/ebook/qcad_book_preview_en_2026.pdf (application window, CAD toolbar behaviour)

**Command mode vs normal mode.** QCAD distinguishes two keyboard states. In *normal mode* keys trigger shortcuts (pressing Z then A triggers auto-zoom). In *command mode* all printable keys go into the command line. Space (or clicking the field) enters command mode; Escape returns to normal mode — and if the field already holds text, Escape must be pressed twice. This is a deliberate solution to "shortcuts vs typing" coexistence on the same canvas focus.

- Source: https://qcad.org/doc/qcad/2.2/reference/en/chapter06.html (§6.5/6.6)

**Options toolbar.** Each tool projects its own parameter widgets into the options toolbar (e.g. line angle, spline degree, polyline "Arc" checkbox). Typing a digit while a tool runs routes the number into the first options field; Ctrl+Space (Meta+. in Pro) moves focus to the options toolbar; Space moves focus to the command line. This pairing — *command line for coordinates, options toolbar for tool parameters* — is QCAD's core input model.

- Source: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf (§3.22–3.23 Focus on Command Line / Options Toolbar)
- Source: https://forum.qcad.org/t/feature-suggestion-dynamic-measurement-input-for-lines/9676 (typing routes to first input field; `.L` style toggles)

### 1.2 QCAD — layer list

The layer list is a tree widget. Each row shows: an **eye toggle** (click to show/hide the layer; hidden layers still save and print-disabled layers are separate), a **padlock toggle** (locked layers refuse edits), the **layer name**, and (in Pro, configurable under Application Preferences → Layer List) additional columns for **frozen**, **snappable**, and **plottable/print** states. The *current* layer is highlighted and marked with a small pen icon. Interactions: single click activates a layer (subsequent drawing lands there); double-click opens the layer dialog to rename it or change colour/lineweight/linetype; right-click opens a context menu (add, remove, toggle attributes, show-only-active, purge unused, etc.). Pro adds **hierarchical layers** (sub-layers with a parent, displayed as a collapsible tree), multi-select for delete/edit, "move selection to current layer", and **layer states** — named snapshots of the full on/off/lock/frozen configuration that can be recalled (e.g. one state for printing, another for exporting). Above the list is a toolbar of whole-list operations. Layers never control draw order — that is done by explicit draw-order tools.

- Source: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf (§3.12 Layer List — visibility/lock columns by default, frozen/snappable/plottable optional)
- Source: https://qcad.org/en/tutorial-working-with-layers (click = activate, double-click = edit, eye toggle, layer 0 never deletable)
- Source: https://qcad.org/en/manual/scripts/pro/layer/addlayerpro/doc/addlayerpro (snappable/plottable per layer, sub-layers → hierarchical tree)
- Source: https://qcad.org/en/tutorials/ (1.3 Layers: pen icon for active layer)
- Source: https://qcad.org/en/qcad-features/ (Layer Tools block: hierarchical, multi-select, layer states, purge empty)

### 1.3 QCAD — block list

The block list shows every *block definition* of the document plus two special entries: **Model Space** (always first; the main 1:1 drawing is itself stored as a block) and **layout blocks** (paper spaces, Pro). Interactions: an eye-style column toggles per-block visibility; double-clicking a block (or clicking the column between eye and name) *opens the block for editing* — the canvas switches to that block's contents, and "Return to Main Drawing" (shortcut EM) comes back. Blocks are created from a selection (selection is replaced by a reference; you pick the reference point then a name), inserted by selecting the block then clicking a position (insert offers rotation/scale via options toolbar), renamed, duplicated, removed, and purged when unused. **Edit Block In-place** lets you edit a reference's contents directly in the drawing context; **Edit Block from Reference** jumps into the definition. Entities inside a block live on their own layers; the reference also sits on a layer — effective attributes resolve through By Layer / By Block chains (see edge cases). Nested blocks are supported. The block list also surfaces external references (XRefs) in newer Pro releases, with reload/bind actions.

- Source: https://qcad.org/en/tutorial-1-11-blocks (block list position, double-click/second-column opens block, model space always on top, create-from-selection replaces selection with a reference, reference point + name)
- Source: https://qcad.org/doc/qcad/2.2/reference/en/chapter13.html (double-click toggles visibility; context menu with Toggle Visibility)
- Source: https://qcad.org/en/qcad-features/ (Block Tools: add/edit/copy/rename/remove, nested, hide individual, purge, attributes, XRefs)
- Source: https://qcad.org/en/changelog (Edit Block In-place / from Reference entries; "Disable global scale input in options tool bar when editing layout blocks")

### 1.4 QCAD — command line widget

The command widget combines an input field with a scrollable **command history** log above it (commands, coordinate entries, warnings and errors all land there; errors print in red). Input grammar: absolute cartesian `x,y`, relative cartesian `@x,y`, absolute polar `distance<angle`, relative polar `@distance<angle`. Mathematical expressions are allowed inside coordinates (`5+5,30/6`). A **calculator mode** is built in: an expression prefixed with `=` evaluates and prints (e.g. `=3+4` → 7), and variables can be stored (`=a=5+6`, then `=a/2`). Available functions mirror ECMAScript's Math class written bare (`sin`, `cos`, `sqrt`, `hypot`, `deg2rad`, …) with trigonometry in degrees. Coordinate/number formats (decimal separator, polar separator) are configurable in preferences. Paste of a multi-line coordinate list into the field replays it as if typed, which turns a spreadsheet column into geometry. "Direct distance entry" — typing just a length while a direction is implied — is also supported.

- Source: https://qcad.org/en/manual/scripts/widgets/commandline/doc/commandline (coordinate formats, expressions, `=3+4`, variables, function list)
- Source: https://qcad.org/en/tutorials/drawing/using_a_list_of_coordinates/ (paste coordinate list replay)
- Source: https://forum.qcad.org/t/option-to-set-relative-coordinates-as-the-default/6864 (relative-zero semantics, F8 distance/angle readout, direct distance entry)
- Source: https://forum.qcad.org/t/old-school-command-scripting/9756 (history log; scripts not natively replayed from .scr — use XC/run script tool)

### 1.5 QCAD — status bar

Left: coordinate widget (absolute + relative, cartesian + polar). Middle: mouse widget (what the two buttons do right now). Right: selection count. Toggleable as a whole via View → Status Bar.

- Source: https://qcad.org/doc/qcad/2.2/reference/en/chapter06.html (§6.4)

### 1.6 QCAD — drawing preferences & printing

*Drawing preferences* (per document, distinct from application preferences) set the drawing unit, grid, dimension style defaults, and the print page: paper size, scale, multi-page glue margins, crop marks and page tags. *Print preview* is a distinct mode (Ctrl+Shift+P) that shows the drawing on a white virtual sheet; its options toolbar carries a **scale combo** (e.g. 1:100, 2:1), a **fit-to-paper** button, an **auto-add-pages** button for poster printing, a **move-paper-position** drag tool, and **centre-on-page**. The preview does not modify geometry — it only fixes the paper mapping used by Print and PDF Export. Multi-page ("poster") printing tiles the sheet across N×M pages with configurable overlap ("glue margins") and optional cross-hair crop marks and page tags like "B3". **Layout blocks + viewports** (Pro) are the sheet-based path: a layout block is a paper-sized block whose contents are drawn in paper coordinates; *viewports* are rectangles referencing the model space at a chosen scale/angle, with draggable corner/centre/pan handles; the viewport's frame lives on a layer that is typically set "unplottable" so the frame never prints. PDF export reuses the print-preview state; PDF/A-1b is supported.

- Source: https://qcad.org/tutorial-printing/ (drawing preferences, print preview steps, scale, glue margins, PDF export, thin-line troubleshooting)
- Source: https://qcad.org/en/blog/121-qcad-3-release-multipage-printing (poster printing, auto-add-pages, crop marks, page tags like B3)
- Source: https://qcad.org/en/tutorial-2-4-layouts-and-viewports (layout blocks as paper, viewport creation flow, unplottable viewport layer, frame visibility vs content)
- Source: http://www.qcad.org/en/manual/scripts/file/printpreview/doc/printpreview (preview is display-only; scale combo; fit; move paper)
- Source: https://qcad.org/en/qcad-features/ (Printing block: model-space to scale, print preview, per-block printer settings, multiple pages, crop marks, page tags, printer calibration, layout tabs)

### 1.7 QCAD — DXF/DWG round-trip and scripting

QCAD Community reads and writes DXF (R12 era through modern) and, in the paid editions, reads and writes DWG across a very wide version range (R2.5 → R32). Import maps foreign entities onto native ones (beziers → splines, fills → solid hatches, masks → wipeouts) and carries over layers, colours, linetypes, lineweights, drawing unit and dimension format. Anonymous (system-generated) blocks coming from dimensions/hatches are handled with special care in DXF R12 writing (a changelog fix exists for name mixups). ECMAScript (JavaScript) is the scripting interface: scripts can create/modify entities, add whole interactive tools, run a script shell and debugger, and reach almost the entire Qt API. Separately, headless command-line tools ship for batch conversion and extraction (`dwg2pdf`, `dwg2svg`, `bbox`, `dwghatch`, …).

- Source: https://qcad.org/en/qcad-features/ (File Formats: version list; Imported entities; ECMAScript Interface block; Command Line Tools)
- Source: https://qcad.org/en/command-line-tools/ (batch tools)
- Source: https://qcad.org/en/changelog (FS#1216 anonymous-block name mixup on DXF/DWG write; R12 text/block compatibility improvements)

### 1.8 LibreCAD — main window layout

LibreCAD's window is assembled from **dock widgets** and **toolbars** that can float, dock on any side, stack into tabs, or live on a second monitor. Standard docks: **Layer List**, **Block List**, **Command Line**, **Library Browser** (insert reusable DXF-based parts), **Pen Wizard**, plus optional per-family tool docks (Line, Circle, Curve, Dimension, Ellipse, Info, Modify, Polyline) that act as floating toolboxes. The **Tool Options toolbar** is the context-sensitive strip that every parameterised tool populates — and LibreCAD's documentation is unusually blunt that it *must always be enabled* because two whole features (Print Preview adjustments and Block insert options) exist **only** there, with no menu or command-line equivalent. The status bar shows the current layer and the minor/major grid spacing (plus, per the user guide, selection totals). Menus can be "torn off" and float. Everything is re-arrangeable and persisted.

- Source: https://docs.librecad.org/en/stable/ref/widgets.html (dock widget list, dock areas, tabbing)
- Source: https://docs.librecad.org/en/latest/ref/toolbars.html (Toolbars; Tool Options section; "Print Preview and Block operations require the use of the Tool Options toolbar as there are no menu or command line equivalencies")
- Source: https://docs.librecad.org/en/stable/getstart/configure.html (status bar contents; layout configurability; torn-off menus)
- Source: https://tutorials.mediaket.net/software-tutorials/librecad-tutorial.html (widgets toggling; dock rearrangement) — third-party but consistent with the official manual.

### 1.9 LibreCAD — layer list

Three zones inside one dock: a **filter box** at the top (substring or wildcard `*`/`?` matching to find layers fast, e.g. `*01`), a row of **list-wide buttons** (add layer, add quickly, remove, and "show all / hide all" — labelled Defreeze/Freeze), and the list itself. Each layer row carries per-layer icon buttons: **visible/hidden** (freeze/defreeze) and **locked/unlocked**, plus the layer name; a double-click/edit action opens attributes (name, pen colour, width, linetype). The active layer is highlighted; layer "0" exists in every drawing and cannot be renamed (it is the "no layer" carrier that block contents are supposed to sit on). Right-clicking a layer opens a context menu. There is no hierarchical nesting — LibreCAD layers are flat.

- Source: https://docs.librecad.org/en/latest/ref/widgets.html?layer-list-dock (filter with wildcards, list operators, per-layer visible/lock icons)
- Source: https://docs.librecad.org/en/stable/ref/widgets.html (stable-version equivalent)
- Source: https://www.asor.org/wp-content/uploads/2022/05/ASOR_Tutorial_03-05_LibreCAD-Layers.pdf (layer 0 cannot be renamed; +/− buttons; edit attributes; eye toggles) — third-party tutorial, consistent with official docs.
- Source: https://docs.librecad.org/en/latest/guides/blocks.html (layer "0" = "no layer" for blocks)

### 1.10 LibreCAD — block list

Block list dock buttons: **show all blocks / hide all blocks** (freeze/defreeze), **create block from selected entities** (they must be on layer "0"; the selection is replaced by one reference), **add an empty block** (which then opens for editing), **remove the active block**, plus actions reachable from the selected row: **edit the active block in a separate window** (close the window → definition saved → every instance updates), **save the active block to a file** for reuse, and insert. Inserting offers two paths — from the list (tool options: rotation **angle**, scale **factor**, and **array** columns/rows with column/row **spacing**, so one insert can stamp a full grid) and from the **Library Browser** with its own options bar. An array-inserted group behaves as one selectable instance. Blocks are the mechanism behind the library: save block → library folder → insert anywhere.

- Source: https://docs.librecad.org/en/latest/guides/blocks.html (create from selection on layer 0; edit in separate window; save to file; array behaves as one instance)
- Source: https://docs.librecad.org/en/latest/ref/toolbars.html (Block insert tool options: Angle, Factor, Array, Spacing)
- Source: https://docs.librecad.org/en/latest/ref/widgets.html (block list dock buttons: show/hide all, create, add empty, remove)

### 1.11 LibreCAD — command line dock

Three components: a **prompt line**, a **command output window** (history/log), and a drop-down menu offering: **Keycode Mode** toggle (a one/two-letter command-entry mode where keystrokes land in the prompt instead of acting as shortcuts), **load a command file**, and **paste commands**. Typed commands accept prefixes (`l`, `li`, `line` all start the 2-point line tool). Many tools echo both a textual prompt ("Specify Distance <10> or select entity or [through]") and their Tool Options widgets simultaneously. The command line doubles as a **calculator**: the `cal` command evaluates expressions, and coordinate fields accept inline arithmetic (`100<45+15`). Relative coordinates use `@x,y` and polar `d<a`; a numeric-keypad-friendly variant types two decimal points in place of the comma (`10..20` ≡ `@10,20`). User-defined aliases live in a `librecad.alias` file (line format: alias, tab, full command).

- Source: https://docs.librecad.org/en/latest/guides/cmdline.html (three components; drop-down functions; prompt + tool-options duality; cal command)
- Source: https://docs.librecad.org/en/latest/ref/fundamentals.html (coordinate entry; `..` keypad trick; arithmetic in coordinates)
- Source: https://docs.librecad.org/en/latest/ref/widgets.html (dock drop-down: keycode mode, command file, paste)
- Source: https://tutorialtactic.com/blog/librecad-shortcuts (librecad.alias customisation; verified alias examples) — third-party, consistent with official alias-file behaviour.

### 1.12 LibreCAD — drawing preferences & printing

*Current Drawing Preferences* (per document) has five tabs: **Paper** (format from ISO/ANSI/Arch/custom, orientation, four margins, and the **number of horizontal/vertical pages** for tiling), **Units**, **Grid**, **Dimensions** (including a **General Scale** that scales dimension text/arrows to suit the print scale), and **Splines**. Drawings are authored 1:1; output scale is chosen at print time. **Print preview** shows a white virtual page (grey bands for margins) and its Tool Options bar carries: scale drop-down *and* editable text box, a **"fixed" checkbox** that locks the chosen scale against later zoom changes, **fit to page**, **centre to page**, and black/white & greyscale toggles. **Tiled printing** repeats the page in an H×V grid to compose one large sheet; margins reduce the assembled size. Known quirks: the scale box requires pressing Enter to apply; if "fixed" is unticked the preview follows the main-view zoom. A console converter (`librecad dxf2pdf …`) mirrors the GUI with flags for scale, paper, margins, pages, greyscale/mono and fit/centre.

- Source: https://docs.librecad.org/en/stable/ref/draw-prefs.html (five tabs; paper tab fields; number of pages)
- Source: https://docs.librecad.org/en/stable/guides/completion.html (print preview description; tiled printing; fixed checkbox; scale entry)
- Source: https://docs.librecad.org/en/latest/guides/dwg-setup.html (draw 1:1; General Scale couples print scale to dimension text)
- Source: https://docs.librecad.org/en/latest/guides/console-tool.html (dxf2pdf options incl. `-s 0.01` for 1:100)
- Source: https://forum.librecad.org/Print-Scale-td5707755.html and https://forum.librecad.org/printing-to-1-87-scale-td5720326.html (must re-tick "fixed" before leaving preview; arbitrary scales need typing + Enter)

### 1.13 LibreCAD — DXF/DWG round-trip and scripting

Native format is DXF; DWG support is described by the project as "basic" and depends on the bundled libdxfrw library (a build-time option). Supported DXF read versions: R12, R14, 2000, 2004, 2007 (newer releases extend this). Export writes DXF, DWG (when built with DWG support), PDF and SVG; console converters exist (`dxf2pdf`, `dxf2png`, `dxf2svg`, `dxf2dwg`, `dwg2dxf`, …) with a `-V` version switch (r12 … r2018). Practical guidance in the community: prefer mid-range DXF versions for imports, since very old versions force converters to explode hatches into line soup and approximate splines as polylines. Layer names, colours and linetypes generally survive; block definitions and inserts survive as references. Scripting differs from QCAD: LibreCAD is plugin-based (C++ plugins, contributed externally) rather than offering an embedded ECMAScript API surface.

- Source: https://dokuwiki.librecad.org/doku.php/usage:faq (DXF versions R12/14/2000/2004/2007; "support of DWG format by LibreCAD is basic"; blank-PDF FAQ)
- Source: https://github.com/librecad/librecad (console converters table, `-V` versions, libdxfrw, shp import)
- Source: https://cadblockdwg.com/blog/librecad-reads-dxf-only-converting-dwg-blocks (round-trip consequences: hatches exploded to segments on old versions, splines approximated, layers/blocks survive) — third-party, consistent with the FAQ.
- Source: https://forum.librecad.org/DWG-support-enhancement-td5729577.html (DWG support status, libdxfrw origin)

---

## 2. Tool inventory tables

Legend: **Q** = QCAD, **L** = LibreCAD. ✓ = present in that app's public feature list/manual; (P) = QCAD Professional add-on; ✗ = absent. Where both have a tool but with different construction methods, the difference is noted.

### 2.1 Draw tools

| Tool | Q | L | Notes (behavioural summary) |
|---|---|---|---|
| Line from 2 points | ✓ | ✓ | Both: click/type start, click/type end; segment continuation chains from relative zero. URL: https://qcad.org/en/qcad-features/ ; https://docs.librecad.org/en/latest/ref/tools.html |
| Line at absolute angle | ✓ | ✓ | Start point + angle (+ length in L). URLs: same |
| Horizontal / Vertical line | ✓ | ✓ | Convenience wrappers of angle tool. URLs: same |
| Angle bisector | ✓ | ✓ | Two lines + length. URLs: same |
| Parallel / offset line (distance) | ✓ | ✓ | Q: LP; L: "Parallel" with distance & copies in tool options. URLs: same |
| Parallel through point | ✓ | ✓ | Q: LG. URLs: same |
| Tangent (point→circle) | ✓ | ✓ | URLs: same |
| Tangent (two circles) | ✓ | ✓ | URLs: same |
| Orthogonal / tangent to line+arc | ✓ | ✓ | Q: LN "Orthogonal / Tangent". URLs: same |
| Relative-angle line | ✓ | ✓ | Line centred on a point at an angle relative to an existing line. URLs: same |
| Freehand line | ✓ | ✓ | Sketch mode. URLs: same |
| Construction lines (infinite) & rays | ✓ | ✓ | Q: XLines/Rays + horizontal/vertical construction lines (XH/XV). URLs: same |
| Polyline (line+arc segments) | ✓ | ✓ | Both: tick "Arc" per segment; tangential continuation options; close button. URLs: https://qcad.org/en/manual/scripts/pro/draw/polyline/drawpolylinepro/doc/drawpolylinepro ; https://docs.librecad.org/en/latest/ref/tools.html |
| Polyline from existing segments | ✓ | ✓ | Join separate lines/arcs into one polyline (Q "Polyline from Segments/Selection"; L "Create Polyline from Existing Segments"). URLs: same |
| Polyline node editing | ✓ | ✓ | Insert/append/delete node(s), delete between nodes, trim segments, relocate start, change segment type (Q richer set). URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (OA/OP/OD/OB/OT/OR/OX) ; https://docs.librecad.org/en/latest/ref/tools.html (pi/pn/pd/pr) |
| Polyline equidistant copies | ✓ | ✓ | N parallel polylines at given distance. URLs: same as above |
| Rectangle (2 corner) | ✓ | ✓ | URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (RE) ; https://docs.librecad.org/en/latest/ref/tools.html |
| Rectangle with size / 3-point | ✓ | ✓ | Q: RS, R3; L: Rectangle 3 points. URLs: same |
| Polygon (centre–corner / centre–side / two-side-points) | ✓ | ✓ | Both with side-count option. URLs: same |
| Arc: centre, point, angles | ✓ | ✓ | URLs: https://qcad.org/en/qcad-features/ (Arcs) ; https://docs.librecad.org/en/latest/ref/tools.html (Curve) |
| Arc: 3 points | ✓ | ✓ | URLs: same |
| Arc: 2 points + radius / angle / length / height | ✓ | ✓ | Q has all four; L lists 3-point + tangential. URLs: same |
| Arc: concentric | ✓ | ✓ | Offset-style concentric copy. URLs: same |
| Arc: tangentially connected | ✓ | ✓ | Continuation arc from a line end. URLs: same |
| Circle: centre + point | ✓ | ✓ | URLs: https://qcad.org/en/qcad-features/ (Circles) ; https://docs.librecad.org/en/latest/ref/tools.html (Circle) |
| Circle: centre + radius / diameter | ✓ | ✓ | URLs: same |
| Circle: 2 opposite points | ✓ | ✓ | URLs: same |
| Circle: 2 points + radius | ✓ | ✓ | URLs: same |
| Circle: 3 points | ✓ | ✓ | URLs: same |
| Circle: concentric | ✓ | ✓ | URLs: same |
| Circle tangential combinations | ✓ | ✓ | Q: tangent+2pts, tangent+point+radius, 2 tangents+point, 2 tangents+radius, 3 tangents; L: subset (2 circles + point; 3 tangential; concentric; inscribed). URLs: same |
| Ellipse: axis (centre + major + minor) | ✓ | ✓ | URLs: https://qcad.org/en/qcad-features/ (Ellipses) ; https://docs.librecad.org/en/latest/ref/tools.html (Ellipse) |
| Ellipse: foci / 4-point / centre+3pt | ✗ (has centre+2pts+angles) | ✓ | L-specific constructions. URLs: same |
| Ellipse arc | ✓ | ✓ | URLs: same |
| Ellipse inscribed in quadrilateral | ✓ | ✓ | Q: EQ; L: ei. URLs: same |
| Ellipse parallel curve (offset) | ✓ | ✗ | Q: EC/EG. URLs: same |
| Spline from control points | ✓ | ✓ | Q: degree selectable 1–5 open/closed; L: degree 1–3. URLs: https://qcad.org/en/qcad-features/ (Splines) ; https://docs.librecad.org/en/latest/ref/tools.html (Spline) |
| Spline from fit points | ✓ (P) | ✓ | Q: SL (cubic); L: "Spline through points". URLs: same |
| Spline fit-point insert/remove/simplify | ✓ (P) | partial (node tools on polylines) | URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (NI/NA/NR/NP) |
| Point (single) | ✓ | ✓ | URLs: same |
| Points on a line / M×N array of points | ✓ (P) | partial (point series tools) | URLs: https://qcad.org/en/qcad-features/ (Points); L: "Line of Points" per tutorial |
| Hatch from selection (trimmed boundary) | ✓ | ✓ | Both fill a closed boundary; Q adds "Auto hatch" (click inside area) and hatch-from-segments (untrimmed). URLs: https://qcad.org/en/qcad-features/ (Hatches & Fills) ; https://docs.librecad.org/en/latest/ref/tools.html (Hatch) |
| Text (single-line) / MText | ✓ | ✓ | Fonts, height, angle, alignment, special symbols. URLs: https://qcad.org/en/qcad-features/ (Texts) ; https://docs.librecad.org/en/latest/ref/tools.html (Others: MText/Text) |
| Text along entity / leaders as text | ✓ (P) | ✗ | Q Pro extras. URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (TG) |
| Bitmap/image insert | ✓ | ✓ (import) | Q: scalable/rotatable/transparency image underlay; L: imports images. URLs: https://qcad.org/en/qcad-features/ (Raster Images) |
| Wipeout (masking polygon) | ✓ (P) | ✗ | URLs: https://qcad.org/en/qcad-features/ (Wipeouts) |

### 2.2 Dimension tools

| Tool | Q | L | Notes |
|---|---|---|---|
| Aligned | ✓ | ✓ | Two extension points then dimension-line position; preview follows cursor. URL: https://qcad.org/en/manual/…/dimaligned (see https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §6.1) ; https://docs.librecad.org/en/latest/ref/tools.html (Dimension) |
| Linear (rotated at arbitrary angle) | ✓ | ✓ | Angle set in options toolbar; H/V are presets. URLs: same |
| Horizontal / Vertical | ✓ | ✓ | Same flow minus the angle. URLs: same |
| Radial (radius) | ✓ | ✓ | Pick circle/arc then dimension position; "R" prefix. URLs: same |
| Diametric | ✓ | ✓ | "Ø" prefix. URLs: same |
| Angular (two lines) | ✓ | ✓ | Pick both lines then arc position. URLs: same |
| Ordinate (X/Y from origin) | ✓ | ✗ | Q: DO with movable origin. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §6.7 |
| Leader (arrow + text) | ✓ | ✓ | Q: DE/LD; L: ld. URLs: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §6.8 ; https://docs.librecad.org/en/latest/ref/tools.html (Leader) |
| Arc length | ✓ (P) | ✗ | Q: DG with arc-symbol placement. URL: https://qcad.org/en/manual/scripts/pro/draw/dimension/dimarclength/doc/dimarclength |
| Baseline / Continue chains | ✓ (P) | partial (dimregen only) | Q: DB/DC. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Datum / Tolerance (GD&T) | ✓ (P) | ✗ | URL: https://qcad.org/shortcuts/shortcuts_en.pdf (DT, TO) |
| Label override with `<>` placeholder | ✓ | ✓ | Empty label = auto measurement; `<>` embeds the measured value inside custom text. URLs: https://qcad.org/doc/qcad/2.2/reference/en/chapter24.html ; https://docs.librecad.org/en/latest/ref/tools.html (Dimension options) |
| Move/reset label position | ✓ | ✓ | Drag the label's grab dot (Q FAQ); L: drag text. URLs: https://qcad.org/en/faq/ ; https://docs.librecad.org/en/latest/ref/tools.html |
| Dimension style / per-entity overrides | ✓ (P) | partial (dimregen) | Q: DIM variable set (DIMTXT, DIMGAP, …). URL: https://qcad.org/en/qcad-features/ (Dimensions) |

### 2.3 Modify tools

| Tool | Q | L | Notes |
|---|---|---|---|
| Move / Copy | ✓ | ✓ | Reference point → target; dialog/checkbox offers delete original / keep original / **multiple copies** (Q) or keep-original + copies count (L). URLs: https://qcad.org/en/tutorial-1-9-modification ; https://docs.librecad.org/en/latest/ref/tools.html (Modify) |
| "Use current attributes / layer" choice | ✓ | ✓ | Both move dialogs let copies adopt current layer+pen instead of source attributes. URLs: https://qcad.org/en/manual/scripts/modify/mirror/doc/mirror (dialog described for mirror; same dialog family) ; https://docs.librecad.org/en/latest/ref/tools.html |
| Rotate | ✓ | ✓ | Centre + angle (mouse- or dialog-entered); copy count; angle by reference/target points. URL: https://qcad.org/en/manual/scripts/modify/rotate/doc/rotate ; L manual (ro) |
| Scale | ✓ | ✓ | Reference point + factor; Q supports factor-by-mouse via reference→target; L factor per axis. URLs: modify manuals |
| Mirror (axis by two points) | ✓ | ✓ | Keep/delete original; adopt-current-attributes choice. URL: https://qcad.org/en/manual/scripts/modify/mirror/doc/mirror |
| Flip horizontal / vertical | ✓ | ✗ (mirror covers it) | Q one-click flips. URL: https://qcad.org/shortcuts/shortcuts_en.pdf (FH/FV) |
| Move + rotate combined | ✓ | ✓ | One op: translate then rotate by given angle, with copies. URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (MR) ; L tools table (mr) |
| Rotate two (rotate + counter-rotate) | ✓ | ✓ | Rotate around absolute centre while aligning a second reference. URLs: Q R2 shortcut sheet; L tools table (r2) |
| Align reference points / Align | ✓ (P) | ✗ | Move+rotate+optional scale to map two reference points onto two targets. URL: https://qcad.org/en/qcad-features/ (Align reference points) |
| Offset (distance) | ✓ | ✓ | Lines/arcs/circles/ellipses; Q also offsets polylines (P) with miter/round joins. URLs: https://qcad.org/en/qcad-features/ (Offset & Trim) ; L tools table (o/pa) |
| Offset through point | ✓ | ✓ | Q: OH. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Trim (one limiting entity) | ✓ | ✓ | First click limiting entity, then click each entity **on the part to keep**. URL: https://qcad.org/en/tutorial-1-9-modification ; https://docs.librecad.org/en/latest/ref/tools.html (tm) |
| Trim both (corner) | ✓ | ✓ | Two entities mutually trimmed to their intersection. URL: https://qcad.org/en/tutorial-1-9-modification ; L: t2 |
| Lengthen / shorten by amount | ✓ | ✓ | Click near the end to grow; amount in options. URL: https://qcad.org/en/tutorial-1-9-modification ; L: le |
| Stretch (window + reference→target) | ✓ | ✓ | URLs: https://qcad.org/en/qcad-features/ ; L tools table (ss) |
| Divide (cut at point) | ✓ | ✓ | Entity + cutting point (needs a snap). URL: https://qcad.org/en/manual/scripts/modify/divide/doc/divide ; L: di |
| Break out segment / gap | ✓ | ✗ | Q: remove a piece between two points on an entity; gap removes a piece of given length. URLs: https://qcad.org/en/qcad-features/ (Break & Extend) |
| Auto trim (smart trim/extend nearest) | ✓ | ✗ | Single click trims/extends to closest intersections. URL: https://qcad.org/en/qcad-features/ |
| Chamfer / Bevel | ✓ | ✓ | Setback distances; Q: CH. URLs: Q features; L tools table (ch) |
| Round / Fillet | ✓ | ✓ | Radius; Q adds round-all-corners and single-click rounding for polylines (P). URLs: same |
| Reverse direction | ✓ | ✓ | Swap start/end of lines/arcs/splines (matters for wall joins, hatch boundaries). URL: https://qcad.org/doc/qcad/latest/developer/group__ecma__modify.html (Reverse) ; L: revert |
| Explode | ✓ | ✓ | Q: dimensions, block refs, hatches (P: ellipses/splines→polylines, text→polylines); L: blocks/polylines; text→letters. URLs: https://qcad.org/en/qcad-features/ ; https://docs.librecad.org/en/latest/ref/tools.html |
| Edit text / edit hatch | ✓ | ✗ (use properties) | Q dedicated edit tools. URL: https://qcad.org/shortcuts/shortcuts_en.pdf (MT/MH) |
| Properties / Attributes | ✓ | ✓ | Q: property editor for one/many entities; L: `mp` (single, geometry+pen) and `ma` (multi, pen/layer). URLs: https://qcad.org/en/qcad-features/ ; https://edwinsaul.com/docs/LibreCAD/LibreCAD_commandFull.pdf |
| Draw order (front/back/raise/lower) | ✓ | ✓ | Layers do NOT control order in either app. URLs: https://www.qcad.org/en/tutorial-1-3-layers ; https://docs.librecad.org/en/latest/ref/tools.html (Order) |
| Detect duplicates / zero-length | ✓ (P) | ✗ | URL: https://qcad.org/en/qcad-features/ |
| Split into equal parts | ✓ (P) | partial (line divide tool) | URL: https://qcad.org/en/qcad-features/ |
| Quick modify (one-click move/rotate presets) | ✓ (P) | ✗ | Pre-configured nudge distance / angle. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §2.11 |
| Delete | ✓ | ✓ | Del key; both. URLs: respective manuals |

### 2.4 Selection

| Tool | Q | L | Notes |
|---|---|---|---|
| Single pick (toggle) | ✓ | ✓ | Click selects; click again deselects (L toggle; Q with Ctrl). URLs: https://qcad.org/en/tutorials/selection ; https://docs.librecad.org/en/latest/ref/fundamentals.html |
| Shift/ctrl additive click | ✓ | ✓ | URLs: same |
| Window (fully-enclosed, L→R, blue solid) | ✓ | ✓ | Dragging left→right selects only fully-inside entities. URLs: https://qcad.org/en/tutorials/selection ; https://docs.librecad.org/en/latest/ref/tools.html (Select) |
| Crossing (touching, R→L, green dashed) | ✓ | ✓ | Dragging right→left also selects entities merely crossed. URLs: same |
| Polygon area select | ✓ (P) | ✗ | Snap-assisted polygon; right-click closes. URL: https://qcad.org/en/tutorials/selection |
| Fence (entities crossed by a line) | ✓ | ✓ | Q: "Select Intersected Entities"; L: same with (De-)select variants. URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (TX) ; L tools table |
| Contour (connected chain pick) | ✓ | ✓ | Double-click a chain (Q) or contour tool (L); Shift+double-click removes it. URLs: https://qcad.org/en/tutorials/selection ; L tools table |
| Select layer (all on clicked entity's layer) | ✓ | ✓ | URLs: Q shortcuts TL; L tools table |
| Select block references | ✓ | ✗ | Q: BK/BX. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Invert selection | ✓ | ✓ | URLs: Q shortcuts TI; L tools table (is) |
| Select all / deselect all | ✓ | ✓ | Ctrl+A / Ctrl+K(+Esc in L). URLs: same |
| Selection modes: replace / add / remove / intersect | ✓ | ✗ (add via Shift only) | Q tools carry an explicit mode widget. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §4 Select |
| Selection filter (type + property criteria) | ✓ (P) | ✗ (filters exist in core selection set only) | Q widget filters by type/colour/layer/text-content etc. URL: https://qcad.org/en/tutorials/selection-filter |
| Selection viewer / overlapping-entity pick | ✓ | ✓ | Q: "Select desired entity from overlapping entities"; L: repeated clicks cycle. URLs: https://qcad.org/en/qcad-features/ (Selection) |

### 2.5 Snapping

| Snap | Q | L | Notes |
|---|---|---|---|
| Free | ✓ | ✓ | No restriction. URLs: https://qcad.org/en/qcad-features/ (Snap) ; https://docs.librecad.org/en/latest/ref/snaps.html |
| Grid | ✓ | ✓ | URLs: same |
| Endpoints | ✓ | ✓ | Also circle quadrants, text alignment points (L). URLs: same |
| On entity (nearest point on path) | ✓ | ✓ | URLs: same |
| Centre | ✓ | ✓ | L also snaps ellipse foci. URLs: same |
| Middle | ✓ | ✓ | L's middle snap has a "middle points" count: 2 → trisection points. URLs: same |
| Middle of two points | ✓ | ✓ | Q: pick two points, snap their midpoint (centre of a rectangle without aux lines). URL: https://qcad.org/ebook/qcad_book_preview_en_2026.pdf (snap table) |
| Intersection | ✓ | ✓ | L: not implemented for polylines (documented limitation). URLs: same |
| Intersection manual (two picked entities) | ✓ | ✗ | Q: SY. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Perpendicular | ✓ | ✓ | URLs: features/snaps pages |
| Tangential | ✓ | ✓ | URLs: same |
| Reference points | ✓ | ✗ | Q: snap to entity reference handles. URL: https://qcad.org/en/qcad-features/ |
| Distance from endpoint | ✓ | ✓ | Typed distance along the entity from the nearest end. URLs: same |
| Distance manual (between two picked points) | ✓ | ✗ | Q: SH. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Distance polynomial (even division) | ✓ | ✗ | Q: distance-polynomial family (divide entity into n parts at snap points). URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §8 Snap |
| Auto snap | ✓ | ✗ (exclusive modes instead) | Q combines candidate snaps by proximity priority. URLs: features; L snaps page |
| Coordinate / polar-coordinate entry as snap | ✓ | ✓ (typed coordinates) | Q: SX/SO widgets. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf §8.16–8.17 |
| Restrictions: horizontal / vertical / orthogonal | ✓ | ✓ | Constrain cursor to an axis line through relative zero. URLs: same |
| Restrict angle+length | ✓ | ✗ | Q: EL. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Relative zero: set / lock | ✓ | ✓ | The last-used point, drawn as a small red circle+cross; coordinates default relative to it; lock freezes auto-move. URLs: https://qcad.org/en/tutorials/coordinates ; https://docs.librecad.org/en/latest/ref/snaps.html |
| Lock snap (sticky) | ✓ (P) | ✓ ("Exclusive snap mode" toggles multi vs single) | Q locks the current snap; L remembers one-mode vs multi-mode sets. URLs: same |
| X/Y from points (component snaps) | ✓ (P) | ✗ | Take x from one point, y from another. URL: https://qcad.org/shortcuts/shortcuts_en.pdf (PX/PY) |

### 2.6 Info / measurement

| Tool | Q | L | Notes |
|---|---|---|---|
| Cursor position (absolute+relative, cartesian+polar) | ✓ (status bar) | ✓ (status bar) | URLs: https://qcad.org/doc/qcad/2.2/reference/en/chapter06.html ; https://docs.librecad.org/en/stable/getstart/configure.html |
| Distance point → point | ✓ | ✓ | Prints distance + ΔX/ΔY/angle. URLs: https://qcad.org/shortcuts/shortcuts_en.pdf (IP) ; https://docs.librecad.org/en/latest/ref/tools.html (Info) |
| Distance entity → point | ✓ | ✓ | Shortest distance. URLs: same |
| Distance entity → entity | ✓ | ✗ | URL: https://qcad.org/shortcuts/shortcuts_en.pdf (IN) |
| Angle (two lines) | ✓ | ✓ | URLs: same |
| Polygonal area (+ circumference) | ✓ | ✓ | Click vertices. URLs: same |
| Arc/circle/ellipse area | ✓ | ✗ | URL: https://qcad.org/shortcuts/shortcuts_en.pdf (IC) |
| Polyline area | ✓ (P) | ✗ | URL: https://qcad.org/en/qcad-features/ |
| Total length of selected entities | ✓ | ✓ | L also shows the total in the status bar after modify ops. URLs: Q shortcuts IS; L tools table; https://tutorials.mediaket.net/software-tutorials/librecad-tutorial.html |
| Display distance/angle overlay (F8) | ✓ | ✗ | Live two-pick measurement HUD. URL: https://qcad.org/shortcuts/shortcuts_en.pdf (F8) |

---

## 3. Edge cases & nuances

**QCAD**

1. **Layers never control draw order.** Users coming from graphics software expect z-order from layers; QCAD's docs explicitly deny it and provide draw-order tools instead (https://www.qcad.org/en/tutorial-1-3-layers).
2. **Layer "0" is permanent** — never deletable, never renamable, and recommended to keep empty (https://qcad.org/en/tutorial-1-3-layers).
3. **By Layer / By Block resolution.** Entities inside block references resolve attributes through a chain: entity's own attribute → its layer → or "By Block" → the *insertion* layer of the reference. Changing the reference's layer recolours only the By-Block entities (https://qcad.org/en/tutorial-layers-blocks-and-attributes).
4. **Linetype/lineweight invisibility in model space.** Patterns are sized in millimetres; on a 1:100 building plan they vanish unless "screen-based linetypes" is on or the global linetype scale is raised (e.g. ×1000 for metres). They do render correctly in print preview (https://qcad.org/en/tutorial-working-with-layers, Troubleshooting).
5. **Trim keeps what you click.** The trim tool always retains the portion on the *clicked* side — the tutorial has the user undo and re-click on the other side when the wrong half vanished (https://qcad.org/en/tutorial-1-9-modification).
6. **Command/normal mode double-Escape.** Escape exits command mode; if the field has text, a second Escape is needed (https://qcad.org/doc/qcad/2.2/reference/en/chapter06.html).
7. **Dimension label placeholder.** An empty label prints the measured value; `<>` inside a custom label embeds the auto measurement at that spot (https://qcad.org/doc/qcad/2.2/reference/en/chapter24.html).
8. **Move-label grab dot.** Dimension labels move via a small blue dot at the label centre; the pointer must come very close to it (https://qcad.org/en/faq/).
9. **Draft mode hides hatches, big text and images** — a recurring "my hatch disappeared" support question (https://qcad.org/en/faq/).
10. **Print preview purple rectangle** signals glue-margin overlap — a mis-set multi-page margin, not a drawing error (https://qcad.org/tutorial-printing/).
11. **Grey background in print preview** means paper/scaling settings don't match the drawing extent (https://qcad.org/tutorial-printing/).
12. **Anonymous blocks** (system-generated definitions behind dimensions/hatches) have caused DXF name-mixup bugs on write; they must be kept in sync but out of the user's way (https://qcad.org/en/changelog, FS#1216).
13. **Relative zero auto-moves** to every newly created point; its lock (RL) is what makes repeated relative entries predictable. F8 cycles a live distance/angle HUD: both → distance only → angle only → off (https://forum.qcad.org/t/option-to-set-relative-coordinates-as-the-default/6864).
14. **Pasting a coordinate list** replays it through the active tool — an efficient bulk-entry path with no separate import wizard (https://qcad.org/en/tutorials/drawing/using_a_list_of_coordinates/).
15. **Viewport layer trick.** Hiding the viewport's layer hides only the frame, never the contents — which is exactly what enables a clean "how it prints" preview (https://qcad.org/en/tutorial-working-with-layout-blocks-and-viewports).

**LibreCAD**

1. **Tool Options toolbar is load-bearing.** Print preview controls and block-insert options exist *only* there; the manual instructs users to keep it always enabled (https://docs.librecad.org/en/latest/ref/toolbars.html).
2. **Snap Intersection ignores polylines** — a documented functional gap (https://docs.librecad.org/en/latest/ref/snaps.html).
3. **Divide needs a snap.** Without a snap mode on, the cut point can't be placed precisely on an infinitesimally thin line (https://docs.librecad.org/en/latest/ref/tools.html; wiki users manual).
4. **Print-scale quirk.** The scale text box applies only after pressing **Enter**; tabbing out "works sometimes, but it is not consistent" (https://forum.librecad.org/printing-to-1-87-scale-td5720326.html).
5. **"Fixed" checkbox must be re-ticked** before leaving print preview, otherwise the next zoom silently changes the print scale (https://forum.librecad.org/Print-Scale-td5707755.html).
6. **Fit-to-page is a trap for stray entities.** Plain Print (no preview) fits to an arbitrary scale, so a leftover speck far away shrinks the whole sheet; the forum recommends a scaled drawing frame or always using preview (https://forum.librecad.org/discovering-print-plot-where-is-the-scale-setting-td5714538.html).
7. **Draft mode hides hatches and shows an empty rectangle for dimension text** (https://dokuwiki.librecad.org/doku.php/usage:faq).
8. **Blank PDF on export** = no paper format chosen in Current Drawing Preferences (https://dokuwiki.librecad.org/doku.php/usage:faq).
9. **Layer "0" semantics for blocks.** Block contents should live on layer "0" (= "no layer") so instances can adopt per-insert attributes; entities on other layers keep their own (https://docs.librecad.org/en/latest/guides/blocks.html).
10. **Alias drift across versions.** Between 2.1 and 2.2 several aliases changed meaning (`dr` linear→radial dimension, `pp` parallel→polygon, `da` aligned→angular) — a migration trap for muscle memory and for third-party shortcut lists (https://tutorialtactic.com/blog/librecad-shortcuts).
11. **Array-inserted blocks are one instance.** Selecting any member selects all; separate instances require individual inserts (https://docs.librecad.org/en/latest/guides/blocks.html).
12. **DXF-version lottery on import.** Old-version conversions explode hatches into thousands of segments and approximate splines as polylines; the practical advice is to try a mid-range version first (https://cadblockdwg.com/blog/librecad-reads-dxf-only-converting-dwg-blocks).

**Shared**

- **Window colour = mode.** Both apps signal window vs crossing selection with box fill/border style (blue solid vs green dashed in both) — an established convention Lintel already mirrors (URLs in §2.4).
- **1:1 authoring, scale at print.** Both manuals insist geometry is drawn full size and scaling happens only at output (https://docs.librecad.org/en/latest/guides/dwg-setup.html; https://qcad.org/tutorial-printing/).
- **Dimension text must be pre-scaled.** Because output shrinks the plan, dimension text/arrows need a global scale factor (Q: dimension preferences; L: "General Scale") sized for the print scale, not model units (https://docs.librecad.org/en/latest/guides/dwg-setup.html).

---

## 4. Candidate capabilities for Lintel

Lintel already has: walls/doors/windows, aligned dimensions, layers panel (visibility/lock), construction geometry, command palette, snap chips, window/crossing/fence selection, print-to-PDF with sheets/title blocks/viewports, block definitions + instances. Candidates below are ordered by fit; each carries a behavioural summary, an acceptance-criteria sketch, and a source URL.

**C-1. Options row that follows the active tool.** Both apps project the active tool's parameters into a dedicated strip (QCAD options toolbar; LibreCAD Tool Options) so values are editable before each click, with typed digits routed to the first field (QCAD). Lintel's command palette could gain a per-tool parameter row. *Acceptance:* starting the wall tool shows length/thickness fields in one fixed dock location; typing a digit focuses the first field; Escape returns focus to canvas. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf (§3.22–3.23); https://docs.librecad.org/en/latest/ref/toolbars.html.

**C-2. Command history / output log.** QCAD's command widget pairs the input with a scrollable log where every command, coordinate and error lands (errors in red); LibreCAD has the same output window. Lintel's command palette is transient; a dockable history would make typed sessions auditable. *Acceptance:* all accepted/refused commands append to a scrollable log; refused entries render in the error colour; the log survives palette close. URL: https://qcad.org/doc/qcad/2.2/reference/en/chapter06.html (§6.5); https://docs.librecad.org/en/latest/guides/cmdline.html.

**C-3. Calculator input in coordinate fields.** QCAD evaluates `=expr`, stores variables, and accepts expressions inside coordinates (`5+5,30/6`); LibreCAD accepts `100<45+15`. For a floor-plan tool this is high-leverage: users compute offsets inline. *Acceptance:* coordinate fields accept arithmetic expressions; `=expr` in the palette prints the result to the log; variables persist per session. URL: https://qcad.org/en/manual/scripts/widgets/commandline/doc/commandline; https://docs.librecad.org/en/latest/ref/fundamentals.html.

**C-4. Relative zero point (set + lock + marker).** Both apps maintain a "relative zero" anchor that auto-moves to the last created point, is drawn as a small red circle+cross, and can be set or locked. Lintel's snap system could adopt it for chained wall drawing with `@dx,dy` entry. *Acceptance:* after each placement the anchor moves to the new point; the marker renders in the error/accent colour at a fixed pixel size; Set/Lock commands exist; locked anchors ignore auto-move. URL: https://qcad.org/en/tutorials/coordinates; https://docs.librecad.org/en/latest/ref/snaps.html.

**C-5. Multiple copies in move/copy.** QCAD's move dialog offers delete-original / keep-original / N-copies where each copy steps by the reference→target delta (their tutorial builds a whole row of benches this way); LibreCAD has the same copies count. *Acceptance:* Move/Copy gains a copies field (1 = plain move, ≥2 = evenly spaced series); undo reverts the whole series atomically. URL: https://qcad.org/en/tutorial-1-9-modification; https://docs.librecad.org/en/latest/ref/tools.html.

**C-6. Offset (parallel) with distance + through-point for walls-as-polylines.** Both apps offset lines/arcs/circles by distance or through a point; QCAD additionally offsets whole polylines with miter/round joins. Lintel's wall chains are polylines — a single "offset wall run" command maps directly onto room outlines. *Acceptance:* selecting a connected wall chain and a distance creates a parallel chain on the clicked side with joined corners; through-point variant derives the distance from a picked point. URL: https://qcad.org/en/qcad-features/ (Offset & Trim); https://docs.librecad.org/en/latest/ref/tools.html (Offset).

**C-7. Trim with "click the part to keep" semantics + trim-both.** QCAD trims one entity to a limiting entity, retaining the clicked side; trim-both forms a corner from two entities. Lintel has walls but no trim — the natural target is construction geometry and wall-end cleanup. *Acceptance:* Trim(limiting, target, keepSide) shortens/extends the target exactly to the limiting entity; TrimBoth resolves a corner; wrong-side clicks are undoable in one step. URL: https://qcad.org/en/tutorial-1-9-modification; https://docs.librecad.org/en/latest/ref/tools.html (Trim/Trim Two).

**C-8. Rotate with copy count and mouse-angle fallback.** Rotation around a picked centre with keep/delete-original, N copies, and optional angle-by-reference/target points. *Acceptance:* Rotate(center, angle, copies) on any selection; angle may be given by two points; copies distribute evenly. URL: https://qcad.org/en/manual/scripts/modify/rotate/doc/rotate.

**C-9. Middle-of-two-points snap + distance-along-entity snap.** QCAD snaps to the midpoint of any two picked points (rectangle centre with zero construction) and to a point at a typed distance from the nearest endpoint; LibreCAD's middle snap generalises to trisection ("middle points = 2"). Both are cheap, high-value snap additions to Lintel's existing snap set. *Acceptance:* a "mid between two picks" chip takes two picks and marks the midpoint; a "distance along" chip accepts a length and snaps that far from the hovered endpoint. URL: https://qcad.org/ebook/qcad_book_preview_en_2026.pdf (snap table); https://docs.librecad.org/en/latest/ref/snaps.html.

**C-10. Total-length-of-selection readout.** LibreCAD shows the summed length of the current selection in the status bar (their modify tutorial flags it for material estimation); QCAD has an explicit Info tool for it. For floor plans this is a tape-measure for skirting/coving. *Acceptance:* with walls (and/or construction lines) selected, the status bar shows total length in the active unit; empty selection hides it. URL: https://docs.librecad.org/en/latest/ref/tools.html (Info: total length); https://qcad.org/shortcuts/shortcuts_en.pdf (IS).

**C-11. Dimension label `<>` placeholder + fixed-label mode.** Empty label = measured value; `<>` embeds the measured value inside custom text ("clear 1200 typ."). *Acceptance:* a dimension's label may be empty (auto value) or contain `<>` once, substituted at render; a label containing a literal only prints literally. URL: https://qcad.org/doc/qcad/2.2/reference/en/chapter24.html.

**C-12. Poster/multi-page print with glue margins, crop marks, page tags.** QCAD tiles a scaled plan across N×M pages with overlap margins, optional cross-hairs at page corners and page tags ("B3"), plus auto-add-pages. Lintel's PDF pipeline already does sheets; true tiling of one oversized sheet is the missing sibling for users without A2 printers. *Acceptance:* Export sheet at scale S larger than the paper yields a tiled PDF (N×M) with configurable overlap; crop marks and page tags are togglable; page order is row-major and documented. URL: https://qcad.org/en/blog/121-qcad-3-release-multipage-printing; https://qcad.org/tutorial-printing/.

**C-13. Fit-to-page and centre-on-page as explicit PDF actions.** Both apps offer fit-to-paper and centre-on-page in preview; LibreCAD's forum shows the failure mode when a stray entity inflates the extent (fit produces a postage stamp). *Acceptance:* Lintel's export computes extents from printed layers only (construction geometry excluded), warns when extents exceed the sheet, and offers fit/centre at 1:100-style presets. URL: https://qcad.org/tutorial-printing/; https://forum.librecad.org/discovering-print-plot-where-is-the-scale-setting-td5714538.html.

**C-14. Per-layer print/no-print flag separate from visibility.** QCAD layers carry a plottable column distinct from the eye toggle (view on screen, still excluded from output — used for viewport frames); LibreCAD's docks freeze/hide only. *Acceptance:* layers gain a "prints" flag; unprintable layers render on canvas but never in PDF; the eye toggle remains view-only. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf (§3.12; viewport-layer workflow in the layouts tutorial).

**C-15. Layer states (named visibility/lock snapshots).** QCAD Pro stores/recalls whole-configuration layer states (print set vs export set). Cheap to implement on Lintel's existing layer model. *Acceptance:* Save Layer State stores visibility+lock per layer; Apply restores it; states are project-persisted and undoable. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf (§ Layer States).

**C-16. Block definition opening as an editable sub-context.** Both apps edit a block in a separate window (LibreCAD) or by drilling into the definition (QCAD), with all instances auto-updating on close. Lintel's block definitions are inspector-edited; a "edit definition contents" mode with instance auto-update would close the loop. *Acceptance:* entering a definition shows only its entities; edits commit to the definition; every instance reflects them; leaving returns to the level view. URL: https://docs.librecad.org/en/latest/guides/blocks.html (edit in separate window; instances update); https://qcad.org/en/tutorial-1-11-blocks.

**C-17. Block insert with rotation/scale/array.** LibreCAD's insert options include angle, factor, and array rows/columns/spacing; QCAD inserts with angle/scale via options. *Acceptance:* placing an instance exposes rotation and scale before placement; array mode stamps rows×columns at a spacing and undoes as one command. URL: https://docs.librecad.org/en/latest/ref/toolbars.html (Block insert tool options); https://qcad.org/en/qcad-features/ (Insert array of blocks).

**C-18. Layer filter box with wildcards.** LibreCAD's layer dock filters by substring or `*`/`?` patterns. For plan files with dozens of layers this is a small, beloved feature. *Acceptance:* typing filters the layer panel live; `*01` matches suffixes; clearing restores the full list. URL: https://docs.librecad.org/en/latest/ref/widgets.html?layer-list-dock.

**C-19. Selection modes replace/add/remove/intersect.** QCAD's selection tools carry an explicit four-mode widget (replace default; add; remove; intersect). Lintel has additive selection; "remove" and "intersect" complete the algebra. *Acceptance:* window/fence selections honour the active mode; intersect keeps only already-selected matches; the mode is visible in the status bar. URL: https://qcad.org/user_manual_pdf/qcad_reference_manual_en.pdf (§4 Select).

**C-20. Coordinate-list paste replay.** QCAD pastes a multi-line coordinate list into the command field and replays it through the active tool. For Lintel: paste a surveyor's wall run into the wall tool. *Acceptance:* pasting `x,y` lines into the canvas while a point-consuming tool is active places each point in order; refused lines are reported without aborting the rest. URL: https://qcad.org/en/tutorials/drawing/using_a_list_of_coordinates/.

---

## 5. Not applicable (with reasons)

| Feature | Reason not applicable to Lintel |
|---|---|
| Full DXF/DWG read-write (R11→R32) | Lintel is a native floor-plan editor with its own JSON format; a complete DWG stack is out of class. If interop is ever needed, DXF export of flattened geometry is the honest scope (both apps' docs show how lossy mapping is: hatches→fills, beziers→splines). URLs: https://qcad.org/en/qcad-features/; https://dokuwiki.librecad.org/doku.php/usage:faq |
| ECMAScript scripting API + script debugger | A whole runtime + API surface; Lintel's command palette and Rust core serve the same need at far lower cost. URL: https://qcad.org/en/qcad-features/ (Automation) |
| Library Browser with 5000+ parts, external libraries | Content-distribution feature, not a drafting behaviour; Lintel's block definitions cover reuse. URL: https://qcad.org/en/qcad-features/ (Library) |
| Spline/NURBS tool family (degrees 1–5, fit-point editing) | Architectural floor plans are line/arc work; splines add solver and rendering complexity with no plan-drawing use case today. URLs: §2.1 sources |
| Ellipse foci/4-point/centre+3pt constructions and ellipse offset curves | Mechanical-drafting constructions; not floor-plan relevant. URLs: §2.1 sources |
| Hatch pattern engine (68+ .pat patterns, auto-hatch) | Lintel's PDF sheets use plan symbols; a full pattern engine is a large rendering subsystem. A single "poche" solid fill for walls would cover the architectural need. URL: https://qcad.org/en/qcad-features/ (Hatches & Fills) |
| Print layouts as paper-space blocks with rotated viewports | Lintel already has sheets + viewports (CAP-114/CAP-118); QCAD's rotated-viewport concept is explicitly noted as unsupported elsewhere — not worth porting. URL: https://qcad.org/en/manual/scripts/pro/draw/viewport/addviewport/doc/addviewport |
| Isometric grid/projections | 3D-ish view modes out of Lintel's 2D-plan scope. URL: https://qcad.org/shortcuts/shortcuts_en.pdf (JO/JT/JR/JL) |
| Bitmap underlay + image tracing (vectorisation) | Raster workflow is a separate product concern; import/validation complexity is high. URL: https://qcad.org/en/qcad-features/ (Raster Images; Trace images) |
| Wipeouts/masks | PDF clipping in Lintel's viewports covers the masking need. URL: https://qcad.org/en/qcad-features/ (Wipeouts) |
| XRefs (external references with reload/bind) | Multi-file drawing management beyond a single-plan tool; block library saving covers reuse. URL: https://qcad.org/en/changelog |
| Selection filter widget by property predicates | Lintel's selection filters (type/layer) already cover the plan-scale need; the general predicate engine is QCAD-Pro-sized. URL: https://qcad.org/en/tutorials/selection-filter |
| GD&T tolerances, datum leaders, ordinate/baseline/continue dimension chains | Mechanical dimensioning; architectural plans need aligned/H/V (+ maybe leaders), which Lintel has or C-11 extends. URL: https://qcad.org/shortcuts/shortcuts_en.pdf |
| Plugin system (C++ plugins, LibreCAD) | Tied to LibreCAD's Qt architecture; Lintel's extension story is its command/core layer. URL: https://docs.librecad.org/en/latest/ref/widgets.html |
| Torn-off menus, tear-off toolbars, style sheets | Qt desktop chrome; egui panels re-arrange differently and II.30 token rules govern Lintel's UI. URL: https://docs.librecad.org/en/stable/getstart/configure.html |
| Freehand sketch line | Not precise-drafting behaviour; construction geometry serves layout scratch work. URL: https://docs.librecad.org/en/latest/ref/tools.html (Freehand Line) |

---

## 10-line summary of highest-value findings

1. **Options-toolbar pattern is the core input loop** in both apps: the active tool projects its parameters into one always-visible strip, and typed digits route to the first field — Lintel's palette should grow a per-tool parameter row (C-1).
2. **Command line = coordinates + calculator + history**: `x,y`, `@x,y`, `d<a`, `@d<a`, inline arithmetic (`5+5,30/6`), `=expr` calculator with variables, and a scrollable error-coloured log — the highest-leverage keyboard upgrade for Lintel (C-2, C-3).
3. **Relative zero point** (auto-moving last-point anchor, red circle-cross marker, settable/lockable) is the backbone of chained relative entry in both apps (C-4).
4. **Move/copy with N-copies** (each copy stepping by the drag delta) builds rows/columns of furniture in one gesture; same dialog family carries "use current layer/attributes" (C-5).
5. **Offset/trim are the architectural workhorses**: polyline offset with miter/round joins maps directly to wall runs, and trim's "click the part you keep" rule is the convention to copy (C-6, C-7).
6. **Print preview is a mode, not a dialog**: scale combo, fit-to-page, centre-on-page, move-paper, and poster tiling with glue margins + crop marks + page tags — directly portable to Lintel's PDF export (C-12, C-13).
7. **Per-layer print flag ≠ visibility** (QCAD plottable column) enables on-screen-only helpers like viewport frames; layer states snapshot whole configurations (C-14, C-15).
8. **Block UX loop**: edit definition in a separate context with instances auto-updating; insert with angle/scale/array options; freeze/hide per block (C-16, C-17).
9. **Status bar carries measurement context**: cursor position (absolute+relative, cartesian+polar), mouse-button role hint, selection count — and LibreCAD's total-length-of-selection readout is a perfect tape-measure for plans (C-10).
10. **Documented edge cases to design around**: snap-intersection fails on polylines (L), print-scale needs Enter + a "fixed" lock (L), draft mode hides hatches (both), stray entities wreck fit-to-page (L), linetypes vanish at plan scale without a global linetype scale (Q), and layers never control draw order (both).
