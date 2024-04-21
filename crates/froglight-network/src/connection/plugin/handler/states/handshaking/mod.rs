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

/// A trait for handling the [`Handshaking`] state.
pub trait HandshakeHandler: Version
where
    Serverbound: NetworkDirection<Self, Handshaking>,
    Handshaking: State<Self>,
{
    /// Performs the handshake for the connection.
    fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send + Sync;
}
