use std::{future::Future, time::Duration};

use froglight_protocol::{
    packet::ServerStatus,
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_21_0;

/// A trait for performing a status requests to a server.
#[expect(clippy::type_complexity)]
pub trait PerformStatusRequest: Version
where
    Serverbound: NetworkDirection<Self, Handshake> + NetworkDirection<Self, Status>,
    Handshake: State<Self>,
    Status: State<Self>,
{
    fn status_request(
        conn: Connection<Self, Handshake, Serverbound>,
    ) -> impl Future<
        Output = Result<
            (Connection<Self, Status, Serverbound>, ServerStatus, Duration),
            ConnectionError,
        >,
    > + Send
           + Sync;
}

pub(super) async fn perform_status_request<V: Version + PerformStatusRequest>(
    conn: Connection<V, Handshake, Serverbound>,
) -> Result<(ServerStatus, Duration), ConnectionError>
where
    Serverbound: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    #[cfg(debug_assertions)]
    bevy_log::debug!("Querying: \"{}:{}\"", conn.info.get_address(), conn.info.get_port());
    V::status_request(conn).await.map(|(_, status, duration)| (status, duration))
}
