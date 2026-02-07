//! Generated biome types, attributes, and features.
//! 
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

macro_rules! generate {
    (@attributes) => {};

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
        prop: { foliage: $foliage:literal, grass: $grass:literal, water: $water:literal, precip: $precip:literal, temp: $temp:literal, downfall: $downfall:literal },
        attr: $attr:expr,
        feat: $feat:expr
    }),*) => {
        $(
            impl crate::biome::BiomeType<$version> for $ident {
                const METADATA: &'static crate::biome::BiomeMetadata = {
                    static METADATA: crate::biome::BiomeMetadata = unsafe { crate::biome::BiomeMetadata::new::<$ident, $version>(
                        froglight_common::identifier::Identifier::new_static($string),
                        $global,
                        $foliage, $grass, $water, $precip, $temp, $downfall,
                        $attr,
                        $feat,
                    ) };
                    &METADATA
                };
            }
        )*

        crate::implement_biomes!($version => unsafe {
            crate::storage::BiomeStorage::new_static(&[
                $(
                    <$ident as crate::biome::BiomeType<$version>>::METADATA,
                )*
            ])
        });
    };
}

pub mod attribute;
pub mod biome;
pub mod feature;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;

