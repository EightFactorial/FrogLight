use std::{net::SocketAddr, num::NonZeroU16, sync::Arc};

use async_std_resolver::{
    lookup::{Ipv4Lookup, Ipv6Lookup, SrvLookup},
    lookup_ip::LookupIp,
    AsyncStdResolver, ResolveError,
};
use bevy_ecs::{component::Component, system::Resource};
use bevy_tasks::{IoTaskPool, Task};
use thiserror::Error;
use tldextract::{TldExtractor, TldOption};

/// A DNS resolver
///
/// Lookups are performed asynchronously and functions return
/// tasks that can be polled to get the result.
#[derive(Debug, Clone, Resource)]
pub struct Resolver {
    pub(crate) resolver: Arc<AsyncStdResolver>,
    pub(crate) extractor: Arc<TldExtractor>,
}

impl Resolver {
    /// Creates a new [`ResolverResource`] from the given client.
    #[must_use]
    pub fn new(resolver: AsyncStdResolver) -> Self {
        Self {
            resolver: Arc::new(resolver),
            extractor: Arc::new(TldOption::default().naive_mode(true).build()),
        }
    }

    /// Performs an [`Ipv4Lookup`] for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn ipv4_lookup(&self, query: &str) -> ResolverIpv4Task {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverIpv4Task {
            task: IoTaskPool::get().spawn(async move { resolver.ipv4_lookup(query).await }),
        }
    }

    /// Performs an [`Ipv6Lookup`] for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn ipv6_lookup(&self, query: &str) -> ResolverIpv6Task {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverIpv6Task {
            task: IoTaskPool::get().spawn(async move { resolver.ipv6_lookup(query).await }),
        }
    }

    /// Performs an ip lookup for the given query.
    ///
    /// This can be either an IPv4 or IPv6 address.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn lookup_ip(&self, query: &str) -> ResolverIpTask {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverIpTask {
            task: IoTaskPool::get().spawn(async move { resolver.lookup_ip(query).await }),
        }
    }

    /// Performs a [`SrvLookup`] for the given query.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn srv_lookup(&self, query: &str) -> ResolverSrvTask {
        let resolver = Arc::clone(&self.resolver);
        let query = query.to_string();

        ResolverSrvTask {
            task: IoTaskPool::get().spawn(async move { resolver.srv_lookup(query).await }),
        }
    }

    /// Parses a URL and performs a lookup for the server.
    ///
    /// This will first check for a `SRV` at `_minecraft._tcp.DOMAIN.COM` and
    /// perform a lookup on the resulting record if necessary.
    ///
    /// If there is no record, it will perform an `A/AAAA` lookup for the url.
    ///
    /// # Note
    /// This uses bevy's [`IoTaskPool`](bevy_tasks::IoTaskPool).
    #[must_use]
    pub fn server_lookup(&self, query: &str) -> ResolverServerTask {
        let resolver = Arc::clone(&self.resolver);
        let extractor = Arc::clone(&self.extractor);
        let query = query.to_string();

        ResolverServerTask {
            task: IoTaskPool::get()
                .spawn(ResolverServerTask::url_lookup(resolver, extractor, query)),
        }
    }
}

/// A macro that creates a task for a resolver lookup.
macro_rules! create_task {
    ($task:ident, $result:ty) => {
        /// An in-progress
        /// `
        #[doc = stringify!($result)]
        /// `
        /// task.
        #[derive(Debug, Component)]
        pub struct $task {
            task: Task<Result<$result, ResolveError>>,
        }

        impl $task {
            /// Blocks the current thread and polls the task once.
            ///
            /// # Note
            /// If the task returns `Some`, do not poll again and drop the task.
            pub fn poll_once(&mut self) -> Option<Result<$result, ResolveError>> {
                bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
            }
        }
    };
}

create_task!(ResolverIpv4Task, Ipv4Lookup);
create_task!(ResolverIpv6Task, Ipv6Lookup);
create_task!(ResolverIpTask, LookupIp);
create_task!(ResolverSrvTask, SrvLookup);

/// An in-progress server lookup task.
///
/// Returns a [`SocketAddr`] if successful, which can
/// be used to connect to a server.
#[derive(Debug, Component)]
pub struct ResolverServerTask {
    task: Task<Result<SocketAddr, ResolverError>>,
}

impl ResolverServerTask {
    /// Blocks the current thread and polls the task once.
    ///
    /// # Note
    /// If the task returns `Some`, do not poll again and drop the task.
    pub fn poll_once(&mut self) -> Option<Result<SocketAddr, ResolverError>> {
        bevy_tasks::block_on(bevy_tasks::poll_once(&mut self.task))
    }

    /// Determines if a lookup is required and performs it.
    pub(crate) async fn url_lookup(
        resolver: Arc<AsyncStdResolver>,
        extractor: Arc<TldExtractor>,
        query: String,
    ) -> Result<SocketAddr, ResolverError> {
        let mut query = query.as_str();
        let mut port: Option<NonZeroU16> = None;

        if let Some((split_host, split_port)) = query.split_once(':') {
            port = split_port.parse::<u16>().ok().and_then(NonZeroU16::new);
            query = split_host;
        }

        let extracted = extractor.extract_naive(query)?;
        if let (Some(domain), Some(suffix)) = (&extracted.domain, &extracted.suffix) {
            let srv_query = format!("_minecraft._tcp.{domain}.{suffix}");
            Self::srv_lookup(&resolver, query, &srv_query, port).await
        } else {
            Self::lookup_ip(&resolver, query, port).await
        }
    }

    /// Performs a SRV lookup on the given domain.
    async fn srv_lookup(
        resolver: &AsyncStdResolver,
        query: &str,
        srv_query: &str,
        mut port: Option<NonZeroU16>,
    ) -> Result<SocketAddr, ResolverError> {
        let Ok(srv_lookup) = resolver.srv_lookup(srv_query).await else {
            return Self::lookup_ip(resolver, query, port).await;
        };

        let srv = srv_lookup.iter().next().ok_or(ResolverError::NoSrvRecordsFound)?;
        let target = srv.target().to_utf8();

        // Don't overwrite the port if it's already set
        if port.is_none() {
            port = NonZeroU16::new(srv.port());
        }

        Self::lookup_ip(resolver, &target, port).await
    }

    /// Performs an A/AAAA lookup for the given target.
    async fn lookup_ip(
        resolver: &AsyncStdResolver,
        target: &str,
        port: Option<NonZeroU16>,
    ) -> Result<SocketAddr, ResolverError> {
        let lookup = resolver.lookup_ip(target).await?;
        let address = lookup.iter().next().ok_or(ResolverError::NoAddressFound)?;
        let port = port.map_or(25565, NonZeroU16::get);

        Ok(SocketAddr::from((address, port)))
    }
}

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error(transparent)]
    TldError(#[from] tldextract::TldExtractError),
    #[error(transparent)]
    ResolveError(#[from] ResolveError),
    #[error("No SRV records found")]
    NoSrvRecordsFound,
    #[error("No A/AAAA records found")]
    NoAddressFound,
    #[error("Query has either no host or domain")]
    NoHostOrDomain,
}
