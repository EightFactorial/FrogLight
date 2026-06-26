use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::TypePath;
use froglight_entity::prelude::{EntityId, EntityUuid};
use froglight_world::prelude::ChunkPos;
use hashbrown::hash_map::Iter;

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
    ($ty:ty => $token:tt ( $iter_fn:ident $query_fn:ident ) ) => {
        impl $crate::instance::data::InstanceData for $ty {
            #[inline]
            fn iter(instance: &SessionInstance) -> Iter<'_, Self, Entity> { instance.$token.iter() }

            #[inline]
            fn query(&self, instance: &SessionInstance) -> Option<Entity> {
                instance.$token.get(self).copied()
            }

            #[inline]
            fn insert(self, instance: &mut SessionInstance, entity: Entity) -> Option<Entity> {
                instance.$token.insert(self, entity)
            }

            #[inline]
            fn remove(&self, instance: &mut SessionInstance) -> bool {
                instance.$token.remove(self).is_some()
            }
        }

        impl SessionInstance {
            #[inline]
            #[must_use]
            #[doc = concat!("Get an iterator over all [`Entity`]-[`", stringify!($ty), "`] pairs in the [`SessionInstance`].")]
            pub fn $iter_fn(&self) -> Iter<'_, $ty, Entity> { self.$token.iter() }

            #[inline]
            #[must_use]
            #[doc = concat!("Query the [`SessionInstance`] for the associated [`Entity`] of a given [`", stringify!($ty), "`].")]
            pub fn $query_fn(&self, data: &$ty) -> Option<Entity> { self.$token.get(data).copied() }
        }
    };
}

create_data!(EntityId => entity_id (iter_id query_id));
create_data!(EntityUuid => entity_uuid (iter_uuid query_uuid));
create_data!(ChunkPos => chunk_pos (iter_chunk query_chunk));
