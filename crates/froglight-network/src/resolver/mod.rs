//! Resolver

pub use async_std_resolver::{
    config::{LookupIpStrategy, ResolverConfig, ResolverOpts},
    AsyncStdResolver, ResolveError as AsyncStdResolveError,
};

#[allow(clippy::module_inception)]
mod resolver;
pub use resolver::Resolver;

mod plugin;
pub use plugin::ResolverPlugin;
