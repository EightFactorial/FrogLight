use bevy::prelude::*;
use mc_rs_proto::{
    versions::{
        state::{Configuration, Play},
        v1_20_1::V1_20_1,
    },
    State,
};

use crate::networking::network::Network;

impl Network for V1_20_1 {
    const HAS_CONFIGURATION_STATE: bool = false;

    fn config_packet(packet: <Configuration as State<Self>>::Clientbound) {
        info!("Config packet: {:?}", packet);
    }

    fn play_packet(packet: <Play as State<Self>>::Clientbound) {
        info!("Play packet: {:?}", packet);
    }
}
