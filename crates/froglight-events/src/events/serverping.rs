use std::sync::Arc;

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    event::{Event, EventReader, EventWriter},
    schedule::{common_conditions::on_event, IntoSystemConfigs},
    system::{Res, Resource},
};
use froglight_network::{
    connection::events::{RecvPacket, SendPacket},
    states::Play,
    versions::v1_20_0::{self, V1_20_0},
};

use crate::systemsets::{EventPostUpdateSet, EventPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<AutomaticServerPing>().add_event::<ServerPingEvent>();

    app.add_systems(PreUpdate, ServerPingEvent::send_serverping_events.in_set(EventPreUpdateSet));
    app.add_systems(
        PostUpdate,
        ServerPingEvent::serverping_responder
            .run_if(AutomaticServerPing::is_enabled)
            .run_if(on_event::<ServerPingEvent>())
            .in_set(EventPostUpdateSet),
    );
}

/// A ping packet sent by the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ServerPingEvent {
    /// A random value
    pub value: u32,
}

impl ServerPingEvent {
    /// A system that sends serverping events.
    fn send_serverping_events(
        mut v1_20_0: EventReader<RecvPacket<V1_20_0, Play>>,
        mut writer: EventWriter<Self>,
    ) {
        for packet in v1_20_0.read() {
            if let v1_20_0::play::PlayClientboundPackets::PlayPing(packet) = &*packet.packet {
                writer.send(ServerPingEvent { value: packet.ping });
            }
        }
    }

    /// A system that automatically responds to server ping packets.
    fn serverping_responder(
        mut recv_v1_20_0: EventReader<RecvPacket<V1_20_0, Play>>,
        mut send_v1_20_0: EventWriter<SendPacket<V1_20_0, Play>>,
    ) {
        for event in recv_v1_20_0.read() {
            if let v1_20_0::play::PlayClientboundPackets::PlayPing(packet) = &*event.packet {
                send_v1_20_0.send(SendPacket {
                    packet: Arc::new(v1_20_0::play::PlayServerboundPackets::PlayPong(
                        v1_20_0::play::PlayPongC2SPacket { pong: packet.ping },
                    )),
                    connection: Some(event.connection),
                });
            }
        }
    }
}

/// A [`Resource`] that determines whether the client should automatically
/// respond to server ping packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct AutomaticServerPing(pub bool);

impl Default for AutomaticServerPing {
    fn default() -> Self { Self(true) }
}

impl AutomaticServerPing {
    /// Returns `true` if [`AutomaticServerPing`] is enabled.
    #[must_use]
    pub fn enabled(&self) -> bool { self.0 }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that returns `true` if
    /// [`AutomaticServerPing`] is enabled.
    #[must_use]
    pub fn is_enabled(res: Res<Self>) -> bool { res.enabled() }
}
