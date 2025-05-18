//! TODO

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
    text::{FormattedTextRef, content::TextContent},
    translate::TextTranslations,
};

#[cfg(feature = "ansi")]
mod ansi;
#[cfg(test)]
mod tests;

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
        let is_numbered = formatted.contains("%0$s");

        for (index, argument) in arguments.iter().enumerate() {
            let resolved = argument.as_message(t)?;

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
