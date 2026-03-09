use thiserror::Error;

#[derive(Error, Debug)]
pub enum FiremarkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("PDF error: {0}")]
    Pdf(#[from] lopdf::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Font error: {0}")]
    Font(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, FiremarkError>;
