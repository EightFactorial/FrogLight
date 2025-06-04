#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "agent")]
pub mod agent;
#[cfg(feature = "bevy")]
pub mod plugin;
pub mod resolver;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    #[cfg(feature = "agent")]
    pub use crate::agent::FroglightAgent;
    #[cfg(feature = "bevy")]
    pub use crate::plugin::ResolverPlugin;
    pub use crate::resolver::{
        FroglightResolver,
        hickory::{ResolverConfig, ResolverOpts},
    };
}
