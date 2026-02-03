//! Generated item types.
//!
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

#[expect(unused, reason = "WIP")]
macro_rules! generate {
    (@components) => {};

    (@items $($tt:tt)*) => {};

    (@items @all $($ident:ident),*) => {
        /// An enum containing all possible item types.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum AnyItem {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] item type.")]
                $ident,
            )*
        }

        $(
            impl From<$ident> for AnyItem {
                fn from(_: $ident) -> Self {
                    AnyItem::$ident
                }
            }
            impl TryFrom<AnyItem> for $ident {
                type Error = ();

                fn try_from(value: AnyItem) -> Result<Self, Self::Error> {
                    match value {
                        AnyItem::$ident => Ok($ident),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
    (@items @single $ident:ident) => {
        #[doc = concat!("The [`", stringify!($ident), "`] item type.")]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $ident;
    };
    (@items @single @block $ident:ident) => {
        #[cfg(feature = "froglight-block")]
        pub use froglight_block::generated::block::$ident;
        #[cfg(not(feature = "froglight-block"))]
        #[doc = concat!("The [`", stringify!($ident), "`] item type.")]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $ident;
    };

    (@version) => {};
}

pub mod component;
pub mod item;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
