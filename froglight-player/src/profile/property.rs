use alloc::string::String;
use core::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "facet")]
use base64::prelude::*;
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

    /// Returns `true` if the set contains no properties.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

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
