use facet::{AllocError, ReflectError};

/// TODO
#[derive(Debug, Clone)]
pub struct DeserializeError;

impl From<AllocError> for DeserializeError {
    fn from(_value: AllocError) -> Self { todo!() }
}
impl From<ReflectError> for DeserializeError {
    fn from(_value: ReflectError) -> Self { todo!() }
}
