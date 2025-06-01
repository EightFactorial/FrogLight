//! TODO

mod attribute_set;
pub use attribute_set::EntityAttributeSet;

pub mod generated;

mod resolver;
pub use resolver::EntityAttributeResolver;

mod storage;
pub use storage::{AppEntityAttributeStorage, EntityAttributeStorage, GlobalEntityAttributeId};

mod traits;
pub use traits::{EntityAttributeExt, EntityAttributeTrait, StaticEntityAttribute};
