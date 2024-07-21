#[cfg(feature = "client")]
mod application;
#[cfg(feature = "client")]
pub use application::ApplicationPlugins;

mod basic;
pub use basic::BasicPlugins;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::ClientPlugins;

#[cfg(feature = "presets")]
mod headless;
#[cfg(feature = "presets")]
pub use headless::HeadlessPlugins;

mod taskpool;
pub use taskpool::TASKPOOL_SETTINGS;
