//! Placeholder

/// All the data types that an entity can have.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum EntityDataType {
    /// Placeholder
    Placeholder,
}
