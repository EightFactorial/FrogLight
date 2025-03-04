#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

mod regex;

mod read;
use read::ReadCompat;

mod write;
use write::WriteCompat;

use super::Snbt;
use crate::{
    convert::{ConvertError, ConvertNbt},
    nbt::NbtCompound,
};

/// The legacy SNBT format.
///
/// Used in versions `v1.21.4` and older.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct Compat;
impl super::SnbtType for Compat {}

impl ConvertNbt for Snbt<Compat> {
    fn from_compound(nbt: &NbtCompound) -> Result<Self, ConvertError> {
        let mut content = String::new();
        NbtCompound::write_to_string(nbt, &mut content);
        Ok(Self::new_unchecked(content.into()))
    }

    fn as_compound(&self) -> Result<NbtCompound, ConvertError> {
        NbtCompound::read_from_string(self.as_str()).map(|(nbt, _)| nbt)
    }
}
