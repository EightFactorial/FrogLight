//! TODO:
//!   1. Carry `TextFormatting` down through `Text` children.

use froglight_nbt::{
    convert::ConvertError,
    nbt::NbtCompound,
    prelude::{FromCompound, IntoCompound},
};

use super::{Text, TextColor, TextContent, TextFormatting};

impl FromCompound for Text {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for Text {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for TextContent {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TextContent {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for TextFormatting {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TextFormatting {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for TextColor {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TextColor {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}
