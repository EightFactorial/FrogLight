//! Common data structures used by the protocol.

mod newtype;
pub use newtype::*;

mod other;
pub use other::*;

mod position;
pub use position::*;

mod special;
pub use special::*;

mod resource_key;
pub use resource_key::{ResourceKey, ResourceKeyError};
