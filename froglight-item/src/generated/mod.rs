//! Generated item types.
//!
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

#[expect(unused, reason = "WIP")]
macro_rules! generate {
    (@components) => {};

    (@items @all $($ident:ident),*) => {
        /// An enum containing all vanilla item types.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum VanillaItem {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] item type.")]
                $ident,
            )*
        }

        $(
            #[automatically_derived]
            impl From<$ident> for VanillaItem {
                #[inline]
                fn from(_: $ident) -> Self {
                    VanillaItem::$ident
                }
            }

            #[automatically_derived]
            impl PartialEq<VanillaItem> for $ident {
                #[inline]
                fn eq(&self, other: &VanillaItem) -> bool {
                    matches!(other, VanillaItem::$ident)
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for VanillaItem {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    matches!(self, VanillaItem::$ident)
                }
            }

            #[automatically_derived]
            impl PartialEq<crate::item::Item> for $ident {
                #[inline]
                fn eq(&self, other: &crate::item::Item) -> bool {
                    other.is_item::<$ident>()
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for crate::item::Item {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    matches!(self, crate::item::Item::$ident)
                }
            }
        )*

        #[automatically_derived]
        impl PartialEq<crate::item::Item> for VanillaItem {
            #[allow(unreachable_patterns, reason = "Nonexhaustive")]
            fn eq(&self, other: &crate::item::Item) -> bool {
                match self {
                    $(
                        VanillaItem::$ident => other.is_item::<$ident>(),
                    )*
                    _ => unreachable!("All variants of `VanillaItem` should be covered in the match arms."),
                }
            }
        }
        #[automatically_derived]
        impl PartialEq<VanillaItem> for crate::item::Item {
            #[allow(unreachable_patterns, reason = "Nonexhaustive")]
            fn eq(&self, other: &VanillaItem) -> bool {
                match other {
                    $(
                        VanillaItem::$ident => self.is_item::<$ident>(),
                    )*
                    _ => unreachable!("All variants of `VanillaItem` should be covered in the match arms."),
                }
            }
        }
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
    (@items $($(@$attr:tt)? $ident:ident),*) => {
        $(
            generate!(@items @single $(@$attr)? $ident);
        )*
        generate!(@items @all $($ident),*);
    };

    (@version) => {};
}

pub mod component;
pub mod item;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.
