//! A string used to identify a resource.
#![allow(unexpected_cfgs)]

use std::{borrow::Borrow, fmt::Display};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use compact_str::CompactString;
use derive_more::{Deref, DerefMut};
use thiserror::Error;

/// A string used to identify a resource.
///
/// All keys are made of a namespace and a path, separated by a colon.
///
/// Internally just a wrapper around a [`CompactString`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ResourceKey(#[cfg_attr(feature = "bevy", reflect(ignore))] CompactString);

/// An error that occurred while creating a [`ResourceKey`]
#[derive(Debug, Error)]
pub enum ResourceKeyError {
    /// [`ResourceKey`] is empty
    #[error("ResourceKey is empty")]
    Empty,
    /// [`ResourceKey`] starts with a colon
    #[error("ResourceKey `{0}` starts with a colon")]
    LeadingColon(CompactString),
    /// [`ResourceKey`] ends with a colon
    #[error("ResourceKey `{0}` ends with a colon")]
    TrailingColon(CompactString),
    /// [`ResourceKey`] contains multiple colons
    #[error("ResourceKey `{0}` contains {1} colons")]
    MultipleColons(CompactString, usize),
}

impl ResourceKey {
    /// The default namespace for [`ResourceKey`].
    ///
    /// When creating a [`ResourceKey`] without
    /// a namespace, this will be used.
    pub const DEFAULT_NAMESPACE: CompactString = CompactString::new_inline("minecraft");

    /// Creates a new [`ResourceKey`]
    ///
    /// If there is no colon in the key, the
    /// [`DEFAULT_NAMESPACE`](Self::DEFAULT_NAMESPACE) will be used.
    ///
    /// If you want to properly handle errors, use [`ResourceKey::try_new`]
    /// instead.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_components::resourcekey::ResourceKey;
    ///
    /// let key = ResourceKey::new("minecraft:stone");
    /// assert_eq!(key, "minecraft:stone");
    ///
    /// let key = ResourceKey::new("stone");
    /// assert_eq!(key, "minecraft:stone");
    /// ```
    ///
    /// # Panics
    /// - If the key is empty
    /// - If the key contains a leading or trailing colon
    /// - If the key contains more than one colon
    pub fn new(key: impl Into<CompactString>) -> Self { Self::try_new(key).unwrap() }

    /// Attempt to create a new [`ResourceKey`]
    ///
    /// # Errors
    /// - If the key is empty
    /// - If the key contains a leading or trailing colon
    /// - If the key contains more than one colon
    pub fn try_new(key: impl Into<CompactString>) -> Result<Self, ResourceKeyError> {
        let key = key.into();

        // Keys must not be empty
        if key.is_empty() {
            return Err(ResourceKeyError::Empty);
        }

        // No leading or trailing colons
        if key.starts_with(':') {
            return Err(ResourceKeyError::LeadingColon(key));
        } else if key.ends_with(':') {
            return Err(ResourceKeyError::TrailingColon(key));
        }

        match key.matches(':').count() {
            // If there is no colon, use the default namespace
            0 => Ok(Self(Self::DEFAULT_NAMESPACE.clone() + ":" + &key)),
            // If there is exactly one colon, return the key
            1 => Ok(Self(key)),
            // If there is more than one colon, return an error
            n => Err(ResourceKeyError::MultipleColons(key, n)),
        }
    }

    /// Splits the key into the namespace and path
    ///
    /// # Examples
    /// ```rust
    /// use froglight_components::resourcekey::ResourceKey;
    ///
    /// let key = ResourceKey::new("minecraft:stone");
    /// assert_eq!(key.split(), ("minecraft", "stone"));
    ///
    /// let key = ResourceKey::new("air");
    /// assert_eq!(key.split(), ("minecraft", "air"));
    /// ```
    #[inline]
    #[must_use]
    pub fn split(&self) -> (&str, &str) {
        self.split_once(':')
            .unwrap_or_else(|| unreachable!("ResourceKeys always have a namespace and path"))
    }

    /// Returns the namespace of the key
    ///
    /// # Examples
    /// ```rust
    /// use froglight_components::resourcekey::ResourceKey;
    ///
    /// let key = ResourceKey::new("minecraft:dirt");
    /// assert_eq!(key.namespace(), "minecraft");
    ///
    /// let key = ResourceKey::new("froglight:error");
    /// assert_eq!(key.namespace(), "froglight");
    /// ```
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str { self.split().0 }

    /// Returns the path of the key
    ///
    /// # Examples
    /// ```rust
    /// use froglight_components::resourcekey::ResourceKey;
    ///
    /// let key = ResourceKey::new("minecraft:grass");
    /// assert_eq!(key.path(), "grass");
    ///
    /// let key = ResourceKey::new("froglight:error");
    /// assert_eq!(key.path(), "error");
    /// ```
    #[inline]
    #[must_use]
    pub fn path(&self) -> &str { self.split().1 }

    /// Creates a new inline [`ResourceKey`] at compile time.
    ///
    /// Must contain exactly one colon (`:`).
    ///
    /// Note: Trying to create a long string that can't be inlined, will fail to
    /// build.
    ///
    /// See [`CompactString::new_inline`](CompactString) for more information.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_components::resourcekey::ResourceKey;
    ///
    /// const AIR: ResourceKey = ResourceKey::new_inline("minecraft:air");
    /// assert_eq!(AIR, "minecraft:air");
    ///
    /// const ERROR: ResourceKey = ResourceKey::new_inline("froglight:error");
    /// assert_eq!(ERROR, "froglight:error");
    /// ```
    ///
    /// # Panics
    /// - If the key is empty
    /// - If the key contains does not contain exactly one colon
    /// - If the key is too long to be inlined
    #[must_use]
    pub const fn new_inline(key: &str) -> Self {
        assert!(!key.is_empty(), "ResourceKey must not be empty");

        // Count the number of colons
        let mut colon_count = 0;
        {
            let bytes = key.as_bytes();
            let len = bytes.len();

            let mut index = 0;
            while index < len {
                if bytes[index] == b':' {
                    colon_count += 1;
                }

                index += 1;
            }
        }

        match colon_count {
            0 => panic!("ResourceKey must contain at least one `:`"),
            1 => Self(CompactString::new_inline(key)),
            _ => panic!("ResourceKey must contain at most one `:`"),
        }
    }

    /// Checks if the key is a valid [`ResourceKey`]
    ///
    /// A valid [`ResourceKey`] must:
    /// - Not be empty
    /// - Not start or end with a colon
    /// - Contain exactly one colon
    ///
    /// # Examples
    /// ```rust
    /// use froglight_components::resourcekey::ResourceKey;
    ///
    /// assert!(!ResourceKey::is_valid("minecraft:"));
    /// assert!(!ResourceKey::is_valid(":stone"));
    /// assert!(!ResourceKey::is_valid(""));
    ///
    /// assert!(ResourceKey::is_valid("minecraft:stone"));
    /// assert!(ResourceKey::is_valid("froglight:dirt"));
    /// assert!(ResourceKey::is_valid("any:thing"));
    ///
    /// assert!(!ResourceKey::is_valid("minecraft:stone:"));
    /// assert!(!ResourceKey::is_valid(":minecraft:stone"));
    /// assert!(!ResourceKey::is_valid("minecraft::stone"));
    /// assert!(!ResourceKey::is_valid("stone"));
    /// ```
    #[must_use]
    pub fn is_valid(key: &str) -> bool {
        if key.is_empty() {
            return false;
        }

        if key.starts_with(':') || key.ends_with(':') {
            return false;
        }

        key.matches(':').count() == 1
    }
}

impl AsRef<str> for ResourceKey {
    fn as_ref(&self) -> &str { self.as_str() }
}

impl AsRef<CompactString> for ResourceKey {
    fn as_ref(&self) -> &CompactString { &self.0 }
}

impl Borrow<str> for ResourceKey {
    fn borrow(&self) -> &str { self.as_str() }
}

impl Borrow<CompactString> for ResourceKey {
    fn borrow(&self) -> &CompactString { &self.0 }
}

impl Display for ResourceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl From<ResourceKey> for String {
    fn from(key: ResourceKey) -> Self { key.0.into_string() }
}

impl From<ResourceKey> for CompactString {
    fn from(key: ResourceKey) -> Self { key.0 }
}

impl TryFrom<&str> for ResourceKey {
    type Error = ResourceKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> { Self::try_new(value) }
}

impl TryFrom<String> for ResourceKey {
    type Error = ResourceKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> { Self::try_new(value) }
}

impl TryFrom<CompactString> for ResourceKey {
    type Error = ResourceKeyError;

    fn try_from(value: CompactString) -> Result<Self, Self::Error> { Self::try_new(value) }
}

impl PartialEq<str> for ResourceKey {
    fn eq(&self, other: &str) -> bool { self.as_str() == other }
}
impl PartialEq<&str> for ResourceKey {
    fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
}

impl PartialEq<String> for ResourceKey {
    fn eq(&self, other: &String) -> bool { self.as_str() == other.as_str() }
}

impl PartialEq<CompactString> for ResourceKey {
    fn eq(&self, other: &CompactString) -> bool { self.as_str() == other.as_str() }
}

#[cfg(feature = "hashbrown")]
impl hashbrown::Equivalent<str> for ResourceKey {
    fn equivalent(&self, key: &str) -> bool { self.as_str() == key }
}

#[cfg(feature = "hashbrown")]
impl hashbrown::Equivalent<String> for ResourceKey {
    fn equivalent(&self, key: &String) -> bool { self.as_str() == key.as_str() }
}

#[cfg(feature = "hashbrown")]
impl hashbrown::Equivalent<CompactString> for ResourceKey {
    fn equivalent(&self, key: &CompactString) -> bool { self.as_str() == key.as_str() }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ResourceKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ResourceKey::try_new(CompactString::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "inspector")]
impl bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive for ResourceKey {
    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        _: &dyn std::any::Any,
        _: bevy_inspector_egui::egui::Id,
        _: bevy_inspector_egui::reflect_inspector::InspectorUi<'_, '_>,
    ) -> bool {
        ui.add_sized(
            ui.available_size(),
            bevy_inspector_egui::egui::TextEdit::singleline(&mut self.to_string()),
        )
        .changed()
    }

    fn ui_readonly(
        &self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        _: &dyn std::any::Any,
        _: bevy_inspector_egui::egui::Id,
        _: bevy_inspector_egui::reflect_inspector::InspectorUi<'_, '_>,
    ) {
        ui.add_sized(ui.available_size(), bevy_inspector_egui::egui::Label::new(self.as_str()));
    }
}
