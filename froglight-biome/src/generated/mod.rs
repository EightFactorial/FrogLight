//! Generated biome types, attributes, and features.
//! 
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

macro_rules! generate {
    (
        @attributes @single
        @newtype $ident_lit:literal $ident:ident Vec<$ty:ty>
        $(
            => { $( $extra_ident:ident { $( $extra_name:tt : $extra_ty:ty ),+ } ),+ }
        )?
    ) => {
        #[repr(transparent)]
        #[doc = concat!("The [`", stringify!($ident), "`] biome attribute type.")]
        #[derive(Debug, Clone, PartialEq, Facet)]
        pub struct $ident(pub Vec<$ty>);
        $($(
            generate! {
                @attributes @struct
                $extra_ident { $( $extra_name : $extra_ty ),+ }
            }
        )*)*
    };
    (
        @attributes @single
        @newtype $ident_lit:literal $ident:ident $ty:ty
        $(
            => { $( $extra_ident:ident { $( $extra_name:tt : $extra_ty:ty ),+ } ),+ }
        )?
    ) => {
        #[repr(transparent)]
        #[doc = concat!("The [`", stringify!($ident), "`] biome attribute type.")]
        #[derive(Debug, Clone, PartialEq, Facet)]
        pub struct $ident(pub $ty);
        $($(
            generate! {
                @attributes @struct
                $extra_ident { $( $extra_name : $extra_ty ),+ }
            }
        )*)*
    };

    (
        @attributes @single
        @object $ident_lit:literal $ident:ident
        { $( $field_name:tt : $field_ty:ty ),+ }
        $(
            => { $( $extra_ident:ident { $( $extra_name:tt : $extra_ty:ty ),+ } ),+ }
        )?
    ) => {
        #[doc = concat!("The [`", stringify!($ident), "`] biome attribute type.")]
        #[derive(Debug, Clone, PartialEq, Facet)]
        pub struct $ident {
            $(
                #[doc = concat!("The `", stringify!($field_name), "` field")]
                #[facet(default)]
                pub $field_name: Option<$field_ty>
            ),+
        }
        $($(
            generate! {
                @attributes @struct
                $extra_ident { $( $extra_name : $extra_ty ),+ }
            }
        )*)*
    };

    (
        @attributes @struct
        $ident:ident { $( $field:ident : $ty:ty ),+ }
    ) => {
        #[doc = concat!("The [`", stringify!($ident), "`] attribute data type.")]
        #[derive(Debug, Clone, PartialEq, Facet)]
        pub struct $ident {
            $(
                #[doc = concat!("The `", stringify!($field), "` field")]
                pub $field: $ty,
            )*
        }
    };

    (
        @attributes
        $(
            @$token:tt $ident_lit:literal $ident:ident
            $(
                $newtype_ty:ty
            )?
            $(
                { $( $field_name:tt : $field_ty:ty ),+ }
            )?
            $(
                => { $( $extra_ident:ident { $( $extra_name:tt : $extra_ty:ty ),+ } ),+ }
            )?
            ,
        )*
    )
    => {
        $(
            generate! {
                @attributes @single
                @$token $ident_lit $ident
                $($newtype_ty)?
                $({ $( $field_name : $field_ty ),+ })?
                $( => { $( $extra_ident { $( $extra_name : $extra_ty ),+ } ),+ } )?
            }

            impl crate::biome::AttributeType for $ident {
                const IDENTIFIER: froglight_common::identifier::Identifier<'static> = froglight_common::identifier::Identifier::new_static($ident_lit);
            }
        )*
    };

    (@biomes $($ident:ident),* $(,)?) => {
        $(
            #[doc = concat!("The [`", stringify!($ident), "`] biome type.")]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $ident;
        )*

        /// An enum containing all vanilla biome types.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum VanillaBiome {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] biome type.")]
                $ident,
            )*
        }

        $(
            #[automatically_derived]
            impl From<$ident> for VanillaBiome {
                #[inline]
                fn from(_: $ident) -> Self {
                    VanillaBiome::$ident
                }
            }

            #[automatically_derived]
            impl PartialEq<VanillaBiome> for $ident {
                #[inline]
                fn eq(&self, other: &VanillaBiome) -> bool {
                    matches!(other, VanillaBiome::$ident)
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for VanillaBiome {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    matches!(self, VanillaBiome::$ident)
                }
            }

            #[automatically_derived]
            impl PartialEq<crate::biome::Biome> for $ident {
                #[inline]
                fn eq(&self, other: &crate::biome::Biome) -> bool {
                    other.is_biome::<$ident>()
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for crate::biome::Biome {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    self.is_biome::<$ident>()
                }
            }
        )*

        #[automatically_derived]
        impl PartialEq<crate::biome::Biome> for VanillaBiome {
            #[allow(unreachable_patterns, reason = "Nonexhaustive")]
            fn eq(&self, other: &crate::biome::Biome) -> bool {
                match self {
                    $(
                        VanillaBiome::$ident => other.is_biome::<$ident>(),
                    )*
                    _ => unreachable!("All variants of `VanillaBiome` should be covered in the match arms."),
                }
            }
        }
        #[automatically_derived]
        impl PartialEq<VanillaBiome> for crate::biome::Biome {
            #[inline]
            fn eq(&self, other: &VanillaBiome) -> bool {
                PartialEq::<crate::biome::Biome>::eq(other, self)
            }
        }
    };

    (@feature) => {};

    (@version $version:ident, $($ident:ident => {
        ident: $string:literal,
        global: $global:literal,
        prop: { foliage: $foliage:literal, dry_foliage: $dry_foliage:literal, grass: $grass:literal, water: $water:literal, precip: $precip:literal, temp: $temp:literal, downfall: $downfall:literal },
        attr: { $(  $ty:ty : $tt:tt  ),* }
    }),*) => {
        $(
            impl crate::biome::BiomeType<$version> for $ident {
                const METADATA: &'static crate::biome::BiomeMetadata = {
                    #[cfg(feature = "biome_data")]
                    static ATTRIBUTES: LazyLock<crate::biome::BiomeAttributeSet> = LazyLock::new(|| {
                        crate::biome::BiomeAttributeSet::new_runtime(alloc::vec![
                            $(
                                (<$ty as crate::biome::AttributeType>::IDENTIFIER, facet_value::value!($tt))
                            ),*
                        ])
                    });

                    static METADATA: crate::biome::BiomeMetadata = unsafe { crate::biome::BiomeMetadata::new::<$ident, $version>(
                        froglight_common::identifier::Identifier::new_static($string),
                        $global,
                        $foliage, $dry_foliage, $grass, $water, $precip, $temp, $downfall,
                        #[cfg(feature = "biome_data")]
                        &ATTRIBUTES,
                    ) };

                    &METADATA
                };
            }
        )*

        static ARRAY: &'static [&'static crate::biome::BiomeMetadata] = &[
            $(
                <$ident as crate::biome::BiomeType<$version>>::METADATA,
            )*
        ];
        crate::implement_biomes!($version => unsafe {
            crate::storage::BiomeStorage::new_static(ARRAY)
        });
    };
}

#[cfg(feature = "biome_data")]
pub mod attribute;
pub mod biome;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;

