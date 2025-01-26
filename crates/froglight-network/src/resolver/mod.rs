//! TODO

use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

pub use hickory_resolver::{
    config::{
        LookupIpStrategy, NameServerConfig, NameServerConfigGroup, ResolverConfig, ResolverOpts,
    },
    lookup::{Lookup, SrvLookup, TxtLookup},
    lookup_ip::LookupIp,
    IntoName, Name, TryParseIp,
};
use hickory_resolver::{error::ResolveError, proto::Executor};

mod provider;
use provider::{FroglightResolver, ResolverConnectionProvider};

use crate::prelude::{ClientConnection, Handshake, ValidState};

/// A resolver for server addresses.
///
/// This resolver is cheaply cloneable and can be shared between threads.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::system::Resource))]
pub struct ServerResolver {
    resolver: Arc<FroglightResolver>,
}

impl ServerResolver {
    /// Create a new [`ServerResolver`].
    #[must_use]
    pub fn new(config: ResolverConfig, options: ResolverOpts) -> Self {
        let resolver = FroglightResolver::new(config, options, ResolverConnectionProvider::new());

        Self { resolver: Arc::new(resolver) }
    }

    /// Create a new [`ServerResolver`] from the system configuration.
    ///
    /// # Errors
    /// Returns an error if the system configuration could not be read.
    pub fn system_config() -> Result<Self, std::io::Error> {
        let resolver = FroglightResolver::from_system_conf(ResolverConnectionProvider::new())?;

        Ok(Self { resolver: Arc::new(resolver) })
    }

    /// Lookup an IP address for a given hostname.
    ///
    /// See [`hickory_resolver::AsyncResolver::lookup_ip`] for more information.
    pub fn lookup_ip<'a, N: IntoName + TryParseIp + 'a>(
        &'a self,
        host: N,
    ) -> impl Future<Output = Result<LookupIp, ResolveError>> + 'a {
        self.resolver.lookup_ip::<N>(host)
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

impl ServerResolver {
    const SRV_PREFIX: &'static str = "_minecraft._tcp.";
    const DEFAULT_PORT: u16 = 25565;

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

        // Parse the port from the address, if it exists.
        let mut port = None;
        if let Some(idx) = address.rfind(':') {
            if let Ok(p) = address[idx + 1..].parse::<u16>() {
                port = Some(p);
                address = &address[..idx];
            }
        }

        // If the address is an IP, connect to it.
        if let Ok(ip) = address.parse::<IpAddr>() {
            let socket = SocketAddr::new(ip, port.unwrap_or(Self::DEFAULT_PORT));
            return ClientConnection::connect_to(address, socket).await;
        }

        // Lookup the domain's SRV records with the `_minecraft._tcp.` prefix.
        if address.starts_with(char::is_alphanumeric) {
            if let Ok(srv) = self.lookup_srv(format!("{}{address}", Self::SRV_PREFIX)).await {
                // Try to connect to every response, returning the first successful connection.
                for srv in srv {
                    let (address, port) = (srv.target().to_string(), port.unwrap_or(srv.port()));
                    let lookup = self.lookup_ip(address.trim_end_matches('.')).await?;

                    if let Ok(conn) = self.join_lookup(&address, lookup, port).await {
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
            match ClientConnection::connect_to(address, socket).await {
                Ok(conn) => return Ok(conn),
                Err(err) => error = Some(err),
            }
        }

        Err(error)
    }
}

#[cfg(feature = "bevy")]
impl bevy_ecs::world::FromWorld for ServerResolver {
    fn from_world(_: &mut bevy_ecs::world::World) -> Self {
        Self::system_config().unwrap_or_else(|err| {
            bevy_log::warn!("Failed to load system resolver, defaulting to cloudflare: {err}");
            Self::new(ResolverConfig::cloudflare(), ResolverOpts::default())
        })
    }
}

#[test]
#[cfg(feature = "bevy")]
fn resolver() {
    use froglight_common::Version;
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

    let resolver = ServerResolver::new(ResolverConfig::cloudflare(), ResolverOpts::default());

    #[cfg(feature = "bevy")]
    bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);

    // `mc.hypixel.net` should resolve to `mc.hypixel.net` and port `25565`.
    {
        let mut connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("mc.hypixel.net"))
                .unwrap();
        assert_eq!(connection.address(), "mc.hypixel.net");
        assert_eq!(connection.as_raw().as_stream().peer_addr().unwrap().port(), 25565);
    }

    // `hypixel.net` should resolve to `mc.hypixel.net` and port `25565`.
    {
        let mut connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("hypixel.net"))
                .unwrap();
        assert_eq!(connection.address(), "mc.hypixel.net");
        assert_eq!(connection.as_raw().as_stream().peer_addr().unwrap().port(), 25565);
    }

    // `localhost` should resolve to `127.0.0.1` and port `25565`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("localhost"));
        assert!(connection.is_err());
    }

    // `127.0.0.1` should resolve to `127.0.0.1` and port `25565`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("127.0.0.1"));
        assert!(connection.is_err());
    }

    // `::1` should resolve to `::1` and port `25565`.
    {
        let connection = futures_lite::future::block_on(resolver.connect_to_server::<Test>("::1"));
        assert!(connection.is_err());
    }

    // `::1:25566` should resolve to `::1` and port `25566`.
    {
        let connection =
            futures_lite::future::block_on(resolver.connect_to_server::<Test>("::1:25566"));
        assert!(connection.is_err());
    }
}
