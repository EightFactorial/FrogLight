use bevy_ecs::prelude::Component;
use bevy_math::IVec3;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

/// A chunk section position.
///
/// This is a chunk's position and a section's height.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Deref, DerefMut, From, Into, Transcode,
)]
pub struct ChunkSectionPos(pub IVec3);

impl ChunkSectionPos {
    pub const ZERO: Self = Self(IVec3::ZERO);

    pub fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }
}
