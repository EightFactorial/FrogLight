use std::marker::PhantomData;

use bevy_ecs::{component::Component, schedule::SystemSet};
use froglight_protocol::traits::Version;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(super) struct ConnectionSet<V: Version>(PhantomData<V>);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(super) struct ConnectionMarker<V: Version>(PhantomData<V>);
