use bevy::prelude::*;

pub mod util;

mod plugins;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) { plugins::setup(app); }
}
