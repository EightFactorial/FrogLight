//! TODO

use alloc::{borrow::Cow, boxed::Box};
#[cfg(feature = "serde")]
use alloc::{format, string::ToString};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_common::prelude::Identifier;
use froglight_nbt::{nbt::mappings::WrapOption, prelude::*};
#[cfg(feature = "serde")]
use serde::{
    __private::{
        de::{Content, ContentDeserializer},
        ser::FlatMapSerializer,
    },
    Deserialize, Deserializer, Serialize, Serializer,
    de::Error,
    ser::SerializeMap,
};
use uuid::Uuid;

/// An interaction to perform when the
/// [`FormattedText`](crate::text::FormattedText) is hovered over.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverInteract {
    /// The action type
    pub action: TextHoverAction,
}

/// An action to perform when the [`FormattedText`](crate::text::FormattedText)
/// is hovered over.
#[derive(Debug, Clone, PartialEq, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub enum TextHoverAction {
    /// Show a text message
    ShowText(Cow<'static, str>),
    /// Show an item
    ShowItem(TextHoverItem),
    /// Show an entity
    ShowEntity(TextHoverEntity),
}

/// An item action to perform when the
/// [`FormattedText`](crate::text::FormattedText) is hovered over.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
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

/// An entity action to perform when the
/// [`FormattedText`](crate::text::FormattedText) is hovered over.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextHoverEntity {
    /// An optional name to display
    #[frog(default, tag = "string",  with = WrapOption, skip_if = Option::is_none)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub name: Option<Cow<'static, str>>,
    /// The entity's type
    #[frog(name = "type", tag = "string")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub kind: Identifier,
    /// The entity's [`Uuid`]
    #[frog(name = "id")]
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    pub uuid: Uuid,
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for TextHoverInteract {
    fn from_compound(_compound: &NbtCompound) -> Result<Self, NbtError> { todo!() }
}
impl IntoCompound for TextHoverInteract {
    fn into_compound(&self) -> Result<NbtCompound, NbtError> { todo!() }
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
            if let Some(text) = content
                .iter()
                .find_map(|(n, c)| (n.as_str() == Some("text")).then_some(c))
                .and_then(|t| t.as_str())
            {
                Ok(TextHoverInteract {
                    action: TextHoverAction::ShowText(Cow::Owned(text.to_string())),
                })
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

        // -----------------------------------------------------------------------------------------

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
