use super::{State, ValidState};

/// The direction of the
/// [`RawConnection`](froglight_network::connection::RawConnection).
pub trait Direction<V: ValidState<S>, S: State> {
    /// The type of packet received by the connection.
    type Recv: Send + Sync;
    /// The type of packet sent by the connection.
    type Send: Send + Sync;
}

/// A [`RawConnection`](froglight_network::connection::RawConnection)
/// from a [`Client`] to a [`Server`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client;
impl<V: ValidState<S>, S: State> Direction<V, S> for Client {
    type Recv = V::Clientbound;
    type Send = V::Serverbound;
}

/// A [`RawConnection`](froglight_network::connection::RawConnection)
/// from a [`Server`] to a [`Client`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Server;
impl<V: ValidState<S>, S: State> Direction<V, S> for Server {
    type Recv = V::Serverbound;
    type Send = V::Clientbound;
}
