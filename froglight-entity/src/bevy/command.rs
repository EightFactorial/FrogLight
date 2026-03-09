use bevy_ecs::{system::EntityCommand, world::EntityWorldMut};

use crate::{entity::EntityDataSet, prelude::EntityBundle};

/// A [`EntityCommand`] that applies an [`EntityDataSet`] to an entity.
#[derive(Debug, Clone)]
pub struct ApplyEntityDataSet(EntityDataSet<'static>);

impl ApplyEntityDataSet {
    /// Creates a new [`ApplyEntityDataSet`] command.
    #[inline]
    #[must_use]
    pub const fn new(dataset: EntityDataSet<'static>) -> Self { Self(dataset) }
}

impl EntityCommand for ApplyEntityDataSet {
    fn apply(self, mut entity: EntityWorldMut) {
        if let Some(bundle) = entity.get_mut::<EntityBundle>() {
            // SAFETY: TODO
            let bundle = unsafe { EntityBundle::new_unchecked(self.0, bundle.metadata()) };
            entity.insert(bundle);
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_entity", "Attempted to apply an `EntityDataSet` to an entity without an `EntityBundle`");
        }
    }
}
