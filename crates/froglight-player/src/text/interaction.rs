//! [`InteractComponent`]

use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_common::prelude::Identifier;
use froglight_nbt::nbt::NbtCompound;
use serde::{
    __private::{
        de::{Content, ContentDeserializer},
        ser::FlatMapSerializer,
    },
    Deserializer, Serializer,
    de::Error,
    ser::SerializeMap,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Actions to take when interacting with a [`FormattedText`].
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextInteraction {
    /// Text to insert when the component is interacted with.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub insertion: Option<Cow<'static, str>>,
    /// An action to perform when the component is clicked.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "clickEvent", skip_serializing_if = "Option::is_none")
    )]
    pub click: Option<TextClickInteract>,
    /// An action to perform when the component is hovered over.
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "hoverEvent", skip_serializing_if = "Option::is_none")
    )]
    pub hover: Option<TextHoverInteract>,
}

impl TextInteraction {
    /// An empty [`InteractComponent`].
    pub const EMPTY: Self = Self { insertion: None, click: None, hover: None };

    /// Create a new empty [`InteractComponent`].
    #[must_use]
    pub const fn empty() -> Self { Self::EMPTY }

    /// Update the [`InteractComponent`] with the given insertion text.
    #[inline]
    #[must_use]
    pub fn with_insert(mut self, insert: impl Into<Cow<'static, str>>) -> Self {
        self.insertion = Some(insert.into());
        self
    }

    /// Update the [`InteractComponent`] with the given [`TextClickInteract`].
    #[inline]
    #[must_use]
    pub fn with_click(mut self, click: impl Into<TextClickInteract>) -> Self {
        self.click = Some(click.into());
        self
    }

    /// Update the [`InteractComponent`] with the given [`TextHoverInteract`].
    #[inline]
    #[must_use]
    pub fn with_hover(mut self, hover: impl Into<TextHoverInteract>) -> Self {
        self.hover = Some(hover.into());
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// An interaction to perform when the [`FormattedText`] is clicked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextClickInteract {
    /// The action type
    pub action: TextClickAction,
    /// The value to pass to the action
    pub value: Cow<'static, str>,
}

/// The action to perform when the [`FormattedText`] is clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum TextClickAction {
    /// A URL to open in the browser.
    #[cfg_attr(feature = "serde", serde(rename = "open_url"))]
    OpenUrl,
    /// A file to open on the computer.
    #[cfg_attr(feature = "serde", serde(rename = "open_file"))]
    OpenFile,
    /// A chat command to send to the server.
    #[cfg_attr(feature = "serde", serde(rename = "run_command"))]
    RunCommand,
    /// Fill in a field in the chat command.
    #[cfg_attr(feature = "serde", serde(rename = "suggest_command"))]
    SuggestCommand,
    /// Change to a page in a written book.
    #[cfg_attr(feature = "serde", serde(rename = "change_page"))]
    ChangePage,
    /// Copy the text to the clipboard.
    #[cfg_attr(feature = "serde", serde(rename = "copy_to_clipboard"))]
    CopyToClipboard,
}

// -------------------------------------------------------------------------------------------------

/// An interaction to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverInteract {
    /// The action type
    pub action: TextHoverAction,
}

/// An action to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub enum TextHoverAction {
    /// Show a text message
    ShowText(Cow<'static, str>),
    /// Show an item
    ShowItem(TextHoverItem),
    /// Show an entity
    ShowEntity(TextHoverEntity),
}

/// An item action to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverItem {
    /// The item's identifier
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    pub ident: Identifier,
    /// Optionally, the number of items in the stack
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub count: Option<u32>,
    /// Additional NBT components
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub components: Option<NbtCompound>,
}

/// An entity action to perform when the [`FormattedText`] is hovered over.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverEntity {
    /// An optional name to display
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub name: Option<Cow<'static, str>>,
    /// The entity's type
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: Identifier,
    /// The entity's [`Uuid`]
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    pub uuid: Uuid,
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl Serialize for TextHoverInteract {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map: S::SerializeMap;

        match &self.action {
            TextHoverAction::ShowText(text) => {
                map = ser.serialize_map(Some(2))?;
                map.serialize_entry("action", "show_text")?;
                map.serialize_entry("text", text)?;
            }
            TextHoverAction::ShowItem(item) => {
                map = ser.serialize_map(Some(4))?;
                map.serialize_entry("action", "show_item")?;
                item.serialize(FlatMapSerializer(&mut map))?;
            }
            TextHoverAction::ShowEntity(entity) => {
                map = ser.serialize_map(Some(4))?;
                map.serialize_entry("action", "show_entity")?;
                entity.serialize(FlatMapSerializer(&mut map))?;
            }
        }

        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TextHoverInteract {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        /// Deserialize a [`TextHoverInteract`]
        /// with a [`TextHoverAction::ShowText`].
        fn text<'de, D: Deserializer<'de>>(
            content: &[(Content<'de>, Content<'de>)],
        ) -> Result<TextHoverInteract, D::Error> {
            if let Some(text) =
                content.iter().find_map(|(n, c)| (n.as_str() == Some("text")).then_some(c))
            {
                if let Some(text) = text.as_str() {
                    let text = Cow::Owned(text.to_string());
                    Ok(TextHoverInteract { action: TextHoverAction::ShowText(text) })
                } else {
                    Err(Error::custom("expected `text` to be a string"))
                }
            } else {
                Err(Error::custom("expected a `text` field"))
            }
        }

        /// Deserialize a [`TextHoverInteract`] with either a
        /// [`TextHoverAction::ShowItem`] or [`TextHoverAction::ShowEntity`].
        fn action<'de, D: Deserializer<'de>, T: Into<TextHoverAction> + Deserialize<'de>>(
            content: Content<'de>,
        ) -> Result<TextHoverInteract, D::Error> {
            let de = ContentDeserializer::<'de, D::Error>::new(content);
            Ok(TextHoverInteract { action: T::deserialize(de)?.into() })
        }

        // Deserialize the provided content for interpretation
        let content = Content::deserialize(de)?;
        let Content::Map(content_map) = &content else {
            return Err(Error::custom("expected a map"));
        };

        // Loop over the provided map keys for hints on the type of interaction
        for (item, item_content) in content_map {
            let Some(name) = item.as_str() else { continue };
            match name {
                // Received the type of action
                "action" => {
                    let content_type = item_content
                        .as_str()
                        .ok_or_else(|| Error::custom("expected `action` to be a string"))?;

                    return match content_type {
                        // Deserialize `TextHoverAction::ShowText`
                        "show_text" => text::<D>(content_map),
                        // Deserialize `TextHoverAction::ShowItem`
                        "show_item" => action::<D, TextHoverItem>(content),
                        // Deserialize `TextHoverAction::ShowEntity`
                        "show_entity" => action::<D, TextHoverEntity>(content),
                        unknown => {
                            Err(Error::custom(format!("unknown `action` type: \"{unknown}\"")))
                        }
                    };
                }
                // Guess `TextHoverAction::ShowText`
                "text" => return text::<D>(content_map),
                // Guess `TextHoverAction::ShowItem`
                "components" | "count" => return action::<D, TextHoverItem>(content),
                // Guess `TextHoverAction::ShowEntity`
                "type" | "name" => return action::<D, TextHoverEntity>(content),
                // Ambiguous fields, unable to make any guesses
                "id" => {}
                // Completely unknown fields, return an error
                unk => return Err(Error::custom(format!("unknown field `{unk}`"))),
            }
        }

        Err(Error::custom("no `action` flag and unable to guess type"))
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(feature = "serde")]
fn serde() {
    fn from_str(json: &str) -> TextInteraction { serde_json::from_str(json).unwrap() }
    fn roundtrip(value: &TextInteraction) -> TextInteraction {
        let json = serde_json::to_string(value).unwrap();
        #[cfg(debug_assertions)]
        println!("{json}");
        from_str(&json)
    }

    let none = TextInteraction::default();
    assert_eq!(roundtrip(&none), none);
    assert_eq!(from_str("{}"), none);

    let with_insert = TextInteraction::default().with_insert("Hello, World!");
    assert_eq!(roundtrip(&with_insert), with_insert);
    assert_eq!(from_str(r#"{"insertion":"Hello, World!"}"#), with_insert);

    let click_suggest =
        TextClickInteract { action: TextClickAction::SuggestCommand, value: "@a".into() };
    let with_suggest = TextInteraction::default().with_insert("@a").with_click(click_suggest);
    assert_eq!(roundtrip(&with_suggest), with_suggest);
    assert_eq!(
        from_str(r#"{"insertion":"@a","clickEvent":{"action":"suggest_command","value":"@a"}}"#),
        with_suggest
    );

    let click_url =
        TextClickInteract { action: TextClickAction::OpenUrl, value: "https://github.com".into() };
    let with_open_url = TextInteraction::default().with_click(click_url.clone());
    assert_eq!(roundtrip(&with_open_url), with_open_url);
    assert_eq!(
        from_str(r#"{"clickEvent":{"action":"open_url","value":"https://github.com"}}"#),
        with_open_url
    );

    let hover_text =
        TextHoverInteract { action: TextHoverAction::ShowText("Hello, World!".into()) };
    let with_hover_text =
        TextInteraction::default().with_click(click_url.clone()).with_hover(hover_text);
    assert_eq!(roundtrip(&with_hover_text), with_hover_text);
    assert_eq!(
        from_str(
            r#"{"clickEvent":{"action":"open_url","value":"https://github.com"},"hoverEvent":{"action":"show_text","text":"Hello, World!"}}"#
        ),
        with_hover_text
    );

    let hover_item = TextHoverInteract {
        action: TextHoverAction::ShowItem(TextHoverItem {
            ident: Identifier::const_new("minecraft:diamond"),
            count: Some(1),
            components: None,
        }),
    };
    let with_hover_item =
        TextInteraction::default().with_click(click_url.clone()).with_hover(hover_item);
    assert_eq!(roundtrip(&with_hover_item), with_hover_item);
    assert_eq!(
        from_str(
            r#"{"clickEvent":{"action":"open_url","value":"https://github.com"},"hoverEvent":{"id":"minecraft:diamond","count":1}}"#
        ),
        with_hover_item
    );

    let hover_entity = TextHoverInteract {
        action: TextHoverAction::ShowEntity(TextHoverEntity {
            name: None,
            kind: Identifier::const_new("minecraft:player"),
            uuid: Uuid::nil(),
        }),
    };
    let with_hover_entity = TextInteraction::default().with_hover(hover_entity);
    assert_eq!(roundtrip(&with_hover_entity), with_hover_entity);
    assert_eq!(
        from_str(
            r#"{"hoverEvent":{"type":"minecraft:player","id":"00000000-0000-0000-0000-000000000000"}}"#
        ),
        with_hover_entity
    );
}
