/// Errors that can occur when reading a stream of NBT data.
#[derive(Debug, thiserror::Error)]
pub enum NbtStreamError {
    /// Data ended unexpectedly.
    #[error("Data ended unexpectedly")]
    EndOfStream,
    /// Invalid NBT tag.
    #[error("Invalid NBT tag: \"{0}\"")]
    InvalidTag(u8),
}
