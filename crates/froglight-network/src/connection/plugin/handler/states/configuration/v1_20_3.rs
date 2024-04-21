use froglight_protocol::{states::Configuration, versions::v1_20_3::V1_20_3};

use crate::connection::{NetworkDirection, Serverbound};

impl super::ConfigurationHandler for V1_20_3 {
    fn serverbound_enter_play(
        _packet: &<Serverbound as NetworkDirection<Self, Configuration>>::Send,
    ) -> bool {
        todo!()
    }

    fn clientbound_enter_play(
        _packet: &<Serverbound as NetworkDirection<Self, Configuration>>::Recv,
    ) -> bool {
        todo!()
    }
}
