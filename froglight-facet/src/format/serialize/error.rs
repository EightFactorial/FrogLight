use facet::ReflectError;

use crate::format::writer::WriterError;

/// TODO
#[derive(Debug, Clone)]
pub struct SerializeError;

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
