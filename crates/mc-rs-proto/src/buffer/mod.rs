use thiserror::Error;

mod decode;
mod encode;
mod var_decode;
mod var_encode;

/// A trait for types that can be encoded into a buffer.
pub trait Encode {
    /// Encode a value into a buffer.
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError>;
}

/// A trait for types that can be var-encoded into a buffer.
pub trait VarEncode {
    /// Encodes this value into the given buffer.
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError>;
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TryInto error: {0}")]
    TryInto(#[from] std::num::TryFromIntError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// A trait for types that can be decoded from a buffer.
pub trait Decode: Sized {
    /// Decode a value from a buffer.
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError>;
}

/// A trait for types that can be var-decoded from a buffer.
pub trait VarDecode: Sized {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError>;
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TryInto error: {0}")]
    TryInto(#[from] std::num::TryFromIntError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Boolean error, expected 0 or 1, got {0}")]
    Boolean(u8),
    #[error("String too long: {0}")]
    StringTooLong(u32),
    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
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
