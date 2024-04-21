use std::marker::PhantomData;

use bevy_ecs::schedule::SystemSet;
use froglight_protocol::traits::Version;

/// A [`SystemSet`](SystemSet) used to group all
/// [`PreUpdate`](bevy_app::PreUpdate) systems related to a specific
/// [`Version`].
#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct ConnectionPreUpdateSet<V: Version>(PhantomData<V>);

impl<V: Version> std::fmt::Debug for ConnectionPreUpdateSet<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionPreUpdateSet<{:?}>", V::default())
    }
}

/// A [`SystemSet`](SystemSet) used to group all
/// [`PostUpdate`](bevy_app::PostUpdate) systems related to a specific
/// [`Version`].
#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct ConnectionPostUpdateSet<V: Version>(PhantomData<V>);

impl<V: Version> std::fmt::Debug for ConnectionPostUpdateSet<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionPostUpdateSet<{:?}>", V::default())
    }
}
