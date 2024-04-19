use froglight_protocol::{
    states::Play,
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

mod v1_20_2;
mod v1_20_3;

pub(crate) trait PlayHandler: Version
where
    Serverbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    // Returns `true` if the connection should enter the `Configuration` state
    // after sending this packet.
    fn serverbound_enter_configuration(
        packet: &<Serverbound as NetworkDirection<Self, Play>>::Send,
    ) -> bool;

    // Returns `true` if the connection should enter the `Configuration` state
    // after receiving this packet.
    fn clientbound_enter_configuration(
        packet: &<Serverbound as NetworkDirection<Self, Play>>::Recv,
    ) -> bool;
}
