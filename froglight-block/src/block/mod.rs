//! TODO

mod attribute;
pub use attribute::{BlockAttribute, BlockAttributes};

mod behavior;
pub use behavior::BlockBehavior;

mod block;
pub use block::{Block, BlockType};

mod metadata;
pub use metadata::BlockMetadata;

mod state;
pub use state::{GlobalId, StateId};
