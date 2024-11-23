//! Common data structures used by the protocol.

pub use uuid::Uuid;

mod entity;
pub use froglight_common::{EntityId, EntityUuid};

mod other;
pub use other::*;

mod position;
pub use position::*;

mod resourcekey;
pub use froglight_common::{ResourceKey, ResourceKeyError};

mod special;
pub use special::*;
