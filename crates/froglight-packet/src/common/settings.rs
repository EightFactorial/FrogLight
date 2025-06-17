//! [`ClientSettings`] and related types.

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use serde::{Deserialize, Serialize};
use smol_str::{SmolStr, ToSmolStr};

use crate::common::PlayerHand;

/// A client's settings.
///
/// Sent to the server during the login process and
/// whenever the client changes their settings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct ClientSettings {
    /// The language the client is using.
    pub language: SmolStr,
    /// The view distance of the client, in chunks.
    pub view_distance: u8,
    /// The types of chat messages to send to the client.
    pub chat_visibility: ChatVisibilityMode,
    /// Whether chat messages should be colored.
    pub chat_colors: bool,
    /// The layers of the player model that should be displayed.
    pub model_layers: PlayerModelLayers,
    /// The player's main hand preference.
    pub main_hand: PlayerHand,
    /// Whether text filtering is enabled.
    pub text_filtering: bool,
    /// Whether the client shows in the server's player list.
    pub allow_listing: bool,
    /// The level of particles displayed by the client.
    pub particles: ParticleMode,
}

impl ClientSettings {
    /// The default [`ClientSettings`] for a player.
    pub const DEFAULT: Self = Self {
        language: SmolStr::new_static("en_us"),
        view_distance: 8,
        chat_visibility: ChatVisibilityMode::Full,
        chat_colors: true,
        model_layers: PlayerModelLayers::ALL,
        main_hand: PlayerHand::Right,
        text_filtering: false,
        allow_listing: true,
        particles: ParticleMode::All,
    };

    /// Set the language of the [`ClientSettings`] inline.
    #[inline]
    #[must_use]
    pub fn with_lang(self, language: impl ToSmolStr) -> Self {
        Self { language: language.to_smolstr(), ..self }
    }

    /// Set the [`PlayerModelLayers`] of the [`ClientSettings`] inline.
    #[inline]
    #[must_use]
    pub fn with_layers(self, layers: PlayerModelLayers) -> Self {
        Self { model_layers: layers, ..self }
    }

    /// Set the [`PlayerHand`] of the [`ClientSettings`] inline.
    #[inline]
    #[must_use]
    pub fn with_hand(self, main_hand: PlayerHand) -> Self { Self { main_hand, ..self } }

    /// Set the [`ParticleMode`] of the [`ClientSettings`] inline.
    #[inline]
    #[must_use]
    pub fn with_particles(self, particles: ParticleMode) -> Self { Self { particles, ..self } }
}

impl Default for ClientSettings {
    fn default() -> Self { Self::DEFAULT }
}

// -------------------------------------------------------------------------------------------------

/// The level of chat messages that should be sent to the client.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[cfg_attr(
    feature = "bevy",
    derive(Reflect),
    reflect(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)
)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum ChatVisibilityMode {
    /// All messages should be sent to the client.
    #[default]
    Full,
    /// Only server messages should be sent to the client.
    System,
    /// No messages should be sent to the client.
    Hidden,
}

// -------------------------------------------------------------------------------------------------

/// The layers of the player model that can be toggled on or off.
///
/// TODO: Add "bitset" flag to `FrogBuf` to read/write as a `BitSet<N>`.
#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct PlayerModelLayers {
    /// Whether the player has a cape.
    pub cape: bool,
    /// Whether the player has the jacket layer.
    pub jacket: bool,
    /// Whether the player has the left sleeve layer.
    pub left_sleeve: bool,
    /// Whether the player has the right sleeve layer.
    pub right_sleeve: bool,
    /// Whether the player has the left pants layer.
    pub left_pants: bool,
    /// Whether the player has the right pants layer.
    pub right_pants: bool,
    /// Whether the player has the hat layer.
    pub hat: bool,
}

impl PlayerModelLayers {
    /// A [`PlayerModelLayers`] with all layers enabled.
    pub const ALL: Self = Self {
        cape: true,
        jacket: true,
        left_sleeve: true,
        right_sleeve: true,
        left_pants: true,
        right_pants: true,
        hat: true,
    };
    /// A [`PlayerModelLayers`] with all layers disabled.
    pub const NONE: Self = Self {
        cape: false,
        jacket: false,
        left_sleeve: false,
        right_sleeve: false,
        left_pants: false,
        right_pants: false,
        hat: false,
    };
}

impl Default for PlayerModelLayers {
    fn default() -> Self { Self::ALL }
}

// -------------------------------------------------------------------------------------------------

/// The level of particles displayed by the client.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[cfg_attr(
    feature = "bevy",
    derive(Reflect),
    reflect(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)
)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub enum ParticleMode {
    /// All particles are displayed.
    #[default]
    All,
    /// Some particles are displayed.
    Decreased,
    /// Only minimal particles are displayed.
    Minimal,
}
