use froglight_protocol::{
    states::Configuration,
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

mod v1_20_2;
mod v1_20_3;

/// A trait for handling the [`Configuration`] state.
pub trait ConfigurationHandler: Version
where
    Serverbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Returns `true` if the connection should enter the `Play` state
    /// after sending this packet.
    fn serverbound_enter_play(
        packet: &<Serverbound as NetworkDirection<Self, Configuration>>::Send,
    ) -> bool;

    /// Returns `true` if the connection should enter the `Play` state
    /// after receiving this packet.
    fn clientbound_enter_play(
        packet: &<Serverbound as NetworkDirection<Self, Configuration>>::Recv,
    ) -> bool;
}
