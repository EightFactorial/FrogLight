#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, string::ToString, vec::Vec};
#[cfg(feature = "std")]
use std::borrow::Cow;

use crate::{prelude::*, text::style::LegacyCode};

impl FormattedText {
    /// Update the [`FormattedText`], applying legacy formatting codes.
    ///
    /// This method will edit the text in place,
    /// potentially altering the text's cryptographic signature.
    pub fn apply_legacy_formatting(&mut self) -> &mut Self {
        if let TextContent::Text(component) = &mut self.content
            && component.text.contains('§')
        {
            let component = core::mem::replace(&mut component.text, Cow::Borrowed(""));
            let mut formatted = Self::format_legacy_text(&component);

            formatted.append(&mut self.children);
            self.children = formatted;
        } else {
            self.children.iter_mut().for_each(|child| {
                child.apply_legacy_formatting();
            });
        }

        self
    }

    /// Split a string into [`FormattedText`] segments with
    /// formatting based on their [`LegacyCode`].
    #[must_use]
    fn format_legacy_text(text: &str) -> Vec<FormattedText> {
        let mut style = TextStyle::EMPTY;
        let mut result = Vec::new();

        let prefixed = text.starts_with('§');
        for (i, mut segment) in text.split_inclusive('§').enumerate() {
            segment = segment.trim_end_matches('§');

            if i == 0 && !prefixed {
                // Append the first segment if it doesn't start with '§'
                result.push(FormattedText::from_string(segment.to_string()));
            } else if let Some((code, segment)) = segment.split_at_checked(1) {
                // If there is a legacy code, apply it to the current style
                if let Some(code) = code.chars().next().and_then(LegacyCode::try_from_char) {
                    code.apply_to_style(&mut style);
                }

                // Append the segment with the current style
                if !segment.is_empty() {
                    result.push(
                        FormattedText::from_string(segment.to_string()).with_style(style.clone()),
                    );
                }
            }
        }

        result
    }
}
