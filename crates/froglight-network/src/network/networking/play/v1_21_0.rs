use froglight_protocol::{states::Play, versions::v1_21_0::V1_21_0};

use super::PlayState;
use crate::{
    connection::{Connection, ConnectionError, Serverbound},
    network::channel::ConnectionTaskChannel,
};

impl PlayState for V1_21_0 {
    async fn perform_play(
        mut _conn: Connection<Self, Play, Serverbound>,
        _task_channel: &ConnectionTaskChannel<Self, Serverbound>,
    ) -> Result<(), ConnectionError> {
        todo!();
    }
}
