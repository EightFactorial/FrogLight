//! TODO

mod attribute;
pub use attribute::BiomeAttributeSet;
#[cfg(feature = "biome_data")]
pub use attribute::{AttributeType, BiomeAttributeStorage};

mod biome;
pub use biome::{Biome, BiomeType};

mod metadata;
pub use metadata::BiomeMetadata;

mod state;
pub use state::GlobalId;
