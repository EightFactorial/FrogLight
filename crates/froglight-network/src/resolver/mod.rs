//! A [`FroglightResolver`] implemented using [`hickory_resolver`].

mod provider;
use provider::{FroglightInnerResolver, ResolverConnectionProvider};

#[expect(clippy::module_inception)]
mod resolver;
pub use resolver::FroglightResolver;

pub mod hickory {
    //! Re-exports of relevant types from the [`hickory_resolver`] crate.

    pub use hickory_resolver::{
        IntoName, Name, TryParseIp,
        config::{
            LookupIpStrategy, NameServerConfig, NameServerConfigGroup, ResolverConfig, ResolverOpts,
        },
        lookup::{Lookup, SrvLookup, TxtLookup},
        lookup_ip::LookupIp,
    };
}
