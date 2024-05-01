use bevy_app::{App, PreUpdate};
use bevy_ecs::{
    event::{Event, EventReader, EventWriter},
    schedule::IntoSystemConfigs,
};
use froglight_network::{
    connection::events::RecvPacket,
    states::Play,
    versions::v1_20_0::{self, V1_20_0},
};

use crate::systemsets::EventPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_event::<TimeUpdateEvent>();

    app.add_systems(PreUpdate, TimeUpdateEvent::send_time_events.in_set(EventPreUpdateSet));
}

/// An event that is sent when the time is updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct TimeUpdateEvent {
    /// The time the world has been running.
    pub time: u64,
    /// The current time in the world.
    pub time_of_day: u64,
}

impl TimeUpdateEvent {
    fn send_time_events(
        mut v1_20_0: EventReader<RecvPacket<V1_20_0, Play>>,
        mut writer: EventWriter<Self>,
    ) {
        for packet in v1_20_0.read() {
            if let v1_20_0::play::PlayClientboundPackets::WorldTimeUpdate(packet) = &*packet.packet
            {
                writer.send(TimeUpdateEvent { time: packet.time, time_of_day: packet.time_of_day });
            }
        }
    }
}
