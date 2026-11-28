# AutoCAD Core Drafting Interactions — Deep Clean-Room Behavioural Note

**Project:** Lintel (2D architectural floor-plan CAD, Rust/egui)
**Date:** November 2026
**Method:** Public black-box sources only — official Autodesk online help (AutoCAD 2024–2027 help pages, onboarding course, official "Have You Tried"/Did-You-Know series, official Autodesk support knowledge-base articles). Every claim is an original behavioural summary in our own words with the source URL. No verbatim copying of documentation text, no pixel descriptions of AutoCAD trade dress, no decompilation, no internals.
**Scope:** Command line, dynamic input, coordinate entry, object snaps, tracking, grips, selection, zoom/pan, drawing aids, measurement, units, and command-repetition nuances — with the precise edge cases for each. Complements (does not duplicate) `design-audit-drafting-uis.md` and `elite-cad-competitive-analysis.md`.

---

## 1) Behaviours (grouped, each with source URL)

### 1.1 Command line

- **Autocomplete + suggestion list.** As the user types a command name, the line auto-completes and a suggestion list appears of commands and system variables sharing the typed letters. The list reorders by usage frequency during the session — most-used commands rise to the top. Options (all individually toggleable): AutoComplete, AutoCorrect, search system variables, search named content (blocks, layers, hatches, text/dimension styles), and mid-string search (match letters anywhere in the word, not only as a prefix). Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-74478DCE-92D4-49B1-93E9-DCF09D57FA0D.htm
- **Content search in the command line.** Beyond commands, typing searches named drawing objects — e.g. typing part of a block name surfaces an entry that inserts the block, part of a layer name surfaces an entry that makes it current. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-74478DCE-92D4-49B1-93E9-DCF09D57FA0D.htm
- **AutoCorrect + synonyms.** Misspellings the user corrects a few times are learned into a user database and offered as suggestions; a synonym list maps user words to commands (e.g. a synonym entry for a common word can surface a command whose name differs). Both files are user-editable. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-74478DCE-92D4-49B1-93E9-DCF09D57FA0D.htm
- **Command aliases.** Short abbreviations start commands (a single letter can start a long-named command; the suggestion list shows the alias in front of the full name). Aliases live in an editable program-parameters file; custom aliases are supported. If a command can be entered transparently, its alias can too. Sources: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-3C87A58F-1980-451D-A9D3-32327FA63D81.htm , https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Customization/files/GUID-FE9AE544-F537-4D3B-8F75-B76484513787.htm
- **In-line command options.** After a command starts, options appear in brackets with one highlighted capital letter; clicking an option or typing its capital letter selects it (upper or lower case accepted). A default value appears in angle brackets — pressing Enter accepts it. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-BB0C3E79-66AF-4557-9140-D31B4CF3C9CF.htm
- **Transparent commands (mid-command interruption).** Prefixing a command with an apostrophe runs it *inside* another command at any point prompt: the interrupted command's prompts are marked with double angle brackets, and when the transparent command finishes, the original command resumes where it left off. Typically permitted for commands that neither select objects, nor create objects, nor end the session — e.g. zoom, pan, grid, snapping settings. Sources: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-BB0C3E79-66AF-4557-9140-D31B4CF3C9CF.htm , https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-3C87A58F-1980-451D-A9D3-32327FA63D81.htm
- **Enter / Spacebar = repeat last command** when no command is active. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-OnBoarding/files/ACD_FOUNDATIONS_MAIN4.html
- **Command history.** Up/Down arrows cycle through commands and values used in the current session; a dropdown arrow next to the input shows recent commands. F2 expands an extended history (or opens a full text window when docked) which can be scrolled, selected, and re-pasted as a command sequence. Source: https://help.autodesk.com/cloudhelp/2027/ENG/AutoCAD-Core/files/GUID-06F6A6E9-C355-43F5-8706-49F0E5E552BF.htm
- **Right-click repeat.** Right-clicking in the command line (or a right-click in the drawing area, configurable) offers recently used commands. Source: https://help.autodesk.com/cloudhelp/2016/ENU/AutoCAD-MAC-Core/files/GUID-3C87A58F-1980-451D-A9D3-32327FA63D81.htm
- **Cancel.** Esc ends the current command and clears the input; Esc also cancels an in-progress selection. Source: https://help.autodesk.com/cloudhelp/2027/ENG/AutoCAD-Core/files/GUID-06F6A6E9-C355-43F5-8706-49F0E5E552BF.htm , https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-OnBoarding/files/ACD_FOUNDATIONS_MAIN4.html

### 1.2 Dynamic input (heads-up UI at the cursor)

- **Three components**, each separately toggleable: pointer input (coordinate entry fields at the crosshairs), dimension input (length/angle fields while drawing or grip-editing), and dynamic prompts (the command prompt text rendered at the cursor). Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm
- **Pointer input.** While a command runs, the crosshair coordinates appear in tooltip entry boxes near the cursor; values typed there are accepted exactly as command-line values. Autocomplete behaves the same as on the command line, so attention can stay at the cursor. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm
- **Relative-by-default.** For the second and later points, typed values default to *relative polar* coordinates (relative Cartesian for the rectangle command) — no `@` needed. `#` forces absolute, `@` forces relative, `*` prefixes coordinates taken as absolute in world coordinates. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm , https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-AE225073-2ECD-41CC-9B8A-61743E8CFBA9.htm
- **Dimension input with Tab-lock.** In polar/dimension fields: type a distance, press Tab, and the field shows a lock icon — the cursor is then constrained to that value on one axis while the other stays live. Cartesian entry: type X, comma, Y, Enter. Polar entry: type distance, Tab, angle, Enter. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-A265C30C-CFD8-405D-BEE2-257B379475E2.htm
- **Options and recent input at the cursor.** A down-arrow indicator after a prompt means options exist: Down arrow cycles to an option (marked), Enter accepts. Up arrow (or a shortcut menu) recalls recent coordinates/values. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-A265C30C-CFD8-405D-BEE2-257B379475E2.htm
- **F12 toggles dynamic input; holding F12 suppresses it temporarily while a command is active.** Source: https://www.autodesk.com/support/technical/article/caas/sfdcarticles/sfdcarticles/How-to-enable-or-disable-dynamic-input-in-AutoCAD.html , https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm

### 1.3 Coordinate entry (command line or dynamic input)

- **Absolute Cartesian:** `x,y`. **Absolute polar:** `distance<angle`. **Relative Cartesian:** `@x,y` — measured from the last picked point, whether picked in this command or a previous one. **Relative polar:** `@distance<angle`. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-392167BC-8032-44D9-B4A9-DF4AC00DF5C4.htm
- **Direct distance entry.** At any point prompt: move the cursor to indicate a direction, then type a distance — the point lands at that distance along the cursor's direction. Works for every command that takes more than one point, and is most efficient with ortho or polar tracking locking the direction. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-92BDE481-49D0-4AED-A6C5-0B78051EAE99.htm
- **Angle override.** Typing `<` followed by an angle at a point prompt locks the cursor to that absolute angle, overriding grid snap, ortho, and PolarSnap — but typed coordinates and object snaps still take precedence over the override. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-92BDE481-49D0-4AED-A6C5-0B78051EAE99.htm
- **Typing a wall/line length while drawing** = direct distance entry combined with a direction lock (ortho, polar, or an angle override): pick the start, move the cursor along the desired direction, type the length, Enter. With dynamic input on, the same action fills the dimension field instead. Sources: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-92BDE481-49D0-4AED-A6C5-0B78051EAE99.htm , https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm

### 1.4 Object snaps

- **Running object snaps** persist across commands and are toggled with F3; the active set is chosen from the status-bar menu. Sources: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-8F5E5431-9EFB-414E-BC6D-2C65EFB2DAC3.htm , https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-Customization/files/GUID-B6A9C126-230D-47F5-8973-464BC80E3736.htm
- **Per-point overrides.** An override snap (Shift+right-click menu, typed name, or toolbar button) applies to *only the next point*, then running snaps resume. Snaps can also be typed at any point prompt (e.g. `end`, `mid`, `per`). Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-8F5E5431-9EFB-414E-BC6D-2C65EFB2DAC3.htm
- **The snap catalogue** (2D): endpoint, midpoint, center, geometric center (centroid of closed polylines/splines), node, quadrant, intersection, extension (temporary extrapolation lines past an endpoint), insertion, perpendicular, tangent, nearest, apparent intersection, parallel (constrains a new segment parallel to a hovered object), none. "Deferred" perpendicular/tangent are armed before the object being drawn exists yet. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-CF5780AD-D1AB-4526-9608-83D7952749E7.htm , http://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-50383F73-4F23-4F70-B4FC-52D5748D80AF.htm
- **AutoSnap visual aids.** Four aids: a *marker* (shape depends on the snap type), a *tooltip* naming the snap, a *magnet* that pulls and locks the cursor onto the nearest snap point, and an optional *aperture box* around the crosshairs defining the search area. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-9882004A-97FA-446A-84AB-13E061965034.htm
- **Overlapping snaps: Tab cycling.** When several snaps are eligible at one spot, Tab cycles the marker through them before the pick commits. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-8F5E5431-9EFB-414E-BC6D-2C65EFB2DAC3.htm
- **Temporary suppression.** Holding F3 turns running snaps off for the duration of the press; a family of Shift+letter temporary override keys suppress specific snaps (endpoint, center, midpoint), suppress tracking, or suppress all snapping/tracking, only while held. Sources: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-CF6E6F93-7D33-4D54-9167-2CC7473DA660.htm , https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-8F5E5431-9EFB-414E-BC6D-2C65EFB2DAC3.htm
- **Mid-between-two-points (MTP/M2P).** Typing `mtp` at any point prompt asks for two points and then uses their midpoint as the specified point; it composes with running snaps and overrides. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-04C9F013-46A3-430E-A085-BF254EA6B1D6.htm
- **Object snap tracking (OTRACK).** With F11 on, hovering a snap point *acquires* it (a small plus sign marks it); as the cursor moves, alignment paths (horizontal, vertical, or at polar angles) are drawn relative to the acquired point, and their intersections with other geometry or other paths define new points; multiple points can be acquired and combined. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm
- **Temporary tracking point.** Typing `tt` at a point prompt sets a one-off reference point (marked with a plus) from which alignment paths are generated; moving the cursor back over the plus removes it. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm
- **Tracking + direct distance.** After acquiring a snap point and riding an alignment path, typing a distance places the point precisely along the path. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm

### 1.5 Polar tracking and PolarSnap

- **Polar tracking** constrains cursor movement to angle increments (predefined set: 90, 60, 45, 30, 22.5, 18, 15, 10, 5 degrees, plus any additional user-specified angles); when the cursor nears a tracked angle, an alignment path and a distance/angle tooltip appear and vanish when the cursor leaves the angle. Toggled with F10. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-7EC3C63D-EA4E-4E65-A676-C3A3627E3F19.htm
- **PolarSnap** snaps cursor movement to distance *increments along* the tracked angle (e.g. every 4 units: 0, 4, 8, 12…); requires both polar tracking and Snap mode set to PolarSnap. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-7EC3C63D-EA4E-4E65-A676-C3A3627E3F19.htm
- **Tracking from the last picked point:** perpendicular and tangent snaps can be tracked from the last point even when object snap tracking is off. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm

### 1.6 Grips

- **What grips are.** Small squares (plus special shapes for some object types) displayed on an object when selected with no command active; they allow stretch/move/rotate/scale/mirror without typing a command. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Three visual states.** Unselected grip colour, hover colour (fill when the cursor pauses on an unselected grip), and selected ("hot") colour are separately configurable, plus a grip contour colour; the grip box size is a setting in device-independent pixels. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Grip mode cycle.** Clicking a grip activates stretch by default; each press of Spacebar/Enter cycles stretch → move → rotate → scale → mirror; the prompt names the active mode. Copy is not a mode but an option available inside every mode (type its letter, then keep placing results). Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Base point option.** Inside any grip mode, a base-point option re-anchors the operation to any snapped point (e.g. rotate about a midpoint instead of the clicked grip). Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Multiple copies with reference snapping.** After making one copy with a typed angle (say 30°), holding Ctrl while dragging makes further copies that *snap to that angle*; releasing Ctrl returns to free placement, and a later Ctrl re-arms snapping at the most recent angle. The same works for distance: one typed move distance becomes the snap increment for Ctrl-copies. Subsequent copy angles/distances reference the **original** object, not the previous copy. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Multifunctional grip menus (hover).** Hovering (not clicking) certain grips opens a small menu of actions specific to the object type and grip; not all grips have menus. Clicking the grip then pressing Ctrl cycles through the menu options instead. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Multi-grip stretch.** Holding Shift *before* selecting grips allows several grips to be hot simultaneously; dragging one then stretches the geometry between them without deforming intermediate segments. Window-selecting multiple grips is not supported. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Performance guard.** Grip display is suppressed above a configurable object-count threshold, and grip display inside block references is a separate toggle. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- **Grip-editing honours all precision entry.** While a grip mode is active, points may be typed (coordinates), object-snapped, or tracked, and dynamic input fields accept the same values. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm

### 1.7 Selection

- **Implied windowing.** Clicking in empty space at a "select objects" prompt automatically starts a rectangular selection: dragging **left→right** creates a *window* (selects only objects fully inside) drawn with a solid boundary; **right→left** creates a *crossing* (selects objects inside *or touched*) drawn with a dashed boundary. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **BOX ambiguity.** The Box option is direction-dependent: right-to-left = crossing, otherwise window. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **Fence.** A multi-segment open polyline selects everything it crosses; it may cross itself. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **Window/crossing polygons (WP/CP).** Freeform polygons close automatically on Enter; they must not self-intersect. WP keeps only fully-inside objects, CP keeps touched ones. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **Lasso.** Press-and-drag in empty space draws a freeform lasso: dragging left starts crossing-lasso, right starts window-lasso; while the button is held, Spacebar cycles lasso modes (crossing → window → fence → crossing). Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm
- **Add/remove semantics.** Default (Add mode): every further pick adds to the set. Shift+click removes a picked object; Shift+window removes a whole windowful. A Remove mode exists as a typed option as well. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm , https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **PICKADD / PICKFIRST.** PICKADD (default 2): picks accumulate without a modifier; with the alternate setting, each new pick *replaces* the set unless Shift is held. PICKFIRST (default 1): noun/verb selection — select objects *before* starting a command and many editing commands act on that pre-selection (Delete key included). Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm , https://help.autodesk.com/cloudhelp/2020/ENU/AutoCAD-Core/files/GUID-DD2A36B6-1196-4205-9A07-32056292F0B5.htm
- **Previous / Last.** `P` recalls the most recent selection set (cleared by operations that delete objects; ignored after switching spaces); `L` selects the most recently created visible object. `U` at a selection prompt undoes the last pick. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **All.** Selects everything in the current space except objects on frozen or locked layers. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
- **Property-based selection.** Quick Select (QSELECT) filters by object type + one property + operator (equality, inequality, wildcard) and can be applied to the whole drawing or to the current selection; Select Similar matches selected objects' type plus chosen matching properties (e.g. layer + block name); the FILTER command builds reusable multi-criteria filter lists and can be invoked transparently at a selection prompt. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm , https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-FBBA809F-9BD4-4A34-B671-0B8A920B18A4.htm
- **Selection cycling.** When several objects overlap under the cursor, a badge/tooltip lists the candidates and lets the user choose which one to pick; Ctrl+W toggles the behaviour. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm , https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-Customization/files/GUID-B6A9C126-230D-47F5-8973-464BC80E3736.htm
- **Pickbox.** A small square replaces the crosshairs whenever a selection prompt is active; its pixel size is configurable. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm

### 1.8 Zoom and pan

- **Wheel = zoom toward the cursor.** Rolling the wheel zooms in/out; the point under the cursor is the stationary anchor, so the user can centre a target area by positioning the cursor *outside* it and scrolling — the area of interest drifts toward centre while zooming. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-10D3C9A9-22E8-4047-AED1-5E96C6C28866.htm
- **Wheel-drag = pan** (middle button drag); double-click wheel = zoom extents. Source: https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-FC9DE424-67E0-46E8-BCD7-C08B716F392D.htm , https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-10D3C9A9-22E8-4047-AED1-5E96C6C28866.htm
- **Zoom rate is adjustable** (ZOOMFACTOR, default 60; smaller values give finer steps per wheel notch) and the wheel direction is invertible (ZOOMWHEEL). Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-10D3C9A9-22E8-4047-AED1-5E96C6C28866.htm
- **Zoom command options.** Realtime (drag up/down; right-click offers a shortcut menu to switch between pan/zoom options), Previous (up to 10 stored views back), Window (two corners; the picked rectangle is fitted to the viewport, so its aspect need not match), All (grid limits *or* extents, whichever is larger), Extents (largest view containing all objects), Object (fit selected objects, which can be pre-selected before the command), Center (centre point + height value), Dynamic (a movable/resizable view box), Scale (plain number = relative to grid limits; number + `x` = relative to current view; number + `xp` = relative to paper-space units). Source: https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm
- **Realtime zoom feedback.** The cursor becomes a magnifier with +/− signs; a sign disappears at the corresponding zoom limit; releasing the pick button pauses zooming and the operation can be resumed elsewhere; Enter or Esc exits. Source: https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm
- **Pan changes only the view**, never object positions or magnification; zoom changes only magnification, never object sizes. Source: https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-FC9DE424-67E0-46E8-BCD7-C08B716F392D.htm
- **Back/Forward view navigation** complements Zoom Previous for hopping between a few working views. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-10D3C9A9-22E8-4047-AED1-5E96C6C28866.htm
- **Touchpad gestures** (pinch zoom, two-finger pan) are supported. Source: https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-FC9DE424-67E0-46E8-BCD7-C08B716F392D.htm

### 1.9 Drawing aids (grid, snap, ortho)

- **Grid** is a reference pattern of lines or dots over the plane; it is display-only and never plots. Adaptive display thins the grid when zoomed out and subdivides below the set spacing when zoomed in, scaled by the "major line every N" setting. Grid spacing of 0 inherits the snap spacing. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-FEA6BC6E-D81E-4AD2-BD4C-70078C57709A.htm , https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-66D637C9-6C47-420C-ADD1-83B64C73217A.htm
- **Grid snap** restricts the crosshairs to X/Y intervals; snap spacing is independent of grid spacing (a wide grid with a fine snap is a standard combination). Toggled with F9 / Ctrl+B. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-66D637C9-6C47-420C-ADD1-83B64C73217A.htm
- **Ortho** (F8 / Ctrl+L) constrains movement to horizontal/vertical. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-Customization/files/GUID-B6A9C126-230D-47F5-8973-464BC80E3736.htm
- **Exclusivity rules:** ortho and polar tracking cannot both be on; PolarSnap and grid snap cannot both be on. Turning one mode on silently supersedes the other. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-7EC3C63D-EA4E-4E65-A676-C3A3627E3F19.htm
- **Grid/snap alignment:** the grid and snap pattern aligns to the coordinate-system origin; rotating the drawing axes rotates crosshairs and grid together. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-FEA6BC6E-D81E-4AD2-BD4C-70078C57709A.htm

### 1.10 Measurement (MEASUREGEOM)

- **One command, six measures:** Quick, Distance (with running totals over multiple points), Radius (arc/circle/polyline-arc radius *and* diameter), Angle (arc centre-vertex, circle, two non-intersecting lines, or an explicit vertex + two points), Area (corner points or object; with add/subtract running modes), Volume (2D region + height, add/subtract). Results appear both at the command prompt and in a tooltip near the cursor, formatted in current units. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm
- **Quick mode.** As the cursor moves over and between objects, dimensions/distances/angles display dynamically; exact right angles get a special marker; clicking inside a region enclosed by geometry highlights it and reports its area and perimeter (including enclosed islands); Shift-click accumulates multiple regions' area/perimeter; Shift-click also deselects; simply moving the mouse a small distance clears the selection. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm
- **Area polygon semantics:** the measured polygon behaves like polyline creation (arc segments, close, undo); an unclosed polygon closes implicitly first-to-last point. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm
- **Mode memory:** a Mode option decides whether the command always restarts in Quick or remembers the last-used sub-option. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm

### 1.11 Units and precision

- **Five length formats:** Decimal, Engineering (feet + decimal inches), Architectural (feet + fractional inches), Fractional, Scientific. Engineering/Architectural assume one drawing unit = one inch; the others are unit-agnostic. Source: https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm
- **Precision** sets displayed decimal places, or the smallest fraction displayed for architectural/fractional formats; a live sample output field previews the chosen combination. Source: https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm
- **Angle formats:** decimal degrees, grads (lowercase `g` suffix), radians (`r`), degrees/minutes/seconds (`d`/`'`/`"` notation), surveyor's bearings (N/S angle E/W, always below 90°, cardinal directions collapse to a single letter). A direction control sets 0° orientation and clockwise-vs-counterclockwise positive direction. Source: https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm
- **Entry is format-tolerant:** values may be typed in the drawing's format (e.g. feet-inch-fraction) while the internal value stays a precise real number; the format governs display and parsing, not stored precision. Sources: https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm , https://autocad-student.com/how-autocad-interprets-units

### 1.12 Command repetition & Enter nuances

- Enter at the "Command:" prompt repeats the previous command (Enter or Spacebar both work). Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-OnBoarding/files/ACD_FOUNDATIONS_MAIN4.html
- Enter at an option prompt accepts the angle-bracketed default. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-BB0C3E79-66AF-4557-9140-D31B4CF3C9CF.htm
- Enter/Spacebar during a grip edit cycles grip modes instead of repeating a command. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
- Enter ends realtime zoom; a right-click can be configured to act as Enter mid-command. Sources: https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm , https://jtbworld.com/autocad-2004

---

## 2) Edge cases & nuances

1. **Zoom is limited while a command is active.** Wheel-zooming during an active command (e.g. Move) stops at a certain scale with the message "Cannot move further while a command is active"; the reachable scale depends on the last view regeneration, which can happen in the background. Zooming is *permitted* mid-command, but unbounded zoom-in is not until a regeneration occurs. Source: https://www.autodesk.com/support/technical/article/caas/sfdcarticles/sfdcarticles/AutoCAD-Zoom-only-works-up-to-a-certain-scale-when-command-is-active.html
2. **Zoom All cannot be used transparently** because it always regenerates the view — the only zoom option explicitly barred from mid-command use. Source: https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm
3. **Transparent dialog changes are deferred.** Settings dialogs opened transparently mid-command do not take effect until the interrupted command completes; a system variable changed transparently likewise applies only from the next command. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-BB0C3E79-66AF-4557-9140-D31B4CF3C9CF.htm
4. **Direct distance entry is blocked while a snap-tracking override key is held** — typing a distance during that override does not produce a tracked point. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm
5. **Dynamic-input coordinate base flips contextually.** First point prompt = absolute; second and later = relative polar (relative Cartesian for rectangle-style commands). `#`, `@`, `*` are one-shot overrides of that default. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm
6. **Pasting into a dynamic prompt** requires typing any character and backspacing it first, otherwise the pasted text is interpreted as drawing input rather than prompt text. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-3EBD4C17-F0A5-49FA-B131-4AABE2E727DB.htm
7. **Angle override vs snaps:** the `<angle` lock overrides grid snap/ortho/PolarSnap but *not* typed coordinates or object snaps — explicit precision wins over the direction lock. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-92BDE481-49D0-4AED-A6C5-0B78051EAE99.htm
8. **Mutual exclusion of direction aids:** ortho vs polar tracking, and PolarSnap vs grid snap, are mutually exclusive pairs; enabling one silently disables the other. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-7EC3C63D-EA4E-4E65-A676-C3A3627E3F19.htm
9. **Point acquisition under pressure:** acquired tracking points appear automatically when hovering; in tight areas, holding Shift temporarily prevents acquisition. Moving the cursor back over an acquired `+` or a `tt` point removes it. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm
10. **Multi-snap ambiguity:** with several running snaps eligible at one location, Tab cycles the candidate markers before committing the pick. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-8F5E5431-9EFB-414E-BC6D-2C65EFB2DAC3.htm
11. **Selection-set lifetime:** the Previous set is destroyed by any operation that deletes its objects, and is ignored after switching model/layout space; fence and polygon selections are immune to the PICKADD modifier setting. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm
12. **Lasso start direction sets the mode,** and Spacebar cycles the lasso mode mid-drag (crossing → window → fence) without releasing the mouse button. Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm
13. **Grip copies reference the original.** Repeated grip-copy angles/distances are always measured from the source object, not the most recent copy — so typing 30, 20, −25 yields rotations of 30/20/−25 from the original, not compounding values. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
14. **Ctrl-held grip snapping arms from the last typed value:** the angle/distance used immediately before pressing Ctrl becomes the snap increment for subsequent Ctrl-copies; releasing Ctrl drops the snap, re-pressing re-arms it at the latest used value. With dynamic input off, a copied offset must be typed with the relative `@` prefix. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
15. **Multi-grip selection can't use a window** — only Shift+click; and releasing Shift returns to normal add behaviour. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
16. **Grip display is suppressed on very large selection sets** (configurable threshold) to protect performance. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
17. **Zoom Extents vs Zoom All differ:** Extents fits all objects including those on turned-off layers but *excluding frozen layers*; All fits the larger of grid limits or extents (so it can show empty space). Source: https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-FC9DE424-67E0-46E8-BCD7-C08B716F392D.htm , https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm
18. **Zoom Window aspect mismatch:** the picked rectangle is stretched to fit the viewport shape, so the final view's aspect follows the window, not the picked rectangle. Source: https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-FC9DE424-67E0-46E8-BCD7-C08B716F392D.htm
19. **Realtime zoom limits are signalled by icon state** (the + or − disappears), and zooming pauses when the pick button is released — resumable anywhere, exit via Enter/Esc. Source: https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm
20. **Quick measure's operating envelope:** it is intended for 2D plan views with the world coordinate system and a 2D wireframe display; annotations such as dimensions and hatches, and custom objects, are not measured. Moving the mouse slightly clears an accumulated area selection. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm
21. **Grid display quirks:** a line-style grid will not display when the snap/grid rotation is non-zero; grid spacing 0 falls back to snap spacing; grid density self-adapts to zoom level. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-FEA6BC6E-D81E-4AD2-BD4C-70078C57709A.htm
22. **Surveyor's angle edge:** angles exactly on a cardinal direction display only the compass letter (no degree text), and bearing angles are always shown below 90°. Source: https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm
23. **Architectural/engineering entry assumes inches per unit**, which silently changes the meaning of a bare typed number versus decimal-format drawings. Source: https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm
24. **Esc has layered meanings:** cancels the active command *and* clears the command input, and separately cancels an in-progress selection set; inside a grip edit it exits the grip action. Sources: https://help.autodesk.com/cloudhelp/2027/ENG/AutoCAD-Core/files/GUID-06F6A6E9-C355-43F5-8706-49F0E5E552BF.htm , https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm
25. **Spacebar ≡ Enter at the command prompt** (both start commands and repeat the last one), but Spacebar inside a grip edit cycles modes, and inside a lasso drag cycles lasso modes. Source: https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-OnBoarding/files/ACD_FOUNDATIONS_MAIN4.html , https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm
26. **Selection preview is capped:** below a large object-count threshold, hover highlighting is suppressed to keep performance predictable (configurable). Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm
27. **Off-screen selection is a setting:** whether objects outside the visible view can be included in selections is controlled by a system variable (default allows it). Source: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm
28. **Recent input is bidirectional:** Up arrow in the command window walks back through *commands and values*; in dynamic input, Up arrow (or a context menu) recalls recent *coordinate* values specifically. Source: https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-A265C30C-CFD8-405D-BEE2-257B379475E2.htm

---

## 3) Keyboard shortcut inventory (drafting-relevant)

Compiled from the official shortcut reference (https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-Customization/files/GUID-B6A9C126-230D-47F5-8973-464BC80E3736.htm) and pages cited above. "Toggle" means a persistent mode switch; "hold" means active only while pressed.

| Key | Action | Notes |
|---|---|---|
| F1 | Help | Context-sensitive when a command is active |
| F2 | Expand command history / text window | Floating line expands up/down; docked opens full text window |
| F3 | Toggle running object snaps | Hold F3 = suppress snaps temporarily |
| F7 | Toggle grid | Also Ctrl+G |
| F8 | Toggle ortho | Also Ctrl+L |
| F9 | Toggle grid snap | Also Ctrl+B |
| F10 | Toggle polar tracking | Also Ctrl+U |
| F11 | Toggle object snap tracking | |
| F12 | Toggle dynamic input | Hold F12 = suppress while a command runs |
| Spacebar | ≡ Enter; repeat last command at idle | Cycles grip modes / lasso modes in their contexts |
| Enter | Accept default; repeat last command at idle | |
| Esc | Cancel command / clear input / cancel selection / exit grip action | |
| Up/Down arrows | Cycle recent commands & values | In dynamic input: recent coordinates |
| Tab | Cycle overlapping object-snap candidates | Also cycles fields in dynamic input; cycles command names when autocomplete off |
| Shift+right-click | Object snap override menu | At a point prompt |
| Shift+letter (E,P,D,L,Q,A,V,M,C…) | Temporary snap/tracking overrides (hold) | e.g. suppress endpoint / center / midpoint / all snapping |
| Shift (during selection) | Remove from selection set | With PICKADD=2 |
| Shift (before grip picks) | Select multiple grips | |
| Ctrl+click (on multifunctional grip) | Cycle grip menu options | |
| Ctrl (after a grip copy value) | Snap subsequent copies to last angle/distance | Hold while clicking |
| Ctrl+W | Toggle selection cycling | For overlapping objects |
| Ctrl+J / Ctrl+M | Repeat last command | |
| Ctrl+Shift+L | Recall previous selection set | Equivalent of `P` at a selection prompt |
| Ctrl+A | Select all (unlocked, unfrozen) | |
| Ctrl+Z / Ctrl+Y | Undo / redo | |
| Ctrl+1 | Properties palette | |
| Ctrl+9 | Toggle command line window | |
| Ctrl+I | Toggle coordinate display | |
| Wheel roll | Zoom toward cursor | Rate/direction configurable |
| Wheel double-click | Zoom extents | |
| Wheel drag | Pan | |

---

## 4) Candidate capabilities for Lintel (behavioural summary + acceptance-criteria idea + URL, clean-room framed)

Each candidate is an original behavioural goal for Lintel written from the public summaries above — no copying of AutoCAD assets, icons, strings, or exact visuals. Acceptance criteria are testable headlessly (guide K.13) where possible.

### C1. Command line with aliases, autocomplete, and in-line options
**Behaviour:** Lintel's command bar completes command names as typed, shows a suggestion list that adapts to usage frequency, accepts single-letter aliases, renders options in brackets selectable by their capital letter, and shows the current default in angle brackets accepted by Enter.
**Acceptance criteria (idea):** typing "li" completes to "line"; alias "l" starts line; at a prompt showing `[Undo/Close] <0.0>` pressing Enter accepts 0.0; typing "u" mid-prompt triggers Undo.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-BB0C3E79-66AF-4557-9140-D31B4CF3C9CF.htm

### C2. Transparent zoom/pan mid-command
**Behaviour:** Zoom and pan remain available at any point prompt inside an active command; when the navigation gesture finishes, the interrupted command resumes at the same prompt with its typed-in-progress values intact.
**Acceptance criteria (idea):** start wall, pick first point, wheel-zoom, wheel-pan; the wall preview is still anchored to the first point; Esc after navigation cancels only the wall, not the view change.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-BB0C3E79-66AF-4557-9140-D31B4CF3C9CF.htm

### C3. Enter/spacebar repeats last command; Esc cancels cleanly
**Behaviour:** With no command active, Enter or Spacebar restarts the last command; Esc cancels the running command and clears any pending selection; a cancelled selection leaves the model untouched.
**Acceptance criteria (idea):** draw line, Enter → line restarts at first-point prompt; Esc during window drag → no objects selected, no model change.
**URL:** https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-OnBoarding/files/ACD_FOUNDATIONS_MAIN4.html

### C4. Coordinate entry grammar: absolute, relative `@`, polar `<`, direct distance, angle override
**Behaviour:** Point prompts accept `x,y`; `@dx,dy` from the last point (even across commands); `dist<angle`; direction-then-typed-distance; and a `<angle` prefix that locks the cursor's direction, overridable by snaps or a full coordinate.
**Acceptance criteria (idea):** after picking (100,100), typing `@50<45` yields (135.355,135.355) to 3 dp; typing `<90` then `5` yields a vertical 5-unit segment regardless of ortho state; typing a coordinate during the angle lock ignores the lock.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-92BDE481-49D0-4AED-A6C5-0B78051EAE99.htm

### C5. Heads-up dimension input with Tab-lock while drawing walls
**Behaviour:** While a wall/line is in progress, two fields near the cursor show length and angle (or dx/dy); typing a value and pressing Tab locks that dimension (visual lock marker) while the other remains live; Enter commits. Defaults are relative from the last point.
**Acceptance criteria (idea):** draw wall, type 3600, Tab (length locks), move cursor, angle field updates live, Enter commits a 3600 mm wall at the cursor's angle; with a locked angle 0, typing 2400 + Enter yields a horizontal 2400 wall.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-A265C30C-CFD8-405D-BEE2-257B379475E2.htm

### C6. Running object snaps with markers, per-point overrides, and Tab cycling
**Behaviour:** A configurable set of persistent snaps (endpoint, midpoint, intersection, perpendicular, nearest, centre, quadrant, extension) shows a distinctive marker + name tooltip when the cursor nears a valid point; Shift+right-click offers a one-shot override snap; when two snaps compete, Tab cycles them.
**Acceptance criteria (idea):** with endpoint+midpoint running, hovering a wall end shows the endpoint marker, Tab swaps to midpoint where eligible; a one-shot perpendicular override applies to exactly the next point.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-8F5E5431-9EFB-414E-BC6D-2C65EFB2DAC3.htm

### C7. Snap tracking (acquired alignment paths) and mid-between-two-points
**Behaviour:** Hovering a snap point acquires it (plus-sign marker); moving away draws horizontal/vertical/extension alignment paths from the acquired point; two acquired points combine; a typed distance along a path lands exactly; typing a mid-between modifier asks for two points and uses their midpoint.
**Acceptance criteria (idea):** acquire the corner of room A, ride the horizontal path over room B's midpoint, type 900 → a door centre lands exactly on that path 900 units from the acquired corner; `mtp` between two jamb points centres a door.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-665DC37F-8C3E-414A-9369-72A13C0BE07A.htm

### C8. Ortho/polar tracking with mutual exclusion
**Behaviour:** Ortho locks to horizontal/vertical; polar tracking locks to configurable angle increments with a distance tooltip along the alignment path; enabling one disables the other; both compose with typed distances.
**Acceptance criteria (idea):** with 45° polar increment, moving the cursor snaps the wall preview onto the 45° path with a live length readout; toggling ortho on turns polar off without a dialog.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-7EC3C63D-EA4E-4E65-A676-C3A3627E3F19.htm

### C9. Grip editing for walls/doors/windows with mode cycling
**Behaviour:** Selecting an object with no command active shows grips; dragging an endpoint grip stretches; Enter/space cycles stretch→move→rotate→scale→mirror; a base-point option re-anchors; a copy option multiplies; hovering certain grips (e.g. a wall midpoint) opens a small contextual action menu; object-specific grip menus differ per type.
**Acceptance criteria (idea):** wall endpoint grip drag with polar on lengthens the wall keeping connectivity; clicking the grip and pressing Enter cycles to Move; a door's hinge-side grip menu offers flip actions.
**URL:** https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm

### C10. Selection grammar: implied window/crossing, fence, lasso, Shift/Ctrl set edits
**Behaviour:** Click-drag in empty space selects by direction (left→right = inside-only, right→left = touched-too) with visually distinct boundaries; typed verbs `W/P/F/C` (window, polygon, fence, crossing) exist at selection prompts; Shift removes; Previous recalls the last set; a lasso freeform drag cycles modes with spacebar.
**Acceptance criteria (idea):** a crossing window touching a wall mid-span selects that wall; a fence stroked across three rooms selects every wall/door it crosses; after deleting a set, `P` at the next prompt does not resurrect deleted objects.
**URL:** https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-MAC-Core/files/GUID-0DD5DA73-9DC5-4424-8FED-7BBE3BE52A4D.htm

### C11. Property-based quick select
**Behaviour:** A quick-select panel filters the model by object type + one property + operator (equals, not-equals, wildcard), applicable to the whole plan or the current selection, producing a live selection set.
**Acceptance criteria (idea):** select all doors on layer "Doors-Exterior" via type=door, property=layer, operator=equals; apply to current selection to narrow a prior window selection.
**URL:** https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-D0D5C0C3-F092-448A-8E81-D38F27094639.htm

### C12. Wheel zoom-to-cursor, wheel-drag pan, double-click extents
**Behaviour:** Rolling the wheel zooms anchored at the cursor; dragging the wheel pans; double-clicking the wheel fits all content; zoom rate is configurable.
**Acceptance criteria (idea):** with the cursor over a room corner, each wheel notch keeps that corner fixed under the cursor; double-click fits the whole floor plan; all gestures work mid-command.
**URL:** https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-LT-DidYouKnow/files/GUID-10D3C9A9-22E8-4047-AED1-5E96C6C28866.htm

### C13. Quick measure
**Behaviour:** A measure mode that, as the cursor moves, shows distances between nearby objects and per-object dimensions; clicking an enclosed region highlights it and reports area + perimeter, with additive region selection.
**Acceptance criteria (idea):** hovering between two parallel walls shows the clear width; clicking a room highlights it and reports floor area and perimeter in current units; shift-clicking a second room sums both areas.
**URL:** https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm

### C14. Architectural units & precision
**Behaviour:** Length display formats include decimal and feet-inches-fractional with selectable precision down to a small fraction; a live sample preview shows the format; typed entry accepts the drawing's format.
**Acceptance criteria (idea):** with architectural precision 1/16, a 2743.2 mm length displays as 9'-0"; typed `9'6-1/4` parses to the same internal value regardless of display format.
**URL:** https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm

### C15. Grid as visual reference with adaptive density
**Behaviour:** A non-printing grid whose density adapts to zoom (thins when zoomed out, subdivides when zoomed in) with major/minor line distinction; independent snap spacing can be finer than the visible grid.
**Acceptance criteria (idea):** zooming out repeatedly never exceeds a set grid-line count on screen; a 100 mm snap with a 500 mm grid places points off-grid.
**URL:** https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-FEA6BC6E-D81E-4AD2-BD4C-70078C57709A.htm

---

## 5) Not-applicable items (with reasons)

| Item | Reason not applicable to Lintel |
|---|---|
| 3D object snaps, dynamic UCS, isoplanes, 3D subobject selection (F4/F5/F6, Shift+F1–F5) | Lintel is strictly 2D; no 3D coordinate system or solid subobjects exist. Source set: https://help.autodesk.com/cloudhelp/2024/ENU/AutoCAD-Customization/files/GUID-B6A9C126-230D-47F5-8973-464BC80E3736.htm |
| Paper-space `xp` zoom scaling and layout viewports | Lintel has no paper-space/layout model in scope; the *concept* of plot-scale views belongs to a future output slice, not core drafting. https://help.autodesk.com/cloudhelp/2019/ENU/AutoCAD-Core/files/GUID-66E7DB72-B2A7-4166-9970-9E19CC06F739.htm |
| Insertion scale / block unit auto-scaling and photometric lighting units in UNITS | Floor-plan blocks in Lintel are same-scale by definition; lighting units are 3D-rendering concerns. https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm |
| Apparent-intersection snap | Defined for 3D views where objects visually cross but don't truly intersect; in a 2D plan, visual crossing *is* intersection. https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-CF5780AD-D1AB-4526-9608-83D7952749E7.htm |
| Parallel snap constraint | Less valuable for floor plans (walls are orthogonal or explicitly angled via polar/ortho); keep as backlog if free-form sketching is added. https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-CF5780AD-D1AB-4526-9608-83D7952749E7.htm |
| Dynamic-block custom grips and grip tips | AutoCAD's parametric block authoring system; Lintel's blocks/doors/windows can use plain geometric grips plus fixed menus instead. https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-DidYouKnow/files/GUID-BBEA1F71-EB16-4D49-80D9-970A6909F508.htm |
| Touchpad-gesture navigation, tablet mode, multiple tiled viewports | Platform/input niceties outside core drafting; egui gesture handling is its own slice. https://help.autodesk.com/cloudhelp/2015/ENU/AutoCAD-Core/files/GUID-FC9DE424-67E0-46E8-BCD7-C08B716F392D.htm |
| The REGEN-limited zoom ceiling during commands | This is an AutoCAD implementation artifact of its display-list architecture, not a desirable behaviour; Lintel's renderer should zoom freely mid-command and is only noted as the behaviour floor-plan users may expect to *not* hit. https://www.autodesk.com/support/technical/article/caas/sfdcarticles/sfdcarticles/AutoCAD-Zoom-only-works-up-to-a-certain-scale-when-command-is-active.html |
| Volume measurement | Meaningful only with heights/solids; Lintel measures plan area and lengths. https://help.autodesk.com/cloudhelp/2026/ENU/AutoCAD-Core/files/GUID-5D5B0EE1-DD90-47AE-8A55-642FBFF5E4E4.htm |
| Surveyor's bearing angle format | Land-surveying convention; architectural plans use decimal degrees / DMS. Defer unless a survey-import feature appears. https://help.autodesk.com/cloudhelp/2025/ENU/AutoCAD-Core/files/GUID-75419F91-18B8-47EE-9272-3196ACC95977.htm |
| Multi-user content search in the command line (styles, hatches) | Partially applicable: Lintel can search layers/blocks/room names in its command bar, but hatch-style and dim-style search has no equivalent yet. https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-74478DCE-92D4-49B1-93E9-DCF09D57FA0D.htm |
| AutoCorrect/synonym learning files | Nice-to-have; a fixed alias table plus frequency-ordered suggestions delivers most of the value for Lintel's small command set. https://help.autodesk.com/cloudhelp/2027/ENU/AutoCAD-Core/files/GUID-74478DCE-92D4-49B1-93E9-DCF09D57FA0D.htm |

---

*End of note. All summaries original; all claims traceable to the cited public URLs.*
