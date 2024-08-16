//! Raw assets loaded directly from resource packs.
//!
//! These assets should serialize/deserialize identically,
//! and may require additional processing to be usable.

use bevy_app::App;

pub mod lang;
pub use lang::SingleLanguageMap;

pub mod pack;
pub use pack::ResourcePack;

pub mod pack_meta;
pub use pack_meta::ResourcePackMeta;

pub mod sound;
pub use sound::SoundDefinitionMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    pack::build(app);
    pack_meta::build(app);

    lang::build(app);
    sound::build(app);
}
