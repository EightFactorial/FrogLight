//! TODO:
//!   1. Carry `TextFormatting` down through `Text` children.

use froglight_nbt::{convert::ConvertError, nbt::NbtCompound, prelude::ConvertNbt};

use super::{Text, TextColor, TextContent, TextFormatting};

impl ConvertNbt for Text {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }

    fn as_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl ConvertNbt for TextContent {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }

    fn as_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl ConvertNbt for TextFormatting {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }

    fn as_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl ConvertNbt for TextColor {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }

    fn as_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}
