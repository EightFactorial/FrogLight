use froglight_protocol::{states::Play, versions::v1_20_3::V1_20_3};

use crate::connection::{NetworkDirection, Serverbound};

impl super::PlayHandler for V1_20_3 {
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
