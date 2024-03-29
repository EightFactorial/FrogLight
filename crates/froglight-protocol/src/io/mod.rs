//! IO functions
//!
//! This module contains the IO functions for
//! reading and writing data to and from buffers.

use thiserror::Error;

mod glam;
mod integer;

mod read;
pub(crate) use froglight_macros::FrogRead;
pub use read::FrogRead;

mod var_read;
pub use var_read::FrogVarRead;

mod var_write;
pub use var_write::FrogVarWrite;

mod write;
pub(crate) use froglight_macros::FrogWrite;
pub use write::FrogWrite;

/// An error that occurred while reading data.
#[derive(Debug, Error)]
pub enum ReadError {
    /// An error occurred while reading data.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while deserializing JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// An error occurred while reading a string.
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    /// An error occurred while reading nbt data.
    #[error(transparent)]
    NbtError(#[from] simdnbt::Error),
    /// The buffer ended before the expected data was read.
    #[error("Reached end of buffer, expected {0} bytes, got {1}")]
    EndOfBuffer(usize, usize),
    /// An error occurred while reading a list of items.
    #[error("Expected {0} items, got {1}: {2}")]
    ListError(usize, usize, Box<ReadError>),
    /// An error occurred while reading a bool.
    #[error("Invalid bool: {0}")]
    InvalidBool(u8),
    /// An error occurred while reading an enum.
    #[error("Invalid enum variant: {0} -> {1}")]
    InvalidEnum(i32, &'static str),
    /// An error occurred while reading a packet.
    #[error("Error reading {1} ({0}): {2}")]
    PacketError(u32, &'static str, Box<ReadError>),
}

/// An error that occurred while writing data.
#[derive(Debug, Error)]
pub enum WriteError {
    /// An error occurred while writing data.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error occurred while serializing JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// An error occurred trying to convert integer types.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}
