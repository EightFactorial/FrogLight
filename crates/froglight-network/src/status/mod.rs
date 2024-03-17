//! Server pinging and status requests
use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};
use froglight_protocol::{
    io::{FrogRead, FrogWrite},
    states::{Handshaking, Status},
    traits::{State, Version},
    versions::{v1_20_0::V1_20_0, v1_20_2::V1_20_2, v1_20_3::V1_20_3},
};

mod ping;
pub use ping::*;

#[allow(clippy::module_inception)]
mod status;
pub use status::*;

mod versions;
use versions::Queryable;

/// A plugin that manages server pinging and status requests.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostUpdate, NetworkStatusPostUpdateSet);
        app.add_event::<StatusResponse>().add_event::<PingResponse>();

        setup_version::<V1_20_0>(app);
        setup_version::<V1_20_2>(app);
        setup_version::<V1_20_3>(app);
    }
}

/// Set up the status plugin for the given version.
fn setup_version<V: Queryable>(app: &mut App)
where
    Handshaking: State<V>,
    <Handshaking as State<V>>::ClientboundPacket: FrogRead,
    <Handshaking as State<V>>::ServerboundPacket: FrogWrite,

    Status: State<V>,
    <Status as State<V>>::ClientboundPacket: FrogRead,
    <Status as State<V>>::ServerboundPacket: FrogWrite,
{
    app.configure_sets(
        PostUpdate,
        NetworkStatusVersionSet::<V>::default().in_set(NetworkStatusPostUpdateSet),
    );

    StatusRequest::<V>::build(app);
    PingRequest::<V>::build(app);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct NetworkStatusPostUpdateSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
struct NetworkStatusVersionSet<V: Version>(std::marker::PhantomData<V>);
