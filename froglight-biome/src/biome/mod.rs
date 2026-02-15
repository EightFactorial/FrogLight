//! TODO

#[cfg(feature = "biome_data")]
mod attribute;
#[cfg(feature = "biome_data")]
pub use attribute::{AttributeType, BiomeAttributeSet, BiomeAttributeStorage};

mod biome;
pub use biome::{Biome, BiomeType};

mod metadata;
pub use metadata::BiomeMetadata;

mod state;
pub use state::GlobalId;
