#[derive(Debug, thiserror::Error)]
pub enum NbtStreamError {
    #[error("Data ended unexpectedly")]
    EndOfStream,
    #[error("Invalid NBT tag: \"{0}\"")]
    InvalidTag(u8),
}
