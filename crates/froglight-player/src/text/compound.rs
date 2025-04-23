//! [`FromCompound`] and [`IntoCompound`] implementations for [`FormattedText`]

use froglight_nbt::{
    convert::ConvertError,
    nbt::NbtCompound,
    prelude::{FromCompound, IntoCompound},
};

use super::{
    FormattedContent, FormattedText,
    formatting::{TextColor, TextFormatting},
};

impl FromCompound for FormattedText {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for FormattedText {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for FormattedContent {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for FormattedContent {
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
