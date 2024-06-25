use std::sync::Arc;

use bevy_log::{error, warn};
use froglight_protocol::{
    states::Login,
    versions::v1_21_0::{
        login::{
            EnterConfigurationPacket, LoginClientboundPackets, LoginHelloC2SPacket,
            LoginServerboundPackets,
        },
        V1_21_0,
    },
};

use super::LoginState;
use crate::{
    connection::{Connection, ConnectionError, Serverbound},
    network::channel::ConnectionTaskChannel,
};

impl LoginState for V1_21_0 {
    async fn perform_login(
        mut conn: Connection<Self, Login, Serverbound>,
        task_channel: &ConnectionTaskChannel<Self, Serverbound>,
    ) -> Result<Connection<Self, Login>, ConnectionError> {
        conn.send(LoginHelloC2SPacket {
            username: conn.account().username.clone(),
            uuid: conn.account().uuid,
        })
        .await?;

        loop {
            match futures_lite::future::or(
                // TODO: Check if this is cancel safe
                async { conn.recv().await.map(PacketResult::Send) },
                async {
                    task_channel.login.recv.recv().await.map(PacketResult::Recv).map_err(|_| {
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
                    task_channel.login.send.send(packet.clone()).await.map_err(|_| {
                        error!("Failed to forward packet to Bevy!");
                        ConnectionError::ConnectionClosed
                    })?;

                    // Handle the login process
                    match packet.as_ref() {
                        LoginClientboundPackets::LoginDisconnect(_disconnect_packet) => {
                            return Err(ConnectionError::ConnectionClosed);
                        }
                        LoginClientboundPackets::LoginHello(_encryption_packet) => {
                            error!("Encryption is not supported yet!");
                            return Err(ConnectionError::ConnectionClosed);
                        }
                        LoginClientboundPackets::LoginSuccess(profile_packet) => {
                            let acc = conn.account_mut();

                            if acc.username != profile_packet.profile.name {
                                warn!("Account username does not match the server's response: \"{}\" != \"{}\"", acc.username, profile_packet.profile.name);
                                acc.username = profile_packet.profile.name.clone();
                            }
                            if acc.uuid != profile_packet.profile.uuid {
                                warn!("Account UUID does not match the server's response: \"{}\" != \"{}\"", acc.uuid, profile_packet.profile.uuid);
                                acc.uuid = profile_packet.profile.uuid;
                            }

                            // Acknowledge the login
                            conn.send(EnterConfigurationPacket).await?;

                            // Login is successful
                            return Ok(conn);
                        }
                        #[allow(clippy::cast_possible_wrap)]
                        LoginClientboundPackets::LoginCompression(compression_packet) => {
                            let comp = conn.compression_mut();
                            *comp = Some(compression_packet.threshold as i32);
                        }
                        // Packets are handled by the ECS
                        LoginClientboundPackets::LoginQueryRequest(_)
                        | LoginClientboundPackets::CookieRequest(_) => {}
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
    Send(LoginClientboundPackets),
    Recv(Arc<LoginServerboundPackets>),
}
