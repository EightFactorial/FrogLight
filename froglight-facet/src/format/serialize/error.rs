use facet::ReflectError;

/// TODO
pub struct SerializeError;

impl From<ReflectError> for SerializeError {
    fn from(_value: ReflectError) -> Self { todo!() }
}
