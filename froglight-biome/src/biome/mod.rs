//! TODO

mod attribute;
pub use attribute::BiomeAttributeSet;
#[cfg(feature = "attribute")]
pub use attribute::{AttributeType, BiomeAttributeStorage};

mod biome;
pub use biome::{Biome, BiomeType};

mod feature;
pub use feature::{BiomeFeatureSet, BiomeFeatureStorage, BiomeFeatures, FeatureType};

mod metadata;
pub use metadata::BiomeMetadata;

mod state;
pub use state::GlobalId;
