//! TODO

use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use hickory_resolver::{
    IntoName, ResolveError,
    config::{ResolverConfig, ResolverOpts},
    lookup::{SrvLookup, TxtLookup},
    lookup_ip::LookupIp,
    proto::runtime::Executor,
    system_conf::read_system_conf,
};

use super::{FroglightInnerResolver, ResolverConnectionProvider};
use crate::prelude::{ClientConnection, Handshake, ValidState};

/// A resolver for server addresses.
///
/// This resolver is cheaply cloneable and can be shared between threads.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::resource::Resource))]
pub struct FroglightResolver {
    resolver: Arc<FroglightInnerResolver>,
}

impl FroglightResolver {
    /// Create a new [`FroglightResolver`].
    ///
    /// See [`ResolverConfig`] on how to configure the resolver.
    #[must_use]
    pub fn new(config: ResolverConfig, options: ResolverOpts) -> Self {
        let mut resolver =
            FroglightInnerResolver::builder_with_config(config, ResolverConnectionProvider::new());
        *resolver.options_mut() = options;
        Self { resolver: Arc::new(resolver.build()) }
    }

    /// Create a new [`FroglightResolver`] from the system configuration.
    ///
    /// # Errors
    /// Returns an error if the system configuration could not be read.
    pub fn system_config() -> Result<Self, std::io::Error> {
        let (config, options) = read_system_conf()?;
        Ok(Self::new(config, options))
    }

    /// Lookup an IP address for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::lookup_ip`] for more information.
    pub fn lookup_ip<'a, N: IntoName + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<LookupIp, ResolveError>> + 'a {
        self.resolver.lookup_ip(host)
    }

    /// Lookup SRV records for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::srv_lookup`] for more
    /// information.
    pub fn lookup_srv<'a, N: IntoName + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<SrvLookup, ResolveError>> + 'a {
        self.resolver.srv_lookup::<N>(host)
    }

    /// Lookup TXT records for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::txt_lookup`] for more
    /// information.
    pub fn lookup_txt<'a, N: IntoName + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<TxtLookup, ResolveError>> + 'a {
        self.resolver.txt_lookup::<N>(host)
    }
}

impl FroglightResolver {
    const DEFAULT_PORT: u16 = 25565;
    const SRV_PREFIX: &'static str = "_minecraft._tcp.";

    /// Resolve a server's IP and port from a given address and connect to it.
    ///
    /// # Errors
    /// Returns an error if the address could not be resolved or
    /// from the last connection attempt.
    pub async fn connect_to_server<V: ValidState<Handshake>>(
        &self,
        mut address: &str,
    ) -> Result<ClientConnection<V, Handshake>, std::io::Error> {
        // Trim any whitespace.
        address = address.trim();

        // If the address has a path, remove it.
        if let Some(idx) = address.find(['/', '?']) {
            address = &address[..idx];
        }

        // If the address is a socket address, connect to it.
        if let Ok(socket) = address.parse::<SocketAddr>() {
            #[cfg(feature = "bevy")]
            bevy_log::debug!("Connecting to \"{address}\" at \"{socket}\"");
            return ClientConnection::connect_to(address, socket).await;
        }

        // Parse the port from the address, if it exists.
        let mut port = None;
        if let Some(idx) = address.rfind(':') {
            // Skip if there are two colons in a row, that's not a port.
            if address.as_bytes()[idx - 1] != b':' {
                if let Ok(p) = address[idx + 1..].parse::<u16>() {
                    port = Some(p);
                    address = &address[..idx];
                }
            }
        }

        // If the address is an IP, connect to it.
        if let Ok(ip) = address.trim_start_matches('[').trim_end_matches(']').parse::<IpAddr>() {
            let socket = SocketAddr::new(ip, port.unwrap_or(Self::DEFAULT_PORT));
            #[cfg(feature = "bevy")]
            bevy_log::debug!("Connecting to \"{address}:{}\" at \"{socket}\"", socket.port());
            return ClientConnection::connect_to(address, socket).await;
        }

        // Lookup the domain's SRV records with the `_minecraft._tcp.` prefix.
        if address.starts_with(char::is_alphanumeric) {
            if let Ok(srv) = self.lookup_srv(format!("{}{address}", Self::SRV_PREFIX)).await {
                // Try to connect to every response, returning the first successful connection.
                for srv in srv {
                    let (srv_address, srv_port) =
                        (srv.target().to_string(), port.unwrap_or(srv.port()));
                    let lookup = self.lookup_ip(srv_address.trim_end_matches('.')).await?;

                    #[cfg(feature = "bevy")]
                    bevy_log::debug!(
                        "Connecting to \"{}:{srv_port}\" instead of \"{address}:{}\"",
                        srv_address.trim_end_matches('.'),
                        port.unwrap_or(Self::DEFAULT_PORT)
                    );

                    if let Ok(conn) = self.join_lookup(&srv_address, lookup, srv_port).await {
                        return Ok(conn);
                    }
                }
            }
        }

        // Lookup the domain's A/AAAA records.
        if let Ok(lookup) = self.lookup_ip(address.trim_end_matches('.')).await {
            match self.join_lookup(address, lookup, port.unwrap_or(Self::DEFAULT_PORT)).await {
                Ok(conn) => return Ok(conn),
                Err(Some(err)) => return Err(err),
                Err(None) => {}
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find any servers at given address",
        ))
    }

    /// Attempt to connect to every IP address in the lookup,
    /// returning the first successful connection.
    ///
    /// # Errors
    /// Only returns the last error if all connections fail.
    async fn join_lookup<V: ValidState<Handshake>>(
        &self,
        mut address: &str,
        lookup: LookupIp,
        port: u16,
    ) -> Result<ClientConnection<V, Handshake>, Option<std::io::Error>> {
        address = address.trim_end_matches('.');
        let mut error = None;

        for ip in lookup {
            let socket = SocketAddr::new(ip, port);
            #[cfg(feature = "bevy")]
            bevy_log::debug!("Connecting to \"{address}:{}\" at \"{socket}\"", socket.port());
            match ClientConnection::connect_to(address, socket).await {
                Ok(conn) => return Ok(conn),
                Err(err) => error = Some(err),
            }
        }

        Err(error)
    }
}

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for FroglightResolver {
    fn from_world(_: &mut bevy_ecs::world::World) -> Self {
        Self::system_config().unwrap_or_else(|err| {
            bevy_log::error!("Failed to load system resolver, defaulting to cloudflare: {err}");
            Self::new(ResolverConfig::cloudflare(), ResolverOpts::default())
        })
    }
}

#[test]
#[expect(clippy::too_many_lines)]
fn resolver() {
    use froglight_common::version::Version;
    use froglight_io::prelude::*;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    struct Test;
    impl Version for Test {
        const PROTOCOL_ID: u32 = 0;
        const RESOURCE_VERSION: u32 = 0;
    }

    enum ClientboundTest {}
    impl FrogRead for ClientboundTest {
        fn frog_read(_: &mut impl std::io::Read) -> Result<Self, ReadError> { todo!() }
    }
    impl FrogWrite for ClientboundTest {
        fn frog_write(&self, _: &mut impl std::io::Write) -> Result<usize, WriteError> { todo!() }

        fn frog_len(&self) -> usize { todo!() }
    }

    enum ServerboundTest {}
    impl FrogRead for ServerboundTest {
        fn frog_read(_: &mut impl std::io::Read) -> Result<Self, ReadError> { todo!() }
    }
    impl FrogWrite for ServerboundTest {
        fn frog_write(&self, _: &mut impl std::io::Write) -> Result<usize, WriteError> { todo!() }

        fn frog_len(&self) -> usize { todo!() }
    }

    impl ValidState<Handshake> for Test {
        type Clientbound = ClientboundTest;
        type Serverbound = ServerboundTest;
    }

    let resolver = FroglightResolver::new(ResolverConfig::cloudflare(), ResolverOpts::default());

    #[cfg(feature = "bevy")]
    {
        use futures_lite::StreamExt;

        async fn bind_to(bind: &str) {
            if let Ok(listener) = async_net::TcpListener::bind(bind).await {
                while let Some(Ok(stream)) = listener.incoming().next().await {
                    bevy_log::trace!(
                        "Accepted from \"{}\" at \"{bind}\" ",
                        stream.peer_addr().unwrap()
                    );
                }
            } else {
                bevy_log::error!("Failed to bind to \"{bind}\"!");
            }
        }

        let _ =
            bevy_log::tracing_subscriber::fmt().with_env_filter("froglight=debug,off").try_init();

        bevy_log::debug!("Starting temporary listeners ...");
        let taskpool = bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);
        for socket in ["127.0.0.1:25565", "127.0.0.1:25566", "[::1]:25565", "[::1]:25566"] {
            taskpool.spawn(bind_to(socket)).detach();
        }
    }

    // // `mc.hypixel.net` should resolve to `mc.hypixel.net` and port `25565`.
    // {
    //     let mut connection =
    //         futures_lite::future::block_on(resolver.connect_to_server::<Test>("
    // mc.hypixel.net"))             .unwrap();
    //     assert_eq!(connection.address(), "mc.hypixel.net");
    //     assert_eq!(connection.as_raw().as_stream().peer_addr().unwrap().port(),
    // 25565); }

    // // `hypixel.net` should resolve to `mc.hypixel.net` and port `25565`.
    // {
    //     let mut connection =
    //         futures_lite::future::block_on(resolver.connect_to_server::<Test>("
    // hypixel.net"))             .unwrap();
    //     assert_eq!(connection.address(), "mc.hypixel.net");
    //     assert_eq!(connection.as_raw().as_stream().peer_addr().unwrap().port(),
    // 25565); }

    // `localhost` should resolve to `127.0.0.1` and port `25565`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("localhost"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "localhost");
                assert_eq!(conn.peer_addr().unwrap().port(), 25565);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `localhost:25566` should resolve to `127.0.0.1` and port `25566`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("localhost:25566"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "localhost");
                assert_eq!(conn.peer_addr().unwrap().port(), 25566);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `127.0.0.1` should resolve to `127.0.0.1` and port `25565`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("127.0.0.1"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "127.0.0.1");
                assert_eq!(conn.peer_addr().unwrap().port(), 25565);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `127.0.0.1/test` should resolve to `127.0.0.1` and port `25565`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("127.0.0.1/test"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "127.0.0.1");
                assert_eq!(conn.peer_addr().unwrap().port(), 25565);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `::1` should resolve to `[::1]` and port `25565`.
    {
        let connection = futures_lite::future::block_on(resolver.connect_to_server::<Test>("::1"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "::1");
                assert_eq!(conn.peer_addr().unwrap().port(), 25565);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `::1:25566` should resolve to `[::1]` and port `25566`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("::1:25566"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "::1");
                assert_eq!(conn.peer_addr().unwrap().port(), 25566);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `[::1]` should resolve to `[::1]` and port `25565`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("[::1]"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "[::1]");
                assert_eq!(conn.peer_addr().unwrap().port(), 25565);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }

    // `[::1]:25566` should resolve to `[::1]` and port `25566`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("[::1]:25566"));
        match connection {
            Ok(conn) => {
                assert_eq!(conn.address(), "[::1]:25566");
                assert_eq!(conn.peer_addr().unwrap().port(), 25566);
            }
            Err(err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::ConnectionRefused);
            }
        }
    }
}
