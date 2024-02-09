#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "inspector")]
pub use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(feature = "inspector")]
mod inspector;
#[cfg(feature = "inspector")]
pub use inspector::InspectorEnable;

mod plugin;
pub use plugin::DebugPlugin;

mod schedules;
pub use schedules::DebugUpdateSet;
