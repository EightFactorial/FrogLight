use std::{
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

use compact_str::CompactString;
use derive_more::{Deref, DerefMut};
use mc_rs_macros::Test;
use serde::{Deserialize, Serialize};

use crate::buffer::{Decode, DecodeError, Encode, EncodeError};

/// A wrapper around [`CompactString`] that represents a resource location.
///
/// A resource location is a string that is used to identify a resource in the game.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
    Test,
)]
#[mctest(tests = ["transcode", "decode"], bytes = [19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100])]
#[serde(transparent)]
pub struct ResourceLocation(CompactString);

impl Default for ResourceLocation {
    fn default() -> Self { Self::new("air") }
}

impl ResourceLocation {
    /// The default namespace for [`ResourceLocation`].
    ///
    /// When creating a [`ResourceLocation`] without
    /// a namespace, this will be used.
    pub const DEFAULT_NAMESPACE: CompactString = CompactString::new_inline("minecraft");

    /// Creates a new [`ResourceLocation`].
    ///
    /// # Panics
    /// Panics if the string is empty or contains more than one colon (`:`).
    ///
    /// # Examples
    /// ```
    /// use mc_rs_protocol::types::ResourceLocation;
    ///
    /// let dirt_implicit = ResourceLocation::new("dirt");
    /// assert_eq!(dirt_implicit.as_str(), "minecraft:dirt");
    ///
    /// let dirt_explicit = ResourceLocation::new("minecraft:dirt");
    /// assert_eq!(dirt_explicit.as_str(), "minecraft:dirt");
    ///
    /// let stone_implict = ResourceLocation::new("stone");
    /// assert_eq!(stone_implict.as_str(), "minecraft:stone");
    ///
    /// let stone_explicit = ResourceLocation::new("minecraft:stone");
    /// assert_eq!(stone_explicit.as_str(), "minecraft:stone");
    /// ```
    pub fn new(s: impl Into<CompactString>) -> Self {
        let s: CompactString = s.into();

        assert!(!s.is_empty(), "ResourceLocation must not be empty");

        // If we're in debug mode, we can do some extra checks.
        #[cfg(debug_assertions)]
        {
            if s.starts_with(':') || s.ends_with(':') || !s.is_ascii() {
                panic!("ResourceLocation must not start or end with ':'");
            }
        }

        match s
            .as_str()
            .chars()
            .fold(0u32, |acc, c| acc + u32::from(c == ':'))
        {
            // If there are no colons, we need to add the default namespace.
            0 => Self(Self::DEFAULT_NAMESPACE + ":" + &s),
            // If there is exactly one colon, we can just return the string.
            1 => Self(s),
            // If there are more than one colon, it's invalid.
            _ => panic!("ResourceLocation must contain at most one ':'"),
        }
    }

    /// Attempts to create a new [`ResourceLocation`].
    ///
    /// Returns `None` if the string is empty or contains more than one colon (`:`).
    ///
    /// # Examples
    /// ```
    /// use mc_rs_protocol::types::ResourceLocation;
    ///
    /// let some_andesite = ResourceLocation::try_from("minecraft:andesite").unwrap();
    /// assert_eq!(some_andesite.as_str(), "minecraft:andesite");
    ///
    /// let some_diorite = ResourceLocation::try_from("diorite").unwrap();
    /// assert_eq!(some_diorite.as_str(), "minecraft:diorite");
    ///
    /// let some_granite = ResourceLocation::try_from("granite").unwrap();
    /// assert_eq!(some_granite.as_str(), "minecraft:granite");
    ///
    /// let none_colons = ResourceLocation::try_from("too:many:colons");
    /// assert_eq!(none_colons, None);
    ///
    /// let none_colon_end = ResourceLocation::try_from("mc_rs:");
    /// assert_eq!(none_colon_end, None);
    ///
    /// let none_colon_start = ResourceLocation::try_from(":error");
    /// assert_eq!(none_colon_start, None);
    ///
    /// let none_empty = ResourceLocation::try_from("");
    /// assert_eq!(none_empty, None);
    /// ```
    pub fn try_from(s: impl Into<CompactString>) -> Option<Self> {
        let s: CompactString = s.into();

        // Do some extra checks to make sure the resulting ResourceLocation is valid.
        if s.is_empty() || s.starts_with(':') || s.ends_with(':') || !s.is_ascii() {
            return None;
        }

        match s
            .as_str()
            .chars()
            .fold(0u32, |acc, c| acc + u32::from(c == ':'))
        {
            // If there are no colons, we need to add the default namespace.
            0 => Some(Self(Self::DEFAULT_NAMESPACE + ":" + &s)),
            // If there is exactly one colon, we can just return the string.
            1 => Some(Self(s)),
            // If there are more than one colon, it's invalid.
            _ => None,
        }
    }

    /// Splits the [`ResourceLocation`] into a namespace and path.
    ///
    /// # Examples
    /// ```
    /// use mc_rs_protocol::types::ResourceLocation;
    ///
    /// let dirt = ResourceLocation::new("dirt");
    /// assert_eq!(dirt.split(), ("minecraft", "dirt"));
    ///
    /// let grass_block = ResourceLocation::new("minecraft:grass_block");
    /// assert_eq!(grass_block.split(), ("minecraft", "grass_block"));
    ///
    /// let error = ResourceLocation::new("mc_rs:error");
    /// assert_eq!(error.split(), ("mc_rs", "error"));
    ///
    /// let error_inline = ResourceLocation::new_inline("mc_rs:error");
    /// assert_eq!(error_inline.split(), ("mc_rs", "error"));
    /// ```
    pub fn split(&self) -> (&str, &str) {
        self.split_once(':')
            .expect("ResourceLocation must contain a ':'")
    }

    /// Creates a new inline [`ResourceLocation`] at compile time.
    /// Must contain exactly one colon (`:`).
    ///
    /// Note: Trying to create a long string that can't be inlined, will fail to build.
    ///
    /// See [`CompactString::new_inline`](CompactString) for more information.
    ///
    /// # Examples
    /// ```
    /// use mc_rs_protocol::types::ResourceLocation;
    ///
    /// const WATER: ResourceLocation = ResourceLocation::new_inline("minecraft:water");
    /// assert_eq!(WATER.as_str(), "minecraft:water");
    ///
    /// const LAVA: ResourceLocation = ResourceLocation::new_inline("minecraft:lava");
    /// assert_eq!(LAVA.as_str(), "minecraft:lava");
    ///
    /// const ERROR: ResourceLocation = ResourceLocation::new_inline("mc_rs:error");
    /// assert_eq!(ERROR.as_str(), "mc_rs:error");
    /// ```
    pub const fn new_inline(s: &str) -> Self {
        assert!(!s.is_empty(), "ResourceLocation must not be empty");
        assert!(s.is_ascii(), "ResourceLocation must be ascii");

        let bytes = s.as_bytes();
        let len = bytes.len();

        let mut colon_count = 0;

        let mut index = 0;
        while index < len {
            if bytes[index] == b':' {
                colon_count += 1;
            }

            index += 1;
        }

        match colon_count {
            0 => panic!("ResourceLocation must contain at least one ':'"),
            1 => Self(CompactString::new_inline(s)),
            _ => panic!("ResourceLocation must contain at most one ':'"),
        }
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}

impl From<ResourceLocation> for CompactString {
    fn from(value: ResourceLocation) -> Self { value.0 }
}

impl From<CompactString> for ResourceLocation {
    fn from(value: CompactString) -> Self { Self::new(value) }
}

impl From<ResourceLocation> for String {
    fn from(value: ResourceLocation) -> Self { value.to_string() }
}

impl From<String> for ResourceLocation {
    fn from(value: String) -> Self { Self::new(value) }
}

impl From<&str> for ResourceLocation {
    fn from(value: &str) -> Self { Self::new(value) }
}

impl FromStr for ResourceLocation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::new(s)) }
}

impl Encode for ResourceLocation {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.0.encode(buf)
    }
}

impl Decode for ResourceLocation {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(Self(CompactString::decode(buf)?))
    }
}

#[test]
#[should_panic]
fn test_inline_panic_ascii() { ResourceLocation::new_inline("minecraft:stone🗿"); }

#[test]
#[should_panic]
fn test_inline_panic_length() {
    ResourceLocation::new_inline("minecraft:some_long_string_that_cant_be_inlined");
}

#[test]
#[should_panic]
fn test_inline_panic_colon() { ResourceLocation::new_inline("minecraft:too:many:colons"); }

#[test]
#[should_panic]
fn test_inline_panic_empty() { ResourceLocation::new_inline(""); }
