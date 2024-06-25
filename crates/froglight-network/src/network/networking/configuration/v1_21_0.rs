use std::sync::Arc;

use bevy_log::error;
use froglight_protocol::{
    states::Configuration,
    versions::v1_21_0::{
        configuration::{
            ConfigurationClientboundPackets, ConfigurationServerboundPackets, ReadyC2SPacket,
        },
        V1_21_0,
    },
};

use super::ConfigurationState;
use crate::{
    connection::{Connection, ConnectionError, Serverbound},
    network::channel::ConnectionTaskChannel,
};

impl ConfigurationState for V1_21_0 {
    async fn perform_configuration(
        mut conn: Connection<Self, Configuration, Serverbound>,
        task_channel: &ConnectionTaskChannel<Self, Serverbound>,
    ) -> Result<Connection<Self, Configuration>, ConnectionError> {
        loop {
            match futures_lite::future::or(
                // TODO: Check if this is cancel safe
                async { conn.recv().await.map(PacketResult::Send) },
                async {
                    task_channel.config.recv.recv().await.map(PacketResult::Recv).map_err(|_| {
                        error!("Failed to receive packet from Bevy!");
                        ConnectionError::ConnectionClosed
                    })
                },
            )
            .await?
            {
                PacketResult::Send(packet) => {
                    let packet = Arc::new(packet);

                    // Send all packets to the ECS
                    task_channel.config.send.send(packet.clone()).await.map_err(|_| {
                        error!("Failed to forward packet to Bevy!");
                        ConnectionError::ConnectionClosed
                    })?;

                    // Handle the configuration process
                    if let ConfigurationClientboundPackets::Ready(_) = packet.as_ref() {
                        conn.send(ReadyC2SPacket).await?;

                        // Configuration is done
                        return Ok(conn);
                    }
                }
                PacketResult::Recv(packet) => {
                    conn.send_packet(&packet).await?;
                }
            }
        }
    }
}

enum PacketResult {
    Send(ConfigurationClientboundPackets),
    Recv(Arc<ConfigurationServerboundPackets>),
}
