//! TODO

mod aabb;
pub use aabb::EntityAabb;

pub mod attribute;
pub use attribute::{EntityAttribute, EntityAttributeSet, EntityAttributeType};

mod dataset;
pub use dataset::EntityDataSet;

mod entity;
pub use entity::{EntityBundle, EntityComponentType, EntityType};

#[cfg(feature = "facet")]
mod facet;
#[cfg(feature = "facet")]
pub use facet::DataSetSerializer;

mod metadata;
pub use metadata::EntityMetadata;

mod state;
pub use state::GlobalId;
