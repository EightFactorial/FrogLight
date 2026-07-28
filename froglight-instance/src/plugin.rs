//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::{prelude::*, resource::IsResource};
use froglight_entity::prelude::{EntityId, EntityUuid};
use froglight_world::prelude::{ChunkPos, SharedChunk};

use crate::{
    instance::{
        hook::{discard_hook, insert_hook},
        reflect::ReflectSession,
    },
    prelude::*,
};

/// A [`Plugin`] that ...
pub struct InstancePlugin;

impl Plugin for InstancePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SessionInstance>();

        app.register_type::<EntityId>().register_type_data::<EntityId, ReflectSession>();
        app.register_type::<EntityUuid>().register_type_data::<EntityUuid, ReflectSession>();
        app.register_type::<ChunkPos>().register_type_data::<ChunkPos, ReflectSession>();

        let world = app.world_mut();

        world
            .register_component_hooks::<EntityId>()
            .on_insert(insert_hook::<EntityId>)
            .on_discard(discard_hook::<EntityId>);
        world
            .register_component_hooks::<EntityUuid>()
            .on_insert(insert_hook::<EntityUuid>)
            .on_discard(discard_hook::<EntityUuid>);
        world
            .register_component_hooks::<ChunkPos>()
            .on_insert(insert_hook::<ChunkPos>)
            .on_discard(discard_hook::<ChunkPos>);
    }
}

impl InstancePlugin {
    /// A [`System`] that applies [`BlockEditQueue`]s to [`SessionInstance`]s.
    ///
    /// # Note
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    pub fn apply_blockedits(
        query: Query<(&mut BlockEditQueue, &SessionInstance), Without<IsResource>>,
        mut chunks: Query<&mut SharedChunk, Without<IsResource>>,
    ) {
        for (mut queue, instance) in query {
            queue.apply(instance, chunks.reborrow());
        }
    }
}

// /// Apply the bot's [`BlockEditQueue`].
// #[expect(clippy::type_complexity, reason = "Complex Query Filters")]
// fn apply_blockedit_queue(
//     bot: Single<
//         (&SessionInstance, &mut BlockEditQueue),
//         (With<ClientConnection>, Without<SharedChunk>, Without<IsResource>),
//     >,
//     chunks: Query<&mut SharedChunk>,
// ) {
//     let (instance, mut edit_queue) = bot.into_inner();
//     edit_queue.apply(instance, chunks);
// }
