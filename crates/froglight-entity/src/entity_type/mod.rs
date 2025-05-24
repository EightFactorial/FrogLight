//! TODO

pub mod generated;

mod collider;
pub use collider::{Aabb3d, EntityCollider, EntityEyeHeight};

mod storage;
pub use storage::{AppEntityTypeStorage, EntityTypeStorage, GlobalEntityTypeId};

mod traits;
pub use traits::{EntityTypeExt, EntityTypeTrait, StaticEntityType};

mod resolver;
pub use resolver::EntityTypeResolver;
