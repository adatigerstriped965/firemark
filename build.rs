fn main() {
    // These messages are displayed during `cargo install firemark`.
    // Update them with each release.
    println!("cargo:warning=");
    println!("cargo:warning=firemark v0.1.3 — Release Notes");
    println!("cargo:warning=─────────────────────────────────");
    println!("cargo:warning=  New:");
    println!("cargo:warning=  • Anti-AI strips now auto-detect content-dense regions and overlay them there");
    println!("cargo:warning=    Prevents simple cropping attacks by placing strips over the most important content");
    println!("cargo:warning=  • --qr-code-position: place QR code at any corner or center");
    println!("cargo:warning=  • --qr-code-size: explicit QR code size in pixels");
    println!("cargo:warning=  • WebP and TIFF format support (input and output)");
    println!("cargo:warning=  • 3 new filigrane patterns: plume, constellation, ripple");
    println!("cargo:warning=  • All filigrane patterns now render with more visibility and true per-render randomness");
    println!("cargo:warning=");
    println!("cargo:warning=  Run `firemark --help` to get started.");
    println!("cargo:warning=  GitHub: https://github.com/Vitruves/firemark");
    println!("cargo:warning=");
}
