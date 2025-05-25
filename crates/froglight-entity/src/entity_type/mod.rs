//! TODO

pub mod generated;

mod properties;
pub use properties::{Aabb3d, EntityCollider, EntityEyeHeight, EntityGravity};

mod storage;
pub use storage::{AppEntityTypeStorage, EntityTypeStorage, GlobalEntityTypeId};

mod traits;
pub use traits::{EntityTypeExt, EntityTypeTrait, StaticEntityType};

mod resolver;
pub use resolver::EntityTypeResolver;
