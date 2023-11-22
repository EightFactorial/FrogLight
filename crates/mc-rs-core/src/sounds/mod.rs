use bevy::prelude::*;
use mc_rs_protocol::types::ResourceLocation;

pub(super) fn setup(app: &mut App) { app.add_event::<SoundEvent>(); }

#[derive(Debug, Clone, PartialEq, Event)]
pub struct SoundEvent {
    pub name: ResourceLocation,
    pub kind: SoundEventKind,
    pub position: Option<Vec3>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundEventKind {
    Global,
    Music,
    Jukebox,
    Weather,
    Block,
    Hostile,
    Neutral,
    Player,
    Ambient,
    Voice,
}
