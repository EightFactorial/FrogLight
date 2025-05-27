//! [`Connection`], [`RawConnection`], and [`RawPacket`](raw::RawPacket)

pub mod raw;
pub use raw::RawConnection;

pub mod state;
pub use state::Connection;
use state::{Client, Server};

mod helpers;

/// A [`Connection`] from a [`Client`] to a [`Server`]
pub type ClientConnection<V, S> = Connection<V, S, Client>;
/// A [`Connection`] from a [`Server`] to a [`Client`]
pub type ServerConnection<V, S> = Connection<V, S, Server>;
