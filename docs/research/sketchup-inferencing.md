# SketchUp Interaction Model Research for Lintel (2D Floor-Plan CAD)

*Compiled from public SketchUp help docs, developer docs, and community sources (2026).
All summaries are original behavioural descriptions + source URLs. No proprietary material is included.*

---

## 1) Behaviours (grouped, each with source URL)

### 1.1 Inference types and hints
- SketchUp displays several inference types: point, linear, and shape. Inferences are often combined (e.g., point+linear). Each inference type has its own color and ScreenTip tooltip.
- **Point inferences** (magenta when inside group/component): Origin point, Endpoint (green circle), Midpoint (light blue circle), Arc Midpoint, Intersection, On Face, On Edge, Center, Guide Point, On Line, On Section, Intersection with Hidden Section.
- **Linear inferences** (temporary dotted lines): On Red/Green/Blue Axis, From Point, Through Point, Parallel, Extend Edge, Perpendicular, Perpendicular to Face, Tangent at Vertex.
- **Shape inferences**: Square, Golden Section, Half/Quarter/Three-Quarter Circle, Arc Side and Center, Circle/Polygon Center.
- Source: [Knowing your inference types](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts)

### 1.2 Inference toggling and locking
- In the Line tool, press **Alt** (Mac: Command) once to start drawing, then press again to cycle inference states: All Inferences On → All Inferences Off → Parallel and Perpendicular Only.
- Arrow keys lock drawing direction/plane to axes: ↑ = Blue, ← = Green, → = Red, ↓ = toggle parallel/perpendicular lock.
- **Shift** locks the drawing direction/plane to the active direction/plane. Shift+Alt/Cmd locks the tool to the same face plane (for Rotate/Protractor).
- Source: [Knowing your inference types](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts) and [Linear Inferencing Toggle](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts)

### 1.3 Typing dimensions while drawing (VCB box)
- While any drawing tool is active, type a value in the Measurements Box (lower right) to fix the segment length immediately after placement. No need to click the box – it becomes active as soon as you move the cursor.
- **Negative values** apply in the direction opposite to the indicated drawing direction.
- **Unit suffixes**: `"` for inches, `'` for feet, `mm`, `cm`, `m`. Without a suffix, the model default units apply.
- For rectangles: `length,width` (comma-separated) sets both dimensions. `value,` sets only length, `,value` sets only width. Negative values move opposite to the drawn direction.
- After a shape is drawn, typing a new value in VCB while still on the same tool operation can modify that dimension – but once you move to another tool, the opportunity is lost.
- Source: [Using the Measurements Box](https://help.sketchup.com/en/using-measurements-box), [Drawing Basic Shapes](https://help.sketchup.com/en/sketchup/drawing-basic-shapes), [How do I create a measured line](https://forums.sketchup.com/t/how-do-i-create-a-measured-line-ie-16/12887)

### 1.4 Push/Pull (one-gesture 3D conversion)
- **Push/Pull tool (P)**: creates volume from a face by extruding it. One click on a face + drag = extrusion depth. Click again to set, or type a precise distance in the Measurements Box.
- Can also **cut** a shape out of the model (subtract volume) by pushing a face through to the other side.
- If the pushed face must be parallel with the opposite face to completely remove content – otherwise leave lines on the opposite face first.
- **Copy mode**: Tap **Ctrl** (Windows) / **Option** (Mac) once before pulling to create a copy, leaving the original face in place. Can duplicate on different faces or stack identical extrusions (Ctrl+Option+double-click).
- **Stretch mode**: A hidden modifier that moves a face perpendicular to its drawing plane, useful for early form development.
- Source: [Pushing and Pulling Shapes into 3D](https://help.sketchup.com/en/sketchup/pushing-and-pulling-shapes-3d), [Secrets of Push/Pull](https://danieltal.com/secrets-of-pushpull/)

### 1.5 Components vs Groups
- **Component Definition** holds the "inside" geometry; all instances reference the same definition. Edit one instance → all instances change.
- **Component Instance** is the outside manifestation with its own transformation matrix. Scaling/rotating a component instance does *not* alter the definition or other instances.
- **Make Unique**: context-click → Make Unique creates a new definition for that instance only, breaking the link to the original.
- **Group**: a special case of Component. Editing a group from the UI silently makes it unique. Group definitions do *not* appear in the Components panel.
- **Rule of thumb**: Use Components for repeated/interchangeable objects (doors, windows). Use Groups for unique local objects (individual slabs, walls).
- Scaling a *definition* is not directly exposed in the UI; you scale the *instance*.
- Source: [Entity Overview](https://developer.sketchup.com/article-entity-overview), [Components](https://help.sketchup.com/en/sketchup/components), [Changing a Single Component Instance](https://help.sketchup.com/en/sketchup/changing-single-component-instance)

### 1.6 Tags (formerly Layers) – visibility only, no z-order
- Tags control visibility of grouped geometry. They do **not** control z-order.
- New geometry is untagged by default. Tags work best on groups and components.
- **Tag folder** (introduced 2021) groups tags hierarchically; folder visibility propagates to contained tags.
- **Per-scene tag visibility**: each scene can independently show/hide tags. "Apply Tags to Scene" captures which tags are visible in that scene.
- **Auto-Invisible Layer**: new tags can be set to be invisible in existing scenes automatically (via extension or 2025 native "Show New Tags" feature).
- Source: [Controlling Visibility with Tags](https://help.sketchup.com/en/sketchup/controlling-visibility-tags), [Entity Overview](https://developer.sketchup.com/article-entity-overview)

### 1.7 Scenes – saved view+tag+style states
- A scene is a named snapshot of camera location, style, tag visibility, section planes, fog, shadows, environment, axes location.
- Scenes appear as tabs at the top of the window. Each can be updated as the model changes.
- Properties saved per scene: Camera Location, Visible Tags, Style and Fog, Shadow Settings, Active Section Planes, Axes Location, etc.
- Updating a scene captures the current view's settings. Scenes do *not* save geometry positions; geometry is model-coordinate fixed.
- Source: [Creating Scenes](https://help.sketchup.com/en/sketchup/creating-scenes), [Entity Overview](https://developer.sketchup.com/article-entity-overview)

### 1.8 Styles – edge/face/background/watermark settings
- A style is a collection of settings for Edge, Face, Background, Watermark, and Modeling (guides/axes visibility).
- **Edge settings**: Edges/Back Edges/Profiles/Depth Cue/Extension/Endpoints/Jitter/Color (All Same/By Material/By Axis).
- **Face settings**: Front/Back face color, X-Ray, Wireframe, Hidden Line, Monochrome, Shaded, Shaded with Textures, Photoreal Materials, Transparency, Ambient Occlusion.
- **Background**: Sky, Horizon, Ground colors; watermarks (images) stretched/tiled/positioned.
- **Modeling**: guides, section cuts, helper tool visibility.
- Styles can be saved in-model or browsed from the library. One style per model can be the "active" style; changes must be explicitly updated with "Update Style with Changes".
- Source: [Creating and Editing a Style](https://help.sketchup.com/en/sketchup/creating-and-editing-style), [Edge Styles](https://help.sketchup.com/en/sketchup/edge-styles), [Face Styles](https://help.sketchup.com/en/sketchup/face-styles)

### 1.9 Measurement tool & Tape Measure – measuring + guides simultaneously
- **Tape Measure (T)** has three modes (cycle with **Ctrl/Option**): Create Guide Lines, Create Guide Points, Measure.
- **Measure mode**: click start point, click end point → distance appears in Measurements Box.
- **Create Guide Lines mode** (with + cursor): click an edge/axis → drag → parallel/dashed guide line appears; click to place.
- **Create Guide Points mode**: click an endpoint/origin → click elsewhere → guide point (x) appears.
- Guides are infinite; too many can affect performance.
- Protractor tool also creates angled guide lines.
- Source: [Using Guides](https://help.sketchup.com/en/using-guides), [Precise Modeling with Measurements](https://help.sketchup.com/en/sketchup/measuring-angles-and-distances-model-precisely)

### 1.10 Guides – finite/infinite dashed lines and points
- Guide lines are infinite dashed lines; cannot be resized. Guide points are standalone inference points.
- Created via Tape Measure (Ctrl/Option to toggle modes) or Protractor.
- Can be moved/rotated like other entities. Can be purged when no longer needed.
- Source: [Using Guides](https://help.sketchup.com/en/using-guides)

### 1.11 Materials – Paint Bucket, sample with Alt, replace vs paint
- **Paint Bucket (B)** tool: click to paint a face/edge with the currently selected material.
- **Alt/Command (held)** while using Paint Bucket → activates **Sample** mode: click a painted face to pick up its material, then paint it onto another face. Sample is a toggle (one use) then returns to Paint Bucket.
- **Shift+click** → replaces the material on *every* matching face in the entire model (global replace).
- **Ctrl+Shift+click** (Mac: Option+Shift) → replaces material only on the same group/component as the clicked face.
- Icon overview: Sample (Alt), All Matching (Shift), All Connected (Ctrl/Option), All on Same Object (Ctrl+Shift/Option+Shift).
- Source: [Applying Materials](https://help.sketchup.com/en/sketchup/applying-materials), [Replacing a Material](https://help.sketchup.com/en/sketchup/replacing-material), [Paint bucket sampling tool](https://forums.sketchup.com/t/paint-bucket-sampling-tool/172047)

### 1.12 Sections – section planes, cut views
- Section planes cut the model to reveal internal geometry. Active section planes can be saved per scene.
- Section cuts can be toggled on/off per scene. Section planes can be moved, rotated, and hidden.
- Source: [Entity Overview](https://developer.sketchup.com/article-entity-overview) (section plane mention)

### 1.13 Keyboard shortcut philosophy
- Single-letter tool switching: L=Line, R=Rectangle, C=Circle, A=Arc, E=Eraser, M=Move, Q=Rotate, S=Scale, P=Push/Pull, B=Paint Bucket.
- Modifiers are tapped once (toggle) or held (constraint while dragging). The status bar at the bottom conveys what modifiers currently do.
- Shortcuts are the same on Windows and Mac; modifier keys differ: Windows uses Ctrl/Alt, macOS uses Option/Cmd. Shift is shared.
- Source: [SketchUp Keyboard Shortcuts: Default Keys and Custom Ones](https://sketchupguide.net/sketchup-keyboard-shortcuts/), [Quick Reference Card 2024 Windows](https://download.sketchup.com/QRC2024/QRC2024-SU-Windows-en.pdf)

### 1.14 Modifier keys during drag
- **Ctrl** (Windows) / **Option** (Mac): toggle copy mode while moving/rotating/pushing-pulling. Leaves original in place, places duplicates.
- **Shift**: lock current inference direction or drawing plane (constrain to axis, parallel/perpendicular). In Move: locks direction; in Rotate: locks plane; in Push/Pull: uniformly scales.
- **Alt** (Windows) / **Cmd** (Mac): auto-fold while moving (allows move even if it means adding extra edges/faces); for Rotate/Protractor, frees the protractor from the inferenced plane.
- **Arrow keys** while dragging: lock to X (→), Y (←), Z (↑), or clear (↓) – for Move, Rotate, Protractor, Tape Measure.
- Source: [Flipping, Mirroring, Rotating and Arrays](https://help.sketchup.com/en/sketchup/flipping-mirroring-rotating-and-arrays), [SketchUp Keyboard Shortcuts](https://sketchupguide.net/sketchup-keyboard-shortcuts/), [Quick Reference Card 2024 Windows](https://download.sketchup.com/QRC2024/QRC2024-SU-Windows-en.pdf)

### 1.15 Axes tool and model origin
- **Axes tool**: places the three colored axes (red, green, blue) at any point in the model. Origin is the intersection of all three.
- **Positioning**: double-click to place axes instantly; or place origin, then red direction, then green/blue tilt.
- **Align to face**: context-click a face → Align Axes. Aligns ground plane to that face's plane.
- **Cardinal directions**: solid line = positive direction, dotted line = negative. Red/East, Green/North (solid) / South (dotted), Blue/Up.
- **Component/Group axes**: each group/component has its own local axes/origin, separate from the model axes. Created via "Create component axes" button or "Change Axes" on a component.
- Source: [Adjusting the Drawing Axes](https://help.sketchup.com/en/sketchup/adjusting-drawing-axes), [Class: Sketchup::Axes](https://ruby.sketchup.com/Sketchup/Axes.html)

### 1.16 Status bar hint system
- The status bar (bottom of window) shows four parts: Geo-location, Model Info, Tooltips, and Measurements.
- **Tooltips** area: while using a tool, shows helpful information about the tool including usage and modifier keys. Updated in 2021.1 to be more consistent across tools.
- **Measurements** area: right side of status bar, shows the Value Control Box (VCB) – dimensional input while drawing, accepts typed values.
- In 2021.1, status bar prompts were standardized across core tools; more updates planned.
- Source: [Navigating the SketchUp Interface](https://help.sketchup.com/en/sketchup/user-interface), [Standardizing SketchUp tools in the latest release](https://sketchup.trimble.com/en/blog/article/standardizing-sketchup-tools-in-the-latest-release)

---

## 2) Inference/snapping model detail

### 2.1 Inference priority
- The inference engine prioritizes alignments based on what the cursor hovers over. Hovering too quickly may miss less-common inferences (e.g., center of face, midpoint of edge). Pausing briefly on an endpoint or edge often reveals deeper inferences (e.g., face center, axis perpendicular to face).
- Inference types have a visual hierarchy: magenta colors indicate geometry inside groups/components. ScreenTips always name the inference (e.g., "Midpoint", "On Face", "On Red Axis").
- Linear inferences sometimes display a temporary dotted line while drawing. Pressing **Alt** (Mac: **Cmd**) cycles the Line tool through: All Inferences On → All Inferences Off → Parallel and Perpendicular Only.
- Source: [Knowing your inference types](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts), [The SketchUp Inference Engine, Decoded](https://justnecessary.substack.com/p/the-sketchup-inference-engine-decoded)

### 2.2 Point vs axis vs plane inferences
- **Point inferences** snap to exact locations: endpoints (green), midpoints (light blue), centers (dark blue for on-face), intersections (red X), guides, on-edge (red square).
- **Linear (axis) inferences** align to the three drawing axes (red/green/blue) or to an inferred edge direction (magenta = parallel/perpendicular). Arrow keys lock direction to a specific axis.
- **Plane inferences** (On Face, Perpendicular to Face) snap to a face's plane, ensuring the cursor stays on that face's plane even when orbiting. This avoids the problem where lines appear on-axis until you orbit to check.
- Source: [Knowing your inference types](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts), [Inference system without mystery](https://typeshare.co/slowinskimaciej/posts/inference-system-without-mystery---part-8-of-beginners-guide-to-using-sketchup-for-kitchen-remodeling/)

### 2.3 Snapping priority rules (community observations)
- "Hover, don't rush" – encouraging an inference by pausing briefly often unlocks hidden ones (face center, midpoint, on-face). Straight-to-target snapping usually grabs the wrong point.
- Endpoints are the most reliable (green, don't move). Midpoints (light blue) are next. On Edge (red square) is less precise but useful anywhere along an edge. Face center (dark blue) often requires pausing near an edge first.
- Source: [The SketchUp Inference Engine, Decoded](https://justnecessary.substack.com/p/the-sketchup-inference-engine-decoded), [Snapping every time](https://3dshouse.com/how-to-snaps-points-in-sketchup-fastest/)

### 2.4 Inference locking with Shift and arrow keys
- **Shift** while drawing locks the current inference direction/plane. Example: drawing a line, hold Shift to lock to the axis the line is currently inferring to.
- **Arrow keys** while dragging: ↑ = Blue axis, ← = Green, → = Red, ↓ = toggle parallel/perpendicular lock. Works for Line, Move, Rotate, Protractor, Tape Measure.
- **Shift+Alt/Cmd** (Windows/Mac) locks the tool to the same face plane while allowing the center/center-of-rotation to move (for Rotate/Protractor).
- Source: [Knowing your inference types](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts), [Linear Inferencing Toggle](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts)

---

## 3) Keyboard shortcut inventory (table)

| Tool | Primary Key | Modifier Keys (while dragging) | Modifier Toggles (tap) | Copy Mode | Lock Direction/Plane |
|------|-------------|-------------------------------|------------------------|-----------|----------------------|
| **Line** | L | Shift = lock current inference direction; Arrow keys = lock to axis (↑Blue, ←Green, →Red, ↓ Perp/Par) | Alt/Cmd = cycle inferencing (All On → All Off → Para/Perp only) | — | Shift locks direction; Alt/Cmd after Shift locks face plane |
| **Rectangle** | R | Shift = lock drawing plane for first edge; Arrow keys = lock axis | — | — | Shift locks plane on first click |
| **Circle** | C | Shift = lock to axis/plane; Space = reverse arc direction | Option/Alt = close arc mode (open/pie/segment) | — | Shift locks direction |
| **Arc** | A | Shift = lock direction; Option/Alt = cycle close mode | — | — | Shift locks direction |
| **Push/Pull** | P | Ctrl/Option = copy mode (creates copy, leaves original); Shift = uniform extrusion depth; Ctrl+Shift (rare) | — | Tap Ctrl/Option once before pulling to copy | Shift = uniform; Ctrl/Option = copy |
| **Move** | M | Ctrl/Option = copy mode (toggle); Shift = lock inference direction; Arrow keys = lock to axis (↑Z, ←Y, →X, ↓ clear); Option/Auto-Fold = allow extra edges/faces | — | Tap Ctrl/Option once to copy and repeat | Shift locks direction; Arrow keys lock axis |
| **Rotate** | Q | Shift = lock rotation plane; Arrow keys = lock axis (↑Blue, ←Green, →Red, ↓ clear); Option/Cmd = free protractor center while keeping plane; Shift+Option/Cmd = free both | — | Tap Ctrl/Option to rotate a copy | Shift locks plane; Option frees center |
| **Scale** | S | Ctrl/Option = scale about center (non-uniform); Shift = uniform scaling (keep proportions) | — | Tap Ctrl/Option to scale non-uniformly | Shift = uniform |
| **Paint Bucket** | B | Alt/Command = sample material (toggle: one-click pick-up, then revert); Shift = global material replace; Ctrl+Shift = replace on same object/group only | — | Sample is a one-use toggle; other modifiers persist | — |
| **Eraser** | E | — | — | — | — |
| **Select** | (no default – usually left-click) | Shift = toggle selection status (add/subtract from selection) | — | — | — |
| **Tape Measure** | T | Ctrl/Option = cycle modes (Create Guide Lines → Create Guide Points → Measure); Shift = constrain to inference; Arrow keys = lock axis | — | — | — |
| **Protractor** | (often accessed via toolbar) | Shift = lock protractor plane; Option/Alt = toggle guide creation; Arrow keys = lock axis | — | — | Shift locks plane |

*Shift is shared across all systems (Windows/Mac). Ctrl↔Option and Alt↔Cmd swap between Windows and macOS.*

Source: multiple – [SketchUp Keyboard Shortcuts](https://sketchupguide.net/sketchup-keyboard-shortcuts/), [Quick Reference Card 2024 Windows](https://download.sketchup.com/QRC2024/QRC2024-SU-Windows-en.pdf), [Modifier keys on Mac and Ruby API confusion](https://forums.sketchup.com/t/modifier-keys-on-mac-and-ruby-api-confusion/34069)

---

## 4) Edge cases & nuances

- **VCB after tool change**: Once you finish a drawing operation and switch to another tool, you can no longer type a new dimension to modify the just-created entity. The VCB opportunity is lost.
- **Negative values**: Type a negative number (e.g., `–24, –24`) while drawing a rectangle to move the corner opposite to the indicated direction. Same applies to line length: typing a negative value after placing a segment flips direction.
- **Unit suffix interaction**: If model units are set to inches, typing `16` is interpreted as `16"`. Typing `16'` is feet. You can mix: `20cm, 3 1/4"` when model units are mm. The decimal separator may be `,` or `;` depending on locale.
- **Inference delay**: The inference engine may not show a tooltip until the cursor rests on a point for ~0.5 seconds. Rushing past a point may miss it. Pausing briefly often reveals deeper inferences (face center, midpoint).
- **Push/Pull offset limit**: If a pushed face snaps to an adjacent face's height, it's the inference system at work. Tap **Ctrl/Option** to override and create a new set of faces (but beware of added interior faces for 3D printing).
- **Component vs Group editing**: Double-clicking a component edits its definition (all instances change). Double-clicking a group silently makes it unique, then edits only that instance. This is a common source of confusion.
- **Tag visibility per scene**: A tag's visibility can be different per scene. If you add a new tag after creating scenes, it becomes visible in all scenes until you update each scene (or use the "Show New Tags" feature).
- **Status bar persistence**: In SketchUp 2021.1, modifier tool states (copy, auto-fold, stretch, etc.) persist across multiple operations. Hitting the modifier key again toggles off; changing tools resets to default.
- **Axes origin per component**: Each component/group can have its own local axes/origin, independent of the model axes. This allows inferencing inside a component that is rotated relative to the model (e.g., a pitched roof).
- **Guide persistence**: Guide lines are infinite and remain in the model until purged. Too many guides can degrade performance. Guide points are standalone inference targets.
- Source: various forum posts and help doc edge cases noted above.

---

## 5) Candidate capabilities for Lintel (behavioural summary + acceptance-criteria idea + URL)

| # | Behaviour (summary) | Acceptance‑criteria idea for Lintel | Source URL |
|---|----------------------|-------------------------------------|------------|
| 5.1 | **Typed lengths mid‑draw** – type a number + unit suffix (e.g., `8'`, `10mm`) immediately after starting a line/rectangle to fix that segment's length; negative values flip direction; comma separates length/width for rectangles. | Lintel should accept a typed length value (with optional unit suffix) while any wall/segment tool is active, and fix that segment's length instantly. Negative values should reverse direction relative to the drawn vector. Unit suffixes default to model units if omitted, but allow overriding (in, ft, mm, cm, m). | [Using the Measurements Box](https://help.sketchup.com/en/using-measurements-box), [How do I create a measured line](https://forums.sketchup.com/t/how-do-i-create-a-measured-line-ie-16/12887) |
| 5.2 | **Inference hints & colour-coded tooltips** – magenta/blue/green/red dots with ScreenTip names (Endpoint, Midpoint, On Face, On Edge, On Red/Green/Blue Axis, Parallel, Perpendicular). | Lintel should display a small coloured hint chip + tooltip name when the cursor hovers on a snap‑able point/edge/face. Hints should be suppressed if the user is typing a value, and re‑appear when typing stops. | [Knowing your inference types](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts) |
| 5.3 | **Inference locking (Shift / arrow keys)** – Shift locks the current inference direction; arrow keys lock to Red/Green/Blue axes; ↓ toggles parallel/perpendicular. | Lintel should support a “lock inference” modifier (e.g., Shift) that fixes the current direction, and arrow‑key or equivalent controls to lock to the 2D cardinal axes (horizontal/vertical). | [Linear Inferencing Toggle](https://help.sketchup.com/en/sketchup/introducing-drawing-basics-and-concepts) |
| 5.4 | **Push/Pull‑style extrusion from a face** – one click+drag on a planar segment extrudes to 3D depth; Ctrl/Option toggles copy mode (duplicate while leaving original). | Lintel should support extruding a wall/segment upward (or into 3rd dimension) from a selected face/segment, with a modifier (e.g., Ctrl) to create a copy/duplicate rather than replace. | [Pushing and Pulling Shapes into 3D](https://help.sketchup.com/en/sketchup/pushing-and-pulling-shapes-3d) |
| 5.5 | **Components/Instance model** – definition/instance; Make Unique; scaling instances without affecting definition. | Lintel should have BlockDef/BlockInstance where editing one instance does not affect others, and a "Make Unique" command creates a new block definition for that instance. Scaling a block instance should not change other instances. | [Components](https://help.sketchup.com/en/sketchup/components), [Changing a Single Component Instance](https://help.sketchup.com/en/sketchup/changing-single-component-instance) |
| 5.6 | **Tags = layers (visibility only, no z‑order)** – tags control visibility; per‑scene tag visibility; tag folders; new-tag auto‑invisibility. | Lintel should have tags/layers that control visibility only (no z‑ordering), with per‑scene visibility state, tag folders for organization, and the ability to hide/show all entities on a layer. | [Controlling Visibility with Tags](https://help.sketchup.com/en/sketchup/controlling-visibility-tags) |
| 5.7 | **Scenes = saved view+style+tag state** – named snapshots of camera, style, visible tags, section cuts. | Lintel should support saving “scenes” (named views) that capture the current 2D floor‑plan view, visible layers, and active style, and allow updating/recalling them. | [Creating Scenes](https://help.sketchup.com/en/sketchup/creating-scenes) |
| 5.8 | **Status‑bar hint system** – contextual prompt at bottom telling user what modifier does next (e.g., “Shift = lock direction”). | Lintel should display a persistent or transient status‑bar hint for the active tool, indicating what modifier key(s) do (lock, copy, etc.). | [Standardizing SketchUp tools in the latest release](https://sketchup.trimble.com/en/blog/article/standardizing-sketchup-tools-in-the-latest-release) |

---

## 6) Not‑applicable (3D-only) with reasons

| Item | Reason it is 3D‑only |
|------|----------------------|
| Push/Pull creating volume from a face | Requires a planar face and extrusion into third dimension; Lintel is 2D floor‑plan only. |
| 3D inference (On Face, Perpendicular to Face, From Point through point in 3D space) | Relies on Z‑axis and true 3D geometry; Lintel works in a single plane. |
| Component definitions that propagate across all instances | Useful for 3D models of repeated objects (furniture, doors); less relevant for 2D where each floor‑plan line may be unique. |
| Scene camera location & field‑of‑view | Camera is a 3D concept; Lintel uses orthogonal 2D views. |
| Section planes / cut views | 3D slicing; Lintel may have simple section-line display but not full section planes. |
| 3D styles (depth cue, back edges, extension lines in perspective) | Styles in 3D include perspective and depth; Lintel 2D styles would be different. |
| Tape Measure guide points in 3D space (not on a single plane) | Guide points can be anywhere in 3D; Lintel would need planar‑constrained guides. |
| Axes tool re‑orientation in 3D (Place on sloped face) | Useful for modeling sloped roofs etc.; Lintel works on a flat floor‑plan plane. |
| Paint Bucket global material replace across entire 3D model | 3D models have many faces with materials; Lintel may have simple line‑color layering. |
| Rotate tool free‑protractor center while keeping plane (Shift+Option/Cmd) | 3D rotation scenario; Lintel would have 2D angle locking. |

---

## 10‑line summary of highest-value findings for a 2D floor‑plan tool

1. **Typed lengths mid‑draw** (VCB) let users fix segment length instantly with a number + optional unit suffix; negative values reverse direction – essential for precise wall lengths.
2. **Colour‑coded inference hints + ScreenTips** (magenta/blue/green/red) instantly tell the user what point/edge/face is being snapped to; pausing briefly on a point reveals deeper inferences (face center, midpoint).
3. **Shift‑lock + arrow‑key axis locking** constrains drawing to horizontal/vertical or parallel‑perpendicular, preventing accidental diagonal walls in a floor‑plan.
4. **Push/Pull‑style extrusion** (with Ctrl/Option copy toggle) could add 3D wall height from a 2D segment, but for pure 2D the copy‑modifier is reusable for duplicating walls.
5. **Component/Instance model** (Make Unique) ensures editing one wall type does not change all others, while retaining a shared definition for identical walls – maps directly to Lintel's BlockDef/BlockInstance.
6. **Tags = visibility only, no z‑order** – layers control what is seen, not stacking order; per‑scene tag visibility lets users toggle layer sets (e.g., “ demolition” vs “new construction”).
7. **Scenes as saved view+style+tag states** – named snapshots capture the current floor‑plan view, visible layers, and active style for quick recall (e.g., “Ground floor – existing”).
8. **Status‑bar hint system** conveys what modifier does next (lock, copy) without forcing the user to memorize key combos; placed at bottom of window, non‑intrusive.
9. **Inference priority: hover‑then‑pause** – the engine often hides less-common inferences (face center, midpoint) until the cursor rests; a “give‑it‑a‑moment” approach improves snap accuracy.
10. **Keyboard shortcuts are shared Windows/Mac with Ctrl↔Option, Alt↔Cmd swap**; Shift is universal – Lintel should follow this pattern for cross‑platform consistency.

---