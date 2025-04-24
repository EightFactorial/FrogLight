//! TODO

use bevy::{MinimalPlugins, log::LogPlugin, prelude::*};
use bevy_ecs::{component::HookContext, system::RunSystemOnce, world::DeferredWorld};
use froglight_common::prelude::*;
use froglight_schedule::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default()));

    app.register_type::<EntityA>();
    app.register_type::<EntityB>();

    app.add_systems(Startup, (EntityA::spawn_with_subworld, EntityB::spawn_with_subworld).chain());
    app.add_systems(PostStartup, view_results);

    app.run()
}

fn view_results(world: &mut World) {
    info!("");
    info!("World:");
    view_results_indent(world, 2);
    info!("Exiting...");
    world.send_event(AppExit::Success);
}

fn view_results_indent(world: &World, indent: usize) {
    for entity in world.iter_entities() {
        {
            let components = entity.archetype().components();
            let components = components.map(|id| world.components().get_info(id).unwrap().name());

            info!(
                "{}Entity ({}v{}): {}",
                " ".repeat(indent),
                entity.id().generation(),
                entity.id().index(),
                components.collect::<Vec<_>>().join(", ")
            );
        }

        if let Some(sub_world) = entity.get::<SubWorld>() {
            info!("{}  SubWorld \"{}\":", " ".repeat(indent), sub_world.identifier());
            info!("");
            view_results_indent(sub_world, indent + 4);
            info!("");
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, Component, Reflect)]
#[component(on_add = Self::on_add)]
#[reflect(Debug, Default, SubWorldSync)]
struct EntityA;

impl EntityA {
    const IDENTIFIER: Identifier = Identifier::const_new("example:entity_a");

    /// A [`System`] that spawns an [`EntityA`] with a [`SubWorld`].
    fn spawn_with_subworld(world: &mut World) {
        world.spawn_empty();
        world.spawn(Name::new("RootEntity"));

        info!("Building SubWorld \"{}\"", Self::IDENTIFIER);
        let sub = SubWorld::from_world(Self::IDENTIFIER, world).unwrap();
        world.spawn((EntityA, sub));
    }

    /// A [`ComponentHook`](bevy_ecs::component::ComponentHook)
    /// that logs when an [`EntityA`] is added to a world.
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        info!("Spawned EntityA ({}v{})", ctx.entity.generation(), ctx.entity.index());
        if let Some(mut sub) = world.get_mut::<SubWorld>(ctx.entity) {
            info!("EntityA has SubWorld \"{}\"", sub.identifier());
            sub.run_system_once(EntityB::spawn_with_subworld).unwrap();
        } else {
            error!("EntityA has no SubWorld!");
        }
        info!("");
    }
}

impl SubWorldSync for EntityA {
    fn initialize(_: &Identifier, _: &mut World, _: &mut World) {
        info!("Running EntityA's SubWorld init");
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, Component, Reflect)]
#[component(on_add = Self::on_add)]
#[reflect(Debug, Default, SubWorldSync)]
struct EntityB;

impl EntityB {
    const IDENTIFIER: Identifier = Identifier::const_new("example:entity_b");

    /// A [`System`] that spawns an [`EntityA`] with a [`SubWorld`].
    fn spawn_with_subworld(world: &mut World) {
        info!("Building SubWorld \"{}\"", Self::IDENTIFIER);
        let sub = SubWorld::from_world(Self::IDENTIFIER, world).unwrap();
        world.spawn((EntityB, sub));
    }

    /// A [`ComponentHook`](bevy_ecs::component::ComponentHook)
    /// that logs when an [`EntityA`] is added to a world.
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        info!("Spawned EntityB ({}v{})", ctx.entity.generation(), ctx.entity.index());
        if let Some(mut sub) = world.get_mut::<SubWorld>(ctx.entity) {
            info!("EntityB has SubWorld \"{}\"", sub.identifier());
            sub.spawn(Name::new("NestedEntity"));
            sub.spawn(Transform::default());
            sub.spawn(Transform::default());
        } else {
            error!("EntityB has no SubWorld!");
        }
    }
}

impl SubWorldSync for EntityB {
    fn initialize(_: &Identifier, _: &mut World, _: &mut World) {
        info!("Running EntityB's SubWorld init");
    }
}
