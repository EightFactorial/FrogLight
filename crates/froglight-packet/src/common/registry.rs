//! [`IdentifierSet`] and [`InlineRegistryId`]

#[cfg(all(not(feature = "std"), feature = "registry"))]
use alloc::borrow::Cow;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(all(feature = "std", feature = "registry"))]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
#[cfg(feature = "registry")]
use froglight_registry::prelude::*;

/// A registry name or a set of registry IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum RegistryNameOrSet {
    /// A registry [`Identifier`]
    Identifier(Identifier),
    /// A set of registry IDs
    Set(Vec<u32>),
}

impl RegistryNameOrSet {
    /// Returns `true` if the value is a [`RegistryNameOrSet::Identifier`].
    #[inline]
    #[must_use]
    pub const fn is_identifier(&self) -> bool { matches!(self, RegistryNameOrSet::Identifier(_)) }

    /// Returns `true` if the value is a [`RegistryNameOrSet::Set`].
    #[inline]
    #[must_use]
    pub const fn is_set(&self) -> bool { matches!(self, RegistryNameOrSet::Set(_)) }

    /// Get the list of registry IDs either given or
    /// retrieved from the [`RegistryStorage`] using the [`Identifier`].
    #[must_use]
    #[cfg(feature = "registry")]
    pub fn values<'a, V: Version>(
        &'a self,
        registries: &RegistryStorage<V>,
    ) -> Option<Cow<'a, [u32]>> {
        match self {
            RegistryNameOrSet::Identifier(ident) => registries
                .get_registry(ident)
                .map(|v| Cow::Owned((0..v.values().len() as u32).collect())),
            RegistryNameOrSet::Set(items) => Some(Cow::Borrowed(&items)),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for RegistryNameOrSet {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match <u32 as FrogVarRead>::frog_var_read(buffer)? {
            0 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"RegistrySet::Identifier\" (0)");
                Ok(Self::Identifier(Identifier::frog_read(buffer)?))
            }
            other => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"RegistrySet::IdSet\" ({other})");
                Ok(Self::Set(
                    (0..other.saturating_sub(1))
                        .map(|_| u32::frog_var_read(buffer))
                        .collect::<Result<Vec<u32>, _>>()?,
                ))
            }
        }
    }
}

#[cfg(feature = "io")]
impl FrogWrite for RegistryNameOrSet {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self {
            RegistryNameOrSet::Identifier(identifier) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"RegistrySet::Identifier\" (0)");
                Ok(0u32.frog_var_write(buffer)? + identifier.frog_write(buffer)?)
            }
            RegistryNameOrSet::Set(items) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"RegistrySet::IdSet\" ({})", items.len().saturating_add(1));

                #[expect(clippy::cast_possible_truncation)]
                let mut size = (items.len().saturating_add(1) as u32).frog_var_write(buffer)?;
                for item in items {
                    size += item.frog_var_write(buffer)?;
                }

                Ok(size)
            }
        }
    }

    fn frog_len(&self) -> usize {
        match self {
            RegistryNameOrSet::Identifier(identifier) => {
                0u32.frog_var_len() + identifier.frog_len()
            }
            RegistryNameOrSet::Set(items) => {
                items.iter().fold((items.len().saturating_add(1)).frog_var_len(), |acc, item| {
                    acc + item.frog_var_len()
                })
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A registry item or a registry ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub enum RegistryItemOrId<T> {
    /// An item of type `T`
    Item(T),
    /// A registry ID
    Id(u32),
}

impl<T> RegistryItemOrId<T> {
    /// Returns `true` if the value is a [`RegistryItemOrId::Item`].
    #[inline]
    #[must_use]
    pub const fn is_item(&self) -> bool { matches!(self, RegistryItemOrId::Item(_)) }

    /// Returns `true` if the value is a [`RegistryItemOrId::Id`].
    #[inline]
    #[must_use]
    pub const fn is_id(&self) -> bool { matches!(self, RegistryItemOrId::Id(_)) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<T: FrogRead> FrogRead for RegistryItemOrId<T> {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match <u32 as FrogVarRead>::frog_var_read(buffer)? {
            0 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"InlineRegistryId::Item\" (0)");
                Ok(Self::Item(T::frog_read(buffer)?))
            }
            id => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"InlineRegistryId::Id\" ({id})");
                Ok(Self::Id(id.saturating_sub(1)))
            }
        }
    }
}
#[cfg(feature = "io")]
impl<T: FrogWrite> FrogWrite for RegistryItemOrId<T> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self {
            RegistryItemOrId::Item(item) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"InlineRegistryId::Item\" (0)");
                Ok(0u32.frog_var_write(buffer)? + item.frog_write(buffer)?)
            }
            RegistryItemOrId::Id(id) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"InlineRegistryId::Id\" ({id})");
                Ok(id.saturating_add(1).frog_var_write(buffer)?)
            }
        }
    }

    fn frog_len(&self) -> usize {
        match self {
            RegistryItemOrId::Item(item) => 0u32.frog_var_len() + item.frog_len(),
            RegistryItemOrId::Id(id) => id.saturating_add(1).frog_var_len(),
        }
    }
}

#[cfg(feature = "io")]
impl<T: FrogVarRead> FrogVarRead for RegistryItemOrId<T> {
    fn frog_var_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match <u32 as FrogVarRead>::frog_var_read(buffer)? {
            0 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"InlineRegistryId::Item\" (0)");
                Ok(Self::Item(T::frog_var_read(buffer)?))
            }
            id => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"InlineRegistryId::Id\" ({id})");
                Ok(Self::Id(id.saturating_sub(1)))
            }
        }
    }
}
#[cfg(feature = "io")]
impl<T: FrogVarWrite> FrogVarWrite for RegistryItemOrId<T> {
    fn frog_var_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self {
            RegistryItemOrId::Item(item) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"InlineRegistryId::Item\" (0)");
                Ok(0u32.frog_var_write(buffer)? + item.frog_var_write(buffer)?)
            }
            RegistryItemOrId::Id(id) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"InlineRegistryId::Id\" ({id})");
                Ok(id.saturating_add(1).frog_var_write(buffer)?)
            }
        }
    }

    fn frog_var_len(&self) -> usize {
        match self {
            RegistryItemOrId::Item(item) => 0u32.frog_var_len() + item.frog_var_len(),
            RegistryItemOrId::Id(id) => id.saturating_add(1).frog_var_len(),
        }
    }
}
