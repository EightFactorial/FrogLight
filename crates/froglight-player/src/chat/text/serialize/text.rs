use serde::{
    __private::ser::FlatMapSerializer,
    Deserialize, Deserializer, Serialize, Serializer,
    ser::{SerializeMap, SerializeSeq},
};

use crate::{
    chat::text::{FormattedContent, FormattedText, TextFormatting},
    prelude::FormattedTextRef,
};

impl Serialize for FormattedText {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        Child(FormattedTextRef::new(self), &TextFormatting::DEFAULT).serialize(ser)
    }
}
impl Serialize for FormattedTextRef<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        Child(self.clone(), &TextFormatting::DEFAULT).serialize(ser)
    }
}

/// Serialize a slice of [`FormattedText`] children while inheriting formatting.
struct Children<'a>(&'a [FormattedText], &'a TextFormatting);
impl Serialize for Children<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut ser = ser.serialize_seq(Some(self.0.len()))?;
        self.0.iter().try_for_each(|c| ser.serialize_element(&Child(c.into(), self.1)))?;
        ser.end()
    }
}

/// Serialize a [`FormattedText`] while inheriting formatting.
struct Child<'a>(FormattedTextRef<'a>, &'a TextFormatting);
impl Serialize for Child<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut ser = ser.serialize_map(None)?;

        // Serialize the text content
        match &self.0.content {
            FormattedContent::Text(c) => {
                ser.serialize_entry("type", "text")?;
                c.serialize(FlatMapSerializer(&mut ser))?;
            }
            FormattedContent::Translation(c) => {
                ser.serialize_entry("type", "translatable")?;
                c.serialize(FlatMapSerializer(&mut ser))?;
            }
            FormattedContent::Score(c) => {
                ser.serialize_entry("type", "score")?;
                c.serialize(FlatMapSerializer(&mut ser))?;
            }
            FormattedContent::Selector(c) => {
                ser.serialize_entry("type", "selector")?;
                c.serialize(FlatMapSerializer(&mut ser))?;
            }
            FormattedContent::Keybind(c) => {
                ser.serialize_entry("type", "keybind")?;
                c.serialize(FlatMapSerializer(&mut ser))?;
            }
            FormattedContent::Nbt(c) => {
                ser.serialize_entry("type", "nbt")?;
                c.serialize(FlatMapSerializer(&mut ser))?;
            }
        }

        // Serialize the text's children recursively
        let inherit = self.0.formatting.inherit(self.1);
        if !self.0.children.is_empty() {
            ser.serialize_entry("extra", &Children(&self.0.children, &inherit))?;
        }

        // Serialize the differences from the parent text's formatting
        let diff = inherit.difference(self.1);
        diff.serialize(FlatMapSerializer(&mut ser))?;

        // Serialize the text's interaction settings
        if let Some(interact) = &self.0.interact {
            interact.serialize(FlatMapSerializer(&mut ser))?;
        }

        ser.end()
    }
}

// -------------------------------------------------------------------------------------------------

impl<'de> Deserialize<'de> for FormattedText {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn formatted_text() {
    use std::borrow::Cow;

    use crate::chat::text::formatting::{TextColor, TextFormatting};

    // Test the default formatting.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default(),
        interact: None,
        children: Vec::new(),
    };
    assert_eq!(serde_json::to_string(&text).unwrap(), r#"{"type":"text","text":"Hello, World!"}"#);

    // Test the default formatting with the color set.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default().with_color(TextColor::Red),
        interact: None,
        children: Vec::new(),
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","color":"red"}"#
    );

    // Test the default formatting with bold and italic text.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default().with_bold(true).with_italic(true),
        interact: None,
        children: Vec::new(),
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","bold":true,"italic":true}"#
    );

    // Test the default formatting with children.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default(),
        interact: None,
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("Child").into()),
            formatting: TextFormatting::empty(),
            interact: None,
            children: Vec::new(),
        }],
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":[{"type":"text","text":"Child"}]}"#,
        "The text is missing it's children"
    );

    // Test the default formatting with children who have custom formatting.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default(),
        interact: None,
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("Child").into()),
            formatting: TextFormatting::empty().with_color(TextColor::Red),
            interact: None,
            children: Vec::new(),
        }],
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":[{"type":"text","text":"Child","color":"red"}]}"#,
        "The children's formatting is not being serialized correctly"
    );

    // Test the default formatting with red text and children who inherit it.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default().with_color(TextColor::Red),
        interact: None,
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("Child").into()),
            formatting: TextFormatting::empty(),
            interact: None,
            children: Vec::new(),
        }],
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":[{"type":"text","text":"Child"}],"color":"red"}"#,
        "The child components are unnecessarily including the parent's formatting"
    );

    // Test the default formatting and children who both have red text.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default().with_color(TextColor::Red),
        interact: None,
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("Child").into()),
            formatting: TextFormatting::empty().with_color(TextColor::Red),
            interact: None,
            children: Vec::new(),
        }],
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r#"{"type":"text","text":"Hello, World!","extra":[{"type":"text","text":"Child"}],"color":"red"}"#,
        "The child components are unnecessarily including the parent's formatting"
    );

    // Test the default formatting with children who have matching formatting.
    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, World!").into()),
        formatting: TextFormatting::default(),
        interact: None,
        children: vec![
            FormattedText {
                content: FormattedContent::Text(Cow::Borrowed("Child").into()),
                formatting: TextFormatting::default(),
                interact: None,
                children: Vec::new(),
            },
            FormattedText {
                content: FormattedContent::Text(Cow::Borrowed("Child 2").into()),
                formatting: TextFormatting::empty()
                    .with_color(TextColor::Custom("#111111".into()))
                    .with_bold(true),
                interact: None,
                children: Vec::new(),
            },
        ],
    };
    assert_eq!(
        serde_json::to_string(&text).unwrap(),
        r##"{"type":"text","text":"Hello, World!","extra":[{"type":"text","text":"Child"},{"type":"text","text":"Child 2","color":"#111111","bold":true}]}"##,
        "The child components are unnecessarily including the parent's formatting"
    );
}
