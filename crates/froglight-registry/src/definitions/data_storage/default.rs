use std::{marker::PhantomData, sync::Arc};

use bevy_derive::Deref;
use bevy_ecs::system::Resource;
use froglight_protocol::traits::Version;
use parking_lot::RwLock;

use super::{traits::InitializeRegistry, SimpleRegistry};
use crate::definitions::ConvertKey;

/// A registry that stores the default registry values for a specific
/// [`Version`].
///
/// Can only be modified by bevy [`Plugins`](bevy_app::Plugin).
#[derive(Debug, Clone, Deref, Resource)]
pub struct DefaultRegistry<V: Version, R: ConvertKey> {
    #[deref]
    ids: Vec<R>,
    _v: PhantomData<V>,
}

impl<V: Version, R> Default for DefaultRegistry<V, R>
where
    R: ConvertKey + InitializeRegistry<V>,
{
    fn default() -> Self { Self { ids: R::initialize(), _v: PhantomData } }
}

impl<V: Version, R: ConvertKey> DefaultRegistry<V, R> {
    /// Creates a new [`DefaultRegistry`] with the default values.
    #[must_use]
    pub fn new() -> Self
    where
        R: InitializeRegistry<V>,
    {
        Self::default()
    }

    /// Gets the value with the specified ID.
    #[must_use]
    pub fn get_value(&self, id: u32) -> Option<&R> { self.ids.get(id as usize) }

    /// Gets the ID of the specified value.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_id(&self, value: &R) -> Option<u32>
    where
        R: PartialEq,
    {
        self.ids.iter().position(|v| v == value).map(|i| i as u32)
    }

    /// Creates a new [`SimpleRegistry`] from this [`DefaultRegistry`].
    #[must_use]
    pub fn create_simple(&self) -> SimpleRegistry<R>
    where
        R: Clone,
    {
        SimpleRegistry(Arc::new(RwLock::new(self.ids.clone())))
    }
}
