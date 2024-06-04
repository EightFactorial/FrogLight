//! Resolver

pub use async_std_resolver::{
    config::{LookupIpStrategy, ResolverConfig, ResolverOpts},
    AsyncStdResolver, ResolveError as AsyncStdResolveError,
};
pub use tldextract::{TldExtractError, TldExtractor, TldOption};

#[allow(clippy::module_inception)]
mod resolver;
pub use resolver::{Resolver, ResolverError};

mod plugin;
pub use plugin::ResolverPlugin;
