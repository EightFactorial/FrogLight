mod basic;
pub use basic::BasicPlugins;

#[cfg(feature = "presets")]
mod preset;
#[cfg(feature = "presets")]
pub use preset::headless::HeadlessPlugins;

mod taskpool;
pub use taskpool::TASKPOOL_SETTINGS;
