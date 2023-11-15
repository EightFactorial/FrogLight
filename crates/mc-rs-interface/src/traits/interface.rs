use bevy::{
    math::Rect,
    prelude::{App, Entity, World},
};
use mc_rs_core::ResourceLocation;

use crate::resources::resourcepacks::AtlasKind;

/// A trait for interface components
pub trait InterfaceComponent: 'static {
    /// Setup the interface component systems
    fn setup(app: &mut App);

    /// Build the interface component
    fn build(parent: Entity, world: &mut World);
}

pub trait AtlasData: 'static {
    fn atlas_kind() -> AtlasKind;
    fn path() -> ResourceLocation;
    fn coords() -> Vec<Rect>;
}
