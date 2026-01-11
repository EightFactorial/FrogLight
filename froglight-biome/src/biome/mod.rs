//! TODO

mod attribute;

mod biome;
pub use biome::{Biome, BiomeType};

mod feature;
pub use feature::{BiomeFeatures, BiomeFeatureSet, FeatureSetStorage, FeatureType};

mod metadata;
pub use metadata::BiomeMetadata;

mod state;
pub use state::GlobalId;
