use core::{error, fmt};

use facet::{AllocError, ReflectError, ShapeMismatchError};

use crate::ReaderError;

/// TODO
#[derive(Debug, Clone)]
pub struct DeserializeError;

impl error::Error for DeserializeError {}
impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "DeserializeError") }
}

// -------------------------------------------------------------------------------------------------

impl From<ReaderError> for DeserializeError {
    #[allow(unused_variables, reason = "Temporary")]
    fn from(value: ReaderError) -> Self {
        #[cfg(feature = "std")]
        match value {
            ReaderError::InvalidBool(err) => std::println!("InvalidBool: {err}"),
            ReaderError::EndOfInput(err) => std::println!("EndOfInput: {err}"),
            ReaderError::Reflect(err) => std::println!("ReflectError: {err}"),
            ReaderError::IO(err) => std::println!("IOError: {err}"),
            ReaderError::Other(err) => std::println!("Other: {err}"),
        }
        DeserializeError
    }
}
impl From<AllocError> for DeserializeError {
    #[allow(unused_variables, reason = "Temporary")]
    fn from(value: AllocError) -> Self {
        #[cfg(feature = "std")]
        std::println!("AllocError: {value}");
        DeserializeError
    }
}
impl From<ReflectError> for DeserializeError {
    #[allow(unused_variables, reason = "Temporary")]
    fn from(value: ReflectError) -> Self {
        #[cfg(feature = "std")]
        std::println!("ReflectError: {value}");
        DeserializeError
    }
}
impl From<ShapeMismatchError> for DeserializeError {
    #[allow(unused_variables, reason = "Temporary")]
    fn from(value: ShapeMismatchError) -> Self {
        #[cfg(feature = "std")]
        std::println!("ShapeMismatchError: {value}");
        DeserializeError
    }
}
