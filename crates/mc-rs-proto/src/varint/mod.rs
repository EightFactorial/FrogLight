use thiserror::Error;

mod decode;
mod encode;

pub trait VarEncode: Sized {
    fn var_encode(&self) -> Result<Vec<u8>, VarError>;
}

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
