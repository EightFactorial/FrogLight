#[cfg(feature = "bevy")]
use alloc::{borrow::Cow, boxed::Box, string::String};
use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::{
    component::Component, lifecycle::HookContext, reflect::ReflectCommandExt,
    reflect::ReflectComponent, world::DeferredWorld,
};
#[cfg(feature = "bevy")]
use bevy_reflect::{PartialReflect, Reflect};
#[cfg(feature = "facet")]
use facet::Peek;
use froglight_common::prelude::*;

#[cfg(feature = "bevy")]
use crate::{bevy::EntityBundleEvent, entity::EntityAabb};
use crate::{
    entity::{EntityDataSet, GlobalId, metadata::EntityMetadata},
    generated::datatype::EntityDataType,
    prelude::EntityVersion,
};

/// A bundle of data and metadata for an entity.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", component(on_insert = Self::insert_hook, on_replace = Self::replace_hook))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq, Component))]
pub struct EntityBundle {
    dataset: EntityDataSet<'static>,
    reference: &'static EntityMetadata,
}

impl EntityBundle {
    /// Create a new [`EntityBundle`] of the given type.
    #[inline]
    #[must_use]
    pub fn new<E: EntityType<V>, V: EntityVersion>() -> Self { Self::new_from(E::METADATA) }

    /// Create a new [`EntityBundle`] from the given metadata.
    #[inline]
    #[must_use]
    pub fn new_from(metadata: &'static EntityMetadata) -> Self {
        EntityBundle { dataset: metadata.default_data(), reference: metadata }
    }

    /// Create a new [`EntityBundle`] from the given [`EntityDataSet`] and
    /// [`EntityMetadata`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given `dataset` is valid for the
    /// metadata.
    #[must_use]
    pub const unsafe fn new_unchecked(
        dataset: EntityDataSet<'static>,
        metadata: &'static EntityMetadata,
    ) -> Self {
        Self { dataset, reference: metadata }
    }

    /// Get a reference to the [`EntityDataSet`] of this entity.
    #[inline]
    #[must_use]
    pub const fn dataset(&self) -> &EntityDataSet<'static> { &self.dataset }

    /// Get a mutable reference to the [`EntityDataSet`] of this entity.
    ///
    /// # Safety
    ///
    /// The caller must ensure the dataset is still valid for this entity after
    /// mutation.
    #[inline]
    #[must_use]
    pub const unsafe fn dataset_mut(&mut self) -> &mut EntityDataSet<'static> { &mut self.dataset }

    /// Apply the given [`EntityDataSet`] to this entity.
    ///
    /// # Errors
    ///
    /// Returns the given dataset if it was not compatible with this entity.
    pub fn with_dataset(mut self, dataset: EntityDataSet<'_>) -> Result<Self, EntityDataSet<'_>> {
        for (other_id, other_data) in dataset.to_ref() {
            // Find the corresponding data in this dataset.
            if let Some((_, this_data)) =
                self.dataset.to_mut().iter_mut().find(|(id, _)| id == other_id)
            {
                // If the data types match, set the data.
                if core::mem::discriminant(this_data) == core::mem::discriminant(other_data) {
                    *this_data = other_data.clone();
                } else {
                    #[cfg(feature = "tracing_ext")]
                    tracing::error!(target: "froglight_entity", "Incompatible dataset, expected index {other_id} to be \"{}\", found \"{}\"", this_data.variant_name(), other_data.variant_name());
                    return Err(dataset);
                }
            } else {
                // Otherwise, push the new data to the dataset.
                self.dataset.to_mut().push((*other_id, other_data.clone()));
            }
        }
        Ok(self)
    }

    /// Get the string identifier of this entity.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { self.reference.identifier() }

    /// Get the [`EntityMetadata`] of this entity.
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &'static EntityMetadata { self.reference }

    /// Get the [`GlobalId`] of this entity type.
    #[inline]
    #[must_use]
    pub fn global_id(&self) -> GlobalId { self.reference.global_id() }

    /// Inspect this entity's data using the given function.
    ///
    /// Requires the `bevy` feature to be enabled.
    #[cfg(feature = "bevy")]
    pub fn inspect_reflect(&self, f: impl FnMut(Box<dyn PartialReflect>)) {
        self.reference.inspect_reflect(&self.dataset, f);
    }

    /// Inspect this entity's data using the given function.
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    pub fn inspect_peek(&self, f: impl FnMut(Peek<'_, '_>)) {
        self.reference.inspect_peek(&self.dataset, f);
    }

    /// Returns `true` if this entity is of type `E`.
    #[inline]
    #[must_use]
    pub fn is_entity<E: 'static>(&self) -> bool { self.reference.is_entity::<E>() }

    /// Returns `true` if this entity is of version `V`.
    #[inline]
    #[must_use]
    pub fn is_version<V: 'static>(&self) -> bool { self.reference.is_version::<V>() }

    /// Get the [`TypeId`] of the entity type.
    #[inline]
    #[must_use]
    pub const fn entity_ty(&self) -> TypeId { self.reference.entity_ty() }

    /// Get the [`TypeId`] of the version type.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.reference.version_ty() }
}

#[cfg(feature = "bevy")]
impl EntityBundle {
    fn insert_hook(mut world: DeferredWorld, ctx: HookContext) {
        let (entities, mut commands) = world.entities_and_commands();
        let Ok(entity) = entities.get(ctx.entity) else { return };

        let bundle = entity.get::<Self>().unwrap();

        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_entity", "Applying EntityBundle \"{}\" to Entity {}", bundle.identifier(), ctx.entity);

        let mut commands = commands.entity(ctx.entity);
        bundle.inspect_reflect(|c| {
            commands.insert_reflect(c);
        });

        commands.insert(*bundle.metadata().aabb());
        commands.trigger(EntityBundleEvent::new);
    }

    fn replace_hook(mut world: DeferredWorld, ctx: HookContext) {
        let (entities, mut commands) = world.entities_and_commands();
        let Ok(entity) = entities.get(ctx.entity) else { return };

        let bundle = entity.get::<Self>().unwrap();

        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_entity", "Removing EntityBundle \"{}\" from Entity {}", bundle.identifier(), ctx.entity);

        let mut commands = commands.entity(ctx.entity);
        bundle.inspect_reflect(|c| {
            commands.remove_reflect(Cow::Owned(String::from(c.reflect_type_path())));
        });

        commands.remove::<EntityAabb>();
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all entity types.
pub trait EntityType<V: EntityVersion>: 'static {
    /// The [`EntityMetadata`] for this entity type.
    const METADATA: &'static EntityMetadata;
    /// The [`TypeId`]s of the components that make up this entity.
    const COMPONENTS: &'static [TypeId];
    /// The default [`EntityDataSet`] for this entity type.
    const DATASET: EntityDataSet<'static>;

    /// Inspect this entity's data using the given function.
    ///
    /// Requires the `bevy` feature to be enabled.
    #[cfg(feature = "bevy")]
    fn inspect_reflect(dataset: &EntityDataSet, f: &mut dyn FnMut(Box<dyn PartialReflect>));

    /// Inspect this entity's data using the given function.
    ///
    /// Requires the `facet` feature to be enabled.
    #[cfg(feature = "facet")]
    fn inspect_peek(dataset: &EntityDataSet, f: &mut dyn FnMut(Peek<'_, '_>));
}

/// A trait implemented by all entity component types.
pub trait EntityComponentType: Clone + Sized + 'static {
    /// Try to create the component from the given [`EntityDataType`].
    fn try_from_data(data: &EntityDataType) -> Option<Self>;
    /// Convert this component into an [`EntityDataType`].
    fn into_data(self) -> EntityDataType;
}
