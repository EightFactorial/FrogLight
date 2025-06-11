#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;

mod regex;

mod read;
use read::ReadCompat;

mod write;
use write::WriteCompat;

use super::Snbt;
use crate::{
    convert::{FromCompound, IntoCompound, NbtError},
    nbt::NbtCompound,
};

/// The legacy SNBT format.
///
/// Used in versions `v1.21.4` and older.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct Compat;
impl super::SnbtType for Compat {}

impl FromCompound for Snbt<Compat> {
    fn from_compound(nbt: &NbtCompound) -> Result<Self, NbtError> {
        let mut content = String::new();
        NbtCompound::write_to_string(nbt, &mut content);
        Ok(Self::new_unchecked(content.into()))
    }
}
impl IntoCompound for Snbt<Compat> {
    fn into_compound(&self) -> Result<NbtCompound, NbtError> {
        NbtCompound::read_from_string(self.as_str()).map_or_else(
            |err| Err(NbtError::ConversionError(core::any::type_name::<Self>(), Box::new(err))),
            |(val, _)| Ok(val),
        )
    }
}
