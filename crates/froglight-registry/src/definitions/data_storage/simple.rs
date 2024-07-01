use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::system::Resource;
use froglight_protocol::traits::Version;
use parking_lot::RwLock;

use super::DefaultRegistry;
use crate::definitions::ConvertKey;

/// A registry that stores the currently active registry values.
///
/// Can be modified by bevy [`Plugins`](bevy_app::Plugin) and connected servers.
#[derive(Debug, Clone, Deref, Resource)]
pub struct SimpleRegistry<R: ConvertKey>(pub(crate) Arc<RwLock<Vec<R>>>);

impl<R> SimpleRegistry<R>
where
    R: ConvertKey,
{
    /// Creates a new empty [`SimpleRegistry`].
    #[must_use]
    pub fn new_empty() -> Self { Self(Arc::new(RwLock::new(Vec::new()))) }

    /// Creates a new [`SimpleRegistry`] from a [`DefaultRegistry`].
    #[must_use]
    pub fn from_default<V: Version>(default: &DefaultRegistry<V, R>) -> Self
    where
        R: Clone,
    {
        default.create_simple()
    }

    /// Gets the value with the specified ID.
    #[must_use]
    pub fn get_value(&self, id: u32) -> Option<R>
    where
        R: Copy,
    {
        self.0.read().get(id as usize).copied()
    }

    /// Gets the value with the specified ID.
    #[must_use]
    pub fn get_value_cloned(&self, id: u32) -> Option<R>
    where
        R: Clone,
    {
        self.0.read().get(id as usize).cloned()
    }

    /// Gets the ID of the specified value.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_id(&self, value: &R) -> Option<u32>
    where
        R: PartialEq,
    {
        self.0.read().iter().position(|v| v == value).map(|i| i as u32)
    }
}
