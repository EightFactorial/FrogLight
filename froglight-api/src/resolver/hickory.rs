//! TODO

use std::{
    fmt::{self, Debug, Display},
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use async_io::Timer;
use async_net::{TcpStream, UdpSocket};
use async_trait::async_trait;
use futures_lite::{AsyncRead, AsyncWrite, future::or, pin};
pub use hickory_resolver::{Resolver as HickoryResolver, config::ResolverConfig};
use hickory_resolver::{
    name_server::GenericConnector,
    proto::{
        ProtoError,
        runtime::{RuntimeProvider, Spawn, Time},
        tcp::DnsTcpStream,
        udp::DnsUdpSocket,
    },
};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

/// A DNS resolver implementation using [`hickory_resolver`].
#[derive(Clone)]
pub struct Resolver(HickoryResolver<GenericConnector<DnsProvider>>);

impl Resolver {
    /// Creates a new [`DnsResolver`].
    ///
    /// Uses the default configuration and Cloudflare's public DNS servers.
    ///
    /// See [`DnsResolver::new_from`] to create a resolver with a custom
    /// configuration.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Creates a new [`DnsResolver`] from a [`Resolver`].
    #[inline]
    #[must_use]
    pub const fn new_from(resolver: HickoryResolver<GenericConnector<DnsProvider>>) -> Self {
        Self(resolver)
    }

    /// Returns a reference to the inner [`Resolver`].
    #[inline]
    #[must_use]
    pub const fn as_resolver(&self) -> &HickoryResolver<GenericConnector<DnsProvider>> { &self.0 }

    /// Returns a mutable reference to the inner [`Resolver`].
    #[inline]
    #[must_use]
    pub const fn as_resolver_mut(&mut self) -> &mut HickoryResolver<GenericConnector<DnsProvider>> {
        &mut self.0
    }
}

impl Debug for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DnsResolver").finish_non_exhaustive()
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new_from(
            HickoryResolver::builder_with_config(
                ResolverConfig::cloudflare(),
                GenericConnector::new(DnsProvider),
            )
            .build(),
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`Spawn`] and [`RuntimeProvider`] for DNS operations.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DnsProvider;

impl Spawn for DnsProvider {
    #[cfg(not(feature = "bevy"))]
    fn spawn_bg<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), ProtoError>> + Send + 'static,
    {
        blocking::unblock(|| async_io::block_on(future)).detach();
    }

    #[cfg(feature = "bevy")]
    fn spawn_bg<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), ProtoError>> + Send + 'static,
    {
        bevy_tasks::IoTaskPool::get().spawn(future).detach();
    }
}

impl RuntimeProvider for DnsProvider {
    type Handle = DnsProvider;
    type Tcp = TcpWrap;
    type Timer = DnsTimer;
    type Udp = UdpWrap;

    fn create_handle(&self) -> Self::Handle { Self }

    fn connect_tcp(
        &self,
        server_addr: SocketAddr,
        bind_addr: Option<SocketAddr>,
        timeout: Option<Duration>,
    ) -> Pin<Box<dyn Send + Future<Output = io::Result<Self::Tcp>>>> {
        Box::pin(async move {
            let socket =
                Socket::new(Domain::for_address(server_addr), Type::STREAM, Some(Protocol::TCP))?;

            socket.set_nonblocking(true)?;
            if let Some(bind_addr) = bind_addr {
                socket.bind(&SockAddr::from(bind_addr))?;
            }

            if let Some(timeout) = timeout {
                socket.connect_timeout(&SockAddr::from(server_addr), timeout)?;
            } else {
                socket.connect(&SockAddr::from(server_addr))?;
            }

            Ok(TcpWrap(TcpStream::try_from(std::net::TcpStream::from(socket))?))
        })
    }

    fn bind_udp(
        &self,
        local_addr: SocketAddr,
        server_addr: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = io::Result<Self::Udp>>>> {
        Box::pin(async move {
            let socket =
                Socket::new(Domain::for_address(local_addr), Type::DGRAM, Some(Protocol::UDP))?;

            socket.set_nonblocking(true)?;
            socket.bind(&SockAddr::from(local_addr))?;

            socket.connect(&SockAddr::from(server_addr))?;
            Ok(UdpWrap(UdpSocket::try_from(std::net::UdpSocket::from(socket))?))
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`TcpStream`] to implement [`DnsTcpStream`].
#[repr(transparent)]
pub struct TcpWrap(pub TcpStream);

/// A wrapper around [`UdpSocket`] to implement [`DnsUdpSocket`].
#[repr(transparent)]
pub struct UdpWrap(pub UdpSocket);

impl DnsTcpStream for TcpWrap {
    type Time = DnsTimer;
}

impl AsyncRead for TcpWrap {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        <TcpStream as AsyncRead>::poll_read(Pin::new(&mut self.get_mut().0), cx, buf)
    }
}

impl AsyncWrite for TcpWrap {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        <TcpStream as AsyncWrite>::poll_write(Pin::new(&mut self.get_mut().0), cx, buf)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        <TcpStream as AsyncWrite>::poll_flush(Pin::new(&mut self.get_mut().0), cx)
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        <TcpStream as AsyncWrite>::poll_close(Pin::new(&mut self.get_mut().0), cx)
    }
}

#[async_trait]
impl DnsUdpSocket for UdpWrap {
    type Time = DnsTimer;

    fn poll_recv_from(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        let fut = UdpSocket::recv_from(&self.0, buf);
        pin!(fut);

        fut.poll(cx)
    }

    fn poll_send_to(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
        target: SocketAddr,
    ) -> Poll<io::Result<usize>> {
        let fut = UdpSocket::send_to(&self.0, buf, target);
        pin!(fut);

        fut.poll(cx)
    }
}

// -------------------------------------------------------------------------------------------------

/// An implementation of [`Time`] using [`async-io::Timer`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DnsTimer;

/// An error indicating a timeout occurred.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeoutError;

#[async_trait]
impl Time for DnsTimer {
    async fn delay_for(duration: Duration) { Timer::after(duration).await; }

    async fn timeout<F: 'static + Future + Send>(
        duration: Duration,
        future: F,
    ) -> Result<F::Output, io::Error> {
        or(async { Ok(future.await) }, async {
            Self::delay_for(duration).await;
            Err(io::Error::new(io::ErrorKind::TimedOut, TimeoutError))
        })
        .await
    }
}

impl std::error::Error for TimeoutError {}
impl Display for TimeoutError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Debug::fmt(self, f) }
}
