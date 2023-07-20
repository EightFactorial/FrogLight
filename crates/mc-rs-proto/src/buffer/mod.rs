use thiserror::Error;

use self::varint::VarError;

mod decode;
mod encode;

pub mod varint;

/// A trait for types that can be encoded into a buffer.
pub trait Encode {
    /// Encode a value into a buffer.
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError>;
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Varint error: {0}")]
    Var(#[from] VarError),
}

/// A trait for types that can be decoded from a buffer.
pub trait Decode: Sized {
    /// Decode a value from a buffer.
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError>;
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TryInto error: {0}")]
    TryInto(#[from] std::num::TryFromIntError),
    #[error("Varint error: {0}")]
    Var(#[from] VarError),
    #[error("Boolean error, expected 0 or 1, got {0}")]
    Boolean(u8),
    #[error("String too long: {0}")]
    StringTooLong(u32),
    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
