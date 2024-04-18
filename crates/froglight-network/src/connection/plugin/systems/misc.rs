use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{component::Component, schedule::SystemSet};
use froglight_protocol::traits::Version;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(super) struct ConnectionSet<V: Version>(PhantomData<V>);

impl<V: Version> Debug for ConnectionSet<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionSet<{:?}>", V::default())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct ConnectionMarker<V: Version>(PhantomData<V>);

impl<V: Version> Debug for ConnectionMarker<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionMarker<{:?}>", V::default())
    }
}
