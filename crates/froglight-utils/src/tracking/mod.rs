//! Adds tracking components and systems.

use bevy_app::App;

#[cfg(feature = "froglight-world")]
mod chunk_positions;
#[cfg(feature = "froglight-world")]
pub use chunk_positions::*;

#[cfg(feature = "froglight-entity")]
mod entity_positions;
#[cfg(feature = "froglight-entity")]
pub use entity_positions::*;

#[doc(hidden)]
#[allow(unused_variables)]
pub(super) fn build(app: &mut App) {
    #[cfg(feature = "froglight-world")]
    {
        chunk_positions::build(app);
    }
    #[cfg(feature = "froglight-entity")]
    {
        entity_positions::build(app);
    }
}
