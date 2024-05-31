use bevy_app::App;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

mod model;
pub use model::*;

mod multipart;
pub use multipart::*;

mod variant;
pub use variant::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<BlockStateDefinition>();

    model::build(app);
    multipart::build(app);
    variant::build(app);
}

/// A block state definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockStateDefinition {
    /// A block state defined using variants
    Variants(BlockStateVariants),
    /// A block state defined using multiparts
    Multipart(BlockStateMultiparts),
}
