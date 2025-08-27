#![expect(unreachable_pub, reason = "These types shouldn't be public...")]

use alloc::boxed::Box;
use core::{
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use std::io::{Error, ErrorKind};

use async_io::Timer;
use async_net::{TcpStream, UdpSocket};
use futures_lite::{AsyncRead, AsyncWrite, FutureExt, future::or};
use hickory_resolver::{
    config::{NameServerConfig, ResolverOpts},
    name_server::{ConnectionProvider, GenericConnector},
    proto::{
        ProtoError,
        runtime::{Executor, RuntimeProvider, Spawn, Time},
        tcp::DnsTcpStream,
        udp::DnsUdpSocket,
    },
};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

/// A [`ConnectionProvider`] implementation using [`async-io`] and
/// [`async-net`].
#[derive(Default, Clone)]
pub struct ResolverProvider {
    pub(super) conn: GenericConnector<ResolverRuntime>,
}

impl ConnectionProvider for ResolverProvider {
    type Conn = <GenericConnector<ResolverRuntime> as ConnectionProvider>::Conn;
    type FutureConn = <GenericConnector<ResolverRuntime> as ConnectionProvider>::FutureConn;
    type RuntimeProvider = ResolverRuntime;

    fn new_connection(
        &self,
        config: &NameServerConfig,
        options: &ResolverOpts,
    ) -> Result<Self::FutureConn, Error> {
        self.conn.new_connection(config, options)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResolverRuntime;

impl RuntimeProvider for ResolverRuntime {
    type Handle = RuntimeHandle;
    type Tcp = RuntimeTcp;
    type Timer = RuntimeTimer;
    type Udp = RuntimeUdp;

    fn create_handle(&self) -> Self::Handle { RuntimeHandle }

    fn connect_tcp(
        &self,
        server_addr: SocketAddr,
        bind_addr: Option<SocketAddr>,
        timeout: Option<Duration>,
    ) -> Pin<Box<dyn Send + Future<Output = Result<Self::Tcp, Error>>>> {
        // Create a future to connect to the server
        let connect = async move {
            #[cfg(feature = "tracing")]
            tracing::debug!("Connecting: {server_addr} (local: {bind_addr:?})");

            let socket = match server_addr {
                SocketAddr::V4(..) => Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)),
                SocketAddr::V6(..) => Socket::new(Domain::IPV6, Type::STREAM, Some(Protocol::TCP)),
            }?;

            if let Some(bind_addr) = bind_addr {
                socket.bind(&SockAddr::from(bind_addr))?;
            }

            socket.set_tcp_nodelay(true)?;
            socket.set_nonblocking(true)?;
            socket.connect(&SockAddr::from(server_addr))?;

            Ok(RuntimeTcp(TcpStream::try_from(std::net::TcpStream::from(socket))?))
        };

        match timeout {
            // If no timeout is specified, return the `connect` future directly
            None => Box::pin(connect),
            // Otherwise, return a future that times out after the specified duration
            Some(timeout) => {
                Box::pin(async move { RuntimeTimer::timeout(timeout, connect).await.flatten() })
            }
        }
    }

    #[allow(clippy::used_underscore_binding, reason = "May or may not be used")]
    fn bind_udp(
        &self,
        local_addr: SocketAddr,
        _server_addr: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = Result<Self::Udp, Error>>>> {
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            tracing::debug!("Binding: {local_addr} (remote: {_server_addr})");

            let socket = match local_addr {
                SocketAddr::V4(..) => Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)),
                SocketAddr::V6(..) => Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP)),
            }?;

            // if let Some(server_addr) = server_addr {}

            socket.set_nonblocking(true)?;
            socket.bind(&SockAddr::from(local_addr))?;

            Ok(RuntimeUdp(UdpSocket::try_from(std::net::UdpSocket::from(socket))?))
        })
    }
}

// -------------------------------------------------------------------------------------------------

pub struct RuntimeExecutor;

impl Executor for RuntimeExecutor {
    fn new() -> Self { Self }

    #[cfg(not(feature = "bevy"))]
    fn block_on<F: Future>(&mut self, future: F) -> F::Output { async_io::block_on(future) }

    #[cfg(feature = "bevy")]
    fn block_on<F: Future>(&mut self, future: F) -> F::Output { bevy_tasks::block_on(future) }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeHandle;

impl Spawn for RuntimeHandle {
    #[cfg(not(feature = "bevy"))]
    fn spawn_bg<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), ProtoError>> + Send + 'static,
    {
        blocking::unblock(|| RuntimeExecutor.block_on(future)).detach();
    }

    #[cfg(feature = "bevy")]
    fn spawn_bg<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), ProtoError>> + Send + 'static,
    {
        match bevy_tasks::IoTaskPool::try_get() {
            Some(pool) => {
                pool.spawn(future).detach();
            }
            None => {
                blocking::unblock(|| RuntimeExecutor.block_on(future)).detach();
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeTimer;

#[async_trait::async_trait]
impl Time for RuntimeTimer {
    async fn delay_for(duration: Duration) { Timer::after(duration).await; }

    async fn timeout<F: 'static + Future + Send>(
        duration: Duration,
        future: F,
    ) -> Result<F::Output, std::io::Error> {
        or(async { Ok(future.await) }, async {
            Self::delay_for(duration).await;
            Err(Error::new(ErrorKind::TimedOut, "resolver future timed out"))
        })
        .await
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RuntimeTcp(TcpStream);

impl DnsTcpStream for RuntimeTcp {
    type Time = RuntimeTimer;
}

impl AsyncRead for RuntimeTcp {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        <TcpStream as AsyncRead>::poll_read(Pin::new(&mut self.get_mut().0), cx, buf)
    }
}
impl AsyncWrite for RuntimeTcp {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        <TcpStream as AsyncWrite>::poll_write(Pin::new(&mut self.get_mut().0), cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        <TcpStream as AsyncWrite>::poll_flush(Pin::new(&mut self.get_mut().0), cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        <TcpStream as AsyncWrite>::poll_close(Pin::new(&mut self.get_mut().0), cx)
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RuntimeUdp(UdpSocket);

#[async_trait::async_trait]
impl DnsUdpSocket for RuntimeUdp {
    type Time = RuntimeTimer;

    fn poll_recv_from(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<(usize, SocketAddr), Error>> {
        Box::pin(self.0.recv_from(buf)).poll(cx)
    }

    fn poll_send_to(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
        target: SocketAddr,
    ) -> Poll<Result<usize, Error>> {
        Box::pin(self.0.send_to(buf, target)).poll(cx)
    }
}
