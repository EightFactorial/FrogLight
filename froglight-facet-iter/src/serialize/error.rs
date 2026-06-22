use core::{error, fmt};

use facet::ReflectError;

use crate::writer::WriterError;

/// TODO
#[derive(Debug, Clone)]
pub struct SerializeError;

impl error::Error for SerializeError {}
impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "SerializeError") }
}

// -------------------------------------------------------------------------------------------------

impl From<ReflectError> for SerializeError {
    #[allow(unused_variables, reason = "Temporary")]
    fn from(value: ReflectError) -> Self {
        #[cfg(feature = "std")]
        std::println!("ReflectError: {value:?}");
        SerializeError
    }
}
impl From<WriterError> for SerializeError {
    #[allow(unused_variables, reason = "Temporary")]
    fn from(value: WriterError) -> Self {
        #[cfg(feature = "std")]
        std::println!("WriterError: {value:?}");
        SerializeError
    }
}
