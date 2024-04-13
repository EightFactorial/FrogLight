//! Conversions between
//! [`ResourceKeys`](froglight_protocol::common::ResourceKey),
//! IDs and registry values.

pub use froglight_macros::FrogRegistry;

mod default;
pub use default::DefaultIdRegistry;

mod simple;
pub use simple::SimpleIdRegistry;

mod traits;
pub use traits::InitializeIdRegistry;
