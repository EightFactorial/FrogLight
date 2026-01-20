//! TODO

mod groups;
pub use groups::GlobalPlugins;

pub mod plugins;

mod r#trait;
pub use r#trait::{PluginType, ReflectInventory};
