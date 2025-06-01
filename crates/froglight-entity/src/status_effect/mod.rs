//! TODO

pub mod generated;

mod category;
pub use category::StatusEffectCategory;

mod resolver;
pub use resolver::StatusEffectResolver;

mod storage;
pub use storage::{AppStatusEffectStorage, GlobalStatusEffectId, StatusEffectStorage};

mod traits;
pub use traits::{StaticStatusEffect, StatusEffectExt, StatusEffectTrait};
