//! [`ServerLink`] and related types

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use smol_str::SmolStr;

use crate::common::UnsizedBuffer;

/// A link sent by the server to the client.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub struct ServerLink {
    /// The text of the label.
    pub label: ServerLinkLabel,
    /// The link text or URL.
    pub link: SmolStr,
}

// -------------------------------------------------------------------------------------------------

/// The label of a server link.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum ServerLinkLabel {
    /// A custom label
    Text(UnsizedBuffer<[u8; 16]>),
    /// A preset label
    Preset(ServerLinkPreset),
}

/// Preset labels for server links.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum ServerLinkPreset {
    BugReport,
    CommunityGuidelines,
    Support,
    Status,
    Feedback,
    Community,
    Website,
    Forums,
    News,
    Announcements,
}
