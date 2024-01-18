use bevy_ecs::entity::Entity;
use derive_more::{Deref, DerefMut, From, Into};

/// A [`ChunkEntity`] is an [`Entity`] that holds a
/// [`Chunk`](crate::Chunk) [`Component`](bevy_ecs::component::Component).
///
/// Just a wrapper around [`Entity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut, From, Into)]
pub struct ChunkEntity(pub Entity);
