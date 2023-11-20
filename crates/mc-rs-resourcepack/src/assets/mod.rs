use bevy::prelude::*;

pub mod resourcepacks;
pub mod textureatlases;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourceAssetPlugin;

impl Plugin for ResourceAssetPlugin {
    fn build(&self, app: &mut App) {
        resourcepacks::setup(app);
        textureatlases::setup(app);
    }
}
