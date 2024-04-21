use std::{future::Future, time::Duration};

use froglight_protocol::{
    packet::ServerStatus,
    states::Status,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

/// A trait for handling the [`Status`] state.
pub trait StatusHandler: Version
where
    Serverbound: NetworkDirection<Self, Status>,
    Status: State<Self>,
{
    /// Performs a status request for the connection.
    fn perform_status_request(
        conn: &mut Connection<Self, Status>,
    ) -> impl Future<Output = Result<ServerStatus, ConnectionError>> + Send + Sync;

    /// Performs a ping request for the connection.
    fn perform_ping_request(
        conn: &mut Connection<Self, Status>,
        payload: u64,
    ) -> impl Future<Output = Result<Duration, ConnectionError>> + Send + Sync;
}
