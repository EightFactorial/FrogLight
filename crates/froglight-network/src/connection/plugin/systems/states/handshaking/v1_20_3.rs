use froglight_protocol::{
    common::ConnectionIntent,
    states::Handshaking,
    traits::Version,
    versions::v1_20_3::{handshaking::HandshakeC2SPacket, V1_20_3},
};

use crate::connection::{Connection, ConnectionError};

impl super::HandshakeHandler for V1_20_3 {
    async fn version_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> Result<(), ConnectionError> {
        conn.send(HandshakeC2SPacket {
            protocol_version: V1_20_3::ID,
            hostname: conn.info.get_address(),
            port: conn.info.get_port(),
            intention: intent,
        })
        .await
    }
}
