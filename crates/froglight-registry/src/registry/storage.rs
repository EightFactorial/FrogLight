use core::{hash::Hash, marker::PhantomData};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
use bevy_platform::{hash::FixedHasher, sync::Arc};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::{prelude::Identifier, vanilla::Vanilla, version::Version};
use froglight_utils::storage::prelude::*;
use indexmap::{Equivalent, IndexMap, IndexSet};

use super::RegistryTrait;

/// A dynamic storage for registries.
///
/// Allows for the registration and retrieval of registries at runtime.
#[repr(transparent)]
#[derive(Debug, Clone, AppStorage)]
#[storage(index(ident = "GlobalRegistryId", inner = "u16"), bevy = "bevy", reflect = "reflect")]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
pub struct RegistryStorage<V: Version>(IndexMap<Identifier, RegistryValueStorage<V>, FixedHasher>);

impl<V: Version> AppRegistryStorage<V> {
    /// Create a new [`AppRegistryStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: RegistryTrait<V> {
        Self::from_storage(RegistryStorage::new())
    }
}

impl<V: Version> RegistryStorage<V> {
    /// Create a new [`RegistryStorage`] with the [`Vanilla`] registry values.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: RegistryTrait<V> {
        let mut storage = Self::new_empty();
        <Vanilla as RegistryTrait<V>>::register(&mut storage);
        storage
    }

    /// Create a new empty [`RegistryStorage`] with no registry values.
    #[must_use]
    pub const fn new_empty() -> Self { Self(IndexMap::with_hasher(FixedHasher)) }

    /// Get a reference to the [`RegistryValueStorage`] for the given registry
    /// name.
    ///
    /// Returns `None` if the registry was not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[must_use]
    pub fn get_registry<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        registry: &Q,
    ) -> Option<&RegistryValueStorage<V>> {
        self.0.get(registry)
    }

    /// Get a mutable reference to the [`RegistryValueStorage`] for the given
    /// registry name.
    ///
    /// Returns `None` if the registry was not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[must_use]
    pub fn get_registry_mut<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &mut self,
        registry: &Q,
    ) -> Option<&mut RegistryValueStorage<V>> {
        self.0.get_mut(registry)
    }

    /// Get a reference to the [`RegistryValueStorage`] for the given registry
    /// index.
    ///
    /// Returns `None` if no registry was registered at that index.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[must_use]
    pub fn get_registry_index(&self, index: GlobalRegistryId) -> Option<&RegistryValueStorage<V>> {
        self.0.get_index(index.into()).map(|(_, val)| val)
    }

    /// Get a mutable reference to the [`RegistryValueStorage`] for the given
    /// registry index.
    ///
    /// Returns `None` if no registry was registered at that index.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[must_use]
    pub fn get_registry_index_mut(
        &mut self,
        index: GlobalRegistryId,
    ) -> Option<&mut RegistryValueStorage<V>> {
        self.0.get_index_mut(index.into()).map(|(_, val)| val)
    }

    /// Get the [`GlobalRegistryId`] for the given registry.
    ///
    /// Returns `None` if the registry was not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[must_use]
    pub fn get_registry_global_id<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        registry: &Q,
    ) -> Option<GlobalRegistryId> {
        self.0.get_index_of(registry).map(GlobalRegistryId::new_unchecked)
    }

    /// Get the [`GlobalRegistryId`] for the given registry value.
    ///
    /// Returns `None` if the registry was not registered,
    /// or the registry does not contain the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[must_use]
    pub fn get_value_global_id<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        registry: &Q,
        value: &Q,
    ) -> Option<GlobalRegistryId> {
        self.get_registry(registry).and_then(|val| val.get_global_id_or_default(value))
    }

    /// Returns `true` if the storage contains
    /// the given registry name.
    #[must_use]
    pub fn contains_registry<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        registry: &Q,
    ) -> bool {
        self.0.contains_key(registry)
    }

    /// Returns `true` if the storage contains
    /// the given registry and registry value.
    #[must_use]
    pub fn contains_value<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        registry: &Q,
        value: &Q,
    ) -> bool {
        self.get_registry(registry).is_some_and(|val| val.contains(value))
    }

    /// Register a new registry with the given name and values.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    pub fn register(
        &mut self,
        registry: impl Into<Identifier>,
        values: impl Into<RegistryValueStorage<V>>,
    ) {
        self.0.insert(registry.into(), values.into());
    }

    /// Register a new registry with the given name, default value, and values.
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    pub fn register_with_default(
        &mut self,
        registry: impl Into<Identifier>,
        default: Option<Identifier>,
        values: impl Into<RegistryValueStorage<V>>,
    ) {
        let mut storage = values.into();
        *storage.default_value_mut() = default;
        self.0.insert(registry.into(), storage);
    }
}

// -------------------------------------------------------------------------------------------------

/// A dynamic storage for registry values.
///
/// Allows for the registration and retrieval of registry values at runtime.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(opaque, Debug, Clone))]
pub struct RegistryValueStorage<V: Version> {
    default: Option<Identifier>,
    values: IndexSet<Identifier, FixedHasher>,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    _phantom: PhantomData<V>,
}

impl<V: Version> RegistryValueStorage<V> {
    /// Create a new [`RegistryValueStorage`] from the given values.
    #[must_use]
    pub fn new<I: IntoIterator<Item = T>, T: Into<Identifier>>(
        default: Option<Identifier>,
        values: I,
    ) -> Self {
        Self {
            default,
            values: values.into_iter().map(Into::into).collect(),
            _phantom: PhantomData,
        }
    }

    /// Create a new [`RegistryValueStorage`] with no registered values.
    #[must_use]
    pub const fn new_empty() -> Self {
        Self { default: None, values: IndexSet::with_hasher(FixedHasher), _phantom: PhantomData }
    }

    /// Get the registry value for the given [`GlobalRegistryId`].
    #[must_use]
    pub fn get_value(&self, index: GlobalRegistryId) -> Option<&Identifier> {
        self.values.get_index(index.into())
    }

    /// Get the registry value for the given [`GlobalRegistryId`],
    /// or the default value if it exists.
    #[must_use]
    pub fn get_value_or_default(&self, index: GlobalRegistryId) -> Option<&Identifier> {
        self.get_value(index).or(self.default.as_ref())
    }

    /// Get the [`GlobalRegistryId`] for the given registry value.
    #[must_use]
    pub fn get_global_id<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        value: &Q,
    ) -> Option<GlobalRegistryId> {
        self.values.get_index_of(value).map(GlobalRegistryId::new_unchecked)
    }

    /// Get the [`GlobalRegistryId`] for the given registry value,
    /// or the default value if it exists.
    #[must_use]
    #[allow(clippy::collapsible_if)]
    pub fn get_global_id_or_default<Q: ?Sized + Hash + Equivalent<Identifier>>(
        &self,
        value: &Q,
    ) -> Option<GlobalRegistryId> {
        self.get_global_id(value).or_else(|| {
            if let Some(default) = self.default.as_ref() {
                if let Some(index) = self.values.get_index_of(default) {
                    return Some(GlobalRegistryId::from(index));
                }

                #[cfg(feature = "trace")]
                tracing::warn!(
                    "Registry value \"{default}\" has no index despite being the default!"
                );
            }

            None
        })
    }

    /// Returns `true` if the registry contains the given value.
    #[must_use]
    pub fn contains<Q: ?Sized + Hash + Equivalent<Identifier>>(&self, value: &Q) -> bool {
        self.values.contains(value)
    }

    /// Get a reference to the default value of the registry.
    #[inline]
    #[must_use]
    pub const fn default_value(&self) -> Option<&Identifier> { self.default.as_ref() }

    /// Get a mutable reference to the default value of the registry.
    #[inline]
    #[must_use]
    pub fn default_value_mut(&mut self) -> &mut Option<Identifier> { &mut self.default }

    /// Get a reference to the internal [`IndexSet`] of registry values.
    #[inline]
    #[must_use]
    pub const fn values(&self) -> &IndexSet<Identifier, FixedHasher> { &self.values }

    /// Get a mutable reference to the internal [`IndexSet`] of registry values.
    #[inline]
    #[must_use]
    pub const fn values_mut(&mut self) -> &mut IndexSet<Identifier, FixedHasher> {
        &mut self.values
    }

    /// Insert a new registry value into the registry.
    ///
    /// Returns `true` if the value was inserted,
    /// or `false` if it was already present.
    pub fn register(&mut self, value: impl Into<Identifier>) -> bool {
        self.values.insert(value.into())
    }
}

impl<I: IntoIterator<Item = T>, T: Into<Identifier>, V: Version> From<I>
    for RegistryValueStorage<V>
{
    fn from(values: I) -> Self { Self::new(None, values) }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> Default for AppRegistryStorage<V>
where Vanilla: RegistryTrait<V>
{
    fn default() -> Self { Self::new() }
}

impl<V: Version> Default for RegistryStorage<V>
where Vanilla: RegistryTrait<V>
{
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

impl From<usize> for GlobalRegistryId {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self { Self(u16::try_from(id).expect("GlobalRegistryId is too large!")) }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u16) }
}

impl From<GlobalRegistryId> for usize {
    fn from(id: GlobalRegistryId) -> Self { usize::from(id.0) }
}
