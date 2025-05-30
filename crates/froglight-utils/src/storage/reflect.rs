use bevy_reflect::FromType;

/// A struct used to operate on reflected `AppStorage` of a type.
#[derive(Clone, Copy)]
pub struct ReflectAppStorage {}

impl<T> FromType<T> for ReflectAppStorage {
    fn from_type() -> Self { ReflectAppStorage {} }
}
