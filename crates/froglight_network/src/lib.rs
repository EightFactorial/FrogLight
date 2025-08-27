#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod connection;
#[cfg(feature = "resolver")]
pub mod resolver;

#[cfg(feature = "async")]
mod async_net;
#[cfg(feature = "tokio")]
mod async_tokio;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::connection::{RawReadable, RawReadableExt, RawWritable, RawWritableExt};
    #[cfg(feature = "resolver")]
    pub use crate::resolver::DnsResolver;
    #[cfg(feature = "ureq")]
    pub use crate::resolver::HttpClient;
}
