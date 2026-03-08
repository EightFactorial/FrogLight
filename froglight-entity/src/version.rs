//! TODO

#[cfg(feature = "facet")]
use facet_minecraft::{
    deserialize::{InputCursor, error::DeserializeValueError},
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};
use froglight_common::prelude::*;

#[cfg(feature = "facet")]
use crate::generated::data::EntityDataType;

/// A [`Version`]'s associated entity data.
pub trait EntityVersion: Version {
    /// This [`Version`]'s deserializer for
    /// [`EntityDataType`](crate::generated::data::EntityDataType)s
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    const DATATYPE_DESERIALIZE: fn(
        &mut InputCursor,
    ) -> Result<EntityDataType, DeserializeValueError>;
    /// This [`Version`]'s erializer for
    /// [`EntityDataType`](crate::generated::data::EntityDataType)s
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    const DATATYPE_SERIALIZE: for<'input, 'facet> fn(
        &'facet (),
        &'input EntityDataType,
        &mut dyn SerializeWriter,
    ) -> Result<
        (),
        SerializeIterError<'input, 'facet>,
    >;
}
