//! Inventory components for Froglight.

use std::{any::TypeId, sync::LazyLock};

use froglight_common::{prelude::Identifier, version::Version};
use froglight_io::prelude::*;
use froglight_nbt::{
    nbt::NbtTag,
    prelude::{FromTag, IntoTag},
};
use hashbrown::HashMap;
use indexmap::IndexMap;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

#[cfg(feature = "v1_21_4")]
mod v1_21_4;
#[cfg(feature = "v1_21_5")]
mod v1_21_5;

mod types;
#[expect(unused_imports, unreachable_pub)]
pub use types::*;

/// A map of inventory components that can be serialized/deserialized.
///
/// Used to identify and serialize/deserialize components over the network.
pub struct InventoryComponents;

static COMPONENTS: LazyLock<StaticMap> = LazyLock::new(StaticMap::default);
type StaticMap = RwLock<HashMap<TypeId, ComponentMap>>;
/// A map of identifiers to component functions.
pub type ComponentMap = IndexMap<Identifier, ComponentFns>;

impl InventoryComponents {
    /// Get access to the [`InventoryComponents`] map.
    ///
    /// If you are using a custom [`Version`] type,
    /// you should use [`InventoryComponents::write`]
    /// to insert a [`ComponentMap`] manually.
    ///
    /// # Panics
    /// Panics if the [`Version`] has not been initialized
    /// by calling [`InventoryComponents::write`], and
    /// the [`Version`] is a custom type.
    pub fn read<V: Version>() -> MappedRwLockReadGuard<'static, ComponentMap> {
        // Insert the components if they do not exist.
        if !{ COMPONENTS.read().contains_key(&TypeId::of::<V>()) } {
            COMPONENTS.write().insert(TypeId::of::<V>(), Self::init_type::<V>());
        }

        RwLockReadGuard::map(COMPONENTS.read(), |data: &HashMap<TypeId, ComponentMap>| {
            data.get(&TypeId::of::<V>())
                .unwrap_or_else(|| unreachable!("Components guaranteed to exist"))
        })
    }

    /// Get mutable access to the [`InventoryComponents`] map.
    pub fn write<V: VersionComponents>() -> MappedRwLockWriteGuard<'static, ComponentMap> {
        RwLockWriteGuard::map(COMPONENTS.write(), |data: &mut HashMap<TypeId, ComponentMap>| {
            data.entry(TypeId::of::<V>()).or_insert_with(V::components)
        })
    }

    /// Initialize the components for the given type,
    /// without requiring a trait bound.
    ///
    /// # Panics
    /// Panics if the version type is not recognized.
    fn init_type<V: Version>() -> ComponentMap {
        macro_rules! generate_match {
            ($($version:ident = $feature:literal),* $(,)?) => {
                match TypeId::of::<V>() {
                    $(
                        #[cfg(feature = $feature)]
                        id if id == TypeId::of::<froglight_common::version::$version>() => froglight_common::version::$version::components(),
                    )*
                    _ => panic!(
                        "No `ComponentMap` for \"{}\", was one not initialized?",
                        std::any::type_name::<V>()
                    ),
                }
            };
        }

        generate_match! {
            V1_21_4 = "v1_21_4",
            V1_21_5 = "v1_21_5",
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait that initializes a [`ComponentMap`] for a specific [`Version`].
pub trait VersionComponents: Version {
    /// Create a [`ComponentMap`].
    fn components() -> ComponentMap;
}

// -------------------------------------------------------------------------------------------------

/// A pair of type-erased functions that
/// read and write components from a buffer.
pub struct ComponentFns {
    read: fn(&mut dyn std::io::Read) -> Result<NbtTag, ReadError>,
    write: fn(&NbtTag, &mut dyn std::io::Write) -> Result<usize, WriteError>,
    write_len: fn(&NbtTag) -> usize,
}

impl ComponentFns {
    /// Read the data from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    pub fn frog_read(&self, mut buffer: &mut impl std::io::Read) -> Result<NbtTag, ReadError> {
        (self.read)(&mut buffer)
    }

    /// Read the data from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    pub fn frog_write(
        &self,
        tag: &NbtTag,
        buffer: &mut impl std::io::Write,
    ) -> Result<usize, WriteError> {
        (self.write)(tag, buffer)
    }

    /// Return the length of the data if it were to be encoded.
    ///
    /// # Panics
    /// Panics if the data cannot be decoded from the tag.
    #[must_use]
    pub fn frog_len(&self, tag: &NbtTag) -> usize { (self.write_len)(tag) }
}

impl<T: FrogRead + FrogWrite + FromTag + IntoTag> From<T> for ComponentFns {
    fn from(_: T) -> Self {
        use std::io::{Error, ErrorKind};

        Self {
            read: |mut buffer| {
                T::frog_read(&mut buffer)?.into_tag().map_err(|_| {
                    ReadError::Io(Error::new(
                        ErrorKind::InvalidData,
                        "Failed to convert data to/from NBT",
                    ))
                })
            },
            write: |tag, mut buffer| {
                T::from_tag(tag)
                    .map_err(|_| {
                        WriteError::Io(Error::new(
                            ErrorKind::InvalidData,
                            "Failed to convert data to/from NBT",
                        ))
                    })?
                    .frog_write(&mut buffer)
            },
            write_len: |tag| {
                T::from_tag(tag).expect("Failed to convert data to/from NBT").frog_len()
            },
        }
    }
}
