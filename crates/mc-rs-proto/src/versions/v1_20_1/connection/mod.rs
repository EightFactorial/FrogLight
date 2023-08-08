use super::V1_20_1;
use crate::versions::state::Configuration;
use mc_rs_macros::impl_state;

impl_state!(
    Configuration,
    V1_20_1,
    Clientbound => {
    },
    Serverbound => {
    },
);
