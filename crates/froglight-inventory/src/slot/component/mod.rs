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

/// A map of inventory components that can be serialized/deserialized.
///
/// Used to identify and serialize/deserialize components over the network.
pub struct InventoryComponents;

static COMPONENTS: StaticMap = LazyLock::new(|| RwLock::new(HashMap::new()));
type StaticMap = LazyLock<RwLock<HashMap<TypeId, ComponentMap>>>;
type ComponentMap = IndexMap<Identifier, ComponentFns>;

impl InventoryComponents {
    /// Get access to the [`InventoryComponents`] map.
    pub fn read<V: VersionComponents>() -> MappedRwLockReadGuard<'static, ComponentMap> {
        // Insert the components if they do not exist.
        if !{ COMPONENTS.read().contains_key(&TypeId::of::<V>()) } {
            COMPONENTS.write().insert(TypeId::of::<V>(), V::components());
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
}

// -------------------------------------------------------------------------------------------------

pub trait VersionComponents: Version {
    fn components() -> ComponentMap;
}

// -------------------------------------------------------------------------------------------------

pub struct ComponentFns {
    read: fn(&mut dyn std::io::Read) -> Result<NbtTag, ReadError>,
    write: fn(&NbtTag, &mut dyn std::io::Write) -> Result<usize, WriteError>,
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
        }
    }
}
