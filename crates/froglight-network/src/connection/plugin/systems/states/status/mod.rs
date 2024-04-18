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
    async fn perform_status_request(
        conn: &mut Connection<Self, Status>,
    ) -> Result<ServerStatus, ConnectionError>;

    async fn perform_ping_request(
        conn: &mut Connection<Self, Status>,
        payload: u64,
    ) -> Result<u64, ConnectionError>;
}
