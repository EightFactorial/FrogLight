use std::sync::Arc;

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{
    common_conditions::{any_with_component, on_event},
    Condition, IntoSystemConfigs,
};
use bevy_log::error;
use bevy_tasks::futures_lite::future::try_zip;
use froglight_core::systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet};
use froglight_protocol::{
    states::{Login, Play},
    versions::v1_20_0::V1_20_0,
};

use super::{fire_legacy_recvpacket, handle_connection_error, listen_legacy_sendpacket};
use crate::connection::{
    channels::LegacyTaskChannel,
    events::{RecvPacket, SendPacket},
    Connection, ConnectionError, ConnectionHandler, ConnectionMarker, LegacyPacketChannel,
    ReadConnection, WriteConnection,
};

impl ConnectionHandler for V1_20_0 {
    type PacketChannels = LegacyPacketChannel<Self>;

    fn version_build(app: &mut App) {
        // Add login and play packet events
        app.add_event::<SendPacket<Self, Play>>()
            .add_event::<RecvPacket<Self, Play>>()
            .add_event::<SendPacket<Self, Login>>()
            .add_event::<RecvPacket<Self, Login>>();

        // Add systems to send packet events
        app.add_systems(
            PreUpdate,
            fire_legacy_recvpacket::<Self>
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .in_set(NetworkPreUpdateSet),
        );

        // Add systems to listen for packet events
        app.add_systems(
            PostUpdate,
            listen_legacy_sendpacket::<Self>
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .run_if(
                    on_event::<SendPacket<Self, Play>>()
                        .or_else(on_event::<SendPacket<Self, Login>>()),
                )
                .in_set(NetworkPostUpdateSet),
        );
    }

    async fn handle_packets(
        conn: Connection<Self, Login>,
        channels: LegacyTaskChannel<Self>,
    ) -> ConnectionError {
        // Split the connection into read and write halves.
        let (read, write) = conn.play().into_split();

        // Handle packets in both directions.
        match try_zip(server_to_bevy(read, channels.clone()), bevy_to_server(write, channels)).await
        {
            Ok(((), ())) => unreachable!("Both tasks should never complete Ok"),
            Err(err) => err,
        }
    }
}

// --- Async functions ---

/// Server -> Bevy
async fn server_to_bevy(
    mut read: ReadConnection<V1_20_0, Play>,
    channels: LegacyTaskChannel<V1_20_0>,
) -> Result<(), ConnectionError> {
    loop {
        match read.recv().await {
            Ok(packet) => {
                if let Err(err) = channels.play.recv.send(Arc::new(packet)).await {
                    error!("Failed to send packet to Bevy: \"{err:?}\"");
                    return Err(ConnectionError::ConnectionClosed);
                }
            }
            Err(err) => handle_connection_error(err)?,
        }
    }
}

/// Bevy -> Server
async fn bevy_to_server(
    mut write: WriteConnection<V1_20_0, Play>,
    channels: LegacyTaskChannel<V1_20_0>,
) -> Result<(), ConnectionError> {
    loop {
        match channels.play.send.recv().await {
            Ok(packet) => {
                if let Err(err) = write.send_packet(&packet).await {
                    error!("Failed to send packet to Server: \"{err:?}\"");
                    return Err(err);
                }
            }
            Err(err) => {
                error!("Failed to receive packet from Bevy: \"{err:?}\"");
                return Err(ConnectionError::ConnectionClosed);
            }
        }
    }
}
