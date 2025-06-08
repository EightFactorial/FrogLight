//! [`Connection`], [`RawConnection`], and [`RawPacket`](raw::RawPacket)

pub mod raw;
pub use raw::RawConnection;

pub mod split;
pub use split::{ReadConnection, WriteConnection};

pub mod state;
use state::{Client, Server};
pub use state::{Connection, ConnectionError};

#[cfg(feature = "crypto")]
mod crypto;
#[cfg(feature = "crypto")]
pub use crypto::ConnectionCrypto;

/// A [`Connection`] from a [`Client`] to a [`Server`]
pub type ClientConnection<V, S> = Connection<V, S, Client>;
/// A [`Connection`] from a [`Server`] to a [`Client`]
pub type ServerConnection<V, S> = Connection<V, S, Server>;
