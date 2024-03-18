use std::{net::SocketAddr, num::NonZeroU16, sync::Arc};

use async_std_resolver::{AsyncStdResolver, ResolveError};
use bevy_ecs::system::Resource;
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

    /// Determines if a lookup is required and performs it.
    pub(crate) async fn url_lookup(&self, mut query: &str) -> Result<SocketAddr, ResolverError> {
        let mut port: Option<NonZeroU16> = None;

        if let Some((split_host, split_port)) = query.split_once(':') {
            port = split_port.parse::<u16>().ok().and_then(NonZeroU16::new);
            query = split_host;
        }

        let extracted = self.extractor.extract_naive(query)?;
        if let (Some(domain), Some(suffix)) = (&extracted.domain, &extracted.suffix) {
            let srv_query = format!("_minecraft._tcp.{domain}.{suffix}");
            self.srv_lookup(query, &srv_query, port).await
        } else {
            self.lookup_ip(query, port).await
        }
    }

    /// Performs a SRV lookup on the given domain.
    async fn srv_lookup(
        &self,
        query: &str,
        srv_query: &str,
        mut port: Option<NonZeroU16>,
    ) -> Result<SocketAddr, ResolverError> {
        #[cfg(debug_assertions)]
        bevy_log::trace!("Performing SRV lookup for `{srv_query}`");

        let Ok(srv_lookup) = self.resolver.srv_lookup(srv_query).await else {
            return self.lookup_ip(query, port).await;
        };

        let srv = srv_lookup.iter().next().ok_or(ResolverError::NoSrvRecordsFound)?;
        let target = srv.target().to_utf8();

        // Don't overwrite the port if it's already set
        if port.is_none() {
            port = NonZeroU16::new(srv.port());
        }

        self.lookup_ip(&target, port).await
    }

    /// Performs an A/AAAA lookup for the given target.
    async fn lookup_ip(
        &self,
        target: &str,
        port: Option<NonZeroU16>,
    ) -> Result<SocketAddr, ResolverError> {
        #[cfg(debug_assertions)]
        bevy_log::trace!("Performing A/AAAA lookup for `{target}`");

        let lookup = self.resolver.lookup_ip(target).await?;
        let address = lookup.iter().next().ok_or(ResolverError::NoAddressFound)?;
        let port = port.map_or(25565, NonZeroU16::get);

        Ok(SocketAddr::from((address, port)))
    }
}

/// An error that occurred during a resolver lookup.
#[derive(Debug, Error)]
pub enum ResolverError {
    /// An error occurred getting the top-level domain.
    #[error(transparent)]
    TldError(#[from] tldextract::TldExtractError),
    /// An error occurred resolving the query.
    #[error(transparent)]
    ResolveError(#[from] ResolveError),
    /// No SRV records found.
    #[error("No SRV records found")]
    NoSrvRecordsFound,
    /// No A/AAAA records found.
    #[error("No A/AAAA records found")]
    NoAddressFound,
    /// Query has either no host or domain.
    #[error("Query has either no host or domain")]
    NoHostOrDomain,
}
