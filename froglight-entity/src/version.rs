//! TODO

#[cfg(feature = "facet")]
use facet_minecraft::{
    deserialize::{InputCursor, error::DeserializeValueError},
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};
use froglight_common::prelude::*;

#[cfg(feature = "facet")]
use crate::generated::datatype::EntityDataType;
use crate::storage::EntityStorage;
#[cfg(feature = "std")]
use crate::storage::GlobalEntityStorage;

/// A [`Version`]'s associated entity data.
pub trait EntityVersion: Version {
    /// The [`GlobalEntityStorage`] for this [`Version`].
    #[cfg(feature = "std")]
    const ENTITY: &'static std::sync::LazyLock<GlobalEntityStorage>;

    /// Get the [`GlobalEntityStorage`] for this [`Version`].
    #[inline]
    #[must_use]
    #[cfg(feature = "std")]
    fn entities() -> &'static GlobalEntityStorage { Self::ENTITY }

    /// Create a new [`EntityStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`EntityStorage`] each time it is called!
    ///
    /// Unless you are in a `no_std` environment, you should probably be using
    /// [`EntityVersion::entities`] or the associated constant.
    fn new_entity() -> EntityStorage;

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
