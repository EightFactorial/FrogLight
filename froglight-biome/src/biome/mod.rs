//! TODO

mod attribute;
pub use attribute::{AttributeType, BiomeAttributeData, BiomeAttributeSet, BiomeAttributeStorage};

mod biome;
pub use biome::{Biome, BiomeType};

mod feature;
pub use feature::{BiomeFeatureSet, BiomeFeatures, BiomeFeatureStorage, FeatureType};

mod metadata;
pub use metadata::BiomeMetadata;

mod state;
pub use state::GlobalId;
