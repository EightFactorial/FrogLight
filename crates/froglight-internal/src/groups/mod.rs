mod application;
pub use application::ApplicationPlugins;

mod basic;
pub use basic::BasicPlugins;

#[cfg(feature = "presets")]
mod headless;
#[cfg(feature = "presets")]
pub use headless::HeadlessPlugins;

mod taskpool;
pub use taskpool::TASKPOOL_SETTINGS;
