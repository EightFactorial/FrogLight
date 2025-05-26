#[cfg(not(feature = "std"))]
use alloc::{format, string::ToString, vec::Vec};

use serde::{
    __private::{
        de::{Content, ContentDeserializer, ContentRefDeserializer},
        ser::FlatMapSerializer,
    },
    Deserialize, Deserializer, Serialize, Serializer, de,
    ser::{SerializeMap, SerializeSeq},
};

use super::content::TextContent;
use crate::{
    prelude::*,
    text::{
        FormattedTextRef,
        content::{
            KeybindComponent, ScoreComponent, SelectorComponent, TextComponent, TranslateComponent,
            ValueComponent,
        },
        style::TextStyle,
    },
};

impl Serialize for FormattedText {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        Child(FormattedTextRef::new(self), &TextStyle::EMPTY).serialize(ser)
    }
}
impl Serialize for FormattedTextRef<'_, '_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        Child(*self, &TextStyle::EMPTY).serialize(ser)
    }
}

/// Serialize a slice of [`FormattedText`] children while inheriting formatting.
struct Children<'a>(&'a [FormattedText], &'a TextStyle);
impl Serialize for Children<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut ser = ser.serialize_seq(Some(self.0.len()))?;
        self.0.iter().try_for_each(|c| ser.serialize_element(&Child(c.into(), self.1)))?;
        ser.end()
    }
}

/// Serialize a [`FormattedText`] while inheriting formatting.
struct Child<'a>(FormattedTextRef<'a, 'a>, &'a TextStyle);
impl Serialize for Child<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map: S::SerializeMap;

        // Prepare formatting arguments
        let inherit = self.0.style.inherit(self.1);
        let diff = inherit.difference(self.1);

        // Serialize the text content
        match &self.0.content {
            TextContent::Text(c) => {
                if diff.is_empty() && self.0.interaction.is_empty() && self.0.children.is_empty() {
                    return ser.serialize_str(&c.text);
                }

                map = ser.serialize_map(None)?;
                map.serialize_entry("type", "text")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            TextContent::Translation(c) => {
                map = ser.serialize_map(None)?;
                map.serialize_entry("type", "translatable")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            TextContent::Score(c) => {
                map = ser.serialize_map(None)?;
                map.serialize_entry("type", "score")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            TextContent::Selector(c) => {
                map = ser.serialize_map(None)?;
                map.serialize_entry("type", "selector")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            TextContent::Keybind(c) => {
                map = ser.serialize_map(None)?;
                map.serialize_entry("type", "keybind")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
            TextContent::Nbt(c) => {
                map = ser.serialize_map(None)?;
                map.serialize_entry("type", "nbt")?;
                c.serialize(FlatMapSerializer(&mut map))?;
            }
        }

        // Serialize the text's children recursively
        if !self.0.children.is_empty() {
            map.serialize_entry("extra", &Children(&self.0.children, &inherit))?;
        }

        // Serialize the differences from the parent text's formatting
        diff.serialize(FlatMapSerializer(&mut map))?;

        // Serialize the text's interaction settings
        self.0.interaction.serialize(FlatMapSerializer(&mut map))?;

        map.end()
    }
}

// -------------------------------------------------------------------------------------------------

impl<'de> Deserialize<'de> for FormattedText {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        match Content::deserialize(de)? {
            Content::String(string) => Ok(Self::from_string(string)),
            Content::Str(str) => Ok(Self::from_string(str.to_string())),
            ref content @ Content::Map(ref map) => {
                let children = match map
                    .iter()
                    .find_map(|(n, c)| (n.as_str() == Some("extra")).then_some(c))
                {
                    Some(children) => Vec::deserialize(ContentRefDeserializer::new(children))?,
                    None => Vec::new(),
                };

                Ok(Self {
                    content: Deserialize::deserialize(ContentRefDeserializer::new(content))?,
                    style: Deserialize::deserialize(ContentRefDeserializer::new(content))?,
                    interaction: Deserialize::deserialize(ContentRefDeserializer::new(content))?,
                    children,
                })
            }
            _other => Err(de::Error::custom("expected a string or map")),
        }
    }
}

impl<'de> Deserialize<'de> for TextContent {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        /// Deserialize a [`TextContent`] using the provided content type.
        fn deserialize<'de, D: Deserializer<'de>, T: Into<TextContent> + Deserialize<'de>>(
            content: Content<'de>,
        ) -> Result<TextContent, D::Error> {
            Ok(T::deserialize(ContentDeserializer::<'de, D::Error>::new(content))?.into())
        }

        // Deserialize the provided content for interpretation
        let content = Content::deserialize(de)?;
        let Content::Map(content_map) = &content else {
            return Err(de::Error::custom("expected a map"));
        };

        // Loop over the provided map keys for hints on the type of interaction
        for (item, item_content) in content_map {
            let Some(name) = item.as_str() else { continue };
            match name {
                // Received the type of content
                "type" => {
                    let content_type = item_content
                        .as_str()
                        .ok_or_else(|| de::Error::custom("expected `type` to be a string"))?;

                    return match content_type {
                        "text" => deserialize::<D, TextComponent>(content),
                        "translatable" => deserialize::<D, TranslateComponent>(content),
                        "score" => deserialize::<D, ScoreComponent>(content),
                        "selector" => deserialize::<D, SelectorComponent>(content),
                        "keybind" => deserialize::<D, KeybindComponent>(content),
                        "nbt" => deserialize::<D, ValueComponent>(content),
                        unknown => {
                            Err(de::Error::custom(format!("unknown content `type`: \"{unknown}\"")))
                        }
                    };
                }
                // Guess `TextContent::Text`
                "text" => return deserialize::<D, TextComponent>(content),
                // Guess `TextContent::Translation`
                "translate" => {
                    return deserialize::<D, TranslateComponent>(content);
                }
                // Guess `TextContent::Score`
                "score" => return deserialize::<D, ScoreComponent>(content),
                // Guess `TextContent::Selector`
                "selector" => return deserialize::<D, SelectorComponent>(content),
                // Guess `TextContent::Keybind`
                "keybind" => return deserialize::<D, KeybindComponent>(content),
                // Guess `TextContent::Nbt`
                "nbt" => {
                    return deserialize::<D, ValueComponent>(content);
                }
                // Ambiguous or unknown fields, unable to make any guesses
                _ => {}
            }
        }

        Err(de::Error::custom("no `type` flag and unable to guess type"))
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn formatted_text() {
    #[cfg(not(feature = "std"))]
    use alloc::{borrow::Cow, vec};
    #[cfg(feature = "std")]
    use std::borrow::Cow;

    use crate::{
        prelude::*,
        text::{
            TextInteraction,
            style::{IntegerColor, PresetColor},
        },
    };

    fn from_str(json: &str) -> FormattedText { serde_json::from_str(json).unwrap() }
    fn roundtrip(value: &FormattedText) -> FormattedText {
        let json = serde_json::to_string(value).unwrap();
        #[cfg(all(debug_assertions, feature = "std"))]
        println!("{json}");
        from_str(&json)
    }

    // Test the default formatting.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default(),
        interaction: TextInteraction::default(),
        children: Vec::new(),
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(from_str(r#"{"text":"Hello, World!"}"#), text);

    // Test the default formatting with the color set.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default().with_color(PresetColor::Red),
        interaction: TextInteraction::default(),
        children: Vec::new(),
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(from_str(r#"{"text":"Hello, World!","color":"red"}"#), text);

    // Test the default formatting with bold and italic text.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default().with_bold(true).with_italic(true),
        interaction: TextInteraction::default(),
        children: Vec::new(),
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(from_str(r#"{"bold":true,"italic":true,"text":"Hello, World!"}"#), text);

    // Test the default formatting with children.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default(),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("Child").into()),
            style: TextStyle::default(),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":["Child"]}"#,
        "The text is missing it's children"
    );

    // Test the default formatting with children who have custom formatting.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default(),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("Child").into()),
            style: TextStyle::default().with_color(PresetColor::Red),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":[{"type":"text","text":"Child","color":"red"}]}"#,
        "The children's formatting is not being serialized correctly"
    );

    // Test the default formatting with red text and children who inherit it.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default().with_color(PresetColor::Red),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("Child").into()),
            style: TextStyle::default(),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":["Child"],"color":"red"}"#,
        "The child components are unnecessarily including the parent's formatting"
    );

    // Test the default formatting and children who both have red text.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default().with_color(PresetColor::Red),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("Child").into()),
            style: TextStyle::default().with_color(PresetColor::Red),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    assert_ne!(roundtrip(&text), text);
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":["Child"],"color":"red"}"#,
        "The child components are unnecessarily including the parent's formatting"
    );

    // Test the default formatting with children who have matching formatting.
    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, World!").into()),
        style: TextStyle::default(),
        interaction: TextInteraction::default(),
        children: vec![
            FormattedText {
                content: TextContent::Text(Cow::Borrowed("Child").into()),
                style: TextStyle::default(),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            },
            FormattedText {
                content: TextContent::Text(Cow::Borrowed("Child 2").into()),
                style: TextStyle::default()
                    .with_color(IntegerColor::try_from_hex("111111").unwrap())
                    .with_bold(true),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            },
        ],
    };
    assert_eq!(roundtrip(&text), text);
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r##"{"type":"text","text":"Hello, World!","extra":["Child",{"type":"text","text":"Child 2","color":"#111111","bold":true}]}"##,
        "The child components are unnecessarily including the parent's formatting"
    );
}
