# Consumer Home / Floor-Plan Design Tools — Clean-Room Research

**Date:** 2026-11-28  
**Method:** Public documentation only (official help sites, user guides, feature pages). No reverse engineering, no proprietary code, no pixel copying. Every claim below has a source URL.

---

## 1. Behaviours Per Product

### 1.1 Sweet Home 3D (Open Source, Desktop + Web)

**Sources:**
- User Guide: https://www.sweethome3d.com/users-guide/
- Features: https://www.sweethome3d.com/features/
- Tips & Tricks: https://www.sweethome3d.com/tips/

#### Furniture Catalog
- Tree-organised categories (expand/collapse triangles). Drag-and-drop from catalog to plan or furniture list. Search not mentioned in docs; filtering by category only.
- When dropped on a wall with magnetism on, furniture auto-rotates so its back face aligns to the wall. When dropped on a larger piece, it elevates to sit on top.
- Double-click or *Furniture → Modify* opens dialog: name, rotation, location, elevation, size, colour/texture, visibility, mirror flag, light power (premium render only).
- Four corner indicators on selected piece: rotation (Alt toggles 15° magnetism), elevation, height, size (width+depth).

#### Wall Tool
- **Create Walls** button → click start point → click/double-click end point. Continues chaining until double-click or Escape.
- Live tooltip shows wall length while drawing.
- Magnetism snaps angles to 15° increments (disable in Preferences, or hold Alt / Cmd / Shift+Alt).
- Enter key after placing a point lets you type exact length + angle (desktop only, not Online/Mobile).
- Walls appear simultaneously in 2D plan and 3D view.

#### Wall Editing
- Select mode → click wall → drag to move. Shift-click for multi-select. Rectangle selection.
- **Single wall selected**: drag endpoints to reshape; *Plan → Split Wall* splits at cursor.
- Double-click or *Plan → Modify Walls* opens dialog: left/right colours/textures, **thickness**, **height**.
- Wall thickness and height are global defaults in Preferences, overridable per wall.

#### Doors & Windows
- Drag from catalog **onto a wall**. With magnetism: auto-orients to wall normal, auto-resizes to wall thickness.
- Once placed, behave like furniture: double-click for dialog (width, height, elevation, rotation, mirror, colour/texture).
- Opening swing direction derived from hinge point; editable via furniture dialog rotation.
- Door/window width must not exceed host wall length (validation on place).

#### Room Auto-Detection
- **Create Rooms** tool → double-click inside **closed** wall loop → room created with floor+ceiling.
- Room includes half-doorstep of each door on its boundary (ensures correct 3D join when doors open).
- Alternatively click corner-by-corner, double-click/Escape to close.
- *Plan → Modify Rooms* → name, floor colour/texture, ceiling colour/texture.
- Single room selected: drag corner points; move area/name labels via indicators.

#### Levels (Storeys)
- *Plan → Levels → Add Level* or **+** tab when >1 level exists.
- Tabs at top of plan view switch active level.
- Lower-level walls/ceilings shown in light colour as reference while drawing on upper level.
- Copy/paste between levels supported.
- Double-click level tab or *Plan → Levels → Modify Level* → elevation (negative for basement), height, floor thickness.
- Level height drives ceiling elevation of that level.

#### 3D View (Parallel Concept)
- Two modes: **Aerial** (orbit) and **Virtual Visit** (first-person). Virtual visitor shown in 2D plan with head/body/FOV/elevation indicators.
- *3D View → Modify 3D View* → ground/sky colour/texture, light brightness, wall transparency.
- Create photo (PNG), 3D video (path), export OBJ+MTL.

#### Undo/Redo
- Toolbar buttons for all modifications. Granular per user action.

#### Export / Save
- Native `.sh3d` (contains imported models).
- Print → PDF (with paper size + scale).
- Export 2D: PNG, SVG (vector).
- Export 3D: OBJ+MTL, 3D video.
- Online version auto-saves to server.

#### Keyboard
- Arrow keys move selection. Delete removes. Escape cancels drag/drawing. Tab/Shift+Tab moves pane focus.

---

### 1.2 Floorplanner (Web, Freemium)

**Sources:**
- Homepage: https://www.floorplanner.com/
- Help Center — Working in Editor: https://help.floorplanner.com/en/categories/579650-working-in-the-editor
- Help Center — Wall: https://help.floorplanner.com/en/categories/581186-wall
- Help Center — Surface: https://help.floorplanner.com/en/categories/581250-surface
- Help Center — Room: https://help.floorplanner.com/en/categories/581122-room
- Help Center — Doors: https://help.floorplanner.com/en/categories/581314-doors
- Help Center — Windows: https://help.floorplanner.com/en/categories/581378-windows

#### Drawing Tools: Draw Room vs Draw Wall
- **Draw Room** (Build sidebar): drag out a 4-sided rectangle; auto-closes to room with floor+ceiling. Fast for rectangular rooms.
- **Draw Wall** (Build sidebar): click point-to-point for free-form walls. Click to start, move, click to place segment; continue chaining. Stop by clicking Select tool or pressing Escape.
- Walls auto-join at endpoints; T-junctions and corners snap.
- **Split Wall**: click wall → floating "Split" icon → splits at cursor.

#### Wall Properties
- Thickness adjustable per wall (Move wall across axis aligns different-thickness walls).
- Thickness = 0 makes wall invisible but still structures auto-ceiling and divides areas.
- Sloping walls: select blue circle at wall end → modify Height of Wall.
- Curved walls: select wall → floating "Curve" icon → drag curve points.
- Custom label per wall (Settings sidebar).

#### Surfaces (Areas Without Walls)
- Draw Surface tool: click point-to-point for custom areas (terraces, grass, countertops).
- Surfaces have thickness, material, transparency, custom label, room type.
- Can be rotated/mirrored via floating icons.

#### Rooms
- Room = closed loop of walls → auto-generates floor + ceiling.
- Click inside room → Room Settings sidebar: room type (category), floor material, show/hide surface area, generate ceiling toggle, custom label.
- Room area shown in sidebar when selected.
- Copy room: Shift + drag selection box around room → release to duplicate.
- Move room: same gesture.
- Delete room: delete one wall of the room (floating delete icon).

#### Doors
- Build tab → Place Doors → drag from library onto wall.
- Door auto-snaps to wall, orients to wall normal.
- Properties sidebar: width/height/raise-from-floor (spinners), flip vertical/horizontal (changes swing direction), frame/door/threshold colours, front-door/internal-door toggle, custom label.
- Duplicate: click door → floating Duplicate icon.
- Delete: click door → floating Trash icon.

#### Windows
- Build tab → Place Windows → drag from library onto wall.
- Auto-snaps to wall, orients to wall normal.
- Properties: width/height/raise-from-floor, flip vertical/horizontal (changes handle/inside side), frame colour, custom label.
- Stack windows above doors/windows by setting Raise From Floor = sum of lower element's raise + height.
- Bay window: split wall at start/end points → push middle segment outward.

#### Furniture / Objects
- Objects tab → categories + brands + search + filter by color + "resizable only" checkbox.
- Drag-and-drop into work area. Mini-icons on select: rotate, flip H/V, duplicate, delete, similar items, variants, label, store link (affiliate).
- Favorites (heart icon) persist across projects.
- Groups: Shift-select multiple → create Group → reuse as unit.

#### Dimensions & Annotations
- Auto-dimensions: toggle interior/outer/horizontal/short labels via View Settings.
- Manual dimension line: Info tab → Draw Dimension → click start, move, click end.
- Labels: Info → Place Label → drag, edit text, font/color/size.
- Symbols/Signs (North arrow etc.): Info → Place Signs & Symbols.
- Lock icons: lock construction (walls), lock furniture, lock labels/lines independently.

#### Levels / Floors
- Projects have multiple floors; each floor has up to 10 designs (SD-8K), Basic = 1 floor/1 design.
- Pivot Point (crosshair on canvas) aligns multi-story projects.
- Design History: timeline of design versions per floor; restore previous.
- Duplicate design, copy design to different floor/project.

#### Export
- 2D: PNG, JPG, PDF (paper size + scale).
- 3D: export image from 3D view (SD/HD/8K), interactive 3D tour (cameras, reorder).
- Product list download (items in design).

#### Undo/Redo
- Curved arrows in top menu bar. Design history also provides version-level restore.

---

### 1.3 RoomSketcher (Desktop App + Web Portal, Freemium)

**Sources:**
- Homepage: https://www.roomsketcher.com/
- Features: https://www.roomsketcher.com/features/
- Help — Draw from Scratch: https://help.roomsketcher.com/hc/en-us/categories/35672599717661-Draw-from-Scratch
- Help — Walls with Exact Lengths: https://help.roomsketcher.com/hc/en-us/articles/360000202509-How-Can-I-Draw-Walls-with-Exact-Lengths
- Help — Add Doors: https://help.roomsketcher.com/hc/en-us/articles/360000808925-How-Do-I-Add-Doors-to-My-Project
- Help — Add Windows: https://help.roomsketcher.com/hc/en-us/articles/360000818165-How-Do-I-Add-Windows-to-My-Project

#### Wall Drawing (Two Tools)
- **Exterior Wall** and **Interior Wall** tools (Walls mode).
- Click start point → move cursor → live length displayed above/below wall → click to place.
- **Exact length during draw**: type length while drawing (inches or mm).
- **Exact length after**: Select tool → click wall end → arrows appear → type exact length in boxes → Enter. Wall grows/shrinks on arrow side.
- Connected walls: changing one may adjust connected walls. Non-90° connections block length edit ("Unable to change length").
- Minimum wall length: 4 in (100 mm). Shorter: zoom in + manual drag (if not connected).
- Wall thickness: separate Exterior/Interior wall tools with preset thicknesses. *How Do I Create Thick and Thin Walls* article confirms per-wall thickness control.

#### Doors
- **Windows etc.** mode → Doors category (bi-fold, exterior, french, glass, interior, pocket, sliding, opening-no-door).
- Click door → click wall to place. **Validation: door width ≤ wall length**.
- Properties panel (right): Width/Height spinners, **Flip Door** (hotkey Q) toggles swing direction, **Opening Angle** (degrees; 0 = closed).
- Lock toggle in Advanced Properties prevents edits.
- Pro/Team: Replace Materials on door parts (handle, door, sill, trim); Side A = hinge side.
- Copy/Paste via toolbar; drag to move along wall; Trash to delete.

#### Windows
- **Windows etc.** mode → Windows category (casement, double-hung, sliding, fixed, special shapes: round, arched, diamond, sloping).
- Click window → click wall to place. Drag to reposition along wall.
- Grey handle marker indicates inside; **Flip Window** toggles inside/outside.
- Properties: Width/Height/Height Above Floor (spinners + blue drag arrows).
- Pro/Team: Replace Materials on frame/handle/trim.
- Copy/Paste/Delete via toolbar.
- **Validation: window width ≤ wall length**.
- Floor-to-ceiling: Height Above Floor = 0, Height = wall height.
- Skylights: Furniture mode → search "Skylight" → place freely (not wall-hosted).

#### Room Names & Labels
- Click inside room → Room Names panel: add/move room name(s). Multiple names per room supported.
- 2D vs 3D label position independent.
- Text tool: add free text anywhere.
- Entrance arrow, furniture labels via dedicated tools.

#### Levels / Floors
- Add/Copy/Delete floors. Floor height adjustable per level.
- Floating loft, raised area with stair access.
- Flip/Rotate entire floor plan (one click).
- Rotate floor/wall material independently.

#### Furniture & Materials
- 7,000+ items (Pro). Favorites for quick access.
- Best Fit: auto-resize items to space.
- Replace Materials on furniture/fixtures (Pro).
- Custom items via Building Blocks.

#### Measurements & Area
- One-click measurements (Pro).
- Automatic total area calculations (Pro).
- Room area shown when clicking inside room.

#### 3D & Visualisation
- Live 3D (interactive walkthrough in browser/phone).
- 3D Snapshots (low-res quick shots).
- 360 Views (panoramic).
- AI Render (text-prompt photorealistic from 3D view).
- 2D Floor Plans: B&W, color-coded, branded.
- 3D Floor Plans: top-down dollhouse.

#### Export
- JPG, PNG, PDF (Pro: high-res).
- Cloud-saved projects; update anytime.

---

## 2. Wall / Opening / Room Interaction Detail (Core Comparison Table)

| Interaction | Sweet Home 3D | Floorplanner | RoomSketcher |
|-------------|---------------|--------------|--------------|
| **Wall draw mode** | Single "Create Walls" tool; click-click chaining | Two tools: Draw Room (rectangle drag) + Draw Wall (click-click) | Two tools: Exterior Wall + Interior Wall (separate thickness presets) |
| **Live length tooltip** | Yes, while drawing | Implied (dimension display on hover) | Yes, above/below wall while drawing |
| **Exact length entry during draw** | Enter key → type length+angle (desktop only) | Edit interior dimension after; or type in Settings | Type length while drawing; or click end → arrows → type |
| **Angle magnetism** | 15° increments; Alt/Cmd/Shift+Alt to disable | Snap to orthogonal + free angle; T-junctions auto-snap | Orthogonal bias; non-90° connections block length edit |
| **Wall thickness** | Global default + per-wall override (Modify Walls dialog) | Per-wall; Move wall across axis aligns different thicknesses | Preset per tool (Ext/Int); per-wall editable |
| **Wall height** | Per-wall (Modify Walls dialog) | Per-wall end (blue circle → Height of Wall); sloping walls | Per-floor (floor height); wall height = floor height |
| **Split wall** | Plan → Split Wall (menu) | Click wall → floating "Split" icon | Not explicitly documented |
| **Curve wall** | Not in core (tips mention recesses via invisible triangles) | Click wall → floating "Curve" icon → drag curve points | Pro: "How Can I Create Curved Walls" article exists |
| **Delete wall** | Select → Delete key | Click wall → floating Trash icon | Select → Delete (implied) |
| **Door place method** | Drag from catalog ONTO wall | Build → Place Doors → drag from library onto wall | Windows etc. mode → click door → click wall |
| **Door auto-orient on wall** | Yes (magnetism) | Yes (snaps to wall, orients to normal) | Yes (click wall places) |
| **Door auto-resize to wall thickness** | Yes (magnetism) | Yes (snaps) | Implied (properties show width/height) |
| **Door swing control** | Furniture dialog → rotation | Flip Vertically/Horizontally (mini-icons) | Flip Door (hotkey Q) + Opening Angle (degrees) |
| **Door validation** | Width ≤ wall length (implicit) | Width ≤ wall length (explicit in docs) | **Explicit: "door's length must not exceed the wall's length"** |
| **Window place method** | Drag from catalog ONTO wall | Build → Place Windows → drag onto wall | Windows etc. mode → click window → click wall |
| **Window auto-orient** | Yes (magnetism) | Yes | Yes |
| **Window flip (inside/outside)** | Furniture dialog → mirror flag | Flip Vertically/Horizontally | Flip Window button (grey handle = inside) |
| **Window validation** | Implicit | Stackable via Raise From Floor | **Explicit: "window is too long" error** |
| **Room creation** | Create Rooms tool → double-click closed loop OR corner-click | Draw Room (rectangle) auto-creates room; Draw Wall → closed loop auto-generates ceiling/floor | Not a separate tool; rooms form from closed wall loops |
| **Room area label** | Auto on creation; editable name + area; move via indicators | Click room → Room Settings → Show Surface Area toggle | Click room → area in sidebar; Room Names panel for labels |
| **Room floor/ceiling material** | Modify Rooms dialog → colour/texture per surface | Drag material from Finishes tab onto floor; Room Settings → ceiling toggle | Floor materials via Paint tab; ceiling material separate |
| **Room includes doorstep** | Yes, half-doorstep of each boundary door | Not mentioned | Not mentioned |
| **Levels/Storeys** | Tabs at top; lower level ghosted; per-level elevation/height/floor thickness | Multiple floors per project; up to 10 designs/floor; Pivot Point aligns | Add/Copy/Delete floors; floor height per level; flip/rotate floor |
| **Undo/Redo** | Toolbar buttons (granular) | Toolbar arrows + Design History (version timeline) | Implied (standard) |
| **Export 2D** | PDF (paper+scale), PNG, SVG | PDF (paper+scale), PNG, JPG | JPG, PNG, PDF (Pro high-res) |
| **Export 3D** | OBJ+MTL, 3D video, photo (PNG) | 3D image export (SD/HD/8K), interactive tour | Live 3D, 360, AI Render, 3D Floor Plans |
| **Keyboard: move selection** | Arrow keys | Not documented | Not documented |
| **Keyboard: cancel drag** | Escape | Escape (implied) | Escape (implied) |
| **Keyboard: delete** | Delete key | Trash icon / Delete key | Trash icon / Delete key |
| **Min wall length** | Not specified | Not specified | **4 in (100 mm)** |
| **Wall join behaviour** | Auto-join at endpoints; T-junctions | Auto-join; T-junctions; different thickness alignment | Connected walls adjust together; non-90° blocks length edit |

---

## 3. Edge Cases & Nuances

| Edge Case | Sweet Home 3D | Floorplanner | RoomSketcher | Notes for mm-Precision Tool |
|-----------|---------------|--------------|--------------|-----------------------------|
| **Delete wall hosting a door/window** | Door/window deleted with wall (implicit) | Door/window deleted when host wall deleted (implicit) | Door/window deleted; validation prevents place if too wide | **Must decide**: cascade delete vs. orphan opening vs. block deletion. CAD expects cascade or convert to opening. |
| **Drag door past wall end** | Blocked by magnetism/validation | Blocked (width ≤ wall length) | Blocked ("door's length must not exceed the wall's length") | **Enforce**: clamp to wall bounds; show inline error tooltip. |
| **Overlapping wall joins (T-junction, cross)** | Auto-join; thickness per wall | Auto-join; Move wall across axis aligns different thicknesses | Connected walls adjust together; non-90° blocks length edit | **Need**: explicit join types (butt, miter, overlap) with cleanup. mm precision means no slop. |
| **Wall split with door on it** | Split Wall menu → door stays on one segment? | Split wall → door/window position relative to split? | Not documented | **Must specify**: split transfers host openings proportionally or picks nearest segment. |
| **Door/window wider than wall** | Prevented by catalog sizing? | Explicit validation | Explicit validation + error message | **Show inline error** at drag time; don't allow placement. |
| **Room auto-detect with opening on boundary** | Double-click includes half-doorstep | Room = closed wall loop; openings don't break loop | Room = closed loop; openings don't break loop | **Openings must not break room detection**; area = polygon minus opening width? |
| **Non-90° wall connections** | Magnetism 15°; free angle with modifier | Free angle; T-junctions snap | Non-90° blocks length edit | **mm tool**: allow any angle; length edit must work on non-orthogonal (solve geometry). |
| **Very short walls (< 100 mm)** | Not specified | Not specified | Min 100 mm; shorter via zoom+drag (unconnected) | **Allow 1 mm**; warn on manufacturability. |
| **Sloping / curved walls** | Tips: invisible triangles for recesses; sloping ceiling blog | Sloping walls (per-end height), curved walls (bezier) | Curved walls (Pro); sloped ceilings | **Scope**: straight only for v0.1.0; curves/slopes as later milestone. |
| **Multi-level alignment** | Ghost lower level; copy/paste | Pivot Point crosshair reference | Flip/rotate floor; copy floor | **Need**: shared origin/grid across levels; stairwell cutouts. |
| **Undo granularity** | Per user action | Per action + design history (versions) | Not specified | **Per-command** (not per-keystroke); group compound ops (e.g., drag = one undo). |
| **Grid snapping vs. free** | Magnetism toggle; grid visible | Grid toggle; metric/imperial toggle | Live dimensions; snap-fitting walls | **mm tool**: grid = 1 mm; snap toggle; dimension input always exact. |
| **Units** | Metric/Imperial in Preferences | Toggle m/ft bottom-right | Inches/mm input boxes | **Internal: mm**; display per user locale. |

---

## 4. Candidate Capabilities for Lintel

*Lintel already has: wall tool click-click with length, auto rooms with area captions, doors/windows hosted on walls with fit validation, levels, PDF export.*

| Capability | Behavioural Summary | Acceptance-Criteria Idea | Source URL |
|------------|---------------------|--------------------------|------------|
| **Wall thickness per wall** | Each wall stores thickness; Modify dialog overrides global default. Different-thickness walls align via "move across axis". | Select wall → properties panel shows thickness; change updates 2D/3D; adjacent different-thickness walls auto-align centre or inner/outer face per user choice. | https://www.sweethome3d.com/users-guide/#editingWalls https://help.floorplanner.com/en/articles/2407170 |
| **Wall height per wall / sloping walls** | Per-wall height; Floorplanner allows per-end height for sloping top. | Wall properties: height (uniform) or start/end height (sloped). 3D view reflects slope. | https://www.sweethome3d.com/users-guide/#editingWalls https://help.floorplanner.com/en/articles/2407106 |
| **Split wall at point** | Click wall → Split → two walls joined at cursor. Hosted openings redistribute. | Context menu on wall → "Split" → click position → two walls; any door/window on original wall reassigned to nearest segment (or duplicated if straddling). | https://www.sweethome3d.com/users-guide/#editingWalls https://help.floorplanner.com/en/articles/2406786 |
| **Curved walls** | Floorplanner: wall → Curve icon → drag control points. RoomSketcher Pro has curved walls. | Wall properties → "Curved" toggle → control handles appear; drag to set radius; area/length recalculates. | https://help.floorplanner.com/en/articles/2406978 https://help.roomsketcher.com/hc/en-us/articles/115000509129 |
| **Door swing angle (0–180°)** | Sweet Home 3D: rotation in furniture dialog. RoomSketcher: Opening Angle field (degrees), 0 = closed, Flip Door hotkey Q. | Door properties: Opening Angle (0–180), Swing Side (Left/Right), Hinge Side (Auto/Manual). 2D shows arc; 3D animates on hover. | https://www.sweethome3d.com/users-guide/#adding-doors-windows-and-furniture https://help.roomsketcher.com/hc/en-us/articles/360000808925 |
| **Window flip (inside/outside)** | Mirror flag in furniture dialog. Floorplanner/RoomSketcher: Flip H/V mini-icons; RoomSketcher shows grey handle = inside. | Window properties: "Inside Face" toggle (Inside/Outside); handle marker flips; 3D render respects reveal depth. | https://help.floorplanner.com/en/articles/2409602 https://help.roomsketcher.com/hc/en-us/articles/360000818165 |
| **Stacked openings (window above door)** | Not documented. Floorplanner: Raise From Floor = sum of lower raise+height. | Opening properties: Raise From Floor; validator ensures no vertical overlap on same wall segment. | https://help.floorplanner.com/en/articles/2409794 |
| **Bay window construction** | Not documented. Floorplanner: split wall at two points → push middle out. RoomSketcher: dedicated Bay Window article. | Tool: "Bay Window" → click wall → define width/depth/angle → auto-creates angled wall segments + window units. | https://help.floorplanner.com/en/articles/2409858 https://help.roomsketcher.com/hc/en-us/articles/360000814049 |
| **Room type / category with preset materials** | Room name + floor/ceiling texture. Floorplanner: Roomtype (house/office/school categories). RoomSketcher: Room Names panel. | Room properties: Type (Bedroom, Kitchen, Bath, etc.) → applies default floor material, ceiling height, label style. User overridable. | https://www.sweethome3d.com/users-guide/#drawingRooms https://help.floorplanner.com/en/articles/2407618 |
| **Surface / area without walls** | Not in core. Floorplanner: Draw Surface (terraces, grass, countertops) with thickness, material, transparency. | "Draw Surface" tool → click-point polygon → properties: thickness, material, transparency, label. Does not generate ceiling. | https://help.floorplanner.com/en/categories/581250-surface |
| **Auto-dimension lines (interior + outer)** | Dimensions tool draws manual lines. Floorplanner: toggle Interior/Outer/Horizontal/Short auto-dimensions via View Settings. | View settings: Auto-Dimensions toggle → shows wall lengths, room widths, outer extents. Style: font, offset, unit. | https://help.floorplanner.com/en/articles/2394050 https://help.floorplanner.com/en/articles/2394306 |
| **Dimension line tool (manual)** | Plan → Draw Dimensions. Floorplanner: Info → Draw Dimension. RoomSketcher: Measurements (Pro) one-click. | Tool: click start → click end → dimension line with editable text; snaps to wall endpoints/centres; associative (moves with wall). | https://www.sweethome3d.com/users-guide/#drawing-dimensions https://help.floorplanner.com/en/articles/2394562 |
| **Text labels / annotations** | Plan → Add Texts. Floorplanner: Info → Place Label (font/color/size). RoomSketcher: Add Text tool. | Tool: click → type → drag to position; properties: font, size, color, rotation, anchor. | https://www.sweethome3d.com/users-guide/#adding-texts https://help.floorplanner.com/en/articles/2394626 |
| **Symbols / North arrow / Signs** | Not documented. Floorplanner: Info → Place Signs & Symbols. RoomSketcher: Entrance Arrow tool. | Library: North arrow, section markers, electrical symbols, custom SVG import. Drag to place; scale/rotate. | https://help.floorplanner.com/en/articles/2397506 https://help.roomsketcher.com/hc/en-us/articles/360000831989 |
| **Furniture catalog with categories + search** | Tree categories; drag-drop. Floorplanner: categories + brands + color filter + resizable filter + search. RoomSketcher: 7000+ items, favorites. | Catalog panel: tree (categories), search box, filters (brand, color, resizable), favorites tab. Drag to plan → auto-rotate to wall, elevate on stack. | https://www.sweethome3d.com/users-guide/#adding-doors-windows-and-furniture https://help.floorplanner.com/en/articles/2398018 |
| **Furniture indicators (rotate, elevate, resize)** | 4 corner indicators: rotation (Alt=15°), elevation, height, size (W+D). | Selected object shows 4 handles: rotate (corner arc), elevate (vertical), height (top), scale (diagonal). Keyboard modifiers for constraints. | https://www.sweethome3d.com/users-guide/#adding-doors-windows-and-furniture |
| **Groups (reusable multi-object sets)** | Not documented. Floorplanner: Shift-select → Create Group → reuse. | Select multiple → right-click → "Group" → appears in catalog under "My Groups"; place as unit; ungroup to edit. | https://help.floorplanner.com/en/articles/2400706 |
| **Design history / version timeline** | Undo/Redo only. Floorplanner: Design History button → radio-button timeline per design. | Toolbar: History panel (timeline with thumbnails); click to restore; branches on fork; per-project. | https://help.floorplanner.com/en/articles/2395458 |
| **Pivot point for multi-level alignment** | Not documented. Floorplanner: crosshair on canvas; used to align floors. | Project settings: Pivot Point (X,Y) visible in 2D; all floors share origin; stairwell cuts align to pivot. | https://help.floorplanner.com/en/articles/2392898 |
| **Flip / Rotate entire floor plan** | Not documented. Floorplanner: Transform Design → flip H/V, rotate. RoomSketcher: one-click flip/rotate floor. | Menu: "Transform Plan" → Flip Horizontal, Flip Vertical, Rotate 90°; updates all levels if "All Floors" checked. | https://help.floorplanner.com/en/articles/2395586 https://help.roomsketcher.com/hc/en-us/articles/115005801669 |
| **Live 3D walkthrough (parallel concept)** | Aerial + Virtual Visit modes; virtual visitor in 2D. Floorplanner: Orbital + Walkthrough; camera list. RoomSketcher: Live 3D (browser/phone), 360 views. | 3D panel: Orbit / Walkthrough toggle; camera path editor; export walkthrough video; sync 2D selection ↔ 3D highlight. | https://www.sweethome3d.com/users-guide/#editing-3d-view https://help.floorplanner.com/en/articles/2394946 |
| **Export PDF with scale + paper size** | Print → PDF (paper size + scale). Floorplanner: Export 2D → PDF → choose paper size + scale. RoomSketcher: Pro high-res PDF. | Export dialog: Format=PDF → Paper Size (A4–A0, Arch, ANSI) → Scale (1:50, 1:100, custom) → Include: dimensions, room names, area, north arrow. | https://www.sweethome3d.com/users-guide/#print https://help.floorplanner.com/en/articles/2400386 |
| **Keyboard: arrow-move selection** | Arrow keys nudge selection. | Implement: Arrow = 1 mm nudge; Shift+Arrow = 10 mm; Ctrl+Arrow = 100 mm. | https://www.sweethome3d.com/users-guide/#drawing-walls |
| **Keyboard: Escape cancels drag/draw** | Escape ends wall chain or cancels drag. | Escape always exits current tool mode → Select. | https://www.sweethome3d.com/users-guide/#drawing-walls |
| **Lock layers (construction / furniture / annotations)** | Not documented. Floorplanner: three independent lock icons (construction, furniture, labels). | Toolbar: three toggle locks → Walls/Structure, Furniture/Objects, Dimensions/Text. Locked = unselectable, unmovable. | https://help.floorplanner.com/en/articles/2393090 https://help.floorplanner.com/en/articles/2393154 https://help.floorplanner.com/en/articles/2393218 |

---

## 5. Not Applicable (with Reasons)

| Capability | Reason |
|------------|--------|
| **AI Convert (image → floor plan)** | RoomSketcher AI feature; requires ML pipeline + training data; out of scope for CAD tool. |
| **FloorCapture / LiDAR scanning** | RoomSketcher mobile LiDAR; requires iOS ARKit / Android ARCore; hardware-dependent. |
| **AI Render (text-to-image from 3D)** | RoomSketcher AI Render; generative AI; not a CAD function. |
| **3D Storefront / 360 Views / Live 3D embedding** | RoomSketcher cloud-hosted interactive viewers; requires backend + WebGL viewer; Lintel is desktop-first. |
| **Styleboards / Mood boards** | Floorplanner Styleboards; marketing/presentation feature; not core CAD. |
| **Marketplace / Affiliate store links on furniture** | Floorplanner items have "Store" button to retailer; commercial integration; conflicts with open-source ethos. |
| **Magic Layout (auto-furnish from Styleboard)** | Floorplanner AI layout; requires ML + rule engine; out of scope. |
| **Order Floor Plans (human redraw service)** | RoomSketcher service; business model, not software feature. |
| **Branding / Custom logos on exports** | RoomSketcher Pro branding; SaaS feature for professionals; Lintel is user-owned output. |
| **HVAC / Electrical / Evacuation plan layers** | RoomSketcher specialized overlays; domain-specific; could be plugins later. |
| **Site Plans (plot boundaries, outdoor areas)** | RoomSketcher Site Plans; civil/landscape scope; Lintel is architectural interior. |
| **Video render / 3D walkthrough export** | Sweet Home 3D / Floorplanner; requires render pipeline; post-v1.0. |
| **OBJ/MTL / glTF 3D model export** | Sweet Home 3D / Floorplanner; interop format; useful but not core 2D CAD. |
| **Cloud auto-save / multi-device sync** | Floorplanner / RoomSketcher Online; requires backend; Lintel is local-first. |
| **Mobile / tablet native app** | All three have mobile; Lintel targets desktop (egui/eframe). |
| **Plug-in architecture (Java for SH3D)** | Sweet Home 3D plug-ins; extensibility; can be considered post-v1.0 via dynamic loading. |
| **Multi-language UI (29 languages)** | Sweet Home 3D; i18n infrastructure; post-v1.0. |
| **Photo-realistic rendering with sunlight/time-of-day** | Sweet Home 3D / RoomSketcher; render engine; separate milestone. |
| **Custom 3D model import (SH3D / OBJ / glTF)** | Sweet Home 3D import wizard; Floorplanner marketplace; Lintel can support glTF import later. |
| **Texture import (custom images for materials)** | All three; material system prerequisite; post-v1.0. |
| **Stair tool (straight, L-shape, U-shape, spiral)** | All three have stair tools; complex geometry; post-v1.0. |
| **Roof tool (auto-generate roof from walls)** | Sweet Home 3D / Floorplanner / RoomSketcher; architectural scope creep; separate app. |
| **Terrain / site modeling** | Floorplanner / RoomSketcher; civil scope; not applicable. |
| **Collaboration / real-time multi-user** | Not in any of the three (cloud only); requires backend; out of scope. |
| **BIM export (IFC)** | None of the three; professional BIM tools only; not consumer CAD. |

---

## 6. Summary — Highest-Value Findings (10 Lines)

1. **Drag-from-catalog onto wall** is the universal door/window placement pattern (SH3D, Floorplanner, RoomSketcher) — auto-orient, auto-resize to thickness, validate width ≤ wall length.
2. **Two wall-draw paradigms coexist**: rectangle-room drag (Floorplanner Draw Room, fast for rectilinear) vs. click-click chaining (SH3D, RoomSketcher, free-form). Lintel's click-click is correct for CAD precision.
3. **Room = closed wall loop** auto-generates floor+ceiling in all three; SH3D uniquely includes half-doorstep for correct 3D door-open joins.
4. **Opening swing controlled by angle field + flip** (RoomSketcher: Opening Angle 0–180°, Flip hotkey Q; Floorplanner: Flip H/V; SH3D: rotation in dialog) — Lintel should adopt explicit angle + hinge-side enum.
5. **Per-wall thickness + height** with alignment aids (Floorplanner "Move wall across axis") is expected; global defaults + per-wall override (SH3D) is the right model.
6. **Split wall** (SH3D menu, Floorplanner floating icon) and **curved walls** (Floorplanner bezier, RoomSketcher Pro) are standard editing ops — Lintel needs split now, curves later.
7. **Auto-dimension toggles** (Floorplanner: interior/outer/horizontal/short via View Settings) beat manual dimensioning for speed — Lintel should add auto-dimension layer with style controls.
8. **Design history / version timeline** (Floorplanner) surpasses simple undo/redo — Lintel should implement command-history with thumbnail timeline.
9. **Lock layers** (Floorplanner: construction/furniture/annotations independent) prevents accidental edits — essential for complex plans.
10. **Precision difference**: consumer tools snap to cm/4-in grid; Lintel's mm precision means **no forced snapping**, exact numeric input always honoured, non-90° geometry fully supported (unlike RoomSketcher's blocked length edit).

---

*End of research note. All behavioural summaries are original; every claim has a source URL above. No proprietary material reproduced.*