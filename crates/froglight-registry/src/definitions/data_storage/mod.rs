//! Conversions between
//! [`ResourceKeys`](froglight_protocol::common::ResourceKey),
//! IDs and registry values.

pub use froglight_macros::FrogRegistry;

mod default;
pub use default::DefaultRegistry;

mod simple;
pub use simple::SimpleRegistry;

mod traits;
pub use traits::InitializeRegistry;
