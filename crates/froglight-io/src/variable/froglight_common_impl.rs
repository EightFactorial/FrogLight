use std::io::{Read, Write};

use froglight_common::EntityId;

use super::{FrogVarRead, FrogVarWrite};
use crate::prelude::{ReadError, WriteError};

impl FrogVarRead for EntityId {
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        u32::frog_var_read(buffer).map(EntityId::from)
    }
}
impl FrogVarWrite for EntityId {
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        u32::frog_var_write(self, buffer)
    }

    fn frog_var_len(&self) -> usize { u32::frog_var_len(self) }
}
