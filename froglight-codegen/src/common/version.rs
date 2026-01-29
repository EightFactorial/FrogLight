use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use facet::Facet;

/// A version identifier.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Facet)]
#[facet(transparent)]
pub struct Version(String);

impl Version {
    /// Create a new [`Version`].
    pub fn new<S: ToString + ?Sized>(version: &S) -> Self { Version(ToString::to_string(version)) }

    /// Get the version as a string slice.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Get the version as a feature string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use froglight_codegen::data::version::Version;
    ///
    /// assert_eq!(Version::new("26.1").as_feature(), "v26_1");
    /// assert_eq!(Version::new("1.21.10").as_feature(), "v1_21_10");
    /// ```
    #[must_use]
    pub fn as_feature(&self) -> String { format!("v{}", self.0.replace(['-', '.'], "_")) }
}

// -------------------------------------------------------------------------------------------------

/// A container for version-specific data.
#[repr(transparent)]
#[derive(Default)]
pub struct VersionStorage(HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>);

impl VersionStorage {
    /// Create a new empty [`VersionData`].
    #[must_use]
    pub fn new() -> Self { VersionStorage(HashMap::new()) }

    /// Check if a value of type `T` exists.
    #[inline]
    #[must_use]
    pub fn contains<T: Any + Send + Sync + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    /// Get a reference to a value of type `T`.
    #[inline]
    #[must_use]
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.0.get(&TypeId::of::<T>()).and_then(|b| b.downcast_ref::<T>())
    }

    /// Get a mutable reference to a value of type `T`.
    #[inline]
    #[must_use]
    pub fn get_mut<T: Any + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<T>()).and_then(|b| b.downcast_mut::<T>())
    }

    /// Insert a value of type `T`, returning the previous value if one existed.
    #[inline]
    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, value: T) -> Option<T> {
        self.0
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|b| b.downcast::<T>().ok().map(|b| *b))
    }

    /// Remove a value of type `T`, returning it if one existed.
    #[inline]
    pub fn remove<T: Any + Send + Sync + 'static>(&mut self) -> Option<T> {
        self.0.remove(&TypeId::of::<T>()).and_then(|b| b.downcast::<T>().ok().map(|b| *b))
    }
}
