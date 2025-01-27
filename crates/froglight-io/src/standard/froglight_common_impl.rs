use std::io::{Read, Write};

use froglight_common::{EntityId, EntityUuid, Identifier};
use smol_str::SmolStr;
use uuid::Uuid;

use super::{FrogRead, FrogWrite, ReadError, WriteError};

impl FrogRead for Identifier {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        // SAFETY: Being sent a string from the server, we can assume it is valid.
        unsafe { SmolStr::frog_read(buffer).map(|content| Identifier::new_unchecked(content)) }
    }
}
impl FrogWrite for Identifier {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        SmolStr::frog_write(self.as_ref(), buffer)
    }
    fn frog_len(&self) -> usize { SmolStr::frog_len(self.as_ref()) }
}

impl FrogRead for EntityId {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        u32::frog_read(buffer).map(EntityId::from)
    }
}
impl FrogWrite for EntityId {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        u32::frog_write(self, buffer)
    }

    fn frog_len(&self) -> usize { u32::frog_len(self) }
}

impl FrogRead for EntityUuid {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        Uuid::frog_read(buffer).map(EntityUuid::from)
    }
}
impl FrogWrite for EntityUuid {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        Uuid::frog_write(self, buffer)
    }

    fn frog_len(&self) -> usize { Uuid::frog_len(self) }
}
