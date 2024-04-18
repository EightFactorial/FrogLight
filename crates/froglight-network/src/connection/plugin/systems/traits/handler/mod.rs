use bevy_app::{App, PreUpdate};
use bevy_ecs::{prelude::any_with_component, schedule::IntoSystemSetConfigs};
use froglight_core::systemsets::NetworkPreUpdateSet;
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use crate::connection::{
    events::{RecvPacketEvent, SendPacketEvent},
    plugin::systems::{
        parts::{ConnectionMarker, ConnectionSet},
        states::{
            configuration::ConfigurationState, handshaking::HandshakeState, login::LoginState,
            play::PlayState, status::StatusState,
        },
    },
    NetworkDirection, Serverbound,
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
    V: HandshakeState + StatusState + LoginState + ConfigurationState + PlayState,
    Handshaking: State<V>,
    Serverbound: NetworkDirection<V, Handshaking>,
    Status: State<V>,
    Serverbound: NetworkDirection<V, Status>,
    Login: State<V>,
    Serverbound: NetworkDirection<V, Login>,
    Configuration: State<V>,
    Serverbound: NetworkDirection<V, Configuration>,
    Play: State<V>,
    Serverbound: NetworkDirection<V, Play>,
{
    fn add_systems(_app: &mut App) {}
}
