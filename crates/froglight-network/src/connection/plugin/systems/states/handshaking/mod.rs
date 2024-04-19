use std::future::Future;

use froglight_protocol::{
    common::ConnectionIntent,
    states::Handshaking,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

pub(crate) trait HandshakeHandler: Version
where
    Handshaking: State<Self>,
    Serverbound: NetworkDirection<Self, Handshaking>,
{
    fn version_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send + Sync;
}
