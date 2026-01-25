use reqwest::dns::{Name, Resolve, Resolving};

use crate::resolver::DnsResolver;

impl Resolve for DnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        use std::net::SocketAddr;

        let resolver = self.clone();
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_api::resolver::reqwest", "Resolving IP for \"{}\"", name.as_str());

            resolver.lookup_ip(name.as_str()).await.map(
                |val| -> Box<dyn Iterator<Item = SocketAddr> + Send> {
                    Box::new(val.into_iter().map(move |ip| SocketAddr::new(ip, 0)))
                },
            )
        })
    }
}

#[cfg(feature = "resolver")]
impl Resolve for crate::resolver::hickory::Resolver {
    fn resolve(&self, name: Name) -> Resolving {
        use std::{error::Error, net::SocketAddr};

        let resolver = self.clone();
        Box::pin(async move {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "froglight_api::resolver::reqwest", "Resolving IP for \"{}\"", name.as_str());

            resolver.as_resolver().lookup_ip(name.as_str().to_string()).await.map_or_else(
                |err| -> Result<_, Box<dyn Error + Send + Sync>> { Err(Box::new(err)) },
                |val| -> Result<Box<dyn Iterator<Item = SocketAddr> + Send>, _> {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(target: "froglight_api::resolver::reqwest", "Resolved \"{}\" to IPs {:?}", name.as_str(), val.as_lookup().records());

                    Ok(Box::new(val.into_iter().map(move |ip| SocketAddr::new(ip, 0))))
                },
            )
        })
    }
}
