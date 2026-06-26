use bevy_ecs::{
    change_detection::MaybeLocation, entity::Entity, lifecycle::HookContext,
    relationship::RelationshipHookMode, world::DeferredWorld,
};
use bevy_reflect::FromType;

use crate::instance::{
    data::InstanceData,
    hook::{discard_hook, insert_hook},
};

/// Reflection data
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReflectSession {
    insert_fn: fn(DeferredWorld, Entity, MaybeLocation),
    discard_fn: fn(DeferredWorld, Entity, MaybeLocation),
}

impl ReflectSession {
    /// Trigger the insert hook for the given entity.
    #[track_caller]
    pub(crate) fn on_insert(&self, entity: Entity, world: DeferredWorld) {
        (self.insert_fn)(world, entity, MaybeLocation::caller());
    }

    /// Trigger the discard hook for the given entity.
    #[track_caller]
    pub(crate) fn on_discard(&self, entity: Entity, world: DeferredWorld) {
        (self.discard_fn)(world, entity, MaybeLocation::caller());
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: InstanceData + Clone> FromType<T> for ReflectSession {
    fn from_type() -> Self {
        const HOOK_MODE: RelationshipHookMode = RelationshipHookMode::Run;

        Self {
            insert_fn: |world, entity, caller| {
                if world.get::<T>(entity).is_none() {
                    return;
                }

                let component_id = world.component_id::<T>().unwrap();
                let ctx =
                    HookContext { entity, component_id, caller, relationship_hook_mode: HOOK_MODE };

                insert_hook::<T>(world, ctx);
            },
            discard_fn: |world, entity, caller| {
                if world.get::<T>(entity).is_none() {
                    return;
                }

                let component_id = world.component_id::<T>().unwrap();
                let ctx =
                    HookContext { entity, component_id, caller, relationship_hook_mode: HOOK_MODE };

                discard_hook::<T>(world, ctx);
            },
        }
    }
}
