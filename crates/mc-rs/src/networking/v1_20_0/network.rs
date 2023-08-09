use bevy::prelude::*;
use mc_rs_proto::{
    versions::{
        state::{Configuration, Play},
        v1_20_0::V1_20_0,
    },
    State,
};

use crate::networking::network::Network;

impl Network for V1_20_0 {
    const HAS_CONFIGURATION_STATE: bool = false;

    fn config_packet(_world: &mut World, packet: <Configuration as State<Self>>::Clientbound) {
        info!("Config packet: {:?}", packet);
    }

    fn play_packet(_world: &mut World, packet: <Play as State<Self>>::Clientbound) {
        info!("Play packet: {:?}", packet);
    }
}
