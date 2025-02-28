#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

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
    fn from_compound(_nbt: NbtCompound) -> Result<Self, ConvertError> { todo!() }

    fn into_compound(self) -> NbtCompound { todo!() }
}

#[rustfmt::skip]
#[expect(dead_code)]
impl Compat {
    const BYTE_REGEX: &'static str = r"(-?\d+(b|B))|true|false";
    const SHORT_REGEX: &'static str = r"-?\d+(s|S)";
    const INT_REGEX: &'static str = r"-?\d+";
    const LONG_REGEX: &'static str = r"-?\d+(l|L)";
    const FLOAT_REGEX: &'static str = r"-?\d+\.\d+(f|F)";
    const DOUBLE_REGEX: &'static str = r"-?\d+\.\d+(d|D)?";

    const BYTE_ARRAY_REGEX: &'static str = r"";
    const INT_ARRAY_REGEX: &'static str = r"";
    const LONG_ARRAY_REGEX: &'static str = r"";
}

impl Compat {}
