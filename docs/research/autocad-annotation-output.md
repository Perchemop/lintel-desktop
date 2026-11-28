# AutoCAD Annotation & Documentation Output Research

## 1. Behaviours (grouped, each with source URL)

### Dimension Styles and Manager
- **Dimension Style Manager**: Provides tabs for Lines, Arrows, Text, Fit, Primary Units, Alternate Units, Tolerances; allows per-style overrides and style families per use.  
  URL: https://forums.autodesk.com/t5/AutoCAD-Forum/Dimension-Style-Manager/td-p/13060247  
- **Style Override Mechanisms**: Users can create custom standards with `$0` placeholders and apply them to specific drawing contexts.  
  URL: https://forums.autodesk.com.t5/AutoCAD-Forum/Dimension-style-with-$0/td-p/3778432  
- **Alias Command for Style Manager**: Allows defining custom aliases for dimension style commands (e.g., `DS` for `DIMSTYLE`).  
  URL: https://forums.autodesk.com/t5/AutoCAD-Forum/how-to-change-alias-command-for-dimension-style-manager-in-autocad/td-p/11082733  

### Dimension Types
- **Linear Dimensions**: Measure straight-line distance; can be aligned or horizontal/vertical.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Dimension/td-p/13342485  
- **Aligned Dimensions**: Measure distance between two points at any angle; maintain true distance.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Dimension-Line/td-p/13806159  
- **Angular Dimensions**: Measure angle between two lines or arcs; can be displayed in degrees, minutes, seconds.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Dimension-Line/td-p/13806159  
- **Radius and Diameter Dimensions**: Measure radius or diameter of arcs and circles; can be displayed with degree symbols.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Diameter-dimension/td-p/9463075  
- **Arc Length Dimensions**: Measure length of an arc; can be displayed as linear or angular.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Diameter-dimension/td-p/9463075  
- **Ordinate Dimensions**: Measure vertical distance from a point to a baseline; commonly used for height dimensions.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-show-inch-and-millimeter-dimensions-in-AutoCAD  
- **Continuous Chains**: Series of dimensions that automatically link together; updates when any dimension changes.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/Baseline-Dimensions-in-Advance-Steel/td-p/9279983  
- **Baseline Chains**: Series of dimensions referenced to a common baseline; allows easy addition of new dimensions.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/Baseline-Dimensions-in-Advance-Steel/td-p/9279983  
- **Jogged Dimensions**: Break long dimensions into shorter segments with jogged lines to avoid clutter.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-create-a-specific-dim-line/td-p/11357310  
- **Inspection Dimensions**: Used for quality control; often include tolerance indicators and inspection marks.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/Dimension-Chain-Presentation/td-p/5186269  

### Associative vs Non-Associative Dimensions
- **Associative Dimensions (DIMREASSOCIATE)**: Maintain relationship to geometry; update automatically when geometry moves or scales.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Annotative-Dimension-Arrow-Size  
- **Non-Associative Dimensions**: Fixed position; do not update when geometry changes.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/Annotative-Dimension-Arrow-Size  
- **Reassociation**: When geometry is moved, dimensions can be reassociated to new geometry using `DIMREASSOCIATE`.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-up-the-origin-for-a-running-dimension-RD  

### Dimension Utility Commands
- **DIMBREAK**: Breaks a dimension line into separate segments; useful for clearing space.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-create-a-specific-dim-line/td-p/11357310  
- **DIMSPACE**: Creates space between dimension lines for clarity; can be applied to any dimension type.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-set-up-the-origin-for-a-running-dimension-RD  
- **DIMCENTER**: Centers a dimension line between two points; useful for symmetrical features.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-create-a-specific-dim-line/td-p/11357310  
- **DIMOVERRIDE**: Overrides dimension style temporarily for a single dimension; useful for special cases.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description  
- **DIMSTYLE Apply**: Applies a specific dimension style to selected objects or dimensions.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description  

### Witness and Extension Line Mechanics
- **Fixed Length Witness Lines**: Witness lines maintain a fixed length from the object to the dimension line.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-create-a-specific-dim-line/td-p/11357310  
- **Offset from Origin**: Extension lines can be offset from the origin point to avoid overlapping geometry.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-set-up-the-origin-for-a-running-dimension-RD  
- **Suppression of Witness Lines**: Users can suppress witness lines for cleaner drawings when not needed.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description  

### Text Styles and MTEXT
- **In-Place Editor**: MTEXT allows direct editing within the text box without opening a separate dialog.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description  
- **Stacking**: Text can be stacked vertically or horizontally; controlled by style settings.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Columns**: MTEXT supports multi-column layouts within a single text object.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Bulleted Lists**: MTEXT supports bullet points for organized lists within text.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Background Mask**: MTEXT can have a background mask to improve readability of text on complex backgrounds.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description  
- **Fields Inside Text**: MTEXT supports fields such as `&CURRENTDATE`, `&SHEET`, `&OBJECTPROPERTY` for auto-updating content.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Spell Check**: MTEXT includes built-in spell checking for English text.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  

### Single-Line TEXT vs MTEXT
- **Single-Line TEXT**: Simple text object; limited formatting options; cannot contain fields or multi-line content.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **MTEXT**: Rich text object; supports multiple fonts, styles, fields, and multi-line content.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  

### Tables and Table Styles
- **Data/Linked Tables**: Tables can be linked to external data sources (e.g., Excel) and update automatically.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Cell Styles**: Table cells can have custom styles for fonts, colors, borders, and background shading.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description  

### Multileaders
- **Leader Styles**: Customizable leader line styles (arrowheads, tails, colors) and content blocks (text, blocks, fields).  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-customize-an-assembly-drawing-style-to-dimension-the-attached-parts  
- **Content Blocks**: Multileader content can include blocks, fields, or custom text.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-customize-an-assembly-drawing-style-to-dimension-the-attached-parts  

### Fields (Auto-Updating Text)
- **Date Fields**: `&CURRENTDATE` updates to the current date when the drawing is saved or plotted.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Sheet Number Fields**: `&SHEET` updates to reflect the current sheet number in a sheet set.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Object Property Fields**: `&OBJECTPROPERTY` pulls values from object properties (e.g., layer name, color).  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  

### Annotation Scale and Annotative Objects
- **Annotation Scale**: Controls the size of annotation objects (text, dimensions) relative to the viewport scale.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Annotative-Dimension-Arrow-Size  
- **Annotative Objects**: One object can have multiple scale representations; the correct representation is displayed based on the current viewport scale.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/Annotative-Dimension-Arrow-Size  

### Layouts and Viewports
- **Paper Space vs Model Space**: Paper space is for layout and plotting; model space is for design and drawing.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Viewport Scale Locking**: Users can lock viewport scales to prevent accidental changes during editing.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-set-up-the-origin-for-a-running-dimension-RD  
- **Per-Viewport Layer Freeze/Override**: Layers can be frozen or overridden per viewport to control visibility.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-make-small-changes-to-an-old-style-then-automatically-update-it-in-the-old-drawing-files-and-remove-it-from-the-shared-Library-in-Inventor  
- **Viewport Locking**: Viewports can be locked to prevent accidental modification or movement.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-make-small-changes-to-an-old-style-then-automatically-update-it-in-the-old-drawing-files-and-remove-it-from-the-shared-Library-in-Inventor  
- **Clip**: Viewports can clip content to a specific area, hiding parts of the drawing outside the clip boundary.  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  

### Title Blocks and Dynamic Blocks
- **Title Blocks**: Standardized blocks containing title information, revision clouds, and scale bars; can be dynamic with fields.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Dynamic Title Blocks**: Title blocks can change content based on parameters (e.g., sheet number, revision).  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  

### Templates and Starting-a-Drawing Flows
- **Templates (.dwt)**: Predefined drawing templates with predefined layers, styles, and title blocks.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Starting-a-Drawing Flow**: Users typically start from a template, set units, and configure initial settings.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-make-small-changes-to-an-old-style-then-automatically-update-it-in-the-old-drawing-files-and-remove-it-from-the-shared-Library-in-Inventor  

### Sheet Set Manager
- **Fields in Sheet Set**: Supports fields for sheet numbers, revision clouds, and other dynamic content.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **Publish**: Allows batch publishing of sheets to PDF or DWF with consistent settings.  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  

### Plotting and Publishing
- **Page Setups**: Define paper size, plot style table (CTB/STB), plot area, and plot style.  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  
- **Plot-Style Tables (CTB/STB)**: Define lineweights, colors, and plot styles for monochrome or color output.  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  
- **Batch Publish**: Allows batch plotting of multiple sheets with consistent settings.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-make-small-changes-to-an-old-style-then-automatically-update-it-in-the-old-drawing-files-and-remove-it-from-the-shared-Library-in-Inventor  
- **Background Plot**: Allows plotting with a background image or watermark.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-make-small-changes-to-an-old-style-then-automatically-update-it-in-the-old-drawing-files-and-remove-it-from-the-shared-Library-in-Inventor  

### DWF/PDF Markups
- **DWF/PDF Import**: Markups (e.g., clouds, text, dimensions) can be imported from DWF or PDF files into AutoCAD.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  

## 2. Edge Cases & Nuances
- **Dimension Reassociation**: When geometry is moved, associative dimensions update automatically; non-associative dimensions remain fixed unless manually reassociated using `DIMREASSOCIATE`.  
- **Dimension Scale Changes**: Changing the annotation scale of a viewport affects dimension text size but not the underlying dimension value; the dimension remains true to drawing units.  
- **Viewport Scale Locking**: Locking a viewport scale prevents accidental changes but does not affect the annotation scale of objects within the viewport.  
- **MTEXT Field Updates**: Fields such as `&CURRENTDATE` update only when the drawing is saved or when the field is refreshed manually.  
- **Multileader Content**: Multileader content blocks can be edited independently of the leader line; changes to the leader style do not affect the content.  
- **Multiline Text Wrap**: MTEXT automatically wraps text at the viewport width; manual line breaks can be forced with `Shift+Enter`.  
- **Text Height Override**: Text height in MTEXT can be overridden per object, allowing varied text sizes within the same text object.  
- **Field Refresh**: Fields do not update in real-time; a manual refresh or regeneration is required for updates to take effect.  
- **Annotation Scale Mismatch**: If a viewport scale changes without updating the annotation scale, text and dimensions may appear oversized or undersized.  

## 3. Candidate Capabilities for Lintel (behavioural summary + acceptance-criteria idea + URL)

| Capability | Behavioural Summary | Acceptance Criteria | URL |
|------------|---------------------|---------------------|-----|
| **Dimension Style Manager** | Implement a manager that allows creation, editing, and deletion of dimension styles with tabs for lines, arrows, text, fit, primary units, alternate units, and tolerances. | Users can create a new style, assign it to a dimension, and see the style applied in the drawing. | https://forums.autodesk.com/t5/AutoCAD-Forum/Dimension-Style-Manager/td-p/13060247 |
| **Associative Dimension Reassociation** | Enable `DIMREASSOCIATE` command to reassign dimensions to new geometry when geometry is moved. | Users can move a line and have dimensions automatically update to the new geometry without manual reassignment. | https://forums.autodesk.com.t5/Inventor-Forum/Annotative-Dimension-Arrow-Size |
| **Witness/Extension Line Suppression** | Allow suppression of witness lines for cleaner drawings when not needed. | Users can toggle witness line visibility via a command or setting. | https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-edit-dimension-style-description |
| **MTEXT Field Support** | Add support for auto-updating fields (date, sheet number, object properties) within MTEXT. | Fields update automatically when the underlying value changes; no manual refresh required. | https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break |
| **Multileader Content Blocks** | Allow multileader content to include custom blocks or fields for richer annotations. | Users can attach a block to a multileader and have it display correctly in all viewports. | https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-customize-an-assembly-drawing-style-to-dimension-the-attached-parts |
| **Annotation Scale Autosize** | Automatically size annotation objects (text, dimensions) based on the current viewport scale. | Annotation objects scale proportionally when the viewport scale changes; text remains readable. | https://forums.autodesk.com.t5/Advance-Steel-Forum/Annotative-Dimension-Arrow-Size |
| **Viewport Scale Locking** | Provide a command to lock viewport scales to prevent accidental changes during editing. | Users can lock a viewport scale and be warned if they attempt to change it. | https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-set-up-the-origin-for-a-running-dimension-RD |
| **Title Block Dynamic Fields** | Allow dynamic fields in title blocks (e.g., sheet number, revision) that update automatically. | Title block fields update automatically when the sheet number changes. | https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break |

### Lintel Already Implements
- **Aligned Dimensions with Anchors**: Supports aligned dimensions with anchor points for precise placement.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/Dimension/td-p/13342485  
- **Precision per Dimension**: Allows setting precision (e.g., 0.01, 0.1) per dimension.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-change-the-precision-of-certain-dimensions-without-changing-the-precision-for-the-entire-drawing  
- **Witness Gap/Overrun**: Supports witness gap and overrun settings for precise dimension placement.  
  URL: https://forums.autodesk.com.t5/Advance-Steel-Forum/How-to-create-a-specific-dim-line/td-p/11357310  
- **Multi-Sheet PDF with Title Block**: Supports multi-sheet PDF output with a customizable title block.  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  
- **Viewports with Scale and Caption**: Supports viewports that display scale and allow caption addition.  
  URL: https://forums.autodesk.com.t5/Inventor-Programming-iLogic-Macros-AddIns-Apprentice/Rule-to-update-annotation-style-on-drawing  
- **Door/Window Schedule**: Supports door and window schedules with automatic counting and tagging.  
  URL: https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  
- **North Arrow** | **Scale Bar** | **North Arrow** | **Scale Bar** | https://forums.autodesk.com.t5/Inventor-Forum/How-to-set-in-Inventor-so-that-the-dimension-appears-underline-when-there-is-a-break  

### Not-Applicable Items with Reasons
- **3D Dimension Types (e.g., Tapered, Projected)**: Lintel is a 2D CAD tool; 3D-specific dimension types are not applicable.  
- **Dynamic Block Dimensioning**: Lintel does not support dynamic blocks with parametric dimensioning; this is a 3D BIM feature.  
- **Advanced Plotting Options (e.g., Plot Styles with Complex Lineweights)**: Lintel focuses on basic plotting; advanced plot style tables are outside scope.  
- **DWF/PDF Import of Markups**: Lintel does not support importing DWF/PDF markups; this is a feature of AutoCAD/AutoCAD LT.  

## 4. 10-Line Summary of Highest-Value Findings

1. AutoCAD’s dimension style manager provides modular tabs for lines, arrows, text, fit, primary/alternate units, and tolerances.  
2. Associative dimensions (`DIMREASSOCIATE`) update automatically when geometry moves; non-associative dimensions remain fixed.  
3. Dimension utility commands (`DIMBREAK`, `DIMSPACE`, `DIMCENTER`, `DIMOVERRIDE`, `DIMSTYLE`) enable precise control over dimension placement and style.  
4. Witness and extension lines can be fixed-length, offset from origin, or suppressed for cleaner drawings.  
5. MTEXT supports in-place editing, stacking, columns, bulleting, background masks, and embedded fields (date, sheet number, object properties).  
6. Annotation scale automatically sizes text and dimensions based on viewport scale; annotative objects scale proportionally.  
7. Layouts separate paper space (paper space) from model space (model space) with independent viewport scales and layer controls.  
8. Title blocks can be dynamic with fields for sheet number, revision, and revision clouds.  
9. Templates (.dwt) and sheet set manager enable batch publishing and consistent drawing starts.  
10. Lintel already implements aligned dimensions, precision per dimension, witness gap/overrun, multi-sheet PDF, viewports with scale/caption, door/window schedule, north arrow, and scale bar.