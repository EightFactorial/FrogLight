//! Utilities for tracking entities and chunks.

#[cfg(feature = "froglight-world")]
mod chunkpos;
#[cfg(feature = "froglight-world")]
pub use chunkpos::ChunkPositionMap;

#[cfg(feature = "froglight-world")]
mod entitychunk;
#[cfg(feature = "froglight-world")]
pub use entitychunk::EntityChunkMap;

#[cfg(feature = "froglight-components")]
mod entityid;
#[cfg(feature = "froglight-components")]
pub use entityid::EntityIdMap;

#[cfg(feature = "froglight-components")]
mod entityuuid;
#[cfg(feature = "froglight-components")]
pub use entityuuid::EntityUuidMap;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    #[cfg(feature = "froglight-world")]
    {
        chunkpos::build(app);
        entitychunk::build(app);
    }
    #[cfg(feature = "froglight-components")]
    {
        entityid::build(app);
        entityuuid::build(app);
    }
}
