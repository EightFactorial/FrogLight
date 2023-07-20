use thiserror::Error;

mod decode;
mod encode;

/// A trait for types that can be var-encoded into a buffer.
pub trait VarEncode {
    /// Encodes this value into the given buffer.
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), VarError>;
}

/// A trait for types that can be var-decoded from a buffer.
pub trait VarDecode: Sized {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, VarError>;
}

#[derive(Debug, Error)]
pub enum VarError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TryInto error: {0}")]
    TryInto(#[from] std::num::TryFromIntError),
}

impl PartialEq for VarError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Io(_), Self::Io(_)) | (Self::TryInto(_), Self::TryInto(_))
        )
    }
}
