//! Generated block types, attributes, and metadata.
//!
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

#[expect(unused, reason = "WIP")]
macro_rules! generate {
    (@attributes) => {};

    (@blocks $($ident:ident),* $(,)?) => {
        $(
            #[doc = concat!("The [`", stringify!($ident), "`] block type.")]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $ident;
        )*

        /// An enum containing all possible block types.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AnyBlock {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] block type.")]
                $ident,
            )*
        }

        $(
            impl From<$ident> for AnyBlock {
                fn from(_: $ident) -> Self {
                    AnyBlock::$ident
                }
            }
            impl TryFrom<AnyBlock> for $ident {
                type Error = ();

                fn try_from(value: AnyBlock) -> Result<Self, Self::Error> {
                    match value {
                        AnyBlock::$ident => Ok($ident),
                        _ => Err(()),
                    }
                }
            }
        )*
    };

    (@version) => {};
    (@shape) => {};
}

pub mod attribute;
pub mod block;
pub mod shape;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
