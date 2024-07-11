#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_name)]
#![feature(const_type_id)]

use bevy_app::{App, Plugin};

mod error;
pub use error::InvalidKeyError;

pub mod definitions;

mod traits;
pub use traits::{ConvertId, ConvertKey};

/// The `Registry` Froglight plugin.
///
/// Registers types for [`Reflection`](bevy_reflect)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) { definitions::build(app); }
}
