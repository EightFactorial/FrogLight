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

        /// An enum containing all possible biome types.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AnyBiome {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] biome type.")]
                $ident,
            )*
        }

        $(
            impl From<$ident> for AnyBiome {
                fn from(_: $ident) -> Self {
                    AnyBiome::$ident
                }
            }
            impl TryFrom<AnyBiome> for $ident {
                type Error = ();

                fn try_from(value: AnyBiome) -> Result<Self, Self::Error> {
                    match value {
                        AnyBiome::$ident => Ok($ident),
                        _ => Err(()),
                    }
                }
            }
        )*
    };

    (@feature) => {};
    (@version) => {};
}

pub mod attribute;
pub mod biome;
pub mod feature;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
