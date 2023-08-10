use thiserror::Error;

mod decode;
mod encode;
mod tests;
mod var_decode;
mod var_encode;

/// A trait for types that can be encoded into a buffer.
pub trait Encode {
    /// Encode this value into the given buffer.
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError>;
}

/// A trait for types that can be var-encoded into a buffer.
pub trait VarEncode {
    /// Encodes this value into the given buffer.
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError>;
}

/// An error that can occur while encoding a value into a buffer.
#[derive(Debug, Error)]
pub enum EncodeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TryInto(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("No packets in this state")]
    NoPackets,
}

/// A trait for types that can be decoded from a buffer.
pub trait Decode: Sized {
    /// Decodes a value from the given buffer.
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError>;
}

/// A trait for types that can be var-decoded from a buffer.
pub trait VarDecode: Sized {
    /// Decodes a value from the given buffer.
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError>;
}

/// An error that can occur while decoding a value from a buffer.
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TryInto(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("NBT error")]
    NbtError,
    #[error("Unknown packet id: {0}")]
    UnknownPacketId(u32),
    #[error("Invalid enum id: {0}")]
    InvalidEnumId(i32),
    #[error("Boolean error, expected 0 or 1, got {0}")]
    Boolean(u8),
    #[error("String too long: {0}")]
    StringTooLong(u32),
}

impl PartialEq for EncodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TryInto(l0), Self::TryInto(r0)) => l0 == r0,
            (Self::Io(_), Self::Io(_)) | (Self::Serde(_), Self::Serde(_)) => true,
            _ => false,
        }
    }
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Io(_), Self::Io(_)) | (Self::Serde(_), Self::Serde(_)) => true,
            (Self::TryInto(l0), Self::TryInto(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::StringTooLong(l0), Self::StringTooLong(r0)) => l0 == r0,
            (Self::Utf8(l0), Self::Utf8(r0)) => l0 == r0,
            _ => false,
        }
    }
}
