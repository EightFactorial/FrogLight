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
    fn access<'request: 'access, 'access>(
        accessor: Self::Accessor,
        query: <Self::Query as WorldQuery>::Item<'request>,
        resource: <Marker as InventoryMarker<Self::Resource>>::Reference<'request>,
    ) -> Self::Result<'access>;
}

// -------------------------------------------------------------------------------------------------

/// A marker for the access level of an [`InventoryRequest`].
pub trait InventoryMarker<Type>: 'static {
    /// The component type for the marker.
    type Component<'request>
    where Type: Component + 'request;

    /// The reference type for the marker.
    type Reference<'request>
    where Type: 'request;
}

/// A marker for access to an inventory.
///
/// Provides read-only references to types.
pub struct ReadOnly;
impl<Type> InventoryMarker<Type> for ReadOnly {
    /// The component type for the marker.
    type Component<'request>
        = Ref<'request, Type>
    where Type: Component + 'request;
    /// The reference type for the marker.
    type Reference<'request>
        = &'request Type
    where Type: 'request;
}

/// A marker for mutable access to an inventory.
///
/// Provides mutable references to types.
pub struct Mutable;
impl<Type> InventoryMarker<Type> for Mutable {
    /// The component type for the marker.
    type Component<'request>
        = Mut<'request, Type>
    where Type: Component + 'request;
    /// The reference type for the marker.
    type Reference<'request>
        = &'request mut Type
    where Type: 'request;
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

impl<'request, Type: InventoryRequest<Filter, ReadOnly>, Filter: QueryFilter + 'static>
    Inventory<'_, '_, Type, Filter>
where Type::Query: ReadOnlyQueryData
{
    /// Get the inventory of an [`Entity`].
    ///
    /// # Errors
    /// Returns an error if the entity does not have the required inventory.
    pub fn get(
        &'request self,
        entity: Entity,
        data: Type::Accessor,
    ) -> Result<Type::Result<'request>, QueryEntityError<'request>> {
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
    'request,
    Type: InventoryRequest<
            Filter,
            Mutable,
            Resource = <Type as InventoryRequest<Filter, ReadOnly>>::Resource,
        > + InventoryRequest<Filter, ReadOnly>,
    Filter: QueryFilter + 'static,
> InventoryMut<'_, '_, Type, Filter>
where <<<Type as InventoryRequest<Filter, Mutable>>::Query as QueryData>::ReadOnly as WorldQuery>::Item<'request>: Into<<<Type as InventoryRequest<Filter, ReadOnly>>::Query as WorldQuery>::Item<'request>>
{
    /// Get the inventory of an [`Entity`].
    ///
    /// # Errors
    /// Returns an error if the entity does not have the required inventory.
    pub fn get(
        &'request self,
        entity: Entity,
        data: <Type as InventoryRequest<Filter, ReadOnly>>::Accessor,
    ) -> Result<
        <Type as InventoryRequest<Filter, ReadOnly>>::Result<'request>,
        QueryEntityError<'request>,
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

impl<'request, Type: InventoryRequest<Filter, Mutable>, Filter: QueryFilter + 'static>
    InventoryMut<'_, '_, Type, Filter>
{
    /// Get the inventory of an [`Entity`] mutably.
    ///
    /// # Errors
    /// Returns an error if the entity does not have the required inventory.
    pub fn get_mut(
        &'request mut self,
        entity: Entity,
        data: Type::Accessor,
    ) -> Result<Type::Result<'request>, QueryEntityError<'request>> {
        self.query.get_mut(entity).map(|queried| Type::access(data, queried, &mut self.resource))
    }
}
