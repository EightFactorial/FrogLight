#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

pub mod components;
pub mod events;
pub mod resources;
pub mod systemsets;

/// The `Core` Froglight plugin.
///
/// Adds core [`SystemSets`](bevy_ecs::schedule::SystemSet),
/// [`Events`](bevy_ecs::event::Event),
/// [`Components`](bevy_ecs::component::Component) and
/// [`Resources`](bevy_ecs::system::Resource).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        components::build(app);
        events::build(app);
        resources::build(app);
        systemsets::build(app);
    }

    #[allow(unused_variables)]
    fn finish(&self, app: &mut App) {
        // Add a system to log if the program was built in debug mode.
        #[cfg(debug_assertions)]
        {
            app.add_systems(bevy_app::Startup, || {
                bevy_log::warn!("FrogLight was built in debug mode!");
            });
        }
    }
}
