//! Placeholder

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;

generate! {
    @components
    ComponentPlaceholder(u8) = Byte
}
