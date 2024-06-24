use froglight_protocol::{
    common::ConnectionIntent,
    states::Handshake,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_21_0;

/// A trait that implements the [`Handshake`] state.
pub(super) trait HandshakeState: Version
where
    Handshake: State<Self>,
    Serverbound: NetworkDirection<Self, Handshake>,
{
    fn perform_handshake(
        conn: Connection<Self, Handshake, Serverbound>,
        intent: ConnectionIntent,
    ) -> impl std::future::Future<
        Output = Result<Connection<Self, Handshake, Serverbound>, ConnectionError>,
    > + Send
           + Sync;
}
