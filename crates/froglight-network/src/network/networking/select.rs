use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use super::{configuration::ConfigurationState, login::LoginState, play::PlayState};
use crate::connection::{ConnectionError, NetworkDirection, Serverbound, WriteConnection};

type FnOutput = Result<bool, ConnectionError>;
type PacketType<V, S> = <Serverbound as NetworkDirection<V, S>>::Recv;
type WriteConn<V, S> = WriteConnection<V, S, Serverbound>;

/// A trait for selecting the correct packet function
/// based on the current state.
pub(super) trait PacketFn<V>: State<V>
where
    V: Version,
    Serverbound: NetworkDirection<V, Self>,
{
    fn packet_fn(
        packet: &PacketType<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync;
}

impl<V> PacketFn<V> for Login
where
    V: Version + LoginState,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    #[inline]
    fn packet_fn(
        packet: &PacketType<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync {
        V::end_login(packet, conn)
    }
}

impl<V> PacketFn<V> for Configuration
where
    V: Version + ConfigurationState,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    #[inline]
    fn packet_fn(
        packet: &PacketType<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync {
        V::end_configuration(packet, conn)
    }
}

impl<V> PacketFn<V> for Play
where
    V: Version + PlayState,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
{
    #[inline]
    fn packet_fn(
        packet: &PacketType<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync {
        V::end_play(packet, conn)
    }
}
