//! A manual implementation of the [`HandleConnection`] trait for [`V1_20_0`],
//! since it does not have a [`Configuration`](super::Configuration) state.

use std::sync::Arc;

use bevy_app::App;
use bevy_log::error;
use froglight_protocol::{
    common::{ConnectionIntent, GameProfile},
    packet::ServerStatus,
    states::{Handshaking, Login, Play, Status},
    versions::v1_20_0::V1_20_0,
};

use super::HandleConnection;
use crate::connection::{
    plugin::{
        channel::legacy::async_task::LegacyPacketChannel,
        systems::states::handshaking::HandshakeState,
    },
    Connection, ConnectionError, LegacyChannel,
};

impl HandleConnection for V1_20_0 {
    type Channel = LegacyChannel<Self>;

    fn add_systems(_app: &mut App) {}

    async fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> Result<(), ConnectionError> {
        V1_20_0::version_handshake(conn, intent).await
    }

    async fn perform_status(
        _conn: &mut Connection<Self, Status>,
    ) -> Result<ServerStatus, ConnectionError> {
        todo!()
    }

    async fn perform_login(
        _conn: &mut Connection<Self, Login>,
    ) -> Result<GameProfile, ConnectionError> {
        todo!()
    }

    async fn task_function(conn: Connection<Self, Login>, channel: LegacyPacketChannel<Self>) {
        let conn = conn.play();
        let _ = futures::try_join!(
            listen_from_server(conn.clone(), channel.clone()),
            listen_from_channel(channel, conn)
        );
    }
}

/// Listens for packets from the server and sends them to the channel.
///
/// (Bevy)
/// Server -> Channel
async fn listen_from_server(
    mut conn: Connection<V1_20_0, Play>,
    channel: LegacyPacketChannel<V1_20_0>,
) -> Result<(), ()> {
    loop {
        match conn.recv().await {
            Ok(packet) => {
                // Send the packet to the channel
                if let Err(err) = channel.send_packet(Arc::new(packet)).await {
                    error!("Failed to send packet to Channel: \"{err:?}\"");
                    return Err(());
                }
            }
            Err(err) => {
                error!("Failed to receive packet from Connection: \"{err:?}\"");
                if let Err(err) = channel.errors.send(err).await {
                    error!("Failed to send error to Channel: \"{err:?}\"");
                    return Err(());
                }

                #[cfg(debug_assertions)]
                {
                    error!("Debug: Closing Connection");
                    return Err(());
                }
            }
        }
    }
}

/// Listens for packets from the channel and sends them to the server.
///
/// (Bevy)
/// Channel -> Server
async fn listen_from_channel(
    channel: LegacyPacketChannel<V1_20_0>,
    mut conn: Connection<V1_20_0, Play>,
) -> Result<(), ()> {
    loop {
        match channel.recv_packet().await {
            Ok(packet) => {
                // Send the packet to the server
                if let Err(err) = conn.send_packet(&packet).await {
                    error!("Failed to send packet to Connection: \"{err:?}\"");
                    return Err(());
                }
            }
            Err(err) => {
                error!("Failed to receive packet from Channel: \"{err:?}\"");
                return Err(());
            }
        }
    }
}
