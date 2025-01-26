use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use async_net::{TcpStream, UdpSocket};
use futures_lite::{AsyncRead, AsyncWrite, FutureExt};
use hickory_resolver::{
    config::{NameServerConfig, ResolverOpts},
    name_server::{ConnectionProvider, GenericConnector, RuntimeProvider, Spawn},
    proto::{error::ProtoError, tcp::DnsTcpStream, udp::DnsUdpSocket, Executor, Time},
    AsyncResolver,
};

/// A DNS resolver for server addresses.
///
/// See [`AsyncResolver`] for more information.
pub(super) type FroglightResolver = AsyncResolver<ResolverConnectionProvider>;

#[derive(Clone)]
pub(super) struct ResolverRuntimeProvider;

impl Executor for ResolverRuntimeProvider {
    fn new() -> Self { Self }

    fn block_on<F: Future>(&mut self, future: F) -> F::Output {
        #[cfg(feature = "bevy")]
        {
            bevy_tasks::block_on(future)
        }

        #[cfg(not(feature = "bevy"))]
        {
            async_io::block_on(future)
        }
    }
}

impl RuntimeProvider for ResolverRuntimeProvider {
    type Handle = ResolverRuntimeHandle;
    type Timer = ResolverTime;
    type Udp = ResolverUdpSocket;
    type Tcp = ResolverTcpConnection;

    fn create_handle(&self) -> Self::Handle { ResolverRuntimeHandle }

    fn connect_tcp(
        &self,
        server_addr: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = std::io::Result<Self::Tcp>>>> {
        Box::pin(async move {
            let stream = TcpStream::connect(server_addr).await?;
            stream.set_nodelay(true)?;
            Ok(ResolverTcpConnection(stream))
        })
    }

    fn bind_udp(
        &self,
        local_addr: SocketAddr,
        _server_addr: SocketAddr,
    ) -> Pin<Box<dyn Send + Future<Output = std::io::Result<Self::Udp>>>> {
        Box::pin(async move { Ok(ResolverUdpSocket(UdpSocket::bind(local_addr).await?)) })
    }
}

#[derive(Clone)]
pub(super) struct ResolverConnectionProvider {
    runtime: ResolverRuntimeProvider,
    connection: GenericConnector<ResolverRuntimeProvider>,
}

impl Executor for ResolverConnectionProvider {
    fn new() -> Self {
        Self {
            runtime: ResolverRuntimeProvider,
            connection: GenericConnector::new(ResolverRuntimeProvider),
        }
    }

    fn block_on<F: Future>(&mut self, future: F) -> F::Output { self.runtime.block_on(future) }
}

impl ConnectionProvider for ResolverConnectionProvider {
    type Conn = <GenericConnector<ResolverRuntimeProvider> as ConnectionProvider>::Conn;
    type FutureConn = <GenericConnector<ResolverRuntimeProvider> as ConnectionProvider>::FutureConn;
    type RuntimeProvider = ResolverRuntimeProvider;

    fn new_connection(
        &self,
        config: &NameServerConfig,
        options: &ResolverOpts,
    ) -> Self::FutureConn {
        self.connection.new_connection(config, options)
    }
}

#[derive(Clone)]
pub(super) struct ResolverRuntimeHandle;

impl Spawn for ResolverRuntimeHandle {
    fn spawn_bg<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), ProtoError>> + Send + 'static,
    {
        #[cfg(feature = "bevy")]
        {
            bevy_tasks::IoTaskPool::get().spawn(future).detach();
        }

        #[cfg(not(feature = "bevy"))]
        {
            let _ = async_io::block_on(future);
        }
    }
}

pub(super) struct ResolverTime;

impl Time for ResolverTime {
    fn delay_for<'a>(duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            async_io::Timer::after(duration).await;
        })
    }

    fn timeout<'a, F>(
        duration: Duration,
        future: F,
    ) -> Pin<Box<dyn Future<Output = Result<F::Output, std::io::Error>> + Send + 'a>>
    where
        F: 'static + Future + Send,
    {
        Box::pin(async move {
            futures_lite::future::or::<Result<F::Output, std::io::Error>, _, _>(
                async { Ok(future.await) },
                async {
                    async_io::Timer::after(duration).await;
                    Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "future timed out"))
                },
            )
            .await
        })
    }
}

pub(super) struct ResolverUdpSocket(UdpSocket);

impl DnsUdpSocket for ResolverUdpSocket {
    type Time = ResolverTime;

    fn poll_recv_from(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<(usize, SocketAddr)>> {
        Box::pin(self.0.recv_from(buf)).poll(cx)
    }

    fn recv_from<'a, 'b, 'c>(
        &'a self,
        buf: &'b mut [u8],
    ) -> Pin<Box<dyn Future<Output = std::io::Result<(usize, SocketAddr)>> + Send + 'c>>
    where
        'a: 'c,
        'b: 'c,
        Self: Sync + 'c,
    {
        Box::pin(self.0.recv_from(buf))
    }

    fn poll_send_to(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
        target: SocketAddr,
    ) -> Poll<std::io::Result<usize>> {
        Box::pin(self.0.send_to(buf, target)).poll(cx)
    }

    fn send_to<'a, 'b, 'c>(
        &'a self,
        buf: &'b [u8],
        target: SocketAddr,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<usize>> + Send + 'c>>
    where
        'a: 'c,
        'b: 'c,
        Self: Sync + 'c,
    {
        Box::pin(self.0.send_to(buf, target))
    }
}

pub(super) struct ResolverTcpConnection(TcpStream);

impl DnsTcpStream for ResolverTcpConnection {
    type Time = ResolverTime;
}

impl AsyncRead for ResolverTcpConnection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for ResolverTcpConnection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}
