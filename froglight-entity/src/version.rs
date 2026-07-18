//! TODO

use froglight_common::prelude::*;
#[cfg(feature = "facet")]
use froglight_facet::facet::template::{Reader, ReaderError, Writer, WriterError};
pub use froglight_registry_template::types::OnceLock;

#[cfg(feature = "facet")]
use crate::generated::datatype::EntityDataType;
use crate::storage::EntityStorage;

/// A [`Version`]'s associated entity data.
pub trait EntityVersion: Version {
    /// The [`EntityStorage`] for this [`Version`].
    const ENTITY: &'static OnceLock<EntityStorage>;

    /// Get the [`EntityStorage`] for this [`Version`].
    #[inline]
    fn entities() -> &'static EntityStorage { Self::ENTITY.get_or_init(Self::new_entity) }

    /// Create a new [`EntityStorage`] for this [`Version`].
    ///
    /// # Warning
    ///
    /// This will create a new [`EntityStorage`] each time it is called!
    ///
    /// Unless you are modifying the global, you should probably be using
    /// [`EntityVersion::entities`]!
    #[must_use]
    fn new_entity() -> EntityStorage;

    /// This [`Version`]'s deserializer for [`EntityDataType`]s
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    const DATATYPE_DESERIALIZE: fn(&mut Reader) -> Result<EntityDataType, ReaderError>;
    /// This [`Version`]'s serializer for [`EntityDataType`]s
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    const DATATYPE_SERIALIZE: fn(&EntityDataType, &mut Writer) -> Result<(), WriterError>;
}
