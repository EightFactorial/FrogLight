//! Placeholder

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectComponent;

/// TODO
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect, bevy_ecs::component::Component))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum VanillaEntity {
    /// Placeholder
    Placeholder,
}
