#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "agent")]
pub mod agent;
pub mod resolver;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    #[cfg(feature = "agent")]
    pub use crate::agent::FroglightAgent;
    pub use crate::resolver::{
        FroglightResolver,
        hickory::{ResolverConfig, ResolverOpts},
    };
}
