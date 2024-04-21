use froglight_protocol::{states::Play, versions::v1_20_2::V1_20_2};

use crate::connection::{NetworkDirection, Serverbound};

impl super::PlayHandler for V1_20_2 {
    fn serverbound_enter_config(
        _packet: &<Serverbound as NetworkDirection<Self, Play>>::Send,
    ) -> bool {
        todo!()
    }

    fn clientbound_enter_config(
        _packet: &<Serverbound as NetworkDirection<Self, Play>>::Recv,
    ) -> bool {
        todo!()
    }
}
