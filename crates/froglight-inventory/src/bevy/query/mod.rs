//! TODO

use bevy_ecs::{
    prelude::*,
    query::{QueryData, QueryEntityError, QueryFilter, ReadOnlyQueryData, WorldQuery},
    system::SystemParam,
};
use bevy_reflect::prelude::*;

mod player;

/// A trait for requesting types of inventories.
pub trait InventoryRequest<
    Filter: QueryFilter + 'static,
    Marker: InventoryMarker<Self::Resource> + 'static,
>: 'static
{
    /// The required query data for the inventory.
    type Query: QueryData;
    /// The required resource to access the inventory.
    type Resource: Resource;

    /// Data required to access a specific inventory.
    type Accessor;
    /// The result of accessing the inventory.
    type Result<'a>;

    /// Access the inventory.
    fn access<'a>(
        accessor: Self::Accessor,
        query: <Self::Query as WorldQuery>::Item<'a>,
        resource: <Marker as InventoryMarker<Self::Resource>>::Reference<'a>,
    ) -> Self::Result<'a>;
}

// -------------------------------------------------------------------------------------------------

/// A marker for the access level of an [`InventoryRequest`].
pub trait InventoryMarker<Type>: 'static {
    /// The component type for the marker.
    type Component<'a>: AsRef<Type>
    where Type: Component + 'a;

    /// The reference type for the marker.
    type Reference<'a>
    where Type: 'a;
}

/// A marker for access to an inventory.
///
/// Provides read-only references to types.
pub struct ReadOnly;
impl<Type> InventoryMarker<Type> for ReadOnly {
    /// The component type for the marker.
    type Component<'a>
        = Ref<'a, Type>
    where Type: Component + 'a;
    /// The reference type for the marker.
    type Reference<'a>
        = &'a Type
    where Type: 'a;
}

/// A marker for mutable access to an inventory.
///
/// Provides mutable references to types.
pub struct Mutable;
impl<Type> InventoryMarker<Type> for Mutable {
    /// The component type for the marker.
    type Component<'a>
        = Mut<'a, Type>
    where Type: Component + 'a;
    /// The reference type for the marker.
    type Reference<'a>
        = &'a mut Type
    where Type: 'a;
}

// -------------------------------------------------------------------------------------------------

/// A dummy for [`InventoryRequest`]s that do not require a [`Resource`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Debug, Default, PartialEq, Hash, Resource)]
pub struct Dummy;

// -------------------------------------------------------------------------------------------------

/// A [`SystemParam`] for accessing an entity's inventory.
#[derive(SystemParam)]
pub struct Inventory<
    'w,
    's,
    Type: InventoryRequest<Filter, ReadOnly>,
    Filter: QueryFilter + 'static = (),
> where Type::Query: ReadOnlyQueryData
{
    query: Query<'w, 's, <Type as InventoryRequest<Filter, ReadOnly>>::Query, Filter>,
    resource: Res<'w, <Type as InventoryRequest<Filter, ReadOnly>>::Resource>,
}

impl<'a, Type: InventoryRequest<Filter, ReadOnly>, Filter: QueryFilter + 'static>
    Inventory<'_, '_, Type, Filter>
where Type::Query: ReadOnlyQueryData
{
    /// Get the inventory of an [`Entity`].
    ///
    /// # Errors
    /// Returns an error if the entity does not have the required inventory.
    pub fn get(
        &'a self,
        entity: Entity,
        data: Type::Accessor,
    ) -> Result<Type::Result<'a>, QueryEntityError<'a>> {
        self.query.get(entity).map(|queried| Type::access(data, queried, &self.resource))
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`SystemParam`] for accessing an entity's inventory mutably.
#[derive(SystemParam)]
pub struct InventoryMut<
    'w,
    's,
    Type: InventoryRequest<Filter, Mutable>,
    Filter: QueryFilter + 'static = (),
> {
    query: Query<'w, 's, <Type as InventoryRequest<Filter, Mutable>>::Query, Filter>,
    resource: ResMut<'w, <Type as InventoryRequest<Filter, Mutable>>::Resource>,
}

impl<
    'a,
    Type: InventoryRequest<
            Filter,
            Mutable,
            Resource = <Type as InventoryRequest<Filter, ReadOnly>>::Resource,
        > + InventoryRequest<Filter, ReadOnly>,
    Filter: QueryFilter + 'static,
> InventoryMut<'_, '_, Type, Filter>
where <<<Type as InventoryRequest<Filter, Mutable>>::Query as QueryData>::ReadOnly as WorldQuery>::Item<'a>: Into<<<Type as InventoryRequest<Filter, ReadOnly>>::Query as WorldQuery>::Item<'a>>
{
    /// Get the inventory of an [`Entity`].
    ///
    /// # Errors
    /// Returns an error if the entity does not have the required inventory.
    pub fn get(
        &'a self,
        entity: Entity,
        data: <Type as InventoryRequest<Filter, ReadOnly>>::Accessor,
    ) -> Result<
        <Type as InventoryRequest<Filter, ReadOnly>>::Result<'a>,
        QueryEntityError<'a>,
    > {
        self.query.get(entity).map(|queried| {
            <Type as InventoryRequest<Filter, ReadOnly>>::access(
                data,
                queried.into(),
                &self.resource,
            )
        })
    }
}

impl<'a, Type: InventoryRequest<Filter, Mutable>, Filter: QueryFilter + 'static>
    InventoryMut<'_, '_, Type, Filter>
{
    /// Get the inventory of an [`Entity`] mutably.
    ///
    /// # Errors
    /// Returns an error if the entity does not have the required inventory.
    pub fn get_mut(
        &'a mut self,
        entity: Entity,
        data: Type::Accessor,
    ) -> Result<Type::Result<'a>, QueryEntityError<'a>> {
        self.query.get_mut(entity).map(|queried| Type::access(data, queried, &mut self.resource))
    }
}
