use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use crate::connection::{ConnectionError, NetworkDirection, Serverbound, WriteConnection};

mod v1_21_0;

/// A trait that implements the [`Configuration`] state.
pub(super) trait ConfigurationState: Version
where
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
    Serverbound: NetworkDirection<Self, Login>
        + NetworkDirection<Self, Configuration>
        + NetworkDirection<Self, Play>,
{
    /// Returns `true` if the configuration was successful,
    /// or `false` if the configuration is still in progress.
    fn end_configuration<'a, 'b>(
        packet: &'a <Serverbound as NetworkDirection<Self, Configuration>>::Recv,
        conn: &'b WriteConnection<Self, Configuration, Serverbound>,
    ) -> impl std::future::Future<Output = Result<bool, ConnectionError>> + Send + Sync;

    /// Returns `true` when the end of the configuration has been acknowledged.
    fn config_acknowledged(
        packet: &<Serverbound as NetworkDirection<Self, Configuration>>::Send,
    ) -> bool;
}
