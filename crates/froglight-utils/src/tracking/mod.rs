//! Utilities for tracking entities and chunks.

#[cfg(feature = "froglight-world")]
mod chunkpos;
#[cfg(feature = "froglight-world")]
pub use chunkpos::ChunkPositionMap;

#[cfg(feature = "froglight-world")]
mod entitychunk;
#[cfg(feature = "froglight-world")]
pub use entitychunk::EntityChunkMap;

#[cfg(feature = "froglight-common")]
mod entityid;
#[cfg(feature = "froglight-common")]
pub use entityid::EntityIdMap;

#[cfg(feature = "froglight-common")]
mod entityuuid;
#[cfg(feature = "froglight-common")]
pub use entityuuid::EntityUuidMap;

#[doc(hidden)]
#[allow(unused_variables)]
pub(super) fn build(app: &mut bevy_app::App) {
    #[cfg(feature = "froglight-world")]
    {
        chunkpos::build(app);
        entitychunk::build(app);
    }
    #[cfg(feature = "froglight-common")]
    {
        entityid::build(app);
        entityuuid::build(app);
    }
}
