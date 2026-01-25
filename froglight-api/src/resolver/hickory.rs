//! TODO

use std::{
    fmt::{self, Debug, Display},
    io,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use async_io::Timer;
use async_net::{TcpStream, UdpSocket};
use async_trait::async_trait;
use futures_lite::{AsyncRead, AsyncWrite, future::or};
pub use hickory_resolver::{
    Resolver as HickoryResolver,
    config::{ResolverConfig as HickoryConfig, ResolverOpts as HickoryOpts},
};
use hickory_resolver::{
    name_server::GenericConnector,
    proto::{
        ProtoError,
        runtime::{Executor, RuntimeProvider, Spawn, Time},
        tcp::DnsTcpStream,
        udp::DnsUdpSocket,
    },
};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

/// A [`NetworkResolver`](crate::resolver::NetworkResolver) implementation using
/// [`hickory_resolver`].
#[derive(Clone)]
pub struct Resolver(HickoryResolver<GenericConnector<DnsExecutor>>);

impl Default for Resolver {
    fn default() -> Self { Self::new_cloudflare() }
}

impl Resolver {
    /// Creates a new [`Resolver`] using the default configuration.
    ///
    /// Uses Cloudflare's public DNS servers, see [`HickoryConfig::cloudflare`]
    /// for more details.
    ///
    /// See [`Resolver::new_from`] to create a resolver with a custom
    /// configuration.
    #[must_use]
    pub fn new_cloudflare() -> Self { Self::new_with_config(HickoryConfig::cloudflare(), None) }

    /// Creates a new [`Resolver`].
    ///
    /// Uses Google's public DNS servers, see [`HickoryConfig::google`]
    /// for more details.
    ///
    /// See [`Resolver::new_with_config`] to create a resolver with a custom
    /// configuration.
    #[must_use]
    pub fn new_google() -> Self { Self::new_with_config(HickoryConfig::google(), None) }

    /// Creates a new [`Resolver`].
    ///
    /// Uses Quad9's public DNS servers, see [`HickoryConfig::quad9`]
    /// for more details.
    ///
    /// See [`Resolver::new_with_config`] to create a resolver with a custom
    /// configuration.
    #[must_use]
    pub fn new_quad9() -> Self { Self::new_with_config(HickoryConfig::quad9(), None) }

    /// Creates a new [`Resolver`] from a [`HickoryConfig`] and optional
    /// [`HickoryOpts`].
    #[must_use]
    pub fn new_with_config(config: HickoryConfig, opts: Option<HickoryOpts>) -> Self {
        let mut builder =
            HickoryResolver::builder_with_config(config, GenericConnector::new(DnsExecutor));
        if let Some(opts) = opts {
            builder = builder.with_options(opts);
        }
        Self::new_from_resolver(builder.build())
    }

    /// Creates a new [`Resolver`] from a [`Resolver`].
    #[inline]
    #[must_use]
    pub const fn new_from_resolver(
        resolver: HickoryResolver<GenericConnector<DnsExecutor>>,
    ) -> Self {
        Self(resolver)
    }

    /// Returns a reference to the inner [`Resolver`].
    #[inline]
    #[must_use]
    pub const fn as_resolver(&self) -> &HickoryResolver<GenericConnector<DnsExecutor>> { &self.0 }

    /// Returns a mutable reference to the inner [`Resolver`].
    #[inline]
    #[must_use]
    pub const fn as_resolver_mut(&mut self) -> &mut HickoryResolver<GenericConnector<DnsExecutor>> {
        &mut self.0
    }
}

impl Debug for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resolver").finish_non_exhaustive()
    }
}

impl Deref for Resolver {
    type Target = HickoryResolver<GenericConnector<DnsExecutor>>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Resolver {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

/// An [`Executor`], [`Spawn`], and [`RuntimeProvider`] for DNS operations.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DnsExecutor;

impl Executor for DnsExecutor {
    #[inline]
    fn new() -> Self { Self }

    #[inline]
    fn block_on<F: Future>(&mut self, future: F) -> F::Output { async_io::block_on(future) }
}

impl Spawn for DnsExecutor {
    fn spawn_bg<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), ProtoError>> + Send + 'static,
    {
        #[cfg(feature = "bevy")]
        bevy_tasks::IoTaskPool::get().spawn(future).detach();
        #[cfg(not(feature = "bevy"))]
        blocking::unblock(|| async_io::block_on(future)).detach();
    }
}

impl RuntimeProvider for DnsExecutor {
    type Handle = DnsExecutor;
    type Tcp = TcpStreamWrap;
    type Timer = DnsTimer;
    type Udp = UdpSocketWrap;

    fn create_handle(&self) -> Self::Handle { Self }

    fn connect_tcp(
        &self,
        server_addr: SocketAddr,
        bind_addr: Option<SocketAddr>,
        timeout: Option<Duration>,
    ) -> Pin<Box<dyn Send + Future<Output = io::Result<Self::Tcp>>>> {
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            if let Some(bind_addr) = bind_addr.as_ref() {
                tracing::trace!(target: "froglight_api::resolver", "Creating TCP socket ({bind_addr} -> {server_addr})");
            } else {
                tracing::trace!(target: "froglight_api::resolver", "Creating TCP socket (None -> {server_addr})");
            }

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

            Ok(TcpStreamWrap(TcpStream::try_from(std::net::TcpStream::from(socket))?))
        })
    }

    fn bind_udp(
        &self,
        local_addr: SocketAddr,
        _: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = io::Result<Self::Udp>>>> {
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_api::resolver", "Creating UDP socket ({local_addr})");

            let socket =
                Socket::new(Domain::for_address(local_addr), Type::DGRAM, Some(Protocol::UDP))?;

            socket.set_nonblocking(true)?;
            socket.bind(&SockAddr::from(local_addr))?;

            Ok(UdpSocketWrap(UdpSocket::try_from(std::net::UdpSocket::from(socket))?))
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`TcpStream`] to implement [`DnsTcpStream`].
#[repr(transparent)]
pub struct TcpStreamWrap(pub TcpStream);

/// A wrapper around [`UdpSocket`] to implement [`DnsUdpSocket`].
#[repr(transparent)]
pub struct UdpSocketWrap(pub UdpSocket);

impl DnsTcpStream for TcpStreamWrap {
    type Time = DnsTimer;
}

impl AsyncRead for TcpStreamWrap {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        <TcpStream as AsyncRead>::poll_read(Pin::new(&mut self.get_mut().0), cx, buf)
    }

    #[inline]
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> Poll<io::Result<usize>> {
        <TcpStream as AsyncRead>::poll_read_vectored(Pin::new(&mut self.get_mut().0), cx, bufs)
    }
}

impl AsyncWrite for TcpStreamWrap {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        <TcpStream as AsyncWrite>::poll_write(Pin::new(&mut self.get_mut().0), cx, buf)
    }

    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        <TcpStream as AsyncWrite>::poll_write_vectored(Pin::new(&mut self.get_mut().0), cx, bufs)
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

// TODO: Figure out how this is supposed to work?
// `pin!(fut)` and `fut.poll(cx)` doesn't wake the task...
#[async_trait]
impl DnsUdpSocket for UdpSocketWrap {
    type Time = DnsTimer;

    fn poll_recv_from(
        &self,
        _: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        Poll::Ready(async_io::block_on(UdpSocket::recv_from(&self.0, buf)))
    }

    fn poll_send_to(
        &self,
        _: &mut Context<'_>,
        buf: &[u8],
        target: SocketAddr,
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(async_io::block_on(UdpSocket::send_to(&self.0, buf, target)))
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
    #[inline]
    async fn delay_for(duration: Duration) { Timer::after(duration).await; }

    #[inline]
    async fn timeout<F: Future + Send + 'static>(
        duration: Duration,
        future: F,
    ) -> Result<F::Output, io::Error> {
        or(async { Ok(future.await) }, async {
            Timer::after(duration).await;
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
