use bevy_app::App;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<BlockStateModel>().register_type::<SingleOrMultiModel>();
}

/// Either a single model or a list of models
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMultiModel {
    /// A single model
    Model(BlockStateModel),
    /// A list of models
    List(Vec<BlockStateModel>),
}

/// A block state model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct BlockStateModel {
    /// The key of the model
    pub model: ResourceKey,

    /// The x rotation of the model
    #[serde(default, skip_serializing_if = "BlockStateModel::is_float_zero")]
    pub x: f32,
    /// The y rotation of the model
    #[serde(default, skip_serializing_if = "BlockStateModel::is_float_zero")]
    pub y: f32,

    /// Lock the rotation of the texture to the block.
    ///
    /// This prevents the texture from rotating with the `x` and `y` fields.
    #[serde(default)]
    pub uvlock: bool,

    /// The weight of the model.
    ///
    /// This is used to determine the probability of the model being selected
    /// when multiple models are present.
    #[serde(
        default = "BlockStateModel::weight_default",
        skip_serializing_if = "BlockStateModel::is_default_weight"
    )]
    pub weight: f32,
}

impl BlockStateModel {
    /// Returns `true` if the given float is equal to `0.0`.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_float_zero(f: &f32) -> bool { f.abs() < f32::EPSILON }

    /// The default weight of the model
    #[must_use]
    pub const fn weight_default() -> f32 { 1.0 }
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_default_weight(w: &f32) -> bool { (*w - Self::weight_default()).abs() < f32::EPSILON }
}

/// An iterator over a collection of models
#[derive(Debug, Clone, PartialEq)]
pub struct ModelIterator<'a> {
    models: &'a SingleOrMultiModel,
    index: usize,
}

impl SingleOrMultiModel {
    /// Returns an iterator over the models
    #[must_use]
    pub fn iter_models(&self) -> ModelIterator { ModelIterator { models: self, index: 0 } }
}

impl<'a> Iterator for ModelIterator<'a> {
    type Item = &'a BlockStateModel;
    fn next(&mut self) -> Option<Self::Item> {
        match self.models {
            SingleOrMultiModel::Model(model) => {
                if self.index == 0 {
                    self.index += 1;
                    Some(model)
                } else {
                    None
                }
            }
            SingleOrMultiModel::List(models) => {
                if self.index < models.len() {
                    let model = &models[self.index];
                    self.index += 1;
                    Some(model)
                } else {
                    None
                }
            }
        }
    }
}
