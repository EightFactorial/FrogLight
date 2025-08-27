//! TODO

use alloc::{string::String, sync::Arc};
use core::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Deref,
    str::FromStr,
};

use atomicow::CowArc;

/// A string consisting of a namespace and a path.
#[derive(Clone)]
#[repr(transparent)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, opaque))]
pub struct Identifier(CowArc<'static, str>);

impl Identifier {
    /// The default namespace used when none is provided.
    pub const NAMESPACE: &'static str = "minecraft";

    /// Create a new [`Identifier`] from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the string is not a valid identifier.
    #[must_use]
    pub const fn new_static(ident: &'static str) -> Self {
        assert!(str::is_ascii(ident), "Identifier must be an ASCII string");

        let mut separator = false;
        let mut index = 0;

        while index < ident.len() {
            let c = ident.as_bytes()[index];
            if c == b':' {
                assert!(!separator, "Identifier cannot contain multiple namespaces");
                separator = true;
            } else if
            // Must be 0-9, a-z, '_', '-', or '.'
            !(c.is_ascii_digit()
                || c.is_ascii_lowercase()
                || c == b'_'
                || c == b'-'
                || c == b'.')
                // Allow '/' in the identifier's path
                && !(separator && c == b'/')
            {
                panic!("Identifier contains invalid character");
            }

            index += 1;
        }

        assert!(separator, "Identifier must contain a namespace");

        Self(CowArc::Static(ident))
    }

    /// Try to create a new [`Identifier`] from a string.
    ///
    /// # Note
    ///
    /// If the provided string does not have a namespace,
    /// the [default namespace][Identifier::NAMESPACE] will be used.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid identifier.
    #[inline]
    pub fn try_new<T: AsRef<str> + ?Sized>(ident: &T) -> Result<Self, IdentifierError> {
        Self::try_new_internal(ident.as_ref())
    }

    /// Note: Separated out to prevent monomorphization from duplicating it.
    fn try_new_internal(ident: &str) -> Result<Self, IdentifierError> {
        match Self::validate(ident) {
            Ok(()) => Ok(Self(CowArc::Owned(Arc::from(ident)))),
            Err(IdentifierError::MissingNamespace) => {
                Ok(Self(CowArc::Owned(alloc::format!("{}:{ident}", Self::NAMESPACE).into())))
            }
            Err(err) => Err(err),
        }
    }

    /// Validate if the provided string is a valid identifier.
    ///
    /// # Errors
    ///
    /// Returns an [`IdentifierError`] if the string is not a valid identifier.
    pub fn validate(input: &str) -> Result<(), IdentifierError> {
        if input.is_empty() {
            return Err(IdentifierError::Empty);
        }

        let mut separator = false;
        for (index, char) in input.chars().enumerate() {
            // Mark the separator as seen and allow '/' characters
            if char == ':' {
                if separator {
                    return Err(IdentifierError::MultipleNamespaces);
                }
                separator = true;
                continue;
            }

            // Check if the character is valid (0-9, a-z, '_', '-', '.', or '/' if in path)
            if !(char.is_ascii_digit()
                || char.is_ascii_lowercase()
                || char == '_'
                || char == '-'
                || char == '.'
                || char == '/')
                && !(separator && char == '/')
            {
                return Err(IdentifierError::InvalidCharacter(index));
            }
        }

        // Only return `MissingNamespace` if no other errors were found
        if separator { Ok(()) } else { Err(IdentifierError::MissingNamespace) }
    }

    /// Get a reference to the identifier as a [`str`].
    #[must_use]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            CowArc::Borrowed(s) | CowArc::Static(s) => s,
            CowArc::Owned(s) => s,
        }
    }

    /// Split the [`Identifier`] into its namespace and path components.
    #[must_use]
    pub fn split(&self) -> (&str, &str) {
        self.as_str().split_once(':').unwrap_or_else(|| {
            unreachable!("All Identifiers have a `:` between the namespace and path")
        })
    }

    /// Get the namespace of the [`Identifier`].
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str { self.split().0 }

    /// Get the path of the [`Identifier`].
    #[inline]
    #[must_use]
    pub fn path(&self) -> &str { self.split().1 }

    /// Create a new [`Identifier`] from a string without checking if it is
    /// valid.
    ///
    /// # Safety
    ///
    /// The string must be a valid identifier, following these rules:
    /// - TODO
    ///
    /// [`Identifier::validate`] can be used to check if the string is valid.
    #[must_use]
    pub unsafe fn new_unchecked<T: Into<CowArc<'static, str>>>(ident: T) -> Self {
        Self(ident.into())
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Identifier").field(&self.as_str()).finish()
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool { self.as_str() == other.as_str() }
}
impl Eq for Identifier {}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.as_str().cmp(other.as_str()) }
}

impl Hash for Identifier {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.as_str().hash(state); }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Identifier {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Identifier;

            fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
                formatter.write_str("a valid identifier")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Identifier::try_new(v).map_err(serde::de::Error::custom)
            }
        }

        de.deserialize_str(Visitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Identifier {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_str(self.as_str())
    }
}

// -------------------------------------------------------------------------------------------------

/// An error that can occur when creating an [`Identifier`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifierError {
    /// The identifier is empty.
    Empty,
    /// The identifier is missing a namespace.
    ///
    /// If this is encountered when using [`Identifier::try_new`],
    /// the default namespace [`Identifier::NAMESPACE`] will be used.
    MissingNamespace,
    /// The identifier is missing a path.
    MissingPath,
    /// The identifier contains multiple namespaces.
    MultipleNamespaces,
    /// The identifier contains an invalid character.
    ///
    /// The `usize` is the index of the invalid character.
    InvalidCharacter(usize),
}

impl Debug for IdentifierError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.message())
    }
}
impl Display for IdentifierError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.message())
    }
}

impl IdentifierError {
    /// Return a string describing the error.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            IdentifierError::Empty => "Identifier is empty",
            IdentifierError::MissingNamespace => "Identifier is missing a namespace",
            IdentifierError::MissingPath => "Identifier is missing a path",
            IdentifierError::MultipleNamespaces => "Identifier has multiple namespaces",
            IdentifierError::InvalidCharacter(..) => "Identifier contains invalid an character",
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str { &self.0 }
}
impl Borrow<str> for Identifier {
    fn borrow(&self) -> &str { &self.0 }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool { self.0.as_ref() == other }
}
impl PartialEq<str> for &Identifier {
    fn eq(&self, other: &str) -> bool { self.0.as_ref() == other }
}
impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool { self.0.as_ref() == *other }
}

impl PartialEq<String> for Identifier {
    fn eq(&self, other: &String) -> bool { self.0.as_ref() == other }
}
impl PartialEq<String> for &Identifier {
    fn eq(&self, other: &String) -> bool { self.0.as_ref() == other }
}
impl PartialEq<&String> for Identifier {
    fn eq(&self, other: &&String) -> bool { self.0.as_ref() == *other }
}

impl FromStr for Identifier {
    type Err = IdentifierError;

    fn from_str(ident: &str) -> Result<Self, Self::Err> { Self::try_new_internal(ident) }
}

impl<'a> TryFrom<&'a str> for Identifier {
    type Error = IdentifierError;

    fn try_from(ident: &'a str) -> Result<Self, Self::Error> { Self::try_new_internal(ident) }
}
impl TryFrom<String> for Identifier {
    type Error = IdentifierError;

    fn try_from(ident: String) -> Result<Self, Self::Error> { Self::try_new_internal(&ident) }
}
