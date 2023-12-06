use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourcePackLoaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Texture error: {0}")]
    Texture(#[from] bevy::render::texture::TextureError),
    #[error("Unsupported file extension")]
    UnsupportedExtension,
    #[error("Invalid path")]
    InvalidPath,
}
