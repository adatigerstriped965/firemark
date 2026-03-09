#!/usr/bin/env bash
# Generate test images for all watermark types and key option combinations.
# Run from the project root: bash tests/generate_examples.sh

set -euo pipefail

BIN="cargo run --release --"
INPUT="examples/input/sample.png"
OUT="examples/output"

mkdir -p "$OUT"

echo "=== Building release binary ==="
cargo build --release

echo "=== Generating all 17 types (with -m and -s) ==="

TYPES=(diagonal stamp stencil typewriter handwritten redacted badge ribbon seal frame tile mosaic weave ghost watercolor noise halftone)

for t in "${TYPES[@]}"; do
    echo "  -> $t"
    $BIN "$INPUT" -t "$t" -m "CONFIDENTIAL" -s "Do not distribute" -o "$OUT/${t}.png" 2>/dev/null
done

echo "=== Generating option variants ==="

# Color variants
echo "  -> diagonal (red)"
$BIN "$INPUT" -t diagonal -m "CONFIDENTIEL" -s "Ne pas distribuer" --color "#CC0000" -o "$OUT/diagonal_red.png" 2>/dev/null

echo "  -> stamp (blue)"
$BIN "$INPUT" -t stamp -m "DRAFT" -s "2026-03-07" --color "#1565C0" -o "$OUT/stamp_blue.png" 2>/dev/null

# Opacity variants
echo "  -> tile (high opacity)"
$BIN "$INPUT" -t tile -m "INTERNAL" -s "Restricted" -O 0.8 -o "$OUT/tile_opaque.png" 2>/dev/null

echo "  -> tile (low opacity)"
$BIN "$INPUT" -t tile -m "INTERNAL" -s "Restricted" -O 0.25 -o "$OUT/tile_subtle.png" 2>/dev/null

# Position variants
echo "  -> badge (top-left)"
$BIN "$INPUT" -t badge -m "APPROVED" -s "Quality control" --color "#2E7D32" --position top-left -o "$OUT/badge_topleft.png" 2>/dev/null

echo "  -> ribbon (bottom-left)"
$BIN "$INPUT" -t ribbon -m "CERTIFIED" --color "#1565C0" --position bottom-left -o "$OUT/ribbon_bottomleft.png" 2>/dev/null

# Scale variants
echo "  -> seal (large scale)"
$BIN "$INPUT" -t seal -m "VERIFIED" -s "Official copy" --scale 0.7 -o "$OUT/seal_large.png" 2>/dev/null

echo "  -> seal (small scale)"
$BIN "$INPUT" -t seal -m "VERIFIED" -s "Official copy" --scale 0.2 -o "$OUT/seal_small.png" 2>/dev/null

# Rotation variants
echo "  -> diagonal (steep rotation)"
$BIN "$INPUT" -t diagonal -m "COPY" -s "Do not reproduce" --rotation -60 -o "$OUT/diagonal_steep.png" 2>/dev/null

echo "  -> diagonal (horizontal)"
$BIN "$INPUT" -t diagonal -m "COPY" -s "Do not reproduce" --rotation 0 -o "$OUT/diagonal_horiz.png" 2>/dev/null

# Font weight
echo "  -> stencil (bold)"
$BIN "$INPUT" -t stencil -m "TOP SECRET" -s "Classified" --font-weight bold -o "$OUT/stencil_bold.png" 2>/dev/null

# Main text only (no secondary)
echo "  -> diagonal (no secondary)"
$BIN "$INPUT" -t diagonal -m "WATERMARK" -o "$OUT/diagonal_nosec.png" 2>/dev/null

# Filigrane variants
echo "  -> stamp (no filigrane)"
$BIN "$INPUT" -t stamp -m "DRAFT" -s "2026-03-07" --filigrane none -o "$OUT/stamp_nofiligrane.png" 2>/dev/null

echo "  -> diagonal (guilloche only)"
$BIN "$INPUT" -t diagonal -m "CONFIDENTIEL" --filigrane guilloche -o "$OUT/diagonal_guilloche.png" 2>/dev/null

echo "  -> diagonal (rosette only)"
$BIN "$INPUT" -t diagonal -m "CONFIDENTIEL" --filigrane rosette -o "$OUT/diagonal_rosette.png" 2>/dev/null

echo "  -> diagonal (crosshatch only)"
$BIN "$INPUT" -t diagonal -m "CONFIDENTIEL" --filigrane crosshatch -o "$OUT/diagonal_crosshatch.png" 2>/dev/null

echo "  -> diagonal (border only)"
$BIN "$INPUT" -t diagonal -m "CONFIDENTIEL" --filigrane border -o "$OUT/diagonal_border.png" 2>/dev/null

# PDF test
echo "  -> PDF diagonal"
$BIN "examples/input/sample.pdf" -t diagonal -m "CONFIDENTIAL" -s "Do not distribute" --color "#CC0000" -o "$OUT/diagonal.pdf" 2>/dev/null

echo "  -> PDF stamp"
$BIN "examples/input/sample.pdf" -t stamp -m "CONFIDENTIAL" -s "Internal use only" --color "#1565C0" -o "$OUT/stamp.pdf" 2>/dev/null

echo "  -> PDF badge"
$BIN "examples/input/sample.pdf" -t badge -m "APPROVED" -s "Quality control" --color "#2E7D32" -o "$OUT/badge.pdf" 2>/dev/null

echo ""
echo "=== Done! Generated $(ls -1 "$OUT" | wc -l | tr -d ' ') files in $OUT ==="
ls -la "$OUT"
