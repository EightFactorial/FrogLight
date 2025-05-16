//! [`FormattedText::as_message`] and the ansi equivalent if enabled.
//!
//! TODO: Handle legacy style codes

#[cfg(not(feature = "std"))]
use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
};
#[cfg(feature = "std")]
use std::borrow::Cow;

use crate::{
    prelude::*,
    text::{FormattedTextRef, TextContent},
    translate::TextTranslations,
};

impl FormattedText {
    /// Extract the message as a [`String`]
    ///
    /// # Errors
    /// Returns an error if the [`FormattedText`] is not a message,
    /// or if a translation is not found.
    #[inline]
    pub fn as_message(&self, t: &TextTranslations) -> Result<String, ChatMessageError> {
        FormattedTextRef::new(self).as_message(t)
    }

    /// Extract the message as an [`AnsiString`](nu_ansi_term::AnsiString).
    ///
    /// # Errors
    /// Returns an error if the [`FormattedText`] is not a message,
    /// or if a translation is not found.
    #[inline]
    #[cfg(feature = "ansi")]
    pub fn as_message_ansi<'a>(
        &'a self,
        t: &TextTranslations,
    ) -> Result<nu_ansi_term::AnsiString<'a>, ChatMessageError> {
        FormattedTextRef::new(self).as_message_ansi(t)
    }
}

#[allow(clippy::elidable_lifetime_names)]
impl<'a> FormattedTextRef<'a, '_> {
    /// Extract the message as a [`String`]
    ///
    /// # Errors
    /// Returns an error if the [`FormattedText`] is not a message,
    /// or if a translation is not found.
    pub fn as_message(&self, t: &TextTranslations) -> Result<String, ChatMessageError> {
        // Get the message content
        let mut string = match &self.content {
            TextContent::Text(text) => {
                // TODO: Handle legacy style codes
                text.to_string()
            }
            TextContent::Translation(translate) => {
                // Retrieve the translated message or the fallback, if one exists
                let translation = t.get(translate.translate.as_ref()).map_or_else(
                    || translate.fallback.as_ref().map(AsRef::as_ref),
                    |t| Some(t.as_str()),
                );

                if let Some(translation) = translation {
                    // TODO: Handle legacy style codes
                    // Format and insert the message arguments
                    Self::format_message(translation, &translate.arguments, t)?
                } else {
                    return Err(ChatMessageError::UnknownTranslationKey(
                        translate.translate.clone(),
                    ));
                }
            }

            _ => return Err(ChatMessageError::InvalidMessageContent),
        };

        // Append children messages
        for child in &self.children {
            string.push_str(&child.as_message(t)?.to_string());
        }

        Ok(string)
    }

    fn format_message(
        message: &str,
        arguments: &[FormattedText],
        t: &TextTranslations,
    ) -> Result<String, ChatMessageError> {
        let mut formatted = message.to_string();

        for (index, argument) in arguments.iter().enumerate() {
            let resolved = argument.as_message(t)?;
            // Replace the next occurrence of `%s`
            formatted = formatted.replacen("%s", &resolved, 1);
            // Replace all occurrences of `%<index>$s`
            formatted = formatted.replace(&format!("%{index}$"), &resolved);
        }

        Ok(formatted)
    }

    /// Extract the message as an [`AnsiString`](nu_ansi_term::AnsiString).
    ///
    /// # Errors
    /// Returns an error if the [`FormattedText`] is not a message,
    /// or if a translation is not found.
    #[cfg(feature = "ansi")]
    pub fn as_message_ansi(
        &self,
        t: &TextTranslations,
    ) -> Result<nu_ansi_term::AnsiString<'a>, ChatMessageError> {
        // Get the message content
        let mut string = match &self.content {
            TextContent::Text(text) => {
                // TODO: Handle legacy style codes
                text.to_string()
            }
            TextContent::Translation(translate) => {
                // Retrieve the translated message or the fallback, if one exists
                let translation = t.get(translate.translate.as_ref()).map_or_else(
                    || translate.fallback.as_ref().map(AsRef::as_ref),
                    |t| Some(t.as_str()),
                );

                if let Some(translation) = translation {
                    // TODO: Handle legacy style codes
                    // Format and insert the message arguments
                    Self::format_message_ansi(translation, self.style, &translate.arguments, t)?
                } else {
                    return Err(ChatMessageError::UnknownTranslationKey(
                        translate.translate.clone(),
                    ));
                }
            }

            _ => return Err(ChatMessageError::InvalidMessageContent),
        };

        // Apply the message's style
        string = nu_ansi_term::Style::from(self.style).paint(string).to_string();

        // Append children messages
        for child in &self.children {
            let style = child.style.inherit(self.style);
            let child = FormattedTextRef::new(child).with_style(&style);
            string.push_str(&child.as_message_ansi(t)?.to_string());
        }

        Ok(string.into())
    }

    #[cfg(feature = "ansi")]
    fn format_message_ansi(
        message: &str,
        style: &TextStyle,
        arguments: &[FormattedText],
        t: &TextTranslations,
    ) -> Result<String, ChatMessageError> {
        let mut formatted = message.to_string();

        for (index, argument) in arguments.iter().enumerate() {
            let style = argument.style.inherit(style);
            let argument = FormattedTextRef::new(argument).with_style(&style);
            let resolved = argument.as_message_ansi(t)?.to_string();

            // Replace the next occurrence of `%s`
            formatted = formatted.replacen("%s", &resolved, 1);
            // Replace all occurrences of `%<index>$s`
            formatted = formatted.replace(&format!("%{index}$"), &resolved);
        }

        Ok(formatted)
    }
}

// -------------------------------------------------------------------------------------------------

/// An error that occurred while parsing a chat message.
#[derive(Debug, thiserror::Error)]
pub enum ChatMessageError {
    /// Invalid message content
    #[error("Invalid message content")]
    InvalidMessageContent,
    /// An unknown translation key.
    #[error("Unknown translation key: \"{0}\"")]
    UnknownTranslationKey(Cow<'static, str>),
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "ansi")]
impl From<TextStyle> for nu_ansi_term::Style {
    #[inline]
    fn from(value: TextStyle) -> Self { Self::from(&value) }
}

#[cfg(feature = "ansi")]
impl From<&TextStyle> for nu_ansi_term::Style {
    fn from(value: &TextStyle) -> Self {
        let mut style = nu_ansi_term::Style::new();

        if let Some(color) = &value.color {
            let (r, g, b) = color.as_rgb();
            style = style.fg(nu_ansi_term::Color::Rgb(r, g, b));
        }

        if value.bold.is_some_and(|b| b) {
            style = style.bold();
        }
        if value.italic.is_some_and(|i| i) {
            style = style.italic();
        }
        if value.underlined.is_some_and(|u| u) {
            style = style.underline();
        }
        if value.strikethrough.is_some_and(|s| s) {
            style = style.strikethrough();
        }

        style
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn chat_message() {
    #[cfg(not(feature = "std"))]
    use alloc::{borrow::Cow, vec, vec::Vec};
    #[cfg(feature = "std")]
    use std::borrow::Cow;

    use crate::{prelude::*, text::TextInteraction};

    let t = TextTranslations::default();

    assert_eq!(
        FormattedText::from_string("Hello, World!").as_message(&t).unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string("Hello, World!").as_message(&t).unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string("Hello, World!")
            .with_style(TextStyle::default().with_color(PresetColor::Blue))
            .as_message(&t)
            .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText {
            content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
            style: TextStyle::default(),
            interaction: TextInteraction::default(),
            children: vec![FormattedText {
                content: TextContent::Text(Cow::Borrowed("World!").into()),
                style: TextStyle::default(),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            }],
        }
        .as_message(&t)
        .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText {
            content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
            style: TextStyle::default().with_underline(true),
            interaction: TextInteraction::default(),
            children: vec![FormattedText {
                content: TextContent::Text(Cow::Borrowed("World!").into()),
                style: TextStyle::default().with_underline(false).with_italic(true),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            }],
        }
        .as_message(&t)
        .unwrap(),
        "Hello, World!"
    );
}

#[test]
#[cfg(feature = "ansi")]
fn chat_message_ansi() {
    use std::borrow::Cow;

    use crate::{prelude::*, text::TextInteraction};

    let t = TextTranslations::default();

    assert_eq!(
        FormattedText::from_string("Hello, World!").as_message_ansi(&t).unwrap().to_string(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string("Hello, World!")
            .with_style(TextStyle::default())
            .as_message_ansi(&t)
            .unwrap()
            .to_string(),
        "Hello, World!"
    );

    let text = FormattedText::from_string("Hello, World!")
        .with_style(TextStyle::default().with_color(PresetColor::Blue));
    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[38;2;85;85;255mHello, World!\u{1b}[0m");

    let text = FormattedText::from_string("Hello, World!")
        .with_style(TextStyle::default().with_color(IntegerColor::new(0x999999)));
    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[38;2;153;153;153mHello, World!\u{1b}[0m");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default(),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World!").into()),
            style: TextStyle::default(),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "Hello, World!");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default().with_underline(true),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World!").into()),
            style: TextStyle::default().with_underline(false).with_bold(true),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[4mHello, \u{1b}[0m\u{1b}[1mWorld!\u{1b}[0m");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default().with_underline(true),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World!").into()),
            style: TextStyle::default().with_bold(true),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[4mHello, \u{1b}[0m\u{1b}[1;4mWorld!\u{1b}[0m");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default().with_strikethrough(true),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World").into()),
            style: TextStyle::default().with_underline(true).with_strikethrough(false),
            interaction: TextInteraction::default(),
            children: vec![FormattedText {
                content: TextContent::Text(Cow::Borrowed("!").into()),
                style: TextStyle::default().with_color(PresetColor::Red).with_strikethrough(true),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            }],
        }],
    };

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(
        message,
        "\u{1b}[9mHello, \u{1b}[0m\u{1b}[4mWorld\u{1b}[0m\u{1b}[4;9;38;2;255;85;85m!\u{1b}[0m"
    );

    let text = FormattedText::new("Hello, ")
        .with_style(TextStyle::default().with_strikethrough(true))
        .with_children(vec![
            FormattedText::new("World")
                .with_style(TextStyle::default().with_underline(true).with_strikethrough(false))
                .with_children(vec![FormattedText::new("!").with_style(
                    TextStyle::default().with_color(PresetColor::Red).with_strikethrough(true),
                )]),
        ]);

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(
        message,
        "\u{1b}[9mHello, \u{1b}[0m\u{1b}[4mWorld\u{1b}[0m\u{1b}[4;9;38;2;255;85;85m!\u{1b}[0m"
    );
}
