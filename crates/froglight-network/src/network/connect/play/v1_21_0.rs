use froglight_protocol::{
    common::ConnectionIntent,
    states::{Configuration, Handshake, Login, Play},
    traits::Version,
    versions::v1_21_0::{handshake::HandshakePacket, V1_21_0},
};

use super::PerformServerConnection;
use crate::{
    connection::{Connection, ConnectionError, Serverbound},
    network::channel::AsyncConnectionChannel,
};

impl PerformServerConnection for V1_21_0 {
    async fn perform_handshake(
        mut conn: Connection<Self, Handshake, Serverbound>,
    ) -> Result<Connection<Self, Handshake, Serverbound>, ConnectionError> {
        // Send a Handshake to the server.
        conn.send(HandshakePacket {
            protocol: V1_21_0::ID,
            address: conn.info.get_address(),
            port: conn.info.get_port(),
            intent: ConnectionIntent::Status,
        })
        .await?;

        Ok(conn)
    }

    async fn perform_login(
        _conn: Connection<Self, Login, Serverbound>,
        _channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> Result<Connection<Self, Login, Serverbound>, ConnectionError> {
        todo!()
    }

    async fn perform_configuration(
        _conn: Connection<Self, Configuration, Serverbound>,
        _channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> Result<Connection<Self, Configuration, Serverbound>, ConnectionError> {
        todo!()
    }

    async fn perform_play(
        _conn: Connection<Self, Play, Serverbound>,
        _channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> Result<Option<Connection<Self, Play, Serverbound>>, ConnectionError> {
        todo!()
    }
}
