//! TODO

use core::net::SocketAddr;

use async_net::TcpStream;
use froglight_packet::state::{Handshake, ValidState};

use crate::prelude::ClientConnection;

mod adapter;
pub use adapter::IoAdapter;

mod transport;
#[cfg(feature = "crypto")]
pub use transport::IoCipher;
pub use transport::IoTransport;

impl<V: ValidState<Handshake>> ClientConnection<V, Handshake> {
    /// Connect to a server at the given address,
    /// resolved using the provided
    /// [`FroglightResolver`](crate::prelude::FroglightResolver).
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    #[inline]
    #[cfg(feature = "resolver")]
    pub async fn connect(
        addr: &str,
        resolver: &crate::prelude::FroglightResolver,
    ) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect(addr, resolver).await.map(Self::from_raw)
    }

    /// Connect to a server at the given address,
    /// resolved using the default system resolver.
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    #[inline]
    pub async fn connect_system(addr: &str) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect_system(addr).await.map(Self::from_raw)
    }

    /// Connect to a server at the given [`SocketAddr`].
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    #[inline]
    pub async fn connect_to(socket: SocketAddr) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect_to(socket).await.map(Self::from_raw)
    }
}

// -------------------------------------------------------------------------------------------------

impl IoTransport<TcpStream> {
    /// Create an [`IoTransport`] from an address resolved
    /// using the provided
    /// [`FroglightResolver`](crate::prelude::FroglightResolver).
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    #[inline]
    #[cfg(feature = "resolver")]
    pub async fn connect(
        addr: &str,
        resolver: &crate::prelude::FroglightResolver,
    ) -> Result<Self, std::io::Error> {
        Self::connect_to(resolver.lookup_minecraft(addr).await?).await
    }

    /// Create an [`IoTransport`] from an address resolved
    /// using the system's default resolver.
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    pub async fn connect_system(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr).await?;
        let socket = stream.peer_addr()?;
        stream.set_nodelay(true)?;
        Ok(Self::wrap(stream, socket))
    }

    /// Create an [`IoTransport`] from a [`SocketAddr`].
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    pub async fn connect_to(socket: SocketAddr) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(socket).await?;
        stream.set_nodelay(true)?;
        Ok(Self::wrap(stream, socket))
    }
}
