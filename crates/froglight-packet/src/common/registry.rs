//! [`IdentifierSet`] and [`InlineRegistryId`]

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_common::identifier::Identifier;
#[cfg(feature = "io")]
use froglight_io::prelude::*;

/// A single identifier or a set of registry ids.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum RegistrySet {
    /// An [`Identifier`]
    Identifier(Identifier),
    /// A set of global registry IDs
    IdSet(Vec<u32>),
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for RegistrySet {
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
                Ok(Self::IdSet(
                    (0..other.saturating_sub(1))
                        .map(|_| u32::frog_var_read(buffer))
                        .collect::<Result<Vec<u32>, _>>()?,
                ))
            }
        }
    }
}

#[cfg(feature = "io")]
impl FrogWrite for RegistrySet {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self {
            RegistrySet::Identifier(identifier) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"RegistrySet::Identifier\" (0)");
                Ok(0u32.frog_var_write(buffer)? + identifier.frog_write(buffer)?)
            }
            RegistrySet::IdSet(items) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"RegistrySet::IdSet\" ({})", items.len() + 1);

                #[expect(clippy::cast_possible_truncation)]
                let mut size = (items.len() as u32 + 1).frog_var_write(buffer)?;
                for item in items {
                    size += item.frog_var_write(buffer)?;
                }

                Ok(size)
            }
        }
    }

    fn frog_len(&self) -> usize {
        match self {
            RegistrySet::Identifier(identifier) => 0u32.frog_var_len() + identifier.frog_len(),
            RegistrySet::IdSet(items) => items
                .iter()
                .fold((items.len() + 1).frog_var_len(), |acc, item| acc + item.frog_var_len()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An inlined item or a registry id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub enum InlineRegistryId<T> {
    /// An item of type `T`
    Item(T),
    /// A global registry ID
    Id(u32),
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<T: FrogRead> FrogRead for InlineRegistryId<T> {
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
impl<T: FrogWrite> FrogWrite for InlineRegistryId<T> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self {
            InlineRegistryId::Item(item) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"InlineRegistryId::Item\" (0)");
                Ok(0u32.frog_var_write(buffer)? + item.frog_write(buffer)?)
            }
            InlineRegistryId::Id(id) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"InlineRegistryId::Id\" ({id})");
                Ok(id.saturating_add(1).frog_var_write(buffer)?)
            }
        }
    }

    fn frog_len(&self) -> usize {
        match self {
            InlineRegistryId::Item(item) => 0u32.frog_var_len() + item.frog_len(),
            InlineRegistryId::Id(id) => id.saturating_add(1).frog_var_len(),
        }
    }
}
