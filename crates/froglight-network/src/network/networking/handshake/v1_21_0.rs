use froglight_protocol::{
    common::ConnectionIntent,
    states::Handshake,
    traits::Version,
    versions::v1_21_0::{handshake::HandshakePacket, V1_21_0},
};

use super::HandshakeState;
use crate::connection::{Connection, ConnectionError};

impl HandshakeState for V1_21_0 {
    async fn perform_handshake(
        mut conn: Connection<Self, Handshake>,
        intent: ConnectionIntent,
    ) -> Result<Connection<Self, Handshake>, ConnectionError> {
        conn.send(HandshakePacket {
            protocol: Self::ID,
            address: conn.info.get_address(),
            port: conn.info.get_port(),
            intent,
        })
        .await
        .map(|()| conn)
    }
}
