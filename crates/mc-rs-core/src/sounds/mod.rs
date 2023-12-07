use bevy::prelude::*;
use mc_rs_protocol::types::{enums::SoundType, ResourceLocation};

pub(super) fn setup(app: &mut App) { app.add_event::<SoundEvent>(); }

#[derive(Debug, Clone, PartialEq, Event)]
pub struct SoundEvent {
    pub asset: ResourceLocation,
    pub kind: SoundType,
    pub position: Option<Vec3>,
}
