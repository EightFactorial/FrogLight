//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::{entity::EntityHashMap, prelude::*, resource::IsResource};
use froglight_entity::prelude::{EntityId, EntityUuid};
use froglight_world::prelude::{ChunkPos, SharedChunk};
use parking_lot::Mutex;

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
    /// This system has a lower overhead compared to
    /// [`InstancePlugin::par_apply_blockedits`], but it is **much** slower for
    /// large numbers of [`BlockEditQueue`]s. Use this if, for example, your
    /// program is a client or bot with a single connection.
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    pub fn apply_blockedits(
        query: Query<(&mut BlockEditQueue, &SessionInstance), Without<IsResource>>,
        mut chunks: Query<&mut SharedChunk, Without<IsResource>>,
    ) {
        for (mut queue, instance) in query {
            queue.apply_to(instance, chunks.reborrow());
        }
    }

    /// A [`System`] that applies [`BlockEditQueue`]s to [`SessionInstance`]s in
    /// parallel.
    ///
    /// # Note
    ///
    /// This system has a higher overhead compared to
    /// [`InstancePlugin::apply_blockedits`], but it **much** faster for large
    /// numbers of [`BlockEditQueue`]s. Use this if, for example, your program
    /// has 10+ bots running.
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    pub fn par_apply_blockedits(
        mut query: Query<(&mut BlockEditQueue, &SessionInstance), Without<IsResource>>,
        mut chunks: Query<&mut SharedChunk, Without<IsResource>>,
        cache: Local<Mutex<EntityHashMap<SharedChunk>>>,
    ) {
        // Apply all `BlockEditQueue`s in parallel.
        let chunks_readonly = chunks.as_readonly();
        query.par_iter_mut().for_each(|(mut queue, instance)| {
            let modified = queue.apply_clone(instance, chunks_readonly);
            cache.lock().extend(modified);
        });

        // Replace all `SharedChunk`s.
        for (entity, new) in cache.lock().drain() {
            if let Ok(mut old) = chunks.get_mut(entity) {
                *old = new;
            }
        }
    }
}
