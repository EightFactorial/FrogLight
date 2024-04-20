use bevy_log::{debug, trace};
use froglight_protocol::{
    common::GameProfile,
    states::Login,
    versions::v1_20_2::{
        login::{LoginClientboundPackets, LoginHelloC2SPacket},
        V1_20_2,
    },
};

use crate::connection::{Connection, ConnectionError};

impl super::LoginHandler for V1_20_2 {
    async fn version_login(
        conn: &mut Connection<Self, Login>,
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
                LoginClientboundPackets::LoginSuccess(packet) => {
                    return Ok(packet.profile);
                }
                LoginClientboundPackets::LoginQueryRequest(packet) => {
                    trace!("Received Login Query: {packet:?}");
                }
                LoginClientboundPackets::LoginDisconnect(_) => todo!("Handle disconnect"),
            }
        }
    }
}
