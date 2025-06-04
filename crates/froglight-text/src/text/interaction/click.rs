//! TODO

use alloc::{borrow::Cow, boxed::Box, string::ToString};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_nbt::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An interaction to perform when the
/// [`FormattedText`](crate::text::FormattedText) is clicked.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct TextClickInteract {
    /// The action type
    pub action: TextClickAction,
    /// The value to pass to the action
    #[frog(tag = "string")]
    pub value: Cow<'static, str>,
}

/// The action to perform when the [`FormattedText`](crate::text::FormattedText)
/// is clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
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

impl FromTag for TextClickAction {
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::String(string) => {
                let string = string.to_str_lossy();
                match string.as_ref() {
                    "open_url" => Ok(Self::OpenUrl),
                    "open_file" => Ok(Self::OpenFile),
                    "run_command" => Ok(Self::RunCommand),
                    "suggest_command" => Ok(Self::SuggestCommand),
                    "change_page" => Ok(Self::ChangePage),
                    "copy_to_clipboard" => Ok(Self::CopyToClipboard),
                    _ => Err(NbtError::UnknownVariant(
                        core::any::type_name::<Self>(),
                        string.to_string(),
                    )),
                }
            }
            _ => Err(NbtError::MismatchedTag(core::any::type_name::<Self>(), "String")),
        }
    }
}

impl IntoTag for TextClickAction {
    fn into_tag(&self) -> Result<NbtTag, NbtError> {
        Ok(NbtTag::String(Mutf8String::from_string(match self {
            Self::OpenUrl => "open_url",
            Self::OpenFile => "open_file",
            Self::RunCommand => "run_command",
            Self::SuggestCommand => "suggest_command",
            Self::ChangePage => "change_page",
            Self::CopyToClipboard => "copy_to_clipboard",
        })))
    }
}
