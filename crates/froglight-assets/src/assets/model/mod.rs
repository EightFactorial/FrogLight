use bevy_app::App;
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

mod block_model;
pub use block_model::*;

mod face;
pub use face::*;

mod element;
pub use element::*;

mod item_model;
pub use item_model::*;

mod transform;
pub use transform::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ModelDefinition>()
        .register_type::<BlockModelDefinition>()
        .register_type::<ItemModelDefinition>();
    app.register_type::<GuiLight>()
        .register_type::<ItemModelOverride>()
        .register_type::<ModelTextures>();

    face::build(app);
    element::build(app);
    transform::build(app);
}

/// A model definition
///
/// Block models are found in `assets/{namespace}/models/block`,
/// and item models are found in `assets/{namespace}/models/item`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelDefinition {
    /// A block model definition
    Block(BlockModelDefinition),
    /// An item model definition
    Item(ItemModelDefinition),
}

/// Model textures
///
/// Textures can be referenced using `#texture_name`
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut, Reflect,
)]
#[reflect(Default, Serialize, Deserialize)]
pub struct ModelTextures(pub HashMap<String, ResourceKey>);
