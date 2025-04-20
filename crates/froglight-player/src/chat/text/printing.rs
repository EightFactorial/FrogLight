use super::FormattedContent;
use crate::{chat::translate::TextTranslations, prelude::*};

impl FormattedText {
    /// Extract the message as a [`String`]
    ///
    /// Returns `None` if the [`FormattedText`] is not a chat message,
    /// or if a translation is not found.
    #[inline]
    #[must_use]
    pub fn chat_message(&self, t: &TextTranslations) -> Option<String> {
        FormattedTextRef::new(self).chat_message(t)
    }

    /// Extract the message as an [`AnsiString`](nu_ansi_term::AnsiString).
    ///
    /// Returns `None` if the [`FormattedText`] is not a chat message,
    /// or if a translation is not found.
    #[inline]
    #[must_use]
    #[cfg(feature = "ansi")]
    pub fn chat_message_ansi(&self, t: &TextTranslations) -> Option<nu_ansi_term::AnsiString> {
        FormattedTextRef::new(self).chat_message_ansi(t)
    }
}

impl<'a> FormattedTextRef<'a> {
    /// Extract the message as a [`String`]
    ///
    /// Returns `None` if the [`FormattedText`] is not a chat message,
    /// or if a translation is not found.
    #[must_use]
    pub fn chat_message(&self, t: &TextTranslations) -> Option<String> {
        // Get the message content
        let mut string = match &self.content {
            FormattedContent::Text(text) => text.to_string(),
            FormattedContent::Translation(translate) => {
                // Retrieve the translated message
                let translation = t.get(translate.translate.as_ref()).map_or_else(
                    || translate.fallback.as_ref().map(AsRef::as_ref),
                    |t| Some(t.as_str()),
                )?;
                // Format and insert the message arguments
                Self::format_message(translation, &translate.arguments, t)?
            }

            _ => return None,
        };

        // Append children messages
        for child in &self.children {
            string.push_str(&child.chat_message(t)?.to_string());
        }

        Some(string)
    }

    fn format_message(
        message: &str,
        arguments: &[FormattedText],
        t: &TextTranslations,
    ) -> Option<String> {
        let mut formatted = message.to_string();

        for (index, argument) in arguments.iter().enumerate() {
            let resolved = argument.chat_message(t)?;
            // Replace the next occurrence of `%s`
            formatted = formatted.replacen("%s", &resolved, 1);
            // Replace all occurrences of `%<index>$s`
            formatted = formatted.replace(&format!("%{index}$"), &resolved);
        }

        Some(formatted)
    }

    /// Extract the message as an [`AnsiString`](nu_ansi_term::AnsiString).
    ///
    /// Returns `None` if the [`FormattedText`] is not a chat message,
    /// or if a translation is not found.
    #[must_use]
    #[cfg(feature = "ansi")]
    pub fn chat_message_ansi(&self, t: &TextTranslations) -> Option<nu_ansi_term::AnsiString<'a>> {
        // Get the message content
        let mut string = match &self.content {
            FormattedContent::Text(text) => text.to_string(),
            FormattedContent::Translation(translate) => {
                // Retrieve the translated or fallback message
                let translation = t.get(translate.translate.as_ref()).map_or_else(
                    || translate.fallback.as_ref().map(AsRef::as_ref),
                    |t| Some(t.as_str()),
                )?;

                // Format and insert the message arguments
                Self::format_message_ansi(translation, &self.formatting, &translate.arguments, t)?
            }

            _ => return None,
        };
        // Apply the message's formatting
        string = nu_ansi_term::Style::from(&*self.formatting).paint(string).to_string();

        // Append children messages
        for child in &self.children {
            let formatting = child.formatting.inherit(&self.formatting);
            let child = FormattedTextRef::new_with(child, &formatting);
            string.push_str(&child.chat_message_ansi(t)?.to_string());
        }

        Some(string.into())
    }

    #[cfg(feature = "ansi")]
    fn format_message_ansi(
        message: &str,
        formatting: &TextFormatting,
        arguments: &[FormattedText],
        t: &TextTranslations,
    ) -> Option<String> {
        let mut formatted = message.to_string();

        for (index, argument) in arguments.iter().enumerate() {
            let formatting = argument.formatting.inherit(formatting);
            let argument = FormattedTextRef::new_with(argument, &formatting);
            let resolved = argument.chat_message_ansi(t)?.to_string();

            // Replace the next occurrence of `%s`
            formatted = formatted.replacen("%s", &resolved, 1);
            // Replace all occurrences of `%<index>$s`
            formatted = formatted.replace(&format!("%{index}$"), &resolved);
        }

        Some(formatted)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "ansi")]
impl From<TextFormatting> for nu_ansi_term::Style {
    #[inline]
    fn from(value: TextFormatting) -> Self { Self::from(&value) }
}

#[cfg(feature = "ansi")]
impl From<&TextFormatting> for nu_ansi_term::Style {
    fn from(value: &TextFormatting) -> Self {
        let mut style = nu_ansi_term::Style::new();

        if let Some(color) = &value.color {
            let (r, g, b) = color.as_integer().into_rgb();
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
    use std::borrow::Cow;

    let t = TextTranslations::default();

    assert_eq!(FormattedText::from("Hello, World!").chat_message(&t).unwrap(), "Hello, World!");

    assert_eq!(
        FormattedText::from_string_with("Hello, World!".into(), TextFormatting::empty())
            .chat_message(&t)
            .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string_with(
            "Hello, World!".into(),
            TextFormatting::empty().with_color(TextColor::Blue)
        )
        .chat_message(&t)
        .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("Hello, ").into()),
            formatting: TextFormatting::empty(),
            interact: TextInteraction::default(),
            children: vec![FormattedText {
                content: FormattedContent::Text(Cow::Borrowed("World!").into()),
                formatting: TextFormatting::empty(),
                interact: TextInteraction::default(),
                children: Vec::new(),
            }],
        }
        .chat_message(&t)
        .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("Hello, ").into()),
            formatting: TextFormatting::empty().with_underline(true),
            interact: TextInteraction::default(),
            children: vec![FormattedText {
                content: FormattedContent::Text(Cow::Borrowed("World!").into()),
                formatting: TextFormatting::empty().with_underline(false).with_italic(true),
                interact: TextInteraction::default(),
                children: Vec::new(),
            }],
        }
        .chat_message(&t)
        .unwrap(),
        "Hello, World!"
    );
}

#[test]
#[cfg(feature = "ansi")]
fn chat_message_ansi() {
    use std::borrow::Cow;

    let t = TextTranslations::default();

    assert_eq!(
        FormattedText::from_string("Hello, World!").chat_message_ansi(&t).unwrap().to_string(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string_with("Hello, World!".into(), TextFormatting::empty())
            .chat_message_ansi(&t)
            .unwrap()
            .to_string(),
        "Hello, World!"
    );

    let text = FormattedText::from_string_with(
        "Hello, World!".into(),
        TextFormatting::empty().with_color(TextColor::Blue),
    );
    let message = text.chat_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[38;2;85;85;255mHello, World!\u{1b}[0m");

    let text = FormattedText::from_string_with(
        "Hello, World!".into(),
        TextFormatting::empty().with_color(TextColor::Custom("#999999".into())),
    );
    let message = text.chat_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[38;2;153;153;153mHello, World!\u{1b}[0m");

    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, ").into()),
        formatting: TextFormatting::empty(),
        interact: TextInteraction::default(),
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("World!").into()),
            formatting: TextFormatting::empty(),
            interact: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    let message = text.chat_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "Hello, World!");

    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, ").into()),
        formatting: TextFormatting::empty().with_underline(true),
        interact: TextInteraction::default(),
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("World!").into()),
            formatting: TextFormatting::empty().with_underline(false).with_bold(true),
            interact: TextInteraction::default(),
            children: Vec::new(),
        }],
    };

    let message = text.chat_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[4mHello, \u{1b}[0m\u{1b}[1mWorld!\u{1b}[0m");

    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, ").into()),
        formatting: TextFormatting::empty().with_underline(true),
        interact: TextInteraction::empty(),
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("World!").into()),
            formatting: TextFormatting::empty().with_bold(true),
            interact: TextInteraction::empty(),
            children: Vec::new(),
        }],
    };

    let message = text.chat_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[4mHello, \u{1b}[0m\u{1b}[1;4mWorld!\u{1b}[0m");

    let text = FormattedText {
        content: FormattedContent::Text(Cow::Borrowed("Hello, ").into()),
        formatting: TextFormatting::empty().with_strikethrough(true),
        interact: TextInteraction::empty(),
        children: vec![FormattedText {
            content: FormattedContent::Text(Cow::Borrowed("World").into()),
            formatting: TextFormatting::empty().with_underline(true).with_strikethrough(false),
            interact: TextInteraction::empty(),
            children: vec![FormattedText {
                content: FormattedContent::Text(Cow::Borrowed("!").into()),
                formatting: TextFormatting::empty()
                    .with_color(TextColor::Red)
                    .with_strikethrough(true),
                interact: TextInteraction::empty(),
                children: Vec::new(),
            }],
        }],
    };

    let message = text.chat_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(
        message,
        "\u{1b}[9mHello, \u{1b}[0m\u{1b}[4mWorld\u{1b}[0m\u{1b}[4;9;38;2;255;85;85m!\u{1b}[0m"
    );
}
