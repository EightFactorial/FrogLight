use std::net::IpAddr;

use ureq::{
    config::Config,
    http::Uri,
    unversioned::{
        resolver::{ResolvedSocketAddrs, Resolver},
        transport::NextTimeout,
    },
};

use crate::resolver::DnsResolver;

impl Resolver for DnsResolver {
    fn resolve(
        &self,
        uri: &Uri,
        _: &Config,
        _: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        use std::net::SocketAddr;

        use async_io::block_on;

        let host = uri.host().ok_or_else(|| {
            ureq::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URI is missing a host",
            )))
        })?;

        block_on(self.lookup_ip(host)).map_or_else(
            |err| Err(ureq::Error::Other(err)),
            |ips| {
                let port = uri.port_u16().unwrap_or_else(|| match uri.scheme() {
                    Some(https) if https.as_str() == "https" => 443,
                    Some(http) if http.as_str() == "http" => 80,
                    None | Some(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!(target: "froglight_api::resolver::ureq", "Cannot get URI port, defaulting to port 80");
                        80
                    }
                });

                let mut results = self.empty();
                ips.into_iter().take(16)
                    .for_each(|ip| results.push(SocketAddr::new(ip, port)));
                #[cfg(feature = "tracing")]
                tracing::trace!(target: "froglight_api::resolver::ureq", "Resolved \"{host}\" to {:?}", results.as_ref());

                Ok(results)
            },
        )
    }
}

#[cfg(feature = "resolver")]
impl Resolver for crate::resolver::hickory::Resolver {
    fn resolve(
        &self,
        uri: &Uri,
        config: &Config,
        _: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        use std::net::SocketAddr;

        use async_io::block_on;
        use ureq::config::IpFamily;

        let host = uri.host().ok_or_else(|| {
            ureq::Error::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "URI is missing a host",
            )))
        })?;

        let mut results = self.empty();
        let port = uri.port_u16().unwrap_or_else(|| match uri.scheme() {
            Some(https) if https.as_str() == "https" => 443,
            Some(http) if http.as_str() == "http" => 80,
            None | Some(_) => {
                #[cfg(feature = "tracing")]
                tracing::warn!(target: "froglight_api::resolver::ureq", "Cannot get URI port, defaulting to port 80");
                80
            }
        });

        match config.ip_family() {
            IpFamily::Any => match block_on(self.as_resolver().lookup_ip(host)) {
                Ok(lookup) => {
                    lookup
                        .into_iter()
                        .take(16)
                        .for_each(|ip| results.push(SocketAddr::new(ip, port)));
                }
                Err(err) => Err(ureq::Error::Other(Box::new(err)))?,
            },
            IpFamily::Ipv4Only => match block_on(self.as_resolver().ipv4_lookup(uri.to_string())) {
                Ok(lookup) => lookup
                    .into_iter()
                    .take(16)
                    .for_each(|a| results.push(SocketAddr::new(IpAddr::V4(a.0), port))),
                Err(err) => Err(ureq::Error::Other(Box::new(err)))?,
            },
            IpFamily::Ipv6Only => match block_on(self.as_resolver().ipv6_lookup(uri.to_string())) {
                Ok(lookup) => lookup
                    .into_iter()
                    .take(16)
                    .for_each(|aaaa| results.push(SocketAddr::new(IpAddr::V6(aaaa.0), port))),
                Err(err) => Err(ureq::Error::Other(Box::new(err)))?,
            },
        }

        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_api::resolver::ureq", "Resolved \"{host}\" to {:?}", results.as_ref());

        Ok(results)
    }
}
