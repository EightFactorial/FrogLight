//! [`TextInteraction`]

use alloc::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_nbt::{nbt::mappings::WrapOption, prelude::*};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod click;
pub use click::TextClickInteract;

pub mod hover;
pub use hover::TextHoverInteract;

/// Actions to take when interacting with a
/// [`FormattedText`](crate::text::FormattedText).
#[derive(Debug, Default, Clone, PartialEq, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextInteraction {
    /// Text to insert when the component is interacted with.
    #[frog(default, tag = "string", with = WrapOption, skip_if = Option::is_empty)]
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub insertion: Option<Cow<'static, str>>,
    /// An action to perform when the component is clicked.
    #[frog(default, name = "clickEvent", skip_if = Option::is_empty)]
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "clickEvent", skip_serializing_if = "Option::is_none")
    )]
    pub click: Option<TextClickInteract>,
    /// An action to perform when the component is hovered over.
    #[frog(default, name = "hoverEvent", skip_if = Option::is_empty)]
    #[cfg_attr(
        feature = "serde",
        serde(default, rename = "hoverEvent", skip_serializing_if = "Option::is_none")
    )]
    pub hover: Option<TextHoverInteract>,
}

impl TextInteraction {
    /// The default [`TextInteraction`].
    pub const DEFAULT: Self = Self { insertion: None, click: None, hover: None };

    /// Returns `true` if all fields are `None`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.insertion.is_none() && self.click.is_none() && self.hover.is_none()
    }

    /// Update the [`TextInteraction`] with the given insertion text.
    #[inline]
    #[must_use]
    pub fn with_insert(mut self, insert: impl Into<Cow<'static, str>>) -> Self {
        self.insertion = Some(insert.into());
        self
    }

    /// Update the [`TextInteraction`] with the given [`TextClickInteract`].
    #[inline]
    #[must_use]
    pub fn with_click(mut self, click: impl Into<TextClickInteract>) -> Self {
        self.click = Some(click.into());
        self
    }

    /// Update the [`TextInteraction`] with the given [`TextHoverInteract`].
    #[inline]
    #[must_use]
    pub fn with_hover(mut self, hover: impl Into<TextHoverInteract>) -> Self {
        self.hover = Some(hover.into());
        self
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[cfg(feature = "serde")]
fn serde() {
    use froglight_common::prelude::*;
    use uuid::Uuid;

    use crate::text::interaction::{
        click::TextClickAction,
        hover::{TextHoverAction, TextHoverEntity, TextHoverItem},
    };

    fn from_str(json: &str) -> TextInteraction { serde_json::from_str(json).unwrap() }
    fn roundtrip(value: &TextInteraction) -> TextInteraction {
        let json = serde_json::to_string(value).unwrap();
        #[cfg(all(debug_assertions, feature = "std"))]
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
