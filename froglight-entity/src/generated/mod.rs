//! Generated entity types, data types, and metadata.
//!
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, unused, reason = "Ignore all lints for generated code")]

#[cfg(feature = "bevy")]
use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, lifecycle::HookContext, world::DeferredWorld};

#[cfg(feature = "bevy")]
use crate::{entity::EntityComponentType, prelude::EntityBundle};

macro_rules! generate {
    (@components $($ident:ident($ty:ty) = $variant:ident),* ) => {
        $(
            #[repr(transparent)]
            #[derive(Debug, Clone, PartialEq)]
            #[doc = concat!("The ", stringify!($ident), " entity component.")]
            #[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
            #[cfg_attr(feature = "bevy", component(immutable))]
            #[cfg_attr(feature = "bevy", component(on_insert = super::insert_hook::<$ident>))]
            #[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
            #[cfg_attr(feature = "facet", derive(facet::Facet))]
            pub struct $ident(pub $ty);

            impl crate::entity::EntityComponentType for $ident {
                fn try_from_data(data: &crate::generated::datatype::EntityDataType) -> Option<Self> {
                    if let crate::generated::datatype::EntityDataType::$variant(value) = data {
                        Some($ident(value.clone()))
                    } else {
                        None
                    }
                }

                fn into_data(self) -> crate::generated::datatype::EntityDataType {
                    crate::generated::datatype::EntityDataType::$variant(self.0)
                }
            }

            impl core::ops::Deref for $ident {
                type Target = $ty;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl core::ops::DerefMut for $ident {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<$ty> for $ident {
                #[inline]
                fn from(value: $ty) -> Self {
                    $ident(value)
                }
            }
            impl From<$ident> for $ty {
                #[inline]
                fn from(value: $ident) -> Self {
                    value.0
                }
            }
        )*

        #[cfg(feature = "bevy")]
        pub(super) fn register(app: &mut bevy_app::App) {
            app
                $(.register_type::<$ident>())*;
        }
    };

    (@datatypes $( $name:ident => $ident:ident( $(#[ $($attr:tt)* ])? $ty:ty) ),* $(,)?) => {
        /// An enum containing all entity data types.
        #[repr(u8)]
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
        #[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
        #[cfg_attr(feature = "facet", derive(facet::Facet))]
        pub enum EntityDataType {
            $(
                #[doc = concat!("The [`", stringify!($ty), "`] data type.")]
                $ident($(#[ $($attr)* ])? $ty),
            )*
        }

        impl EntityDataType {
            /// Get the name of this data type.
            #[must_use]
            pub const fn variant_name(&self) -> &'static str {
                match self {
                    $(
                        EntityDataType::$ident(_) => stringify!($ident),
                    )*
                }
            }

            $(
                #[must_use]
                #[doc = concat!("Get the value of this data type as a [`",stringify!($ty),"`], if it is one.\n\nOtherwise, returns `None`.")]
                pub fn $name(&self) -> Option<&$ty> {
                    if let EntityDataType::$ident(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            )*
        }
    };
    (@entities $($ident:ident),* $(,)?) => {
        $(
            #[doc = concat!("The [`", stringify!($ident), "`] entity type.")]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
            #[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
            #[cfg_attr(feature = "facet", derive(facet::Facet))]
            pub struct $ident;
        )*

        /// An enum containing all vanilla entity types.
        #[repr(u8)]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component, bevy_reflect::Reflect))]
        #[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
        #[cfg_attr(feature = "facet", derive(facet::Facet))]
        pub enum VanillaEntity {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] entity type.")]
                $ident,
            )*
        }

        $(
            #[automatically_derived]
            impl From<$ident> for VanillaEntity {
                #[inline]
                fn from(_: $ident) -> Self {
                    VanillaEntity::$ident
                }
            }

            #[automatically_derived]
            impl PartialEq<VanillaEntity> for $ident {
                #[inline]
                fn eq(&self, other: &VanillaEntity) -> bool {
                    matches!(other, VanillaEntity::$ident)
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for VanillaEntity {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    matches!(self, VanillaEntity::$ident)
                }
            }

            #[automatically_derived]
            impl PartialEq<crate::entity::EntityBundle> for $ident {
                #[inline]
                fn eq(&self, other: &crate::entity::EntityBundle) -> bool {
                    other.is_entity::<$ident>()
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for crate::entity::EntityBundle {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    self.is_entity::<$ident>()
                }
            }
        )*

        #[automatically_derived]
        impl PartialEq<crate::entity::EntityBundle> for VanillaEntity {
            #[allow(unreachable_patterns, reason = "Nonexhaustive")]
            fn eq(&self, other: &crate::entity::EntityBundle) -> bool {
                match self {
                    $(
                        VanillaEntity::$ident => other.is_entity::<$ident>(),
                    )*
                    _ => unreachable!("All variants of `VanillaEntity` should be covered in the match arms."),
                }
            }
        }
        #[automatically_derived]
        impl PartialEq<VanillaEntity> for crate::entity::EntityBundle {
            #[inline]
            fn eq(&self, other: &VanillaEntity) -> bool {
                PartialEq::<crate::entity::EntityBundle>::eq(other, self)
            }
        }

        #[cfg(feature = "bevy")]
        pub(super) fn register(app: &mut bevy_app::App) {
            app
                .register_type::<VanillaEntity>()
                $(.register_type::<$ident>())*;
        }
    };

    (@version $version:ident, datatypes: { $($datatype:ident($dataty:ty) = $dataid:literal),* },
        $(
            $ident:ident => {
                ident: $string:literal,
                global: $global:literal,
                components: [
                    $($component:ident = $componentid:literal),*
                ]
            }
        ),*
    ) => {
        $(
            #[automatically_derived]
            impl crate::entity::EntityType<$version> for $ident {
                const METADATA: &'static crate::entity::EntityMetadata = {
                    static METADATA: crate::entity::EntityMetadata = unsafe { crate::entity::EntityMetadata::new::<$ident, $version>(
                        froglight_common::identifier::Identifier::new_static($string),
                        $global,
                    ) };

                    &METADATA
                };

                const COMPONENTS: &'static [core::any::TypeId] = &[
                    $(
                        core::any::TypeId::of::<$component>(),
                    )*
                ];
                const DATASET: crate::entity::EntityDataSet<'static> = crate::entity::EntityDataSet::new_slice(&[]);

                #[cfg(feature = "bevy")]
                #[allow(unused, reason = "Generated code")]
                fn inspect_reflect(dataset: &crate::entity::EntityDataSet, f: &mut dyn FnMut(alloc::boxed::Box<dyn bevy_reflect::PartialReflect>)) {
                    f(alloc::boxed::Box::new(Self));

                    for (index, data) in dataset.to_ref().iter() {
                        match index {
                            $(
                                $componentid if let Some(component) = <$component as crate::entity::EntityComponentType>::try_from_data(data) => {
                                    f(alloc::boxed::Box::new(component));
                                }
                            )*
                            _ => {},
                        }
                    }
                }
                #[cfg(feature = "facet")]
                #[allow(unused, reason = "Generated code")]
                fn inspect_peek(dataset: &crate::entity::EntityDataSet, f: &mut dyn FnMut(facet::Peek<'_, '_>)) {
                    f(facet::Peek::new(&Self));

                    for (index, data) in dataset.to_ref().iter() {
                        match index {
                            $(
                                $componentid if let Some(component) = <$component as crate::entity::EntityComponentType>::try_from_data(data) => {
                                    f(facet::Peek::new(&component));
                                },
                            )*
                            _ => {},
                        }
                    }
                }
            }
        )*

        crate::implement_entities!(
            $version => unsafe {
                crate::storage::EntityStorage::new_static(&[
                    $(
                        <$ident as crate::entity::EntityType<$version>>::METADATA,
                    )*
                ])
            },
            read: { |cursor| {
                let (len, val) = facet_minecraft::deserialize::bytes_to_variable(cursor.as_slice())?;
                cursor.consume(len);

                match val {
                    $(
                        $dataid => match facet_minecraft::from_slice_remainder(cursor.as_slice()) {
                            Ok((value, rem)) => {
                                cursor.consume(cursor.as_slice().len() - rem.len());
                                Ok(crate::generated::datatype::EntityDataType::$datatype(value))
                            }
                            Err(_) => Err(facet_minecraft::deserialize::error::DeserializeValueError::StaticBorrow)
                        }
                    )*
                    _ => Err(facet_minecraft::deserialize::error::DeserializeValueError::StaticBorrow),
                }
            } },
            write: { |(), data, buffer| {
                let mut content = alloc::vec::Vec::with_capacity(8);

                match data {
                    $(
                        crate::generated::datatype::EntityDataType::$datatype(value) => {
                            content.push($dataid);
                            facet_minecraft::to_buffer(value, &mut content).unwrap();
                        }
                    )*
                    _ => todo!(),
                }

                if buffer.write_data(&content) {
                    Ok(())
                } else {
                    Err(facet_minecraft::serialize::error::SerializeIterError::new())
                }
            } }
        );
    };
}

pub mod component;
pub mod datatype;
pub mod entity;

/// Register all generated types with Bevy's reflection system.
#[cfg(feature = "bevy")]
pub(crate) fn register_types(app: &mut bevy_app::App) {
    component::register(app);
    entity::register(app);
}

#[cfg(feature = "bevy")]
fn insert_hook<T: Component + EntityComponentType>(mut world: DeferredWorld, ctx: HookContext) {
    let mut entity = world.entity_mut(ctx.entity);

    let Ok((mut bundle, component)) = entity.get_components_mut::<(&mut EntityBundle, &T)>() else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_entity", "Failed to sync bundle, entity does not have an `EntityBundle`");
        return;
    };

    let Some(index) =
        bundle.metadata().component_tys().iter().position(|ty| *ty == TypeId::of::<T>())
    else {
        #[cfg(feature = "tracing")]
        tracing::warn!(target: "froglight_entity", "Failed to sync bundle, entity should not have a `{}`", core::any::type_name::<T>());
        return;
    };

    // Get the existing data
    let Some((_, existing)) =
        bundle.dataset().to_ref().iter().find(|(i, _)| usize::from(*i) == index)
    else {
        #[cfg(feature = "tracing")]
        tracing::error!(target: "froglight_entity", "Failed to sync bundle, entity should have a `{}` at index {index}", core::any::type_name::<T>());
        #[cfg(feature = "tracing_ext")]
        tracing::debug!(target: "froglight_entity", "Existing dataset: {:?}", bundle.dataset().to_ref());
        return;
    };

    // Create a new data type and apply it if it differs from the existing one
    let component = component.clone().into_data();
    if component != *existing {
        // SAFETY: Id does not change, and the data is guaranteed to be valid
        // as it came from a component with the correct type.
        let bundle = unsafe { bundle.dataset_mut().to_mut() };

        if let Some((_, existing)) = bundle.iter_mut().find(|(i, _)| usize::from(*i) == index) {
            unsafe { *existing = component };
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_entity", "Entity missing bundle component, should have a `{}` at index {index}", core::any::type_name::<T>());
            bundle.push((index.try_into().unwrap(), component));
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;
