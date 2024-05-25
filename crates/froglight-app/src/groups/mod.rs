mod basic;
pub use basic::BasicPlugins;

#[cfg(feature = "bevy_asset")]
mod graphics;
#[cfg(feature = "bevy_asset")]
pub use graphics::GraphicalPlugins;

mod preset;
#[cfg(feature = "bevy_asset")]
pub use preset::app::AppPlugins;
pub use preset::headless::HeadlessPlugins;

mod taskpool;
pub use taskpool::TASKPOOL_SETTINGS;
