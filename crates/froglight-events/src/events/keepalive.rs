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
    app.init_resource::<AutomaticKeepAlive>().add_event::<ServerKeepAlive>();

    app.add_systems(PreUpdate, ServerKeepAlive::send_keepalive_events.in_set(EventPreUpdateSet));
    app.add_systems(
        PostUpdate,
        ServerKeepAlive::keepalive_responder
            .run_if(AutomaticKeepAlive::is_enabled)
            .run_if(on_event::<ServerKeepAlive>())
            .in_set(EventPostUpdateSet),
    );
}

/// A keep alive packet sent by the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ServerKeepAlive {
    /// A random value
    pub value: u64,
}

impl ServerKeepAlive {
    /// A system that sends keep alive events.
    fn send_keepalive_events(
        mut v1_20_0: EventReader<RecvPacket<V1_20_0, Play>>,
        mut writer: EventWriter<Self>,
    ) {
        for packet in v1_20_0.read() {
            if let v1_20_0::play::PlayClientboundPackets::KeepAlive(packet) = &*packet.packet {
                writer.send(ServerKeepAlive { value: packet.value });
            }
        }
    }

    /// A system that automatically responds to keep alive packets.
    fn keepalive_responder(
        mut recv_v1_20_0: EventReader<RecvPacket<V1_20_0, Play>>,
        mut send_v1_20_0: EventWriter<SendPacket<V1_20_0, Play>>,
    ) {
        for event in recv_v1_20_0.read() {
            if let v1_20_0::play::PlayClientboundPackets::KeepAlive(packet) = &*event.packet {
                send_v1_20_0.send(SendPacket {
                    packet: Arc::new(v1_20_0::play::PlayServerboundPackets::KeepAlive(
                        v1_20_0::play::KeepAliveC2SPacket { value: packet.value },
                    )),
                    connection: Some(event.connection),
                });
            }
        }
    }
}

/// A [`Resource`] that determines whether the client should automatically
/// respond to keep alive packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct AutomaticKeepAlive(pub bool);

impl Default for AutomaticKeepAlive {
    fn default() -> Self { Self(true) }
}

impl AutomaticKeepAlive {
    /// Returns `true` if [`AutomaticKeepAlive`] is enabled.
    pub fn enabled(&self) -> bool { self.0 }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that returns `true` if
    /// [`AutomaticKeepAlive`] is enabled.
    pub fn is_enabled(res: Res<Self>) -> bool { res.enabled() }
}
