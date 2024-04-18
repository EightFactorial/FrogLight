use froglight_protocol::{
    states::Handshaking,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

pub(crate) trait HandshakeState: Version
where
    Handshaking: State<Self>,
    Serverbound: NetworkDirection<Self, Handshaking>,
{
    async fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
    ) -> Result<(), ConnectionError>;
}
