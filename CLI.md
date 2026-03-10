# firemark — CLI Reference

> A fast, flexible watermarking tool for images and PDFs, written in Rust.

```
firemark <input> [options]
```

`<input>` can be a single file or a folder path. Supported input formats: `.jpg`/`.jpeg`, `.png`, `.pdf`, `.webp`, `.tif`/`.tiff`.

---

## Table of Contents

- [Input / Output](#input--output)
- [Watermark Type](#watermark-type)
- [Content & Templates](#content--templates)
- [Typography](#typography)
- [Position & Layout](#position--layout)
- [Style & Appearance](#style--appearance)
- [PDF-specific](#pdf-specific)
- [Output Quality](#output-quality)
- [Config & Presets](#config--presets)
- [General](#general)
- [Template Variables](#template-variables)
- [Type Reference](#type-reference)
- [Examples](#examples)
- [Crate Dependencies](#crate-dependencies)

---

## Input / Output

| Flag | Description |
|---|---|
| `<input>` | Input file or folder *(required)* |
| `-o, --output <path>` | Output file path. Supported extensions: `.jpg`, `.jpeg`, `.png`, `.pdf`, `.webp`, `.tif`, `.tiff` |
| `-S, --suffix <suffix>` | Append suffix to output filename: `{name}-{suffix}.ext`. Used when `-o` is omitted or with `-R` |
| `-R, --recursive` | Process folders recursively |
| `-j, --jobs <n>` | Number of parallel worker threads for batch processing (default: CPU count) |
| `--overwrite` | Overwrite existing output files without prompting |
| `-n, --dry-run` | Preview operations without writing any files |

---

## Watermark Type

```
-t, --type <type>
```

Controls the **visual design style** of the watermark. Default: `diagonal`.

### Text Styles

| Value | Description |
|---|---|
| `diagonal` | Classic angled text stretched across the page |
| `stamp` | Bold text inside a circle or rectangle border, like a rubber stamp |
| `stencil` | Blocky, cut-out style lettering |
| `typewriter` | Monospace font, slightly uneven, vintage feel |
| `handwritten` | Cursive/script-style font |
| `redacted` | Black bar(s) with text visible through or underneath, like a censored document |

### Shape Styles

| Value | Description |
|---|---|
| `badge` | Text inside a filled shape: circle, hexagon, or ribbon |
| `ribbon` | Diagonal banner across a corner (pairs well with `--position top-right`) |
| `seal` | Circular embossed seal — main text in the center, secondary text on the outer ring |
| `frame` | Repeated text forming a border around the entire page margin |

### Pattern / Repeat Styles

| Value | Description |
|---|---|
| `tile` | Repeated text or image across the whole page in a regular grid |
| `mosaic` | Tiled, but with slight random rotation and offset per instance |
| `weave` | Diagonal alternating rows of text, offset like a textile or fabric pattern |

### Effect Styles

| Value | Description |
|---|---|
| `ghost` | Very low opacity, single centered mark with a subtle embossed look |
| `watercolor` | Soft, blurred edges with a color wash effect |
| `noise` | Text rendered with a grainy or distorted texture overlay |
| `halftone` | Dot-pattern rendering of the text or image |

---

## Content & Templates

| Flag | Description |
|---|---|
| `-m, --main-text <text>` | Primary watermark text (default: `"firemark"`) |
| `-s, --secondary-text <text>` | Secondary text below or around the main text (default: `{timestamp}`) |
| `-I, --image <path>` | Path to an image file to use as or combine with the watermark |
| `--qr-data <string>` | Data to encode as a QR code watermark (URL, plain text, etc.) |
| `--qr-code-position <pos>` | QR code placement: `center` \| `top-left` \| `top-right` \| `bottom-left` \| `bottom-right` (default: `center`) |
| `--qr-code-size <px>` | QR code size in pixels (default: auto-scaled from image dimensions) |
| `--template <string>` | Full text template using variables (see [Template Variables](#template-variables)) |

---

## Typography

| Flag | Description |
|---|---|
| `-f, --font <name\|path>` | Font name or path to a `.ttf` / `.otf` file (default: built-in sans-serif) |
| `--font-size <pt>` | Font size in points (default: auto-scaled to canvas) |
| `--font-weight <weight>` | `thin` \| `light` \| `regular` \| `bold` \| `black` (default: `regular`) |
| `--font-style <style>` | `normal` \| `italic` (default: `normal`) |
| `--letter-spacing <px>` | Extra spacing between characters in pixels (default: `0`) |

---

## Position & Layout

| Flag | Description |
|---|---|
| `-p, --position <pos>` | Placement: `center` \| `top-left` \| `top-right` \| `bottom-left` \| `bottom-right` \| `tile` (default: `center`) |
| `-r, --rotation <degrees>` | Rotation angle, e.g. `-45` or `30` (default: `-45`) |
| `--margin <px>` | Edge margin in pixels when not tiling (default: `20`) |
| `--scale <0.0–1.0>` | Watermark size relative to the canvas width (default: `0.4`) |
| `--tile-spacing <px>` | Gap between tiles when using `--type tile` or `mosaic` (default: `80`) |
| `--tile-rows <n>` | Force a fixed number of tile rows (overrides `--tile-spacing` vertically) |
| `--tile-cols <n>` | Force a fixed number of tile columns (overrides `--tile-spacing` horizontally) |
| `--offset <x,y>` | Manual pixel offset from the anchor position, e.g. `10,-5` (default: `0,0`) |

---

## Style & Appearance

| Flag | Description |
|---|---|
| `-c, --color <color>` | Watermark color — named (`red`, `gray`, …) or hex `#RRGGBB` (default: `#808080`) |
| `-O, --opacity <0.0–1.0>` | Overall watermark opacity (default: `0.5`) |
| `-b, --background <pattern>` | Background pattern behind watermark: `none` \| `grid` \| `dots` \| `lines` \| `crosshatch` (default: `none`) |
| `--bg-color <color>` | Background pattern color (default: `#CCCCCC`) |
| `--bg-opacity <0.0–1.0>` | Background pattern opacity (default: `0.15`) |
| `--blend <mode>` | Blend mode: `normal` \| `multiply` \| `screen` \| `overlay` \| `soft-light` (default: `normal`) |
| `--border` | Draw a border/outline around the watermark text or image |
| `--border-color <color>` | Border color (default: same as `--color`) |
| `--border-width <px>` | Border stroke width in pixels (default: `1`) |
| `--border-style <style>` | Border line style: `solid` \| `dashed` \| `dotted` (default: `solid`) |
| `--shadow` | Add a drop shadow to the watermark |
| `--shadow-color <color>` | Shadow color (default: `#000000`) |
| `--shadow-offset <x,y>` | Shadow offset in pixels (default: `2,2`) |
| `--shadow-blur <px>` | Shadow blur radius in pixels (default: `4`) |
| `--shadow-opacity <0.0–1.0>` | Shadow opacity (default: `0.4`) |
| `--invert` | Render the watermark in inverted color relative to the canvas underneath |
| `--grayscale` | Force watermark to render in grayscale regardless of `--color` |
| `--filigrane <style>` | Cryptographic security overlay: `full` \| `guilloche` \| `rosette` \| `crosshatch` \| `border` \| `lissajous` \| `moire` \| `spiral` \| `mesh` \| `plume` \| `constellation` \| `ripple` \| `none` (default: `guilloche`) |

---

## PDF-specific

| Flag | Description |
|---|---|
| `--pages <range>` | Pages to watermark — e.g. `1`, `1,3,5`, `2-6`, `1,3-5,8` or `all` (default: `all`) |
| `--skip-pages <range>` | Pages to explicitly skip, using same range syntax |
| `--layer-name <name>` | Name of the PDF Optional Content Group (OCG) layer (default: `"Watermark"`) |
| `--no-flatten` | Disable layer flattening (layers are flattened by default for security, making the watermark non-removable) |
| `--behind` | Place the watermark behind existing PDF content instead of on top |

---

## Output Quality

| Flag | Description |
|---|---|
| `-q, --quality <1–100>` | JPEG output quality (default: `90`) |
| `--dpi <n>` | Output DPI resolution for raster formats (default: `150`) |
| `--strip-metadata` | Strip EXIF and XMP metadata from the output file |
| `--png-compression <0–9>` | PNG compression level (default: `6`) |
| `--color-profile <path>` | Embed an ICC color profile in the output |

---

## Config & Presets

| Flag | Description |
|---|---|
| `--config <path>` | Load options from a TOML configuration file |
| `--preset <name>` | Use a named preset defined in the config file (e.g. `--preset draft`) |
| `--save-preset <name>` | Save the current flags as a named preset into the config file |
| `--list-presets` | List all available presets in the config file |
| `--show-config` | Print the resolved config (merged file + flags) and exit |

### Example `firemark.toml`

```toml
[preset.draft]
watermark_type = "stamp"
main_text = "DRAFT"
color = "#FF0000"
opacity = 0.4
rotation = -30
border = true

[preset.confidential]
watermark_type = "tile"
main_text = "CONFIDENTIAL"
secondary_text = "{date}"
color = "#808080"
opacity = 0.25
tile_spacing = 120
rotation = -45
filigrane = "rosette"
```

---

## General

| Flag | Description |
|---|---|
| `-v, --verbose` | Print detailed per-file progress and operation info |
| `--quiet` | Suppress all output except errors (conflicts with `-v`) |
| `--log <path>` | Write log output to a file in addition to stdout |
| `--no-color` | Disable colored terminal output |
| `-V, --version` | Print version information and exit |
| `-h, --help` | Print help message and exit |

---

## Template Variables

Use these variables inside `--main-text`, `--secondary-text`, or `--template`:

| Variable | Resolves to |
|---|---|
| `{timestamp}` | Full ISO 8601 datetime at processing time, e.g. `2026-03-07T14:30:00Z` |
| `{date}` | Date only, e.g. `2026-03-07` |
| `{time}` | Time only, e.g. `14:30:00` |
| `{filename}` | Source filename without extension, e.g. `report_v2` |
| `{ext}` | Source file extension, e.g. `pdf` |
| `{author}` | Current OS username |
| `{hostname}` | Machine hostname |
| `{page}` | Current page number *(PDF only)* |
| `{total_pages}` | Total page count *(PDF only)* |
| `{uuid}` | Random UUID generated once per file, e.g. `f47ac10b-58cc…` |
| `{counter}` | Auto-incrementing integer across batch output (e.g. `001`, `002`, …) |

---

## Type Reference

Quick pairing guide for `--type` and compatible flags:

| Type | Recommended Flags |
|---|---|
| `diagonal` | `--rotation -45`, `--scale 0.6`, `--opacity 0.3` |
| `stamp` | `--border`, `--rotation 0`, `--font-weight bold` |
| `stencil` | `--font-weight black`, `--letter-spacing 4`, `--opacity 0.5` |
| `typewriter` | `--font <monospace path>`, `--font-style normal` |
| `handwritten` | `--font <script path>`, `--rotation -5` |
| `redacted` | `--color #000000`, `--opacity 1.0`, `--pages` |
| `badge` | `--border`, `--color`, `--scale 0.2`, `--position bottom-right` |
| `ribbon` | `--position top-right`, `--color`, `--font-weight bold` |
| `seal` | `--main-text`, `--secondary-text` (ring text), `--scale 0.35` |
| `frame` | `--font-size`, `--letter-spacing`, `--opacity 0.2` |
| `tile` | `--tile-spacing`, `--rotation -45`, `--opacity 0.2` |
| `mosaic` | `--tile-spacing`, `--rotation` (base angle, randomized ±15°) |
| `weave` | `--tile-rows`, `--tile-cols`, `--opacity 0.25` |
| `ghost` | `--opacity 0.05–0.15`, `--blend overlay`, `--scale 0.7` |
| `watercolor` | `--color`, `--opacity 0.4`, `--shadow --shadow-blur 12` |
| `noise` | `--color`, `--opacity 0.5` |
| `halftone` | `--color`, `--scale 0.5` |

---

## Examples

```bash
# Diagonal "CONFIDENTIAL" watermark tiled across a PDF
firemark report.pdf -t tile -m "CONFIDENTIAL" -r -45 -O 0.3 -o out.pdf

# Stamp-style "DRAFT" in red, rotated, with border
firemark brief.pdf -t stamp -m "DRAFT" -c "#FF0000" -r -30 --border -o brief-draft.pdf

# Batch watermark a folder with suffix, 4 parallel jobs
firemark ./docs -R -t diagonal -m "INTERNAL" -S watermarked -j 4

# Seal with author name on the ring, bottom-right corner
firemark photo.png -t seal -m "© Acme Corp" -s "{author} · {date}" -p bottom-right -o signed.png

# Ghost watermark on all pages except the cover
firemark deck.pdf -t ghost -m "PREVIEW" --skip-pages 1 --opacity 0.08 -o deck-preview.pdf

# Mosaic watermark with a logo image
firemark presentation.pdf -t mosaic -I logo.png --tile-spacing 100 -O 0.2 -o presentation-wm.pdf

# QR code watermark linking to a verification URL
firemark invoice.pdf -t badge --qr-data "https://verify.example.com/abc123" -p bottom-right -o invoice-verified.pdf

# Use a saved config preset
firemark draft_v2.pdf --preset confidential --pages 1,3-5 -o final.pdf

# Typewriter watermark with timestamp, strip metadata
firemark scan.jpg -t typewriter -m "SCANNED" -s "{timestamp}" --strip-metadata -o scan-wm.jpg

# Save current flags as a reusable preset
firemark dummy.pdf -t stamp -m "APPROVED" -c "#00AA00" --border --save-preset approved
```

---
