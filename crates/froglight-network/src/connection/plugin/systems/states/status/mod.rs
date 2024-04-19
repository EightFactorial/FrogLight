use std::future::Future;

use froglight_protocol::{
    packet::ServerStatus,
    states::Status,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

pub(crate) trait StatusState: Version
where
    Status: State<Self>,
    Serverbound: NetworkDirection<Self, Status>,
{
    fn version_status_request(
        conn: &mut Connection<Self, Status>,
    ) -> impl Future<Output = Result<ServerStatus, ConnectionError>> + Send + Sync;

    fn version_ping_request(
        conn: &mut Connection<Self, Status>,
        payload: u64,
    ) -> impl Future<Output = Result<u64, ConnectionError>> + Send + Sync;
}
