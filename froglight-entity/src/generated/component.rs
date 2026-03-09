//! Placeholder

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;

generate! {
    @components
    Placeholder(u8) = Byte
}
