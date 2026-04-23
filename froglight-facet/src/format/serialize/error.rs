use facet::ReflectError;

use crate::format::writer::WriterError;

/// TODO
#[derive(Debug, Clone)]
pub struct SerializeError;

impl From<ReflectError> for SerializeError {
    fn from(_value: ReflectError) -> Self { todo!() }
}
impl From<WriterError> for SerializeError {
    fn from(_value: WriterError) -> Self { todo!() }
}
