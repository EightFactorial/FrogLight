use std::io::Cursor;

use froglight_components::entity::{EntityId, EntityUuid};
use uuid::Uuid;

use crate::protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError};

impl FrogRead for EntityId {
    #[inline]
    fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(EntityId(u32::fg_var_read(buf)?))
    }
}

impl FrogWrite for EntityId {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.0.fg_var_write(buf)
    }
}

impl FrogRead for EntityUuid {
    #[inline]
    fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(EntityUuid(Uuid::fg_read(buf)?))
    }
}

impl FrogWrite for EntityUuid {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.0.fg_write(buf)
    }
}
