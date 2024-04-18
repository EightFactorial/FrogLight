use bevy_app::{App, PreUpdate};
use bevy_ecs::{prelude::any_with_component, schedule::IntoSystemSetConfigs};
use froglight_core::systemsets::NetworkPreUpdateSet;
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use crate::connection::{
    plugin::systems::{
        misc::{ConnectionMarker, ConnectionSet},
        states::{handshaking::HandshakeState, login::LoginState, status::StatusState},
    },
    ConnectionChannel, NetworkDirection, RecvPacketEvent, SendPacketEvent, Serverbound,
};

mod v1_20_0;

pub(crate) trait HandleConnection: Version
where
    Serverbound: NetworkDirection<Self, Play>,
    Handshaking: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Play: State<Self>,
{
    type Channel;

    fn build(app: &mut App) {
        // Add packet events
        app.add_event::<SendPacketEvent<Self>>().add_event::<RecvPacketEvent<Self>>();

        // Add ConnectionSet<V>
        app.configure_sets(
            PreUpdate,
            ConnectionSet::<Self>::default()
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .in_set(NetworkPreUpdateSet),
        );

        // Add systems
        Self::add_systems(app);
    }

    fn add_systems(app: &mut App);
}

impl<V: Version> HandleConnection for V
where
    V: HandshakeState + StatusState + LoginState,
    Serverbound: NetworkDirection<V, Handshaking>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    type Channel = ConnectionChannel<Self>;

    fn add_systems(_app: &mut App) {}
}
