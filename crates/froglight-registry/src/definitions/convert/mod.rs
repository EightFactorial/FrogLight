//! Conversions between
//! [`ResourceKeys`](froglight_protocol::common::ResourceKey),
//! IDs and registry values.

pub use froglight_macros::FrogRegistry;

mod custom;

mod simple;
pub use simple::{DefaultIdRegistry, SimpleIdRegistry};

mod traits;
pub use traits::{ConvertKey, ConvertKeyError, InitializeIdRegistry, MissingKeyError};
