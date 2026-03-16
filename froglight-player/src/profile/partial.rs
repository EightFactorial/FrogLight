#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize, std_traits::ReflectDefault};
#[cfg(feature = "facet")]
use facet_minecraft as mc;
use froglight_common::prelude::Identifier;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    profile::{PlayerProfile, ProfilePropertySet},
    username::Username,
};

/// A player's profile data.
///
/// May be either a [`PartialPlayerProfile`] or a [`PlayerProfile`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct MaybePartialProfile {
    /// The player's profile data.
    #[cfg_attr(feature = "facet", facet(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub profile: ProfileType,
    /// The player's skin data.
    #[cfg_attr(feature = "facet", facet(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub skin: PartialPlayerSkin<'static>,
}

/// Either a [`PartialPlayerProfile`] or a [`PlayerProfile`].
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(untagged))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(untagged))]
pub enum ProfileType {
    /// A [`PartialPlayerProfile`].
    Partial(PartialPlayerProfile),
    /// A [`PlayerProfile`].
    Full(PlayerProfile),
}

impl Default for ProfileType {
    fn default() -> Self { Self::Partial(PartialPlayerProfile::default()) }
}

// -------------------------------------------------------------------------------------------------

/// A partial representation of a player's profile.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PartialPlayerProfile {
    /// The player's username.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub username: Option<Username>,
    /// The player's [`Uuid`].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub uuid: Option<Uuid>,
    /// The player's profile properties.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ProfilePropertySet::is_empty"))]
    pub properties: ProfilePropertySet,
}

/// A partial representation of a player's skin.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PartialPlayerSkin<'a> {
    /// The player's skin texture.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub texture: Option<Identifier<'a>>,
    /// The player's cape texture.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cape: Option<Identifier<'a>>,
    /// The player's elytra texture.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub elytra: Option<Identifier<'a>>,
    /// The player's model type.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub model: Option<u32>,
}
