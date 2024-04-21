use froglight_protocol::{
    common::ConnectionIntent,
    states::Handshaking,
    traits::Version,
    versions::v1_20_0::{handshaking::HandshakeC2SPacket, V1_20_0},
};

use crate::connection::{Connection, ConnectionError};

impl super::HandshakeHandler for V1_20_0 {
    async fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> Result<(), ConnectionError> {
        conn.send(HandshakeC2SPacket {
            protocol_version: V1_20_0::ID,
            hostname: conn.info.get_address(),
            port: conn.info.get_port(),
            intention: intent,
        })
        .await
    }
}
