use std::path::{Path, PathBuf};

use crate::error::{FiremarkError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Jpeg,
    Png,
    Pdf,
}

/// Detect format from file extension.
pub fn detect_format(path: &Path) -> Result<FileFormat> {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => Ok(FileFormat::Jpeg),
        Some("png") => Ok(FileFormat::Png),
        Some("pdf") => Ok(FileFormat::Pdf),
        Some(ext) => Err(FiremarkError::UnsupportedFormat(ext.to_string())),
        None => Err(FiremarkError::UnsupportedFormat("no extension".into())),
    }
}

/// Resolve output path from input, explicit output, and suffix.
///
/// If `output` is `Some`, it is returned directly.  Otherwise the input stem
/// is combined with the suffix (defaulting to `"watermarked"`) and the original
/// extension to produce a sibling path.
pub fn resolve_output_path(input: &Path, output: Option<&Path>, suffix: Option<&str>) -> PathBuf {
    if let Some(out) = output {
        return out.to_path_buf();
    }
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let ext = input.extension().unwrap_or_default().to_string_lossy();
    let suffix = suffix.unwrap_or("watermarked");
    let new_name = format!("{stem}-{suffix}.{ext}");
    input.with_file_name(new_name)
}

/// Check if a path is a supported image/PDF format.
pub fn is_supported(path: &Path) -> bool {
    detect_format(path).is_ok()
}
