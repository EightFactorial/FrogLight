//! TODO

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An interaction to perform when the [`FormattedText`] is clicked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
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
