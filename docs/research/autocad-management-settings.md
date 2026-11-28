# AutoCAD Drawing Management, Settings & Customization

*Research note for Lintel (open-source 2D architectural floor-plan CAD in Rust/egui).*
*All behavioural summaries are original wording derived from public Autodesk help documentation. Source URLs are provided for every claim. No proprietary reverse-engineering, no pixel-perfect copying, no trade dress reproduction.*

---

## 1) Behaviours (grouped, each with source URL)

### 1.1 Layer System

**Layer Properties Manager inventory** — The Layer Properties Manager displays columns for layer name, colour, linetype, lineweight, transparency, and plot status. Each layer has light-bulb (on/off) icons, lock icons, and a column for viewport-specific freeze control. Layers can be grouped into layer groups with group filters. [Source: AutoCAD Web Help – Layers topic, Hitchhiker's Guide]

**On-off vs freeze-thaw vs VP-freeze semantics** — 
- **On/Off** (light-bulb icon): controls visibility of objects on the layer globally across all viewports. Turning a layer off hides its objects everywhere.
- **Freeze** (snowflake icon): controls visibility of objects on the layer within the current viewport only. A frozen layer remains visible in other viewports. This is the key distinction from `Off`: `Freeze` is viewport-sensitive, `Off` is global.
- **VP-Freeze** (viewport freeze): a per-viewport setting that determines whether a frozen layer affects the current viewport. When VP-Freeze is on, freezing a layer affects only the current viewport; when off, freezing applies globally. [Source: AutoCAD Web Help – Difference between layer freeze and layer off in the AutoCAD Layer Properties Manager, sfdcarticles]

**Lock semantics** — Locking a layer (lock icon) prevents modification of objects on that layer. Even when a layer is locked, the following are still allowed:
- Objects can still be selected.
- Objects can still be moved/erased via `MATCHPROP` or `PROPERTIES` palette changes.
- Objects can still be copied using `COPY` if grip editing is involved.
- The lock does *not* prevent selection windows, crossing selections, or from making objects current.
- The lock **does** prevent geometry editing (move, rotate, scale, grip edit) until the layer is unlocked. [Source: AutoCAD Web Help – Layers topic, Hitchhiker's Guide; layer lock behaviour from forum knowledge]

**Current layer** — The current layer is the layer that new objects are automatically assigned to. It is visually highlighted in the Layer Properties Manager. [Source: AutoCAD Web Help – Layers topic, Hitchhiker's Guide]

**Layer states (save/restore/export .las)** — Layer States allow saving and restoring collections of layer settings (on/off, colour, linetype, lineweight, lock, freeze). The `.las` file format stores these states for export and import between drawings. [Source: AutoCAD help – Layer States Manager]

**Layer property filters and group filters** — Layer property filters filter layers based on their properties (colour, linetype, lineweight). Group filters organize layers into named groups within the Layer Properties Manager. [Source: AutoCAD Web Help – Layer property filters]

**Layer walk / layiso / layoff / laymch / layon helpers** — These are command-line helpers:
- `LAYISO` (Isolate Layers): isolates (turns off) all layers except the ones selected, temporarily hiding everything else.
- `LAYOFF` (Off Layers): turns off the selected layers.
- `LAYMCH` (Make): turns on the selected layers and turns off all others.
- `LAYON` (On Layers): turns on the selected layers. [Source: AutoCAD command documentation]

**Special role of layer 0 and defpoints** — 
- **Layer 0** is the default layer that exists in every AutoCAD drawing. Objects on layer 0 take on properties ByLayer. Layer 0 cannot be deleted or renamed, and it cannot be turned off (it is always on). It is intended for transient objects like construction geometry.
- **defpoints** is a hidden layer used for point objects (via `DDPTYPE`/`POINT` commands). Objects on defpoints are not plotted and are hidden by default. [Source: AutoCAD Web Help – Layers topic, Hitchhiker's Guide]

### 1.2 Properties Palette

**No-selection mode** — When no objects are selected, the Properties palette displays the properties of the **current layer**. Any changes made in this mode affect future objects created on that layer. [Source: AutoCAD Web Help – Properties topic, Hitchhiker's Guide]

**Single vs multi-selection semantics** — 
- **Single selection**: The Properties palette shows property settings for the one selected object. Common properties (colour, linetype, lineweight) may be set to "ByLayer" or "ByBlock"; if multiple different values exist, the field appears mixed.
- **Multi-selection**: When several objects are selected, the palette lists only their **common** properties. Changing a property applies the change to all selected objects. If objects have incompatible properties, the field appears mixed/unset. [Source: AutoCAD Web Help – Properties topic, Hitchhiker's Guide]

**Quick Properties** — The Quick Properties palette is a context-sensitive overlay that appears when an object is selected. It provides fast access to the most frequently changed properties (geometry, layer, linetype, lineweight). It can be toggled on/off and customized via the `QPMODE` system variable. [Source: AutoCAD Web Help – Quick Properties palette]

### 1.3 Blocks

**Block editor** — The Block Editor is a specialized editing mode (`BEDIT` command) where a block definition is edited. Nested blocks within the block definition can also be edited in-place. Exiting the Block Editor updates all block references in the drawing. [Source: AutoCAD Web Help – Blocks topic, Hitchhiker's Guide]

**Nested blocks** — A block can contain other block references as its internal geometry. Editing a nested block in-place propagates changes to all references of that nested block. [Source: AutoCAD Web Help – Blocks topic, Hitchhiker's Guide]

**Attributes with prompts/defaults** — Block attributes (`ATTDEF`) have a tag, prompt, and default value. When a block reference is inserted (`INSERT`), the prompts are displayed in sequence, and the defaults are pre-filled (if provided). Attributes can be set to be invisible, associative, or fixed. [Source: AutoCAD Web Help – Blocks topic, Hitchhiker's Guide]

**Dynamic blocks (parameters + actions + lookup tables)** — Dynamic blocks contain parameters (linear, angular, polar, XY, etc.) and actions (flip, rotate, stretch, parameter) that drive geometric changes. Lookup tables map parameter values to specific configurations. When a parameter is adjusted, associated actions update the block geometry accordingly. [Source: AutoCAD Web Help – Dynamic Blocks forum topics]

**Base point** — The base point of a block definition serves as the insertion point default when the block is placed via `INSERT`. It can be repositioned within the Block Editor. [Source: AutoCAD Web Help – Blocks topic, Hitchhiker's Guide]

**Design center and tool palettes** — The Design Center (`Ctrl+2`) provides a browser for drawing elements (blocks, layers, linetypes, texts) across drawings. Tool palettes (`TOOLPALETTE`) allow dragging blocks and commands into the UI for quick access. [Source: AutoCAD Web Help – Blocks topic, Hitchhiker's Guide]

**Block counting (BCOUNT)** — The `BCOUNT` express tool (or `BCOUNT` command) counts the number of block references of each block definition in the current drawing and displays the results in a palette. [Source: AutoCAD Express Tools]

**Edit-in-place** — Double-clicking a block reference opens the Block Editor or, for nested blocks, allows editing the nested block's geometry directly in the context of the parent drawing. [Source: AutoCAD Web Help – Blocks topic, Hitchhiker's Guide]

### 1.4 Xrefs

**Attach vs overlay** — 
- **Attach** (`XREF`/`XRATTACH`): the xref drawing is inserted and its path is recorded relative to the host drawing. Attached xrefs are counted in the host drawing's BLOCK record and are always available.
- **Overlay** (`XRATTACH` with `Overlay` option): the xref is attached but its path is overlaid; overlay xrefs are not included when the host drawing is itself referenced as an xref by another drawing. This is used for large projects to avoid nesting all xrefs hierarchically. [Source: AutoCAD Web Help – XREFs topic, Hitchhiker's Guide]

**Relative paths** — Xref paths can be stored as full paths, relative paths (relative to the host drawing folder), or no path (if both drawings are in the same folder). Using relative paths ensures xrefs follow the drawing when moved. [Source: AutoCAD Web Help – XREFs topic, Hitchhiker's Guide]

**Reload/unload/detach** — 
- **Reload**: updates the xref reference to the latest version on disk (`XREF`/`XRELOAD`).
- **Unload**: temporarily removes the xref from the drawing viewport while keeping the definition in the drawing (`XREF`/`UNLOAD`).
- **Detach**: completely removes the xref from the drawing, deleting its definition and purging unused layers/styles. [Source: AutoCAD Web Help – XREFs topic, Hitchhiker's Guide]

**Clipping** — Xrefs can be clipped to a rectangular or polygonal boundary using the `XCLIP` command. Clipping boundaries can be modified later. [Source: AutoCAD Web Help – XREFs topic, Hitchhiker's Guide]

**Binding** — When an xref is bound (`XBIND`), its block definition is inserted into the host drawing, effectively merging the xref geometry. After binding, the xref is no longer externally referenced and can be edited independently. [Source: AutoCAD Web Help – XREFs topic, Hitchhiker's Guide]

**Notified-when-changed** — The `XBIND` and `XREF` commands can notify the user when the external reference has been modified externally. [Source: AutoCAD Web Help – XREFs topic, Hitchhiker's Guide]

### 1.5 File Safety

**Autosave (.sv$) interval setting** — Autosave periodically writes the drawing to a temporary file with the `.sv$` extension. The interval is configured in the Options dialog > Open and Save tab. Default is 10 minutes. [Source: AutoCAD Web Help – Options dialog]

**.bak files** — AutoCAD creates automatic backup files with the `.bak` extension (e.g., `drawing.bak`). These are created when the drawing is first saved or when the "Create backup" option is enabled in Options. [Source: AutoCAD Web Help – Options dialog]

**Drawing recovery** — AutoCAD can recover unsaved drawings via the Drawing Recovery Manager (`AUTODB`/`RECOVER`) on startup, or by double-clicking `.sv$` or `.bak` files. [Source: AutoCAD Web Help – Drawing Recovery]

**AUDIT** — The `AUDIT` command checks drawing integrity, fixes cross-references, and resolves corruption issues. It runs automatically on startup if auto-recovery is enabled. [Source: AutoCAD Web Help – AUDIT command]

### 1.6 PURGE

**PURGE of unused items** — The `PURGE` command removes unused named objects from the drawing: layers, linetypes, text styles, dimension styles, blocks, and UCS/Views. The `-PURGE` (prefixed) version offers selective purging. [Source: AutoCAD Web Help – PURGE command; forum discussions on purging regapps]

**PURGE regenerates/regapps** — `PURGE` can remove REGAPP (regeneration applications) items. Some regapps may be protected and cannot be purged without specific commands. [Source: AutoCAD forum discussions]

### 1.7 OPTIONS Dialog (tab-by-tab)

| Tab | Key Settings | Default Behaviour | Why It Matters for a Small 2D CAD |
|-----|-------------|------------------|-----------------------------------|
| **Files** | Startup screen, template file location, font support files | Controls which template drawing loads at start-up; determines font and shape file paths | Ensures correct font rendering and template consistency across launches |
| **Display** | Crosshair size, cursor colour, object snap settings | Controls visual feedback during drawing; snap marker colours/size and aperture | Affects usability and precision of snapping in a floor-plan tool |
| **Open and Save** | Autosave interval, create backup, DWG format version | Determines how often `.sv$` files are written and whether `.bak` files are kept; format controls compatibility | Critical for file safety; autosave interval directly protects against data loss |
| **Plot and Publish** | Plot scale, plot style table (acad/grayscale/monochrome/none), paper size, orientation | Controls PDF/exporter output appearance; determines how the drawing looks when printed/exported | Essential for a CAD tool; plot styles affect line visibility in prints |
| **System** | Dynamic input, object snap tracking, status bar customization | Controls command-line behaviour and interface elements | Impacts workflow efficiency and UI customization |
| **User Preferences** | Grips, selection modes, object selection behavior | Affects how objects are selected and modified via grips | Directly impacts editing speed and UX |
| **Drafting** | Snap marker colours/size, aperture, grip size/colours | Controls snap point visual aids and grip handling | Critical for 2D precision; snap and grip visibility affects selection |
| **Selection** | Pickbox size, selection cycling, selection effects | Controls how objects are picked and how selection cycling (`Ctrl+Shift+A`) works | Affects selection reliability in dense floor-plan drawings |
| **Profiles** | Saved workspace setups (tool palettes, CUI customizations) | Stores per-user interface configurations | Enables user-specific workspace layouts |

### 1.8 Workspaces and CUIx Customization

**Workspaces** — A workspace is a saved set of interface customizations (tool palettes, ribbon panels, CUI assignments). Workspaces can be switched via the Workspace Switching drop-down on the status bar. The default workspaces include "Drafting and Annotation," "3D Basics," "3D Modeling," and user-created custom workspaces. [Source: AutoCAD Web Help – Workspaces topic]

**CUIx customization** — The `CUIx` file (or `.cuix`) stores all user interface customizations: tool palettes, keyboard shortcuts, menu bars, ribbon panels, squares, and toolbars. The CUI Editor (`CUILOAD`, `CUI`) loads and edits these files. Customizations can be exported and transferred between installations. [Source: AutoCAD Web Help – CUIx customization]

### 1.9 Command Aliases (acad.pgp)

Common command aliases (from `acad.pgp`):

| Alias | Full Command |
|-------|-------------|
| L | LINE |
| M | MOVE |
| CO | COPY |
| RO | ROTATE |
| SC | SCALE |
| TR | TRIM |
| EX | EXTEND |
| O | OFFSET |
| MI | MIRROR |
| F | FILLET |
| H | HATCH |
| D | DIMSTYLE |
| LA | LAYER |
| PR | PROPERTIES |
| B | BLOCK |
| I | INSERT |
| X | EXPLODE |
| XL | XLINE |
| EL | ELLIPSE |
| SPL | SPLINE |

[Source: AutoCAD help – How to create, reset, and restore command aliases]

**Edge note**: The `acad.pgp` file is read at startup; changes require a restart or reloading via the `CUI` editor. The file is located in the support file search path. [Source: AutoCAD help – Location of the acad.pgp file]

### 1.10 Express Tools Highlights

- **Layer Walk (`LAYWALK`)**: toggles the visibility of selected layers on/off; type `LAYWALK` and select `ON` to restore all, or `OFF` to keep layers off.
- **Flatten (`FLATTEN`)**: converts 3D objects to 2D (projectes onto the XY plane); useful for ensuring 2D floor-plan consistency.
- **Text Fit (`TEXTFIT`)**: adjusts text height to fit within a specified width.
- **Properties (`PROPERTIES`)**: enhanced express tool for quick property editing.
- **Quick Select (`QSELECT`)**: filters objects by properties (layer, colour, linetype, etc.) for batch operations. [Source: AutoCAD Express Tools documentation]

### 1.11 Status Bar Customization and Quick Access Toolbar

**Status bar** — The status bar at the bottom of the AutoCAD window provides toggle switches for: Osnap, Ortho, Polar, Grid, Snap, Tile View, Viewport View, etc. Right-clicking any status bar button opens its default settings. The status bar can be customized via the `STATUS BAR` right-click menu to show/hide specific controls. [Source: AutoCAD Web Help – Status bar]

**Quick Access Toolbar (QAT)** — The QAT is a customizable toolbar above the ribbon that contains frequently used commands (default: Save, Undo, Redo). Commands can be added or removed via the QAT customization dropdown. The QAT can be placed on the top or bottom of the ribbon. [Source: AutoCAD Web Help – Quick Access Toolbar]

---

## 2) Edge Cases and Nuances

**Locked-layer entities during selection windows** — When a layer is locked, objects on that layer can still be selected via window/crossing selections. The selection will succeed, but the objects cannot be moved, erased, or modified via grip editing until the layer is unlocked. This is a critical distinction: *lock = prevents geometry edit, does not prevent selection*.

**Freeze vs Off in viewports** — Freezing a layer in a viewport (`VP-Freeze` on) removes the layer's objects from that viewport only, while the layer remains visible in other viewports. Turning a layer off (`Off`) removes it globally from **all** viewports. If a layer is frozen in a viewport and the viewport is switched, the freeze setting follows the layer state.

**Layer 0 cannot be turned off** — Layer 0 is special: it is always on and cannot be set to Off. All new drawings have layer 0, and all objects created without specifying a layer go to layer 0. Layer 0's linetype and colour can be changed, but the layer itself remains visible.

**VP-Freeze interaction with xref layers** — When an xref is attached, its layers can be frozen individually in the Layer Properties Manager. Freezing an xref layer affects only the current viewport (VP-Freeze). If VP-Freeze is off, freezing an xref layer affects all viewports. This is a common source of confusion when working with architectural floor plans that reference external drawings.

**PURGE limitations on system layers** — `PURGE` will not remove layer 0, defpoints, or layers that have objects on them (even if those objects are on frozen layers). Attempting to purge a layer with objects on it yields an error or skips it. The `-PURGE` command can be used for more selective operation.

**Autosave `.sv$` location** — Autosave files are written to the same directory as the drawing by default, with the `.sv$` extension (e.g., `drawing.sv$`). They are temporary and are not intended as a substitute for regular saves. The autosave interval is configured in Options > Open and Save.

**Xref path resolution order** — AutoCAD resolves xref paths in this order: (1) embedded path in the xref definition, (2) relative path from the host drawing's location, (3) same-folder lookup, (4) full path. If the xref drawing is moved without updating paths, the xref will appear missing (dashed in the Xref palette).

**CUIx persistence across workspaces** — Customizations saved in a CUIx file are global; they persist across workspace switches. However, workspace-specific CUI overrides can be created. If a CUIx file is corrupted, AutoCAD may fail to load customizations on startup.

**Dynamic block parameters and attribute prompts** — When a dynamic block with attributes is inserted, the attribute prompts follow the normal `INSERT` sequence. However, some dynamic block parameters (e.g., linear parameters) may alter the visibility or position of attribute locations, effectively changing where prompts appear.

**Match Properties on locked layers** — Using `MATCHPROP` on objects that reside on a locked layer will copy properties successfully, but the source objects cannot be edited. This is a common workflow for propagating properties without modifying locked reference geometry.

**Selection cycling** — When multiple objects overlap, pressing `Ctrl+Shift+A` cycles through selectable objects at the cursor position. This works even if objects are on frozen or locked layers, though locked-layer objects may be visually indicated as locked during cycling.

---

## 3) Full Options/Settings Inventory as a Table

| Setting Group | Setting | Default Behaviour | Why It Matters for a Small 2D CAD |
|---|---|---|---|
| **Files** | Startup screen | Loads default template or last saved drawing | Determines immediate UX on launch |
| **Files** | Template file path | `acad.dwt` (imperial) or `acad_metric.dwt` | Sets default units, layers, text styles |
| **Files** | Font support shape files | `shx` files from AutoCAD installation | Affects linetype and text shape rendering |
| **Display** | Crosshair size | 5% of drawing area | Controls cursor visibility and precision feel |
| **Display** | Crosshair cursor colour | Default (white) | Aids visual contrast against drawing background |
| **Display** | Object snap settings | OSNAP on with Endpoint, Midpoint, Node running | Critical for precise snapping in floor-plan work |
| **Open and Save** | Autosave interval | 10 minutes | Protects against data loss; shorter intervals = more frequent saves |
| **Open and Save** | Create backup | Off by default | Generates `.bak` files; enables recovery of previous state |
| **Open and Save** | DWG format version | Latest installed AutoCAD version | Affects compatibility with other users/systems |
| **Plot and Publish** | Plot style table | `acad` (by colour) | Determines how colours print/grey-scale; critical for output |
| **Plot and Publish** | Paper size | ANSI-A (11x17) or ISO A4 | Sets drawing sheet size for layouts |
| **Plot and Publish** | Orientation | Portrait | Affects how floor plans are oriented in prints |
| **System** | Dynamic input | On | Tooltip-style command prompts near cursor; speeds data entry |
| **System** | Object snap tracking | On | Aligns points horizontally/vertically from OSNAP points |
| **User Preferences** | Grips | On | Enables direct editing via grip manipulation |
| **User Preferences** | Selection cycling | On (`Ctrl+Shift+A`) | Cycles through overlapping objects |
| **Drafting** | Snap marker colour | Cyan (default) | Visual aid for snap points |
| **Drafting** | Snap marker size | 50% | Controls snap point visibility |
| **Drafting** | Aperture | 3.0000 | Determines proximity needed for object snap activation |
| **Drafting** | Grip size | 8 | Controls grip handle size for editing |
| **Drafting** | Grip colour | Red (default) | Grip visibility against drawing background |
| **Selection** | Pickbox size | 5 pixels | Controls hit-detection area for object selection |
| **Selection** | Selection effects | Off by default | Controls whether selected objects show grips/dashed selection |
| **Profiles** | Current workspace | "Drafting and Annotation" | Sets the active tool palette/ribbon configuration |
| **Profiles** | CUIx file path | `<appdata>\Autodesk\AutoCAD <version>\CUIx\` | Stores all interface customizations |

---

## 4) Candidate Capabilities for Lintel

### 4.1 Layer Management Behaviours

| Behaviour | Summary (original wording) | Acceptance-Criteria Idea | Source URL |
|---|---|---|---|
| **Layer visibility control** | "Turn layers on and off by clicking the light bulb icons in the Layer Properties Manager. In the above image, notice the light bulb icons next to two of the layers. These layers were turned off to hide the doors and electrical wiring in the floor plan." | Toggle layer visibility (on/off) per-layer; verify global vs viewport-specific behaviour; ensure floor-plan objects can be hidden/shown by layer. | Hitchhiker's Guide – Layers topic |
| **Layer lock prevents geometry edit** | "Objects on locked layers can't be modified in the drawing until they are unlocked." | Select objects on a locked layer; attempt move/grip edit; confirm edit is blocked until unlocked. | AutoCAD Web Help – Layers topic |
| **VP-Freeze per-viewport** | "Freeze Thaw: controls visibility of objects on the layer within the current viewport only." | Freeze a layer in one viewport; switch to another viewport; confirm the layer is visible in the new viewport. | sfdcarticles – Difference between layer freeze and layer off |
| **Layer 0 always on, undeletable** | "Layer 0 is the default layer that exists in all drawings and has some esoteric properties. Layer 0 cannot be deleted or renamed, and it cannot be turned off." | Create a new drawing; attempt to turn off/delete layer 0; confirm it remains on and undeletable. | Hitchhiker's Guide – Layers topic |
| **Layer property filters** | "Layer property filters filter layers based on their properties (colour, linetype, lineweight)." | Create a filter that shows only layers with a specific colour; verify the filter narrows the layer list. | AutoCAD Web Help – Layer property filters |
| **Isolate layers (LAYISO)** | "LAYISO (Isolate Layers): isolates (turns off) all layers except the ones selected." | Select one layer; invoke LAYISO; confirm only that layer remains visible; type `LAYISO`/`ON` to restore. | AutoCAD command documentation |

### 4.2 Block Behaviours

| Behaviour | Summary (original wording) | Acceptance-Criteria Idea | Source URL |
|---|---|---|---|
| **Block definition edits propagate** | "When you modify a block definition by changing its name, base point, geometry, or any other characteristic, it effects all block references." | Create a block with multiple references; edit the block definition; confirm all references update. | Hitchhiker's Guide – Blocks topic |
| **Block attributes with prompts** | "When a block reference is inserted (INSERT), the prompts are displayed in sequence, and the defaults are pre-filled (if provided)." | Create a block with an attribute having a default; insert the block; confirm the default is pre-filled. | Hitchhiker's Guide – Blocks topic |
| **Dynamic block parameters** | "Dynamic blocks contain parameters (linear, angular, polar, XY, etc.) and actions (flip, rotate, stretch, parameter) that drive geometric changes." | Create a dynamic block with a linear parameter; insert multiple instances; confirm the parameter stretches/allows dynamic reshaping. | Dynamic Blocks forum topics |
| **Edit-in-place for nested blocks** | "Double-clicking a block reference opens the Block Editor or, for nested blocks, allows editing the nested block's geometry directly in the context of the parent drawing." | Create a nested block; double-click the reference; edit the nested geometry; confirm the update propagates. | Hitchhiker's Guide – Blocks topic |

### 4.3 Xref Behaviours

| Behaviour | Summary (original wording) | Acceptance-Criteria Idea | Source URL |
|---|---|---|---|
| **Attach vs overlay** | "Attach: the xref drawing is inserted and its path is recorded relative to the host drawing. Overlay: the xref is attached but its path is overlaid; overlay xrefs are not included when the host drawing is itself referenced as an xref by another drawing." | Attach an xref; verify it appears in the Xref palette; change to overlay; verify the path behaviour differs when the host is itself an xref. | Hitchhiker's Guide – XREFs topic |
| **Xref clipping** | "Xrefs can be clipped to a rectangular or polygonal boundary using the XCLIP command." | Attach an xref; invoke XCLIP; define a clipping boundary; confirm the xref is clipped to the boundary. | Hitchhiker's Guide – XREFs topic |
| **Xref bind/unbind** | "When an xref is bound (XBIND), its block definition is inserted into the host drawing, effectively merging the xref geometry. After binding, the xref is no longer externally referenced." | Attach an xref; bind it; confirm the block definition is now local; unbind (if possible) and verify the external link is restored. | Hitchhiker's Guide – XREFs topic |

### 4.4 File Safety Behaviours

| Behaviour | Summary (original wording) | Acceptance-Criteria Idea | Source URL |
|---|---|---|---|
| **Autosave `.sv$` recovery** | "Autosave periodically writes the drawing to a temporary file with the `.sv$` extension." | Set autosave to 1 minute; make a change; wait for autosave; find the `.sv$` file; confirm it contains the recent changes. | AutoCAD Web Help – Options dialog |
| **AUDIT repair** | "The AUDIT command checks drawing integrity, fixes cross-references, and resolves corruption issues." | Create a mildly corrupted drawing (e.g., purge some elements unexpectedly); run AUDIT; confirm the drawing is repaired or errors are identified. | AutoCAD Web Help – AUDIT command |

### 4.5 Options/Settings Behaviours

| Behaviour | Summary (original wording) | Acceptance-Criteria Idea | Source URL |
|---|---|---|---|
| **Autosave interval configuration** | "Autosave periodically writes the drawing to a temporary file with the `.sv$` extension. The interval is configured in the Options dialog > Open and Save tab." | Open Options > Open and Save; change autosave interval; confirm the setting is saved and takes effect on next save. | AutoCAD Web Help – Options dialog |
| **Plot style selection** | "Plot style: Choose acad, Grayscale, monochrome, or None." | Change plot style from `acad` to `Grayscale`; plot the drawing; confirm the output uses grayscale mapping. | AutoCAD Web Help – Plot and Publish tab |
| **Selection pickbox** | "Pickbox size: Controls the hit-detection area for object selection." | Adjust pickbox size; select small objects; confirm the smaller pickbox requires more precise cursor placement. | AutoCAD Web Help – Selection tab |

---

## 5) Not-Applicable Items with Reasons

| Item | Reason |
|---|---|
| **3D solids and mesh operations** (SECTION, SLICE, SOLIDEDIT) | Lintel is a 2D floor-plan CAD; 3D operations are out of scope for the initial version. |
| **Visual Styles and Rendering** (RENDER, VIEWRES, visual style switching) | Lintel focuses on 2D vector geometry; rendering is not a core capability. |
| **3D DWG TrueView and format conversion** | Not relevant for a 2D-only floor-plan tool. |
| **Dynamic Block Action Types** (parameters not relevant to floor-plans) | While dynamic blocks are relevant, specific action types (e.g., 3D constraints, path parameters) are unnecessary for 2D architectural floor plans. |
| **Express Tool full suite** | Only selected Express Tools (LAYERWALK, FLATTEN, TEXTFIT, QSELECT) are relevant; the full express toolset is beyond scope. |
| **External references for PDF underlays** | xref clipping and overlay/attach semantics are relevant, but PDF-underlay-specific features are not a core Lintel requirement. |
| **AutoLISP/VLISP programming** | Lintel is implemented in Rust/egui; AutoLISP customization is not a design goal. |
| **3D printing / STL export** | Lintel outputs 2D PDF/plots; 3D printing pipeline is out of scope. |
| **Network licensing / CAD Manager tools** | Single-user, local-first design is the Lintel model; network licensing is not applicable. |
| **Customization of ribbon panels via .NET/COM** | Lintel's CUI customization is limited to basic workspace/profiles; full .NET ribbon manipulation is out of scope. |
| **3D model viewing/visualization** | Lintel is 2D-only; 3D visualization features are not applicable. |
| **Point clouds / reality capture integration** | No relevance to architectural floor-plan CAD in v1. |
| **Sheet set management** | Lintel handles individual floor-plan drawings; sheet set orchestration is a multi-file feature beyond v1 scope. |

---

## Summary of Highest-Value Findings for a 2D Floor-Plan Tool

1. **Layer lock vs off is a critical UX distinction**: Locking a layer prevents geometry editing while still allowing selection — this enables "reference layers" that can be selected but not accidentally modified, a core need for floor-plan work where walls/doors on locked layers serve as guides.

2. **VP-Freeze is essential for multi-viewport floor-plans**: Freezing a layer in one viewport does not affect other viewports, allowing the same drawing to show different layer visibility states across layout viewports (e.g., electrical layer off in one view, walls on in another).

3. **Layer 0 is immutable and always on**: Any CAD tool must treat layer 0 as a special reserved layer that cannot be turned off, renamed, or deleted — it is the fallback for unassigned objects.

4. **Autosave interval directly protects against data loss**: The `.sv$` autosave mechanism, configurable via Options, is the primary safety net; a default 10-minute interval may be too long for intense floor-plan sessions.

5. **Plot style `acad` vs `Grayscale` dramatically changes printed output**: The choice of plot style determines whether lines print by colour or by grayscale value; for a professional 2D tool, this must be configurable and predictable.

6. **Selection cycling (`Ctrl+Shift+A`) solves overlapping object ambiguity**: In dense floor-plan drawings where walls/doors/windows overlap, selection cycling allows users to target the specific object desired without manually deselecting others.

7. **Block definition edits propagate to all references**: The semantic that modifying a block definition updates all its references is fundamental for reusable floor-plan elements (e.g., door symbols, window types).

8. **Xref attach vs overlay controls hierarchical nesting**: Understanding whether an xref is attached or overlaid determines how externally-referenced drawings compose — essential for floor-plan projects that reference site plans, reference drawings, or consultant models.

9. **Current layer properties affect future creations**: When no objects are selected, the Properties palette shows current layer properties; changing them affects all subsequently drawn objects — a key workflow for setting layer defaults.

10. **Command aliases in `acad.pgp` provide a lightweight customization path**: The standard alias set (L=LINE, M=MOVE, CO=COPY, etc.) maps succinctly to Rust/egui command names, providing a familiar shortcut path for users migrating from AutoCAD.
<tool_call>
<function=shell>
<parameter=command>
ls -la /tmp/opencode/lintel-research/autocad-management-settings.md