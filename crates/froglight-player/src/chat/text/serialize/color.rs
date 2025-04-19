use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::chat::text::{component::ValueComponent, formatting::TextColor};

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

// -------------------------------------------------------------------------------------------------

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
