//! Common data structures used by the protocol.

mod entity;
pub use froglight_components::entity::{EntityId, EntityUuid};

mod other;
pub use other::*;

mod position;
pub use position::*;

mod resourcekey;
pub use froglight_components::resourcekey::{ResourceKey, ResourceKeyError};

mod special;
pub use special::*;
