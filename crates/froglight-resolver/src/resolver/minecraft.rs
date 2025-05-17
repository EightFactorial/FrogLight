use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use hickory_resolver::{IntoName, ResolveError};

use super::FroglightResolver;

impl FroglightResolver {
    const DEFAULT_PORT: u16 = 25565;
    const SRV_MC_PREFIX: &str = "_minecraft";
    const SRV_TCP_PREFIX: &str = "_tcp";

    /// Lookup a minecraft server's IP from the given address.
    ///
    /// # Errors
    /// Returns an error if the address is invalid or if the lookup fails.
    pub async fn lookup_minecraft<N: AsRef<str>>(
        &self,
        address: N,
    ) -> Result<SocketAddr, ResolveError> {
        let mut address = address.as_ref();
        let mut port = Self::DEFAULT_PORT;

        // Return early if given a socket or IP address
        if let Ok(sock) = SocketAddr::from_str(address) {
            return Ok(sock);
        } else if let Ok(addr) = IpAddr::from_str(address) {
            return Ok(SocketAddr::new(addr, port));
        } else if let Some(addr) = IntoName::to_ip(&address) {
            return Ok(SocketAddr::new(addr, port));
        }

        // Split off a port if one is present
        if let Some((addr, pt)) = address.rsplit_once(':')
            && pt.chars().all(|c| c.is_ascii_digit())
        {
            let () = pt.parse::<u16>().map_or((), |p| port = p);
            address = addr;
        }

        // Prepare the server address and SRV-prefixed address
        let name = address.into_name()?;
        let srv_name = name
            .prepend_label(Self::SRV_TCP_PREFIX)
            .and_then(|n| n.prepend_label(Self::SRV_MC_PREFIX))?;

        // If a SRV record is found, use the first IP address given
        if let Ok(lookup) = self.lookup_srv(srv_name).await {
            for record in lookup {
                if let Some(ip) = self.lookup_ip(record.target().clone()).await?.iter().next() {
                    return Ok(SocketAddr::new(ip, port));
                }
            }
        }

        // Otherwise, use the address found using A/AAAA records
        if let Some(ip) = self.lookup_ip(name).await?.iter().next() {
            return Ok(SocketAddr::new(ip, port));
        }

        Err(ResolveError::from("could not resolve address into an IP address"))
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn resolve_minecraft() {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use futures_lite::future::block_on;

    #[cfg(feature = "bevy")]
    let _ = bevy_tasks::IoTaskPool::get_or_init(bevy_tasks::TaskPool::new);
    let resolver = FroglightResolver::cloudflare();

    // Test IPv4 addresses
    let addr = block_on(resolver.lookup_minecraft("127.0.0.1")).unwrap();
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565));
    let addr = block_on(resolver.lookup_minecraft("127.0.0.1:8080")).unwrap();
    assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));

    // Test IPv6 addresses
    let addr = block_on(resolver.lookup_minecraft("::1")).unwrap();
    assert_eq!(addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 25565));
    let addr = block_on(resolver.lookup_minecraft("[::1]:1234")).unwrap();
    assert_eq!(addr, SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 1234));

    // Test domain names
    let addr = block_on(resolver.lookup_minecraft("hypixel.net")).unwrap();
    let host = addr.ip();
    assert_eq!(addr.port(), 25565);

    let addr = block_on(resolver.lookup_minecraft("hypixel.net:25565")).unwrap();
    assert_eq!(addr.ip(), host);
    assert_eq!(addr.port(), 25565);

    let addr = block_on(resolver.lookup_minecraft("hypixel.net:80")).unwrap();
    assert_eq!(addr.ip(), host);
    assert_eq!(addr.port(), 80);
}
