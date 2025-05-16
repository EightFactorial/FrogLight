use std::io::{Read, Write};

use froglight_common::{
    entity::{EntityId, EntityUuid},
    identifier::Identifier,
};
use smol_str::SmolStr;
use uuid::Uuid;

use super::{FrogRead, FrogWrite, ReadError, WriteError};

impl FrogRead for Identifier {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        #[cfg(feature = "trace")]
        tracing::trace!("Reading struct \"Identifier\"");
        SmolStr::frog_read(buffer).map(Identifier::new)
    }
}
impl FrogWrite for Identifier {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        #[cfg(feature = "trace")]
        tracing::trace!("Writing struct \"Identifier\"");
        SmolStr::frog_write(self.as_ref(), buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { SmolStr::frog_len(self.as_ref()) }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for EntityId {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        u32::frog_read(buffer).map(EntityId::from)
    }
}
impl FrogWrite for EntityId {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        u32::frog_write(self, buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { u32::frog_len(self) }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for EntityUuid {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        Uuid::frog_read(buffer).map(EntityUuid::from)
    }
}
impl FrogWrite for EntityUuid {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        Uuid::frog_write(self, buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { Uuid::frog_len(self) }
}
