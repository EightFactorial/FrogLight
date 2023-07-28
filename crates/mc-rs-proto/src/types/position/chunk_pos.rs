use bevy_ecs::prelude::Component;
use bevy_math::IVec2;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

/// A chunk position.
///
/// This is a chunk's position in the world.
///
/// Due to internally using an [IVec2], replace 'y' with 'z' when using this type.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut, From, Into, Transcode,
)]
pub struct ChunkPos(pub IVec2);

impl ChunkPos {
    pub const ZERO: Self = Self(IVec2::ZERO);

    pub fn new(x: i32, z: i32) -> Self { Self(IVec2::new(x, z)) }
}