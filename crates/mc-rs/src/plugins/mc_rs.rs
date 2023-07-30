use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct MCRSPlugins;

impl PluginGroup for MCRSPlugins {
    fn build(self) -> PluginGroupBuilder { PluginGroupBuilder::start::<Self>() }
}
