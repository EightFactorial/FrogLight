use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*, time::TimePlugin};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogPlugin::default())
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin)
            .add(FrameCountPlugin)
            .add(TimePlugin)
            .add(TransformPlugin)
            .add(HierarchyPlugin)
    }
}
