use facet::{AllocError, ReflectError, ShapeMismatchError};

/// TODO
#[derive(Debug, Clone)]
pub struct DeserializeError;

impl From<AllocError> for DeserializeError {
    fn from(_value: AllocError) -> Self { todo!() }
}
impl From<ReflectError> for DeserializeError {
    fn from(_value: ReflectError) -> Self { todo!() }
}
impl From<ShapeMismatchError> for DeserializeError {
    fn from(_value: ShapeMismatchError) -> Self { todo!() }
}
