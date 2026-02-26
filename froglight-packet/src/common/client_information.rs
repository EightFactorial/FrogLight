#![expect(clippy::struct_excessive_bools, reason = "Bitfield")]
#![allow(missing_docs, reason = "TODO")]

use alloc::string::String;

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
#[cfg(feature = "facet")]
use facet::{Facet, Partial, Peek};
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    replace_with::replace_with_or_abort,
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};

/// Information about the client and player.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub struct ClientInformation {
    /// The locale of the client, formatted like "en_us".
    pub language: String,
    /// The view distance of the client in chunks.
    pub view_distance: u8,
    /// The types of messages visible to the player.
    pub chat_visibility: ChatVisibility,
    /// Whether server messages should be colored.
    pub chat_colors: bool,
    /// The layers of the player's model.
    pub model: ModelCustomization,
    /// The main hand of the player.
    pub main_hand: PlayerHand,
    /// Whether the client has text filtering enabled.
    pub text_filtering: bool,
    /// Whether the client should show in the server player list.
    pub allows_listing: bool,
    /// The client's particle settings.
    pub particles: ParticleStatus,
}

impl Default for ClientInformation {
    fn default() -> Self {
        Self {
            language: String::from("en_us"),
            view_distance: 8,
            chat_visibility: ChatVisibility::default(),
            chat_colors: true,
            model: ModelCustomization::default(),
            main_hand: PlayerHand::Right,
            text_filtering: false,
            allows_listing: false,
            particles: ParticleStatus::default(),
        }
    }
}

/// The visibility of chat messages.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub enum ChatVisibility {
    #[default]
    Full,
    System,
    Hidden,
}

/// The client's particle level.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub enum ParticleStatus {
    #[default]
    All,
    Decreased,
    Minimal,
}

/// The player's main hand.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
pub enum PlayerHand {
    /// The player is left-handed.
    Left,
    /// The player is right-handed.
    #[default]
    Right,
}

/// A bitfield representing the layers of a player's model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(Facet))]
#[cfg_attr(feature = "facet", facet(mc::serialize = ModelCustomization::SERIALIZE))]
#[cfg_attr(feature = "facet", facet(mc::deserialize = ModelCustomization::DESERIALIZE))]
pub struct ModelCustomization {
    /// The cape layer of the player's model.
    pub cape: bool,
    /// The jacket layer of the player's model.
    pub jacket: bool,
    /// The left sleeve layer of the player's model.
    pub left_sleeve: bool,
    /// The right sleeve layer of the player's model.
    pub right_sleeve: bool,
    /// The left pant leg layer of the player's model.
    pub left_pants: bool,
    /// The right pant leg layer of the player's model.
    pub right_pants: bool,
    /// The hat layer of the player's model.
    pub hat: bool,
}

impl Default for ModelCustomization {
    fn default() -> Self {
        Self {
            cape: true,
            jacket: true,
            left_sleeve: true,
            right_sleeve: true,
            left_pants: true,
            right_pants: true,
            hat: true,
        }
    }
}

#[cfg(feature = "facet")]
impl ModelCustomization {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let byte = cursor.take(1)?[0];
        let model = Self {
            cape: byte & 0b0000_0001 != 0,
            jacket: byte & 0b0000_0010 != 0,
            left_sleeve: byte & 0b0000_0100 != 0,
            right_sleeve: byte & 0b0000_1000 != 0,
            left_pants: byte & 0b0001_0000 != 0,
            right_pants: byte & 0b0010_0000 != 0,
            hat: byte & 0b0100_0000 != 0,
        };
        replace_with_or_abort(partial, |partial| partial.set(model).unwrap());
        Ok(())
    }

    fn facet_serialize<'mem, 'facet>(
        peek: Peek<'mem, 'facet>,
        writer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'mem, 'facet>> {
        let value = peek.get::<Self>()?;
        let mut byte = 0u8;
        if value.cape {
            byte |= 0b0000_0001;
        }
        if value.jacket {
            byte |= 0b0000_0010;
        }
        if value.left_sleeve {
            byte |= 0b0000_0100;
        }
        if value.right_sleeve {
            byte |= 0b0000_1000;
        }
        if value.left_pants {
            byte |= 0b0001_0000;
        }
        if value.right_pants {
            byte |= 0b0010_0000;
        }
        if value.hat {
            byte |= 0b0100_0000;
        }
        if writer.write_data(&[byte]) { Ok(()) } else { Err(SerializeIterError::new()) }
    }
}
