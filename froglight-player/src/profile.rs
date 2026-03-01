//! A player's [`PlayerProfile`]
#![allow(clippy::unsafe_derive_deserialize, reason = "Triggered by deriving `facet` and `serde`")]

use alloc::string::String;
use core::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "facet")]
use base64::prelude::*;
#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize, std_traits::ReflectDefault};
#[cfg(feature = "facet")]
use facet::Span;
#[cfg(feature = "facet")]
use facet_format::{DeserializeErrorKind, ParseError, SerializeError};
#[cfg(feature = "facet")]
use facet_json::{DeserializeError, JsonSerializeError};
use foldhash::fast::RandomState;
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::username::Username;

/// A player's profile
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", component(on_add = Self::add_hook))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct PlayerProfile {
    /// The player's [`Uuid`].
    ///
    /// This is not the same as the player's
    /// [`EntityUuid`](froglight_common::entity::EntityUuid).
    uuid: Uuid,
    /// The player's username.
    ///
    /// The real name of the player, should not be changed.
    username: Username,
    /// The player's profile properties.
    properties: ProfilePropertySet,
}

impl PlayerProfile {
    /// Creates a new [`PlayerProfile`] with the given [`Uuid`] and
    /// [`Username`].
    #[must_use]
    pub fn new(uuid: Uuid, username: Username) -> Self {
        Self { uuid, username, properties: ProfilePropertySet::new() }
    }

    /// Creates a new [`PlayerProfile`] for an offline player with the given
    /// [`Username`].
    #[must_use]
    pub fn new_offline(username: Username) -> Self {
        let uuid = username.uuid_offline();
        Self::new(uuid, username)
    }

    /// Get a reference to the player's [`Username`].
    #[inline]
    #[must_use]
    pub const fn username(&self) -> &Username { &self.username }

    /// Get a reference to the player's [`Uuid`].
    #[inline]
    #[must_use]
    pub const fn uuid(&self) -> &Uuid { &self.uuid }

    /// Get a reference to the [`ProfilePropertySet`].
    #[inline]
    #[must_use]
    pub const fn properties(&self) -> &ProfilePropertySet { &self.properties }

    /// Get a mutable reference to the [`ProfilePropertySet`].
    #[inline]
    #[must_use]
    pub const fn properties_mut(&mut self) -> &mut ProfilePropertySet { &mut self.properties }

    /// An `on_add` hook that inserts a [`Username`] component
    /// if the entity does not already have one.
    #[cfg(feature = "bevy")]
    fn add_hook(mut world: bevy_ecs::world::DeferredWorld, ctx: bevy_ecs::lifecycle::HookContext) {
        if let Ok(entity) = world.get_entity(ctx.entity)
            && !entity.contains::<Username>()
            && let Some(profile) = entity.get::<Self>()
        {
            let username = profile.username().clone();
            world.commands().entity(ctx.entity).insert(username);
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A set of [`ProfileProperty`]s associated with a [`PlayerProfile`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(transparent))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ProfilePropertySet(IndexMap<String, ProfileProperty, RandomState>);

/// A property associated with a [`PlayerProfile`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "bevy", reflect(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ProfileProperty {
    /// The value of the property.
    pub value: String,
    /// An optional signature.
    pub signature: Option<String>,
}

impl ProfilePropertySet {
    /// Creates a new, empty [`ProfilePropertySet`].
    ///
    /// Does not allocate.
    #[must_use]
    pub fn new() -> Self { Self::new_from(IndexMap::with_hasher(RandomState::default())) }

    /// Creates a new [`ProfilePropertySet`] from the given [`IndexMap`].
    #[inline]
    #[must_use]
    pub const fn new_from(map: IndexMap<String, ProfileProperty, RandomState>) -> Self { Self(map) }

    /// Creates a new [`ProfilePropertySet`] with the given capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new_from(IndexMap::with_capacity_and_hasher(capacity, RandomState::default()))
    }

    /// Get a property from the set by its key,
    /// deserializing it into the given type.
    ///
    /// # Errors
    ///
    /// Returns an error if the property is not valid base64,
    /// or it cannot be deserialized into the given type.
    #[cfg(feature = "facet")]
    pub fn get_property<T: ProfilePropertyItem>(&self) -> Result<Option<T>, DeserializeError> {
        self.0
            .get(T::PROPERTY_KEY)
            .map_or(Ok(None), |property| T::from_property(property).map(Some))
    }

    /// Insert a property into the set, signing it with the given function.
    ///
    /// If a property with the same key already exists,
    /// it will be replaced and returned.
    ///
    /// If you don't want to sign the property, use
    /// [`ProfilePropertySet::insert_property_unsigned`] instead.
    ///
    /// # Errors
    ///
    /// Returns an error if the property cannot be serialized.
    #[cfg(feature = "facet")]
    pub fn insert_property<T: ProfilePropertyItem, F: FnOnce(&str) -> String>(
        &mut self,
        item: &T,
        signer: F,
    ) -> Result<Option<ProfileProperty>, SerializeError<JsonSerializeError>> {
        let property = item.to_property(signer)?;
        Ok(self.0.insert(String::from(T::PROPERTY_KEY), property))
    }

    /// Insert a property into the set without signing it.
    ///
    /// If a property with the same key already exists,
    /// it will be replaced and returned.
    ///
    /// # Errors
    ///
    /// Returns an error if the property cannot be serialized.
    #[cfg(feature = "facet")]
    pub fn insert_property_unsigned<T: ProfilePropertyItem>(
        &mut self,
        item: &T,
    ) -> Result<Option<ProfileProperty>, SerializeError<JsonSerializeError>> {
        let property = item.to_property_unsigned()?;
        Ok(self.0.insert(String::from(T::PROPERTY_KEY), property))
    }

    /// Get a reference to the underlying [`IndexMap`] of properties.
    #[inline]
    #[must_use]
    pub const fn as_map(&self) -> &IndexMap<String, ProfileProperty, RandomState> { &self.0 }

    /// Get a mutable reference to the underlying [`IndexMap`] of properties.
    #[inline]
    #[must_use]
    pub const fn as_map_mut(&mut self) -> &mut IndexMap<String, ProfileProperty, RandomState> {
        &mut self.0
    }
}

impl Deref for ProfilePropertySet {
    type Target = IndexMap<String, ProfileProperty, RandomState>;

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_map() }
}
impl DerefMut for ProfilePropertySet {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_map_mut() }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can be used in a [`ProfilePropertySet`].
#[cfg(feature = "facet")]
pub trait ProfilePropertyItem: facet::Facet<'static> + Sized {
    /// The key of the property.
    const PROPERTY_KEY: &'static str;

    /// Creates an item from a [`ProfileProperty`].
    ///
    /// # Errors
    ///
    /// Returns an error if the property is not valid base64,
    /// or it cannot be deserialized into the item.
    fn from_property(property: &ProfileProperty) -> Result<Self, DeserializeError> {
        use alloc::string::ToString;

        let decoded = BASE64_STANDARD.decode(&property.value).map_err(|err| {
            DeserializeError::from(ParseError::new(
                Span::new(0, 0),
                DeserializeErrorKind::Unsupported { message: err.to_string().into() },
            ))
        })?;

        #[cfg(feature = "tracing")]
        if let Ok(str) = core::str::from_utf8(decoded.as_slice()) {
            tracing::debug!("Decoded \"{}\": {str}", Self::PROPERTY_KEY);
        } else {
            tracing::debug!("Decoded \"{}\": <binary>", Self::PROPERTY_KEY);
        }

        facet_json::from_slice::<Self>(&decoded)
    }

    /// Create a [`ProfileProperty`] from the item without signing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the item cannot be serialized.
    fn to_property_unsigned(&self) -> Result<ProfileProperty, SerializeError<JsonSerializeError>> {
        facet_json::to_string_pretty(self)
            .map(|value| ProfileProperty { value: BASE64_STANDARD.encode(value), signature: None })
    }

    /// Create a [`ProfileProperty`] from the item,
    /// signing it with the given function.
    ///
    /// # Errors
    ///
    /// Returns an error if the item cannot be serialized.
    fn to_property<F: FnOnce(&str) -> String>(
        &self,
        signer: F,
    ) -> Result<ProfileProperty, SerializeError<JsonSerializeError>> {
        facet_json::to_string_pretty(self).map(|mut value| {
            value = BASE64_STANDARD.encode(value);
            ProfileProperty { signature: Some(signer(&value)), value }
        })
    }
}
