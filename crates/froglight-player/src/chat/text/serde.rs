//! TODO:
//!   1. Manually flattening/expanding for `TextContent`/`TextFormatting`.
//!   2. Carry `TextFormatting` down through `Text` children.

use serde::{
    __private::ser::FlatMapSerializer, Deserialize, Deserializer, Serialize, Serializer, de::Error,
    ser::SerializeMap,
};

use super::{FormattedContent, FormattedText, component::ValueComponent, formatting::TextColor};

impl Serialize for FormattedText {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map = ser.serialize_map(None)?;

        // Serialize the content
        match &self.content {
            FormattedContent::Text(c) => {
                map.serialize_entry("type", "text")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            FormattedContent::Translation(c) => {
                map.serialize_entry("type", "translatable")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            FormattedContent::Score(c) => {
                map.serialize_entry("type", "score")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            FormattedContent::Selector(c) => {
                map.serialize_entry("type", "selector")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            FormattedContent::Keybind(c) => {
                map.serialize_entry("type", "keybind")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            FormattedContent::Nbt(c) => {
                map.serialize_entry("type", "nbt")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
        }

        // Serialize the children entries
        if !self.children.is_empty() {
            map.serialize_entry("extra", &self.children)?;
        }

        // Serialize the text formatting
        // TODO: Skip default values on the first `TextFormatting`
        self.formatting.serialize(FlatMapSerializer(&mut map))?;

        map.end()
    }
}
impl<'de> Deserialize<'de> for FormattedText {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}

// #[test]
// fn formatted_text() {
//     use std::borrow::Cow;
//
//     use super::{component::TextComponent, formatting::TextFormatting};
//
//     let text = FormattedText {
//         content: FormattedContent::Text(Cow::Borrowed("Hello,
// World!").into()),         formatting: TextFormatting::default(),
//         children: Vec::new(),
//     };
//     assert_eq!(serde_json::to_string(&text).unwrap(), r#"{"type":
// "text", "text":"Hello, World!"}"#); }

// -------------------------------------------------------------------------------------------------

impl Serialize for TextColor {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        ser.serialize_str(self.as_named_str())
    }
}
impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let string = String::deserialize::<D>(de)?;
        TextColor::from_color(string).ok_or_else(|| Error::custom("invalid color"))
    }
}

#[test]
fn text_color() {
    assert_eq!(serde_json::to_string(&TextColor::Aqua).unwrap(), "\"aqua\"");
    assert_eq!(serde_json::to_string(&TextColor::Black).unwrap(), "\"black\"");
    assert_eq!(serde_json::to_string(&TextColor::DarkBlue).unwrap(), "\"dark_blue\"");
    assert_eq!(serde_json::to_string(&TextColor::DarkGreen).unwrap(), "\"dark_green\"");
    assert_eq!(serde_json::to_string(&TextColor::Gold).unwrap(), "\"gold\"");
    assert_eq!(serde_json::to_string(&TextColor::LightPurple).unwrap(), "\"light_purple\"");
    assert_eq!(serde_json::to_string(&TextColor::Red).unwrap(), "\"red\"");
    assert_eq!(serde_json::to_string(&TextColor::Yellow).unwrap(), "\"yellow\"");

    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#000000\"").unwrap(), TextColor::Black);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#0000AA\"").unwrap(), TextColor::DarkBlue);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#00AA00\"").unwrap(), TextColor::DarkGreen);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#00AAAA\"").unwrap(), TextColor::DarkAqua);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#AA0000\"").unwrap(), TextColor::DarkRed);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#AAAAAA\"").unwrap(), TextColor::Gray);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#FFFF55\"").unwrap(), TextColor::Yellow);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#FFFFFF\"").unwrap(), TextColor::White);

    let custom = TextColor::Custom("#123456".into());
    assert_eq!(serde_json::to_string(&custom).unwrap(), "\"#123456\"");
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#123456\"").unwrap(), custom);
}

// -------------------------------------------------------------------------------------------------

impl Serialize for ValueComponent {
    fn serialize<S>(&self, _ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        todo!()
    }
}
impl<'de> Deserialize<'de> for ValueComponent {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}
