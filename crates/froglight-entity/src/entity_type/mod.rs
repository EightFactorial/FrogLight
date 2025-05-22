//! TODO

pub mod generated;

mod storage;
pub use storage::{AppEntityTypeStorage, EntityTypeStorage};

mod traits;
pub use traits::{EntityType, EntityTypeExt, StaticEntityType};
