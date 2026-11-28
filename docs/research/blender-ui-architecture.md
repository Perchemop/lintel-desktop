# Blender UI Architecture & Interaction Philosophy — Research for Lintel (2D CAD)

*Research conducted via public Blender 5.3 Manual and developer documentation.
All behavioural summaries are original rewrites; source URLs provided per item.*
*Clean-room: no proprietary source code, icons, or pixel-perfect UI copying.*

---

## 1) Behaviours (grouped, each with source URL)

### 1.1 Area System — Splittable/Resizable Editors, Area Types, Corner Drag, Join/Split, Persistence per Workspace
- **Area boundaries** are indicated by rounded corners. The Blender window is divided into rectangles called *Areas* that reserve screen space for editors (3D Viewport, Outliner, Properties, etc.) [^areas].
- **Resizing**: Borders between areas can be dragged (LMB) when cursor changes to double-headed arrow. Holding **Ctrl** snaps area sizes to convenient sizes; holding **Shift** moves nearby aligned borders together while resizing [^areas].
- **Splitting**: Dragging from an area corner left/right splits vertically; dragging up/down splits horizontally. A split creates a new area. Drag into another area to replace it [^areas].
- **Joining**: Dragging from an area corner into the space of a second area joins the two areas (displayed brighter). Can join and split at once by dragging a split operation into a separate area [^areas].
- **Swapping contents**: Ctrl+LMB on a corner, drag toward target area, release there. Areas need not be side-by-side but must be in the same window [^areas].
- **Maximize area**: Ctrl+Spacebar expands editor to fill whole window, keeping only that editor's menu bar and status bar. Unlike Focus Mode, menu bar and status bar remain visible [^areas].
- **Area options**: RMB on border opens overlay with Vertical/Horizontal Split indicator, Join direction, Swap Areas [^areas].
- **Persistence**: Areas are grouped into *Workspaces* (see 1.2). Workspace layouts persist per-screen, per-blend-file. Screens can also be used for multi-monitor setups [^areas].

**Source**: [Blender 5.3 Manual — Areas](https://docs.blender.org/manual/en/latest/interface/window_system/areas.html)

### 1.2 Workspaces — Purpose-Built UI Sets Switching Per Task; Panels/Channels Change Per Workspace
- **Workspaces** are predefined window layouts. Each consists of areas containing editors, geared towards a specific task (modeling, animating, scripting, etc.) [^workspaces].
- **Tabs** at the Topbar: click to switch, double-click to rename [^workspaces].
- **Add Workspace**: Adds a new workspace from a predefined template (Modeling, Sculpting, UV Editing, Texture Paint, Shading, Animation, Rendering, Compositing, Geometry Nodes, Scripting) [^workspaces].
- **Default startup** shows the "Layout" workspace with: 3D Viewport (top left), Outliner (top right), Properties (bottom right), Timeline (bottom left) [^workspaces].
- **Other default workspaces**: Modeling, Sculpting, UV Editing, Texture Paint, Shading, Animation, Rendering, Compositing, Geometry Nodes, Scripting [^workspaces].
- **Additional workspaces**: 2D Animation (Grease Pencil), 2D Full Canvas, VFX (Masking, Motion Tracking), Video Editing (Video Editing, Rendering, Compositing) [^workspaces].
- **Controls**: Duplicate, Delete (can't delete last), Reorder to Front/Back, Previous (Ctrl+PageUp), Next (Ctrl+PageDown), Delete Others, RDM [^workspaces].
- **Save/Override**: Workspaces saved in the blend-file; Load UI option on file open [^workspaces].
- **Custom defaults**: A custom set of workspaces can be saved as part of the Defaults [^workspaces].

**Source**: [Blender 5.3 Manual — Workspaces](https://docs.blender.org/manual/en/latest/interface/window_system/workspaces.html)

### 1.3 Outliner — Hierarchical Tree of Collections/Objects; Visibility/Lock/Selectability Toggles; Drag-to-Reparent; Search/Filter
- **Hierarchical tree** of collections and objects in the scene [^outliner] (derived from general Blender UI knowledge; Outliner section of the Manual).
- **Visibility eye toggle**: Shows/hides objects in the viewport [^outliner].
- **Lock toggles**: Selectability, Camera, Render per object/collection [^outliner].
- **Drag-to-reparent**: Drag an object/collection to reparent it under another [^outliner].
- **Search filter**: Press Ctrl+F while hovering the Sidebar to open a search field for finding settings and properties. Matching panels are highlighted; non-matching are grayed out. Press Alt+F to clear search and restore normal appearance [^sidebar].
- **Filter popover**: The search field filters panel contents in-place [^sidebar].

**Source**: [Blender 5.3 Manual — Sidebar (search/filter)](https://docs.blender.org/manual/en/latest/interface/window_system/regions.html) (see also Outliner section)

### 1.4 Properties Editor — Vertical Tab Icon Rail Per Object Type; Context-Sensitive Panels That Hide When Inapplicable
- **Vertical tab icon rail** along the right edge of the Properties editor. Each tab corresponds to an object/type category (Object, Modifiers, Material, etc.) [^properties].
- **Context-sensitive panels**: Available panels depend on active mode, current selection, and object type. Panels hide when inapplicable for the current context — e.g., Modifier panels disappear when no object is selected; Material panels disappear when the active data is not a material [^properties].
- **Main data selectors**: Some editors have a data selector in the header center (e.g., UV/Image Editor selects image, Movie Clip Editor selects clip) [^editors].
- **Display options** go to the right of headers or into Tool Settings regions; popovers can group them to avoid clutter [^editors].
- **Editor-Global options** (snapping, Sync With Outliner for Properties) go to right side of headers or into Options popovers [^editors].

**Source**: [Blender Developer Documentation — Editors](https://developer.blender.org/docs/features/interface/human_interface_guidelines/editors/) and [Blender 5.3 Manual — Properties Editor](https://docs.blender.org/manual/en/latest/interface/properties/)

### 1.5 N-Panel (Sidebar) — Contextual Overlay Per Editor
- **Sidebar** toggled with **N** key (or View › Sidebar in header). Appears on the right side of the editor [^regions].
- **Panels organized into tabs** along the right edge of the Sidebar. Clicking a tab displays its corresponding panels, which can be expanded/collapsed [^regions].
- **Available panels depend on**: editor type, active mode, and current selection [^regions].
- **Toolbar** on the left, toggled with **T** key [^regions].
- **Ctrl+F** while hovering the Sidebar opens a search field for finding settings and properties within the Sidebar [^regions].
- **Alt+F** clears the search and restores normal appearance [^regions].
- **Some editors provide additional tabs** through built-in features or extensions, allowing quick access to commonly used settings and operations [^regions].

**Source**: [Blender 5.3 Manual — Regions](https://docs.blender.org/manual/en/latest/interface/window_system/regions.html)

### 1.6 Keymap Philosophy — Modal Operators: G/R/S Gestural Commands with Numeric Input and Axis Locks Mid-Drag
- **G/R/S** (Grab/Rotate/Scale) are the core transform modal operators. Press G/R/S to initiate, then mouse movement controls the transformation [^numeric].
- **Numeric input**: After pressing the shortcut, type a number to set magnitude, then press Return to confirm. E.g., S 2, Return doubles scale; G 10, Return translates 10 units [^numeric].
- **Simple/Advanced mode**: Simple mode accepts plain numbers; Advanced mode (hold Ctrl or enable via `=`) accepts expressions, units (`cm`, `deg`, etc.), and basic Python operations (`+`, `*`, `/`, `**`) [^numeric].
- **Axis locking**: During a transform, type **X, Y, Z** to lock to the selected axis. Pressing the same key twice constrains to Global axis; a third press removes constraints. Orientation can be set via Transform Orientation selector (Global/Local/Normal) [^axis].
- **Plane locking**: Hold **Shift** + X/Y/Z to lock to a plane (two axes), restricting translation/scale. Rotation is always axis-constrained; plane locking does not apply to rotation [^axis].
- **MMB axis constraint**: After starting a transformation, hold MMB and move mouse in desired direction; release MMB on the axis to constrain to [^axis].
- **Industry Compatible keymap**: A built-in preset that maps keys closer to other 3D apps (Maya, etc.). W/E/R activate transformations as active tools (unlike default G/R/S). Customizable via Preferences › Keymap [^industry].
- **Searchable keymap editor**: Preferences › Keymap lets users filter and reassign shortcuts. Filter by name, key binding, or search text. Add custom keymap configurations, import/export presets [^keymap].
- **Mouse button select**: Preference to select with LMB (default, consistent with most software) or RMB (selects, opens context menu). Affects how selection interacts with tools [^keymap].
- **Spacebar Action**: Three options: Play (starts/stops animation), Tools (opens Toolbar underneath cursor), Search (opens Menu Search). Affects what hovering over a button shows in tooltip [^keymap].
- **Modifier key emulation**: Keymap can map modifier behaviours (Ctrl, Shift, Alt) to mouse actions or other keys for custom workflows [^keymap].

**Source**: [Blender 5.3 Manual — Numeric Input](https://docs.blender.org/manual/en/latest/scene_layout/object/editing/transform/control/numeric_input.html), [Industry Compatible Keymap](https://docs.blender.org/manual/en/latest/interface/keymap/industry_compatible.html), [Keymap](https://docs.blender.org/manual/en/latest/editors/preferences/keymap.html), [Axis Locking](https://docs.blender.org/manual/en/latest/scene_layout/object/editing/transform/control/axis_locking.html)

### 1.7 Modifiers Stack — Non-Destructive Ordered Operations List; Per-Modifier Enable/Disable/Render-Toggle; Drag to Reorder
- **Modifiers** are automatic operations affecting an object's geometry in a *non-destructive* way. Several modifiers can be added to a single object to form *The Modifier Stack* [^mod-intro].
- **Order matters**: Modifiers are evaluated top to bottom. Reordering changes the result. E.g., Mirror above Subdivision Surface smooths across the center seam; reverse smooths each half separately [^mod-intro].
- **Drag to reorder**: Click the grip icon in the top right of a modifier panel and move up/down in the stack. Move to First/Last, Pin to Last (keeps modifier at end, pin icon displayed) [^mod-intro].
- **Per-modifier enable/disable**: Each modifier has icons in its header: Realtime (viewport visibility), Render (render visibility) [^mod-intro]. These toggle the modifier's effect without deleting it.
- **Expand/collapse**: / Expand collapses modifier to show only the header and not its options [^mod-intro].
- **Type icon**: Quick visual reference of the modifier's type [^mod-intro].
- **Name**: Unique per object; default based on modifier type [^mod-intro].
- **On Cage Meshes**: If enabled, modified geometry can be edited directly instead of the original [^mod-intro].
- **Edit Mode / Realtime / Render toggles**: Display modified geometry in Edit Mode; toggle realtime viewport visibility; toggle render visibility [^mod-intro].
- **Apply**: Ctrl+A makes the modifier "real" — converts geometry to match results and deletes the modifier. Cannot apply to Empty types. Applying a non-first modifier ignores stack order [^mod-intro].
- **Delete**: Removes the modifier from the stack [^mod-intro].
- **Stack evaluation**: Modifiers are calculated from top to bottom. The active modifier (selected in stack or Outliner) is used by the Geometry Node Editor [^mod-intro].

**Source**: [Blender 5.3 Manual — Modifiers Introduction](https://docs.blender.org/manual/en/latest/modeling/modifiers/introduction.html)

### 1.8 Undo System — History Depth, Repeat Last, Redo Panel (F9) with Adjustable Parameters After the Fact
- **Ctrl+Z** Undo, **Shift+Ctrl+Z** Redo. Works in all modes, screen-level shortcut [^undoredo].
- **Undo History** (Edit › Undo History): Lists last actions recorded. Top of list = most recent. Click an entry to jump back to that position. History truncates when a new change is made [^undoredo].
- **F9 Adjust Last Operation**: Pops up a panel to tweak parameters of the last operator after it has run. E.g., reduce Circle vertices to 3 for an equilateral triangle. Only works right after the operation; changes context if something else is done first [^undoredo].
- **Repeat Last** (Shift+R): Repeats the last action. E.g., Shift+R after duplicating and moving a mesh duplicates-and-moves again [^undoredo].
- **Repeat History** (Edit › Repeat History): Presents list of last repeated actions; choose which to repeat. Moves selected option to top of stack [^undoredo].
- **Undo steps**: Default 32; max 256 in Preferences › System › Memory & Limits › Undo Steps. Separate counts for Edit Mode and Object Mode [^undoredo].
- **Global Undo**: Records actions outside Edit Mode (duplicating objects, changing panel settings, switching modes). Turning it off saves memory but stops Adjust Last Operation from working [^undoredo].
- **Undo Memory Limit**: Max memory (in MB) for undo; default 0 = unlimited [^undoredo].

**Source**: [Blender 5.3 Manual — Undo & Redo](https://docs.blender.org/manual/en/latest/interface/undo_redo.html)

### 1.9 Themes/Preferences — Full Colour Theming of Every Editor Region; Preset System; Add-Ons Preferences
- **Full colour theming**: Blender allows most interface colors to be changed. If default theme altered, can create a new theme or alter a preexisting one via Preferences › Themes tab [^interface].
- **Theme presets**: Several built-in presets available (Default, Dark, Light, etc.) [^interface].
- **Per-editor coloring**: Toolbar, Sidebar, Header, and individual editor regions can have independent color themes [^interface].
- **Accent color**: Custom accent color can be set for the interface [^interface].
- **Add-ons preferences**: Each add-on adds its own panel in Preferences, with its own settings and options [^interface].
- **Python console**: Preferences can expose Python settings for advanced users [^interface].

**Source**: [Blender 5.3 Manual — Interface](https://docs.blender.org/manual/en/latest/editors/preferences/interface.html)

### 1.10 Editor Header Context Menus and Pie Menus — Quick Radial Menus
- **Pie menus**: Menu whose items are spread radially around the mouse. The fastest way is to press the invoking key(s), move mouse slightly toward a selection, and release. Releasing without moving keeps the menu open for clicking [^pie].
- **8-item limit**: Pie menus should generally be limited to 8 items. Items must always appear at the same position for muscle memory [^pie-layout].
- **Mode pie menu**: Ctrl+Tab brings up a pie menu for quick mode switching in the 3D Viewport. If an Armature is selected, switches between Object Mode and Pose Mode [^header].
- **Context menu** (RMB): Opened over editors, buttons, properties. Contains editor-mode-sensitive operators, Single/All value options, Reset to Default, Copy Data Path, Assign Shortcut [^context].
- **Pie menu items** support key accelerators (underlined letters on each item). Number keys can also select items (1 for first, 2 for second, Alt+1 for 11th, etc.) [^pie].
- **Header context menu** (RMB on header): Toggles Show Header, Show Tool Settings, Show Menus, Flip to Bottom/Top, Vertical/Horizontal Split, Maximize Area, Duplicate Area into New Window, Close Area [^context].
- **Collapsing menus**: RMB on header › uncheck Show Menus collapses header menus to save horizontal space. Click the collapsed icon to access [^menus].

**Source**: [Blender 5.3 Manual — Menus](https://docs.blender.org/manual/en/latest/interface/controls/buttons/menus.html), [Pie Menus](https://docs.blender.org/manual/en/latest/interface/controls/buttons/menus.html#pie-menus), [Context Menu](https://docs.blender.org/manual/en/latest/interface/controls/buttons/menus.html#context-menu), [Headers](https://docs.blender.org/manual/en/latest/interface/editors/3dview/introduction.html#header-region)

### 1.11 Tooltips with Hints — Every Control Shows Short Doc + Shortcut
- **Tooltip appearance**: After hovering mouse cursor over a button or setting for a few moments, a tooltip appears [^tooltips].
- **Tooltip elements**: May contain — Short Description (context-dependent), Shortcut (keyboard/mouse), Value (current property value), Library path (source file), Disabled hint (why value is not editable), Python expression (if Python Tooltips enabled) [^tooltips].
- **Writing style**: Start descriptions with verbs (imperative conjugation). Keep short (2-3 lines). Do not end with period unless multiple sentences. Do not document default values. Do not use described term to explain itself. Mention examples if helpful [^tooltips].
- **Python Tooltips**: Preference option to display a property's Python information below the tooltip [^interface].
- **Disable/global**: Tooltips can be disabled globally via Preferences › Interface › Tooltips. Holding Alt while hovering forces tooltip display even if disabled [^interface].
- **Color swatches**: Hovering a color property displays large swatch preview with hex, RGBA, HSVA values [^tooltips].

**Source**: [Blender 5.3 Manual — Tooltips](https://docs.blender.org/manual/en/latest/getting_started/help.html#toolti), [Blender Developer Documentation — Tooltips](https://developer.blender.org/docs/features/interface/human_interface_guidelines/tooltips/)

### 1.12 Asset Browser — Drag Assets Into Scene
- **Access**: Create new area, click Editor Type button, choose Asset Browser [^asset].
- **Using assets**: Drag from Asset Browser to desired location as a general rule [^asset].
- **Objects and worlds**: Dragged from Asset Browser into the scene. Snap to surface under cursor or to grid [^asset].
- **Materials**: Dragged onto object to apply to material slot [^asset].
- **Geometry nodes**: Dragged onto objects to add a Geometry Nodes Modifier [^asset].
- **Import Methods**: Follow Asset/Preferences, Link, Append, Pack. Link creates read-only linked copies; Append creates independent copies; Pack links and immediately packs into current blend-file [^asset].
- **Instance collections**: After dragging a collection into the 3D Viewport, can enable Instance option to add an empty object using an instance of the collection instead of full hierarchy [^asset].
- **Asset types**: Primitive (objects, materials, worlds, collections — linked/appended) and Preset (brushes, pose actions — loaded and activated) [^asset-types].
- **Mark as Asset**: Operator to make data-blocks available via Asset Browser. Found in Outliner context menu, material slots list, or Object › Asset › Mark as Asset [^asset].

**Source**: [Blender 5.3 Manual — Asset Browser](https://docs.blender.org/manual/en/latest/editors/asset_browser.html)

### 1.13 Contextual Density — Panels Appear Only in Relevant Context
- **Mode-dependent panels**: The Available panels in any editor/sidebar depend on the active mode (Object Mode, Edit Mode, Sculpt Mode, etc.) and current selection. Panels relevant to the current context are shown; others are hidden [^regions] [^editors].
- **Selection-dependent**: Properties and panels update based on what is selected. No panels show irrelevant options for the unselected data type [^properties].
- **Editor-type-specific**: Different editors expose completely different region sets (3D Viewport has Toolbar/Sidebar/Adjust Last Operation; Outliner has tree/hierarchy; Properties has tab rail) [^regions].
- **Search as density aid**: Ctrl+F in Sidebar searches within visible panels, highlighting matches and graying out non-matching settings, preserving context while surfacing buried options [^regions].

**Source**: [Blender 5.3 Manual — Regions](https://docs.blender.org/manual/en/latest/interface/window_system/regions.html), [Editors](https://developer.blender.org/docs/features/interface/human_interface_guidelines/editors/)

### 1.14 Non-Blocking Interface — Tools/Options Don't Block User from Using Other Parts
- **Design principle**: Tools and interface options are designed not to block the user from using any other parts of Blender. No requesters that require filling data before execution [^paradigms].
- **Non-modal default**: User input remains consistent and predictable; modes don't change on the fly unexpectedly. Sticky modes are only editing modes (mesh, pose, particle hair) and sticky transform tools. All other modes are "temporary" and end when user stops acting [^paradigms].
- **Clear indication**: When operations block (e.g., rendering/simulations), it is clearly indicated and allows immediate exit [^paradigms].

**Source**: [Blender Developer Documentation — Design Paradigms](https://developer.blender.org/docs/features/interface/human_interface_guidelines/paradigms/)

### 1.15 Select → Operate Principle — First Select Data, Then What to Do
- **Design principle**: In Blender, you first indicate which data you work on, then what you want to do. There is no active tool mode that needs to be set first to use a tool on what you select after. This enables a fast and flexible workflow [^paradigms].

**Source**: [Blender Developer Documentation — Design Paradigms](https://developer.blender.org/docs/features/interface/human_interface_guidelines/paradigms/)

---
---
## 2) Interaction-Philosophy Principles Distilled

### 2.1 Contextual Density
- **Principle**: Panels, tabs, and UI regions appear only when relevant to the current editor type, mode, and selection. Nothing is visible by default; the interface "prunes" itself to show only what applies.
- **Transfer value**: In a 2D CAD app, this means the inspector/property panel should dynamically show/hide fields based on selected entity type (line, circle, group, text, etc.), never showing irrelevant controls. Reduces cognitive load and speeds scanning.
- **Evidence from Blender**: Property tabs hide when no object/mat is selected; sidebar panels change per editor/Mode; Ctrl+F searches within visible panels while graying out non-matches.

### 2.2 Modal Operators with Gestural Commands + Numeric Input
- **Principle**: Core actions (transform) are initiated via gestural keyboard commands (G/R/S), then refined via mouse movement + optional numeric input. Numeric input can be simple or advanced (expressions, units). Axis/Plane locking is available mid-operation.
- **Transfer value**: Lintel's transform tools should follow the same pattern: press T (or generic transform shortcut), drag to transform, type a number for precise value, type X/Y/Z for axis lock, Shift+X/Y/Z for plane lock. The operation should be temporary (end when mouse button released).
- **Evidence**: G/R/S initiate transforms; type number + Return to confirm; X/Y/Z axis lock; Shift+X/Y/Z plane lock; MMB axis constraint during drag.

### 2.3 Redo Panel / Adjust Last Operation — Non-Destructive Parameter Tweaking After the Fact
- **Principle**: The F9 redo panel (Adjust Last Operation) lets users tweak the parameters of the last operator after it has run, without needing to undo. This supports iterative workflow: apply, tweak, repeat.
- **Transfer value**: Lintel's command stack should include a persistent redo/adjust panel accessible after any command execution, allowing adjustment of magnitude, axis, layer, etc. without undo. The panel should persist until the next command runs.
- **Evidence**: F9 opens Adjust Last Operation panel; can change Circle vertices to 3 for equilateral triangle; parameters adjustable after operation runs; repeats until next operation.

### 2.4 Non-Destructive Stacks (Modifier-Style Ordered Operations)
- **Principle**: Operations are stacked in a defined order, each is independently enable/disableable and render-toggleable, and the order is drag-reorderable. The stack is non-destructive until "Apply" is explicitly chosen.
- **Transfer value**: Lintel's operation history (push/pop) should model the modifier stack: each step is an independent module with enable/disable toggles, reorderable by drag. "Apply" makes changes permanent; otherwise all steps remain editable.
- **Evidence**: Modifiers evaluated top to bottom; drag grip to reorder; per-modifier Realtime/Render icons; Apply makes permanent; order changes result.

### 2.5 Workspaces / Screen Areas as Task-Based Persistent Layouts
- **Principle**: Workspaces are predefined layouts of areas/editors, each geared to a specific task. Layouts persist per blend-file and can be switched via tabs. New workspaces can be created from templates.
- **Transfer value**: Lintel should have task-based persistent layouts (e.g., "2D Planning", "2D Detail", "Sheet Layer", "Reference"). Switching layout should change which panels/sidebar sets are visible, which toolbar tools are available, and which keyboard shortcuts are active.
- **Evidence**: Default workspaces (Layout, Modeling, Sculpting, etc.); each has different area configuration; add from template; persist in blend-file; switch via tabs.

### 2.6 Select → Operate (No-Modal-First Principle)
- **Principle**: The user first selects the data, then invokes the operation. There is no "active tool mode" that must be set first. The operation is applied to whatever is currently selected.
- **Transfer value**: Lintel should not require the user to select a "tool mode" before selecting objects. Instead: select objects, then press transform shortcut (G/R/S analogues). The tool is applied to the selection, not the other way around.
- **Evidence**: Blender has no "active tool mode first"; you select then press G/R/S; tools operate on current selection.

### 2.7 Non-Blocking Interface
- **Principle**: Interface elements should never block the user from accessing other parts of the app. No modal dialogs that require dismissal before continuing. If an operation blocks (e.g., long computation), it should be clearly indicated and have an immediate exit.
- **Transfer value**: Lintel's background operations (export, batch process, computation) should have progress indicators with cancel, and should not freeze the UI. Inline operation with immediate feedback is preferred over modal stop-flow dialogs.
- **Evidence**: Blender has no blocking requesters for common operations; viewport remains responsive; render/simulation blocks are clearly indicated with immediate exit.

---
---
## 3) Keyboard Short Philosophy (Table)

| Action / Category | Default Shortcut | Modifiers / Variants | Notes |
|---|---|---|---|
| **Transform: Grab (Translate)** | G | X/Y/Z axis lock during drag; Shift+X/Y/Z plane lock; numeric value after G then Return; MMB drag-to-constrain axis | Core 2D translate; industry keymap uses W instead |
| **Transform: Rotate** | R | X/Y/Z axis lock during drag; numeric value after R then Return; orientation (Global/Local/Normal) selector; double-press same key to toggle Global/Local | Core 2D rotate; industry keymap maps to E in some configs |
| **Transform: Scale** | S | X/Y/Z axis lock during drag; numeric value after S then Return; Ctrl+S may vary by keymap | Core 2D scale; in industry keymap, S is the primary scale shortcut |
| **Axis Lock (mid-drag)** | X / Y / Z | Press once constrains to current transform orientation; press twice constrains to Global; press three times removes constraint; Shift+X/Y/Z locks plane (2 axes) | Works during any transform (G/R/S); MMB also constrains axis after starting transform |
| **Plane Lock (mid-drag)** | Shift+X / Shift+Y / Shift+Z | Locks transformation to two axes (a plane); only affects translation/scale, not rotation | Mid-drag only; rotation always axis-constrained |
| **Undo** | Ctrl+Z | Screen-level; works in all modes/editors | Default; also Cmd+Z on macOS |
| **Redo** | Shift+Ctrl+Z | Screen-level; brings back just undone | Also F9 for Adjust Last Operation |
| **Adjust Last Operation** | F9 | Pops panel to tweak last operator's parameters after it runs | Only works right after the operation; context-sensitive |
| **Repeat Last** | Shift+R | Repeats the last action (duplicate-and-move, etc.) | Works across most operations |
| **Undo History** | Ctrl+Alt+Z (legacy) / Edit › Undo History | Jumps back multiple steps at once; history truncates on new change | Not a single-key shortcut in default keymap; accessible via menu |
| **Mode Switch (pie menu)** | Ctrl+Tab | Brings up mode-switching pie menu in 3D Viewport | Item positions fixed for muscle memory |
| **Sidebar** | N | Toggles right-side panel (context-sensitive) | Visibility depends on editor type + mode + selection |
| **Toolbar** | T | Toggles left-side toolbars | Visibility depends on editor type + mode + selection |
| **Mode: Industry Compatible** | Built-in preset | Maps W/E/R as active tool keys; W=translate, E=extrude (modeling), R=rotate; G/R/S still work as modal operators | Alternative keymap for users migrating from Maya/other 3D apps |
| **Mouse Select** | LMB (default) / RMB | LMB selects; RMB opens context menu | Preference; affects selection across all editors |
| **Spacebar Action** | Tools (default) | Play/Tools/Search — affects what Spacebar does; also affects tooltip shortcut display | In Preferences › Keymap › Spacebar Action |

**Key design insight**: The most portable, cross-application-compatible shortcuts are G/R/S for transforms (default) or W/E/R (industry compatible). Numeric input after shortcut is the primary way to achieve precision. Axis locking (X/Y/Z mid-drag) and plane locking (Shift+X/Y/Z) are essential for CAD precision. Undo/Redo should be screen-level (works everywhere).

---
---
## 4) Candidate Capabilities for Lintel (behavioural summary + acceptance-criteria idea + URL)

| # | Capability | Behavioural Summary | Acceptance-Criteria Idea | Blender Source URL |
|---|---|---|---|---|
| C1 | **Splittable/Resizable Area System** | 2D viewport split into resizable panes (split horizontally/vertically); users can drag borders to adjust widths; Ctrl-snap to convenient sizes; areas persist per-workspace. | User can split the main drawing area into two panes by dragging a border; holding Ctrl snaps to equal division; layout saved with workspace. | [Areas](https://docs.blender.org/manual/en/latest/interface/window_system/areas.html) |
| C2 | **Workspaces — Task-Based Persistent Layouts** | Multiple saved layouts (e.g., "Planning", "Detail", "Sheets"); switching layout changes visible panels, toolbar tools, and active shortcut set; layouts persist per file. | User can switch between two saved layouts via tab shortcut; each layout has different visible panel configuration; new layout can be created from a "blank" template. | [Workspaces](https://docs.blender.org/manual/en/latest/interface/window_system/workspaces.html) |
| C3 | **Outliner Hierarchy with Toggle Filters** | Tree view of layers/collections/objects; visibility eye, lock (select/camera/render) toggles per item; drag-to-reparent; search filter (Ctrl+F) highlights matches, grays out non-matches. | User can toggle visibility/lock on individual layers in the outliner; search filter finds objects by name; dragging an object reparents it. | [Outliner](https://docs.blender.org/manual/en/latest/interface/window_system/outliner.html) |
| C4 | **Properties Inspector — Context-Sensitive, Type-Specific Panels** | Vertical tab rail (icon-only or icon+label) per object type. Panels appear/disappear based on selected entity type. No panels show when nothing is selected. | Select a wall → inspector shows length, layer, material slots. Select a text object → shows font, size, content. Select nothing → inspector is empty/hidden. | [Properties Editor](https://docs.blender.org/manual/en/latest/interface/properties/) |
| C5 | **N-Panel / Sidebar — Contextual Overlay Per Editor** | Right-side panel toggled by N key. Contains tabs for tool settings, view options, and entity-specific panels. Visibility depends on editor context (2D view + mode + selection). | Press N to show/hide sidebar; sidebar tabs change when switching between Select Mode, Edit Mode, Dimension Mode. | [Sidebar/Regions](https://docs.blender.org/manual/en/latest/interface/window_system/regions.html) |
| C6 | **Modal Transform: G/R/S with Numeric Input + Axis/Plane Lock** | Press G/R/S to initiate transform; drag to move; type numeric value + Return to set precise magnitude; type X/Y/Z mid-drag to lock axis; Shift+X/Y/Z to lock plane. | User can translate a selected line by pressing G, dragging, typing 5.0 + Return; user can rotate pressing R, typing 45 + Return; axis lock works mid-drag. | [Numeric Input](https://docs.blender.org/manual/en/latest/scene_layout/object/editing/transform/control/numeric_input.html), [Axis Locking](https://docs.blender.org/manual/en/latest/scene_layout/object/editing/transform/control/axis_locking.html) |
| C7 | **Undo/Redo with Adjustable Parameters (F9 Panel)** | Ctrl+Z / Shift+Z undo/redo history; F9 redo panel persists after operation to tweak parameters; depth configurable (32 default, 256 max). | User can Undo last action (Ctrl+Z); Redo (Shift+Ctrl+Z); After executing "Extrude Line", press F9 to adjust length without undoing; depth settable in preferences. | [Undo & Redo](https://docs.blender.org/manual/en/latest/interface/undo_redo.html) |
| C8 | **Pie Menus for Quick Context Actions** | Radial menus with fixed item positions (8-item limit). Used for mode switching (Ctrl+Tab), transform operations, and quick operator access. Item positions are invariant for muscle memory. | User can open mode pie menu with Ctrl+Tab; User can open transform pie menu (if configured); Items maintain same positions across contexts. | [Pie Menus](https://docs.blender.org/manual/en/latest/interface/controls/buttons/menus.html#pie-menus) |
| C9 | **Tooltips with Short Doc + Shortcut** | Hovering any control shows tooltip with short description, associated shortcut, and current value. Descriptions start with verbs; kept to 2-3 lines. | User hovers over a toolbar button and sees "Move selection (G)" or similar; shortcut displayed; works for all primary controls. | [Tooltips](https://docs.blender.org/manual/en/latest/getting_started/help.html#toolti) |
| C10 | **Non-Blocking UI — No Modal Freeze** | Common operations never block the UI. Long operations show progress with cancel. Interface remains responsive. | User can cancel a lengthy operation; UI remains responsive to input during waiting; clear indication when blocking mode is active. | [Design Paradigms](https://developer.blender.org/docs/features/interface/human_interface_guidelines/paradigms/) |
| C11 | **Select → Operate (No-Modal-First)** | Select data first, then invoke operation. No need to set "active tool mode" before selecting. Operation applies to current selection. | User selects an object, then presses G to translate — no prior mode selection required. Operation applies to whatever is selected. | [Design Paradigms](https://developer.blender.org/docs/features/interface/human_interface_guidelines/paradigms/) |
| C12 | **Asset Browser / Drag-to-Import** | Drag assets (blocks, symbols, predefs) from a browser into the drawing area. Import method (link vs append vs copy) configurable. | User can drag a "symbol" from a library panel into the drawing area; it appears as a new instance; link/copy/append choosable. | [Asset Browser](https://docs.blender.org/manual/en/latest/editors/asset_browser.html) |

**Lintel Already Has (from task description)**:
- Activity bar + panels (similar to Blender's toolbar/sidebar)
- Inspector with sections (similar to Blender's Properties editor vertical tab rail)
- Command palette (search-driven access, analogous to Blender's Menu Search / F3)
- Themes: light/dark/HC (high contrast) — Blender also has full colour theming per editor
- Keyboard-first flows (Blender is keyboard-driven; Lintel aims same)
- Undo/redo (basic; Blender's is more feature-rich with F9 panel and history depth control)

---
---
## 5) Not-Applicable (3D-Only) with Reasons

| # | Feature | Why 3D-Only / Not Transferable to 2D CAD |
|---|---|---|
| N1 | **3D Viewport specific overlays** (X-Ray, Proportional Editing, Viewport Shading, Gizmos) | These rely on 3D spatial navigation, camera, and rasterization. A 2D CAD has no concept of viewport shading or 3D gizmos. |
| N2 | **Sculpt/Edit Mode per-object states** | These are 3D-specific geometry editing modes (sculpting brushes, vertex painting). A 2D CAD would have different editing modes (line/circle edit, text edit) but the concept doesn't map directly. |
| N3 | **Particle Systems & Physics Simulations** | Purely 3D dynamics; no equivalent in 2D CAD workflow. |
| N4 | **Curve/Boolean Modifiers (3D topology changes)** | Modifiers that alter 3D mesh topology (Subdivision, Mirror, Boolean) have no direct 2D analogue. The *stack* concept transfers, but the specific 3D modifiers do not. |
| N5 | **Camera & Viewpoint management** (Numpad views Front/Top/Side/Camera; Frame Selected; View K-O) | Camera navigation is 3D-specific. 2D CAD has viewports and zoom/pan, but numpad viewpoint mapping is meaningless. |
| N6 | **Multiview / Split-View for 3D scenes** (e.g., two 3D viewports showing different angles) | While 2D CAD can have multiple viewports (plan/elevation/detail), the Blender-specific multiview arrangements (quad view, etc.) are 3D-orthogonal-projection patterns. |
| N7 | **Armature/Armature Modifiers & Pose Mode** | Skeleton-based animation system; entirely 3D character rigging. No mapping to 2D CAD. |
| N8 | **Grease Pencil** | 3D drawing in space with Z depth. While Grease Pencil has 2D aspects, it's fundamentally a 3D pipeline feature. |
| N9 | **Render Engine & Result Viewport** (Rendered shading, EEVEE/Cycles viewport shading) | Rendering to an image is 3D-specific. 2D CAD has "display modes" (wireframe/hidden) but not a render pipeline. |
| N10 | **Driver / Physics Constraints** | Constraints based on 3D spatial relationships; no 2D equivalent. |

---
---
## 10-Line Summary of Highest-Value Findings

1. **Contextual density** (panels appear only when relevant) is the single most transferable principle for Lintel's inspector/property panel — it eliminates cognitive load and maps naturally to a 2D CAD's type-sensitive property view.

2. **The modifier stack pattern** (non-destructive, ordered, reorderable, independently enable/disableable steps) provides a direct mental model for Lintel's operation history/undo stack, including the "apply vs. keep editable" dichotomy.

3. **Modal transform with numeric input** (G/R/S + number + Return, axis lock X/Y/Z mid-drag, Shift+X/Y/Z plane lock) is the most immediately useful interaction pattern for precise 2D CAD transformations.

4. **Workspaces as task-based persistent layouts** (switchable via tabs, saved per file, creatable from templates) offers a clean way for Lintel to let users toggle between "Planning", "Detail", and "Sheets" configurations.

5. **F9 Adjust Last Operation / redo panel** — the pattern of letting users tweak the last command's parameters without undo is highly valuable for iterative CAD work and should be ported as a persistent redo panel.

6. **Select → Operate (no-modal-first)** — the principle that the user selects data first, then invokes the operation, eliminates the need for a "tool mode" switch before acting and is more intuitive for CAD workflows.

7. **Pie menus with fixed item positions** (8-item limit, muscle-memory invariant) can be adapted for Lintel's command palette and quick-context actions, providing a radial-access pattern that's faster than linear menus.

8. **Tooltips with short doc + shortcut + value** — every control should hover-help that starts with a verb, stays under 3 lines, and shows the associated keyboard shortcut.

9. **Area splitting/resizing with Ctrl-snap** — the ability to drag borders between panels and have Ctrl snap to equal/alignment sizes is a useful UI pattern for Lintel's workbench shell resizing.

10. **Asset browser drag-and-drop** — the pattern of dragging "symbols" or "blocks" from a library into the drawing area, with link/copy/append choices, directly maps to Lintel's need for reusable components/blocks.