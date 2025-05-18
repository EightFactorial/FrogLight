use super::ChatMessageError;
use crate::{
    prelude::*,
    text::{FormattedTextRef, content::TextContent},
    translate::TextTranslations,
};

impl FormattedText {
    /// Extract the message as an [`AnsiString`](nu_ansi_term::AnsiString).
    ///
    /// # Errors
    /// Returns an error if the [`FormattedText`] is not a message,
    /// or if a translation is not found.
    #[inline]
    pub fn as_message_ansi<'a>(
        &'a self,
        t: &TextTranslations,
    ) -> Result<nu_ansi_term::AnsiString<'a>, ChatMessageError> {
        FormattedTextRef::new(self).as_message_ansi(t)
    }
}

#[allow(clippy::elidable_lifetime_names)]
impl<'a> FormattedTextRef<'a, '_> {
    /// Extract the message as an [`AnsiString`](nu_ansi_term::AnsiString).
    ///
    /// # Errors
    /// Returns an error if the [`FormattedText`] is not a message,
    /// or if a translation is not found.
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

    fn format_message_ansi(
        message: &str,
        style: &TextStyle,
        arguments: &[FormattedText],
        t: &TextTranslations,
    ) -> Result<String, ChatMessageError> {
        let mut formatted = message.to_string();
        let is_numbered = formatted.contains("%0$s");

        for (index, argument) in arguments.iter().enumerate() {
            let style = argument.style.inherit(style);
            let argument = FormattedTextRef::new(argument).with_style(&style);
            let resolved = argument.as_message_ansi(t)?.to_string();

            if is_numbered {
                // Replace all occurrences of `%<index>$s`
                formatted = formatted.replace(&format!("%{index}$s"), &resolved);
            } else {
                // Replace the next occurrence of `%s`
                formatted = formatted.replacen("%s", &resolved, 1);
            }
        }

        Ok(formatted)
    }
}

// -------------------------------------------------------------------------------------------------

impl From<TextStyle> for nu_ansi_term::Style {
    #[inline]
    fn from(value: TextStyle) -> Self { Self::from(&value) }
}

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
