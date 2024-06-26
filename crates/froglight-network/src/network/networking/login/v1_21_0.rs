use froglight_protocol::{
    states::Login,
    versions::v1_21_0::{
        login::{LoginClientboundPackets, LoginHelloC2SPacket, LoginServerboundPackets},
        V1_21_0,
    },
};

use super::LoginState;
use crate::connection::{
    Connection, ConnectionError, NetworkDirection, Serverbound, WriteConnection,
};

impl LoginState for V1_21_0 {
    async fn perform_login(
        mut conn: Connection<Self, Login, Serverbound>,
    ) -> Result<Connection<Self, Login>, ConnectionError> {
        conn.send(LoginHelloC2SPacket {
            username: conn.account().username.clone(),
            uuid: conn.account().uuid,
        })
        .await?;

        Ok(conn)
    }

    async fn login_state_handle(
        packet: &<Serverbound as NetworkDirection<Self, Login>>::Recv,
        conn: &WriteConnection<Self, Login, Serverbound>,
    ) -> Result<bool, ConnectionError> {
        match packet {
            LoginClientboundPackets::LoginDisconnect(packet) => {
                Err(ConnectionError::ServerError(serde_json::to_string(&packet.reason).unwrap()))
            }
            LoginClientboundPackets::LoginSuccess(..) => Ok(true),
            #[allow(clippy::cast_possible_wrap)]
            LoginClientboundPackets::LoginCompression(packet) => {
                *conn.compression.write().await = Some(packet.threshold as i32);
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn login_ack_handle(packet: &<Serverbound as NetworkDirection<Self, Login>>::Send) -> bool {
        matches!(packet, LoginServerboundPackets::EnterConfiguration(..))
    }
}
