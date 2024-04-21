use std::sync::Arc;

use bevy_log::{debug, error, trace};
use froglight_protocol::{
    common::GameProfile,
    states::Login,
    versions::v1_20_3::{
        login::{LoginClientboundPackets, LoginHelloC2SPacket, LoginQueryResponseC2SPacket},
        V1_20_3,
    },
};

use crate::connection::{
    plugin::channels::parts::TaskPair, Connection, ConnectionError, LoginPlugins,
};

impl super::LoginHandler for V1_20_3 {
    async fn perform_login(
        conn: &mut Connection<Self, Login>,
        channel: &TaskPair<Self, Login>,
        plugins: &LoginPlugins,
    ) -> Result<GameProfile, ConnectionError> {
        // Send the Hello packet
        conn.send(LoginHelloC2SPacket {
            username: conn.account.username.clone(),
            uuid: conn.account.uuid,
        })
        .await?;

        loop {
            // Loop until we receive a LoginSuccess packet
            match conn.recv().await? {
                LoginClientboundPackets::LoginCompression(packet) => {
                    debug!("Setting Login Compression: \"{:?}\"", packet.compression);
                    conn.set_compression(Some(packet.compression));
                }
                LoginClientboundPackets::LoginHello(_) => todo!("Support encryption"),

                LoginClientboundPackets::LoginDisconnect(packet) => {
                    error!("Disconnected during login: \"{}\"", packet.reason);

                    if let Err(err) = channel.recv.send(Arc::new(packet.into())).await {
                        error!("Failed to forward LoginDisconnect: {err:?}");
                    }

                    return Err(ConnectionError::ConnectionClosed);
                }
                LoginClientboundPackets::LoginSuccess(packet) => {
                    return Ok(packet.profile);
                }
                LoginClientboundPackets::LoginQueryRequest(packet) => {
                    debug!("Received LoginQueryRequest: \"{}\"", packet.identifier);

                    // If the channel has a plugin, wait for the plugin to respond
                    if plugins.read().contains(&packet.identifier) {
                        debug!("Forwarding LoginQueryRequest to plugin: \"{}\"", packet.identifier);

                        // Forward the packet to the plugin
                        if let Err(err) = channel.recv.send(Arc::new(packet.into())).await {
                            error!("Failed to forward LoginQueryRequest to plugin: {err:?}");
                            return Err(ConnectionError::ConnectionClosed);
                        }

                        // Get the response from the plugin
                        match channel.send.recv().await {
                            Ok(packet) => {
                                trace!(
                                    "Received LoginQueryResponse from plugin: `{}`",
                                    std::any::type_name_of_val(packet.as_ref())
                                );
                                conn.send_packet(&packet).await?;
                            }
                            Err(err) => {
                                error!("Failed to get LoginQueryResponse from plugin: {err:?}");
                                return Err(ConnectionError::ConnectionClosed);
                            }
                        }
                    } else {
                        debug!(
                            "Client doesn't understand LoginQueryRequest: \"{}\"",
                            packet.identifier
                        );

                        // Respond that the client doesn't understand the query
                        conn.send(LoginQueryResponseC2SPacket { id: packet.id, data: None })
                            .await?;
                    }
                }
            }
        }
    }
}
