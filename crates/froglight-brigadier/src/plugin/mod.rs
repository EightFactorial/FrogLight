//! TODO

use bevy_app::{App, Last, Plugin};
use bevy_core::Name;
use bevy_ecs::{
    event::Events,
    reflect::{AppFunctionRegistry, AppTypeRegistry},
    schedule::{
        InternedScheduleLabel, IntoSystemConfigs, ScheduleLabel, common_conditions::on_event,
    },
    system::Local,
    world::World,
};
use bevy_log::{debug, error};
use derive_more::From;
use tracing::Level;

mod build;
pub use build::BrigadierBuilder;

mod command;
pub use command::{BrigadierCommand, BrigadierCommands};

mod event;
pub use event::BrigadierEvent;

use crate::{
    argument::ArgumentParserPlugin,
    function::{Empty, WorldRef},
    graph::AppBrigadierGraph,
};

/// A plugin for integrating Brigadier into a Bevy application.
///
/// By default this plugin runs commands during [`Last`],
/// but this can be configured using [`BrigadierPlugin::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub struct BrigadierPlugin(InternedScheduleLabel);

impl Default for BrigadierPlugin {
    fn default() -> Self { Self::new(&Last) }
}

impl BrigadierPlugin {
    /// Create a new [`BrigadierPlugin`] with the given [`ScheduleLabel`].
    #[inline]
    #[must_use]
    pub fn new<Schedule: ScheduleLabel>(schedule: &Schedule) -> Self { Self(schedule.intern()) }
}

impl Plugin for BrigadierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ArgumentParserPlugin);

        app.init_resource::<AppBrigadierGraph>();
        app.register_type::<AppBrigadierGraph>();

        app.add_event::<BrigadierEvent>();
        app.register_type::<BrigadierEvent>();

        app.add_systems(self.0, brigadier_listener.run_if(on_event::<BrigadierEvent>));
    }
}

/// A listener and executor for [`BrigadierEvent`]s.
pub fn brigadier_listener(world: &mut World, mut world_ref: Local<WorldRef<Empty>>) {
    let graph = world.resource::<AppBrigadierGraph>().clone();
    let graph = graph.read();

    let types = world.resource::<AppTypeRegistry>().clone();
    let types = types.read();

    let functions = world.resource::<AppFunctionRegistry>().clone();
    let functions = functions.read();

    world.resource_scope::<Events<BrigadierEvent>, _>(|world, mut events| {
        world_ref.with_world(world, |world| {
            for event in events.drain() {
                if tracing::enabled!(Level::DEBUG) {
                    if let Some(name) = world.value().get::<Name>(event.entity()) {
                        debug!("{name} ({}) ran \"{}\"", event.entity(), event.command());
                    } else {
                        debug!("Entity ({}) ran \"{}\"", event.entity(), event.command());
                    }
                }

                if let Err(err) =
                    graph.execute(event.entity(), event.command(), &types, &functions, world)
                {
                    error!("Entity {} failed to execute command, {err}", event.entity());
                }
            }
        });
    });
}
