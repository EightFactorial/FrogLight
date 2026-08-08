use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::TypePath;
use froglight_entity::prelude::{EntityId, EntityUuid};
use froglight_world::prelude::ChunkPos;
use hashbrown::hash_map::{HashMap, Iter};

use crate::prelude::SessionInstance;

/// A trait for types that are part of a [`SessionInstance`].
#[allow(dead_code, unreachable_pub, reason = "Private")]
pub trait InstanceData: Component + TypePath + Sized {
    /// Get an iterator over all data-entity pairs in the [`SessionInstance`].
    #[must_use]
    fn iter(instance: &SessionInstance) -> Iter<'_, Self, Entity>;

    /// Query the [`SessionInstance`] for the associated [`Entity`].
    #[must_use]
    fn query(&self, instance: &SessionInstance) -> Option<Entity>;

    /// Insert the associated [`Entity`] into the [`SessionInstance`].
    ///
    /// Returns the previous entity if one existed.
    fn insert(self, instance: &mut SessionInstance, entity: Entity) -> Option<Entity>;

    /// Remove the associated [`Entity`] from the [`SessionInstance`].
    ///
    /// Returns `true` if an entity was removed.
    fn remove(&self, instance: &mut SessionInstance) -> bool;
}

// -------------------------------------------------------------------------------------------------

macro_rules! create_data {
    ($ty:ty => $token:tt: $query_fn:ident, $map_fn:ident, $iter_fn:ident) => {
        impl $crate::instance::data::InstanceData for $ty {
            #[inline]
            fn iter(instance: &SessionInstance) -> Iter<'_, Self, Entity> { instance.$token.iter() }

            #[inline]
            fn query(&self, instance: &SessionInstance) -> Option<Entity> {
                instance.$token.get(self).copied()
            }

            #[inline]
            fn insert(self, instance: &mut SessionInstance, entity: Entity) -> Option<Entity> {
                instance.entity.insert(entity);
                instance.$token.insert(self, entity)
            }

            #[inline]
            fn remove(&self, instance: &mut SessionInstance) -> bool {
                if let Some(entity) = instance.$token.remove(self) {
                    // Remove from the `entity` set if not present in any of the other maps.
                    if !(
                            instance.entity_id.values().any(|v| &entity == v)
                            || instance.entity_uuid.values().any(|v| &entity == v)
                            || instance.chunk_pos.values().any(|v| &entity == v)
                        )
                    {
                        instance.entity.remove(&entity);
                    }

                    true
                } else {
                    false
                }
            }
        }

        impl SessionInstance {
            #[inline]
            #[must_use]
            #[doc = concat!("Query the [`SessionInstance`] for the associated [`Entity`] of a given [`", stringify!($ty), "`].")]
            pub fn $query_fn(&self, data: &$ty) -> Option<Entity> { self.$token.get(data).copied() }

            #[inline]
            #[must_use]
            #[doc = concat!("Get a reference to the [`HashMap`] of [`", stringify!($ty), "`]-[`Entity`] pairs in the [`SessionInstance`].")]
            pub const fn $map_fn(&self) -> &HashMap<$ty, Entity, foldhash::fast::FixedState> { &self.$token }

            #[inline]
            #[must_use]
            #[doc = concat!("Get an iterator over all [`Entity`]-[`", stringify!($ty), "`] pairs in the [`SessionInstance`].")]
            pub fn $iter_fn(&self) -> Iter<'_, $ty, Entity> { self.$token.iter() }
        }
    };
}

create_data!(EntityId => entity_id: get_id, id_map, iter_id);
create_data!(EntityUuid => entity_uuid: get_uuid, uuid_map, iter_uuid);
create_data!(ChunkPos => chunk_pos: get_chunk, chunk_map, iter_chunk);
