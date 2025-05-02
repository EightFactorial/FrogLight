use std::net::SocketAddr;

use hickory_resolver::IntoName;

use super::FroglightResolver;

impl FroglightResolver {
    /// Lookup a minecraft server's IP from the given address.
    ///
    /// # Errors
    /// TODO
    #[allow(clippy::unused_async)]
    pub async fn lookup_minecraft<N: IntoName>(
        &self,
        _address: N,
    ) -> Result<SocketAddr, std::io::Error> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn resolver_minecraft() {}
