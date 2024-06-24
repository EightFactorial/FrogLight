use std::time::Duration;

use froglight_protocol::{
    packet::ServerStatus,
    states::Status,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_21_0;

/// A trait that implements the [`Status`] state.
pub(super) trait StatusState: Version
where
    Status: State<Self>,
    Serverbound: NetworkDirection<Self, Status>,
{
    fn perform_status_request(
        conn: Connection<Self, Status, Serverbound>,
    ) -> impl std::future::Future<Output = Result<(ServerStatus, Duration), ConnectionError>> + Send + Sync;
}
