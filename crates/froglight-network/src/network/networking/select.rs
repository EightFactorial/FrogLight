use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use super::{configuration::ConfigurationState, login::LoginState, play::PlayState};
use crate::connection::{ConnectionError, NetworkDirection, Serverbound, WriteConnection};

type FnOutput = Result<bool, ConnectionError>;
type SendPacket<V, S> = <Serverbound as NetworkDirection<V, S>>::Send;
type RecvPacket<V, S> = <Serverbound as NetworkDirection<V, S>>::Recv;
type WriteConn<V, S> = WriteConnection<V, S, Serverbound>;

/// A trait for selecting the correct packet function
/// based on the current state.
pub(super) trait PacketFn<V>: State<V>
where
    V: Version,
    Serverbound: NetworkDirection<V, Self>,
{
    /// Returns the correct packet function based on the current state.
    fn packet_state_fn(
        packet: &RecvPacket<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync;

    /// Returns the correct acknowledgement function based on the current state.
    fn packet_ack_fn(packet: &SendPacket<V, Self>) -> bool;
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
    fn packet_state_fn(
        packet: &RecvPacket<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync {
        V::login_state_handle(packet, conn)
    }

    #[inline]
    fn packet_ack_fn(packet: &SendPacket<V, Self>) -> bool { V::login_ack_handle(packet) }
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
    fn packet_state_fn(
        packet: &RecvPacket<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync {
        V::config_state_handle(packet, conn)
    }

    #[inline]
    fn packet_ack_fn(packet: &SendPacket<V, Self>) -> bool { V::config_ack_handle(packet) }
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
    fn packet_state_fn(
        packet: &RecvPacket<V, Self>,
        conn: &WriteConn<V, Self>,
    ) -> impl std::future::Future<Output = FnOutput> + Send + Sync {
        V::play_state_handle(packet, conn)
    }

    #[inline]
    fn packet_ack_fn(packet: &SendPacket<V, Self>) -> bool { V::play_ack_handle(packet) }
}
