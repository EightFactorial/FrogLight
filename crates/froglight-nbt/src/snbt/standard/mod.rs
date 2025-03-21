#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

use super::Snbt;
use crate::{
    convert::{ConvertError, FromCompound, IntoCompound},
    nbt::NbtCompound,
};

/// The standard SNBT format.
///
/// Used in versions `v1.21.5` and newer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct Standard;
impl super::SnbtType for Standard {}

impl FromCompound for Snbt<Standard> {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for Snbt<Standard> {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}
