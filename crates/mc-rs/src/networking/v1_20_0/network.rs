use bevy::prelude::*;
use mc_rs_proto::versions::v1_20_0::{
    connection::ClientboundConfigurationPackets, play::ClientboundPlayPackets, V1_20_0,
};

use crate::networking::network::Network;

impl Network for V1_20_0 {
    const HAS_CONFIGURATION_STATE: bool = false;

    fn config_packet(_world: &mut World, _packet: ClientboundConfigurationPackets) {
        unreachable!("This version does not have a configuration state",);
    }

    fn play_packet(_world: &mut World, packet: ClientboundPlayPackets) {
        info!("Play packet: {:?}", packet);
    }
}
