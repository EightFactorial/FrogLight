//! TODO:
//!   1. Manually flattening/expanding for `TextContent`/`TextFormatting`.
//!   2. Carry `TextFormatting` down through `Text` children.

use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeMap};

use super::{Text, TextColor, TextContent, TextFormatting};

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry(
            "type",
            match self.content {
                TextContent::Text { .. } => "text",
                TextContent::Translation { .. } => "translatable",
                TextContent::Score { .. } => "score",
                TextContent::Selector { .. } => "selector",
                TextContent::Keybind { .. } => "keybind",
                TextContent::Nbt { .. } => "nbt",
            },
        )?;

        map.end()
    }
}
impl<'de> Deserialize<'de> for Text {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

impl Serialize for TextContent {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        todo!()
    }
}
impl<'de> Deserialize<'de> for TextContent {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

impl Serialize for TextFormatting {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        todo!()
    }
}
impl<'de> Deserialize<'de> for TextFormatting {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

impl Serialize for TextColor {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        todo!()
    }
}
impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}
