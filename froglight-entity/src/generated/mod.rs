//! Generated entity types, data types, and metadata.
//!
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, unused, reason = "Ignore all lints for generated code")]

macro_rules! generate {
    (@components $($ident:ident($ty:ty) = $variant:ident),* ) => {
        $(
            #[repr(transparent)]
            #[derive(Debug, Clone, PartialEq)]
            #[doc = concat!("The ", stringify!($ident), " entity component.")]
            #[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect, bevy_ecs::component::Component))]
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

        $(
            #[automatically_derived]
            impl From<$ty> for EntityDataType {
                #[inline]
                fn from(value: $ty) -> Self {
                    EntityDataType::$ident(value)
                }
            }
        )*
    };
    (@entities $($ident:ident),* $(,)?) => {
        $(
            #[doc = concat!("The [`", stringify!($ident), "`] entity type.")]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect, bevy_ecs::component::Component))]
            #[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
            #[cfg_attr(feature = "facet", derive(facet::Facet))]
            pub struct $ident;
        )*

        /// An enum containing all vanilla entity types.
        #[repr(u8)]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect, bevy_ecs::component::Component))]
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

                #[cfg(feature = "bevy")]
                #[allow(unused, reason = "Generated code")]
                fn inspect_reflect(dataset: &crate::entity::EntityDataSet, f: &mut dyn FnMut(&dyn bevy_reflect::PartialReflect)) {
                    f(&Self);

                    for (index, data) in dataset.to_ref().iter() {
                        match index {
                            $(
                                $componentid => {
                                    let component = <$component as crate::entity::EntityComponentType>::try_from_data(data).unwrap();
                                    f(&component);
                                }
                            )*
                            _ => todo!(),
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
                                $componentid => {
                                    let component = <$component as crate::entity::EntityComponentType>::try_from_data(data).unwrap();
                                    f(facet::Peek::new(&component));
                                },
                            )*
                            _ => todo!(),
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
                let mut slice: &[u8] = cursor.as_slice();
                let remainder: &[u8];
                let data: crate::generated::datatype::EntityDataType;

                match cursor.take(1)?[0] {
                    $(
                        $dataid => {
                            let (value, rem) = facet_minecraft::from_slice_remainder(&mut slice).unwrap();
                            data = crate::generated::datatype::EntityDataType::$datatype(value);
                            remainder = rem;
                        }
                    )*
                    _ => todo!(),
                }

                cursor.consume(slice.len() - remainder.len());
                Ok(data)
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

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;
