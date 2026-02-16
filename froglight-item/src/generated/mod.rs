//! Generated item types.
//!
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

#[expect(unused, reason = "WIP")]
macro_rules! generate {
    (@components) => {};

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
    (@items $($(@$attr:tt)? $ident:ident),* $(,)?) => {
        $(
            generate!(@items @single $(@$attr)? $ident);
        )*

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
                    self.is_item::<$ident>()
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
            #[inline]
            fn eq(&self, other: &VanillaItem) -> bool {
                PartialEq::<crate::item::Item>::eq(other, self)
            }
        }
    };

    (@version $version:ident, $($ident:ident => {
        ident: $string:literal,
        global: $global:literal
    }),*) => {
        $(
            impl crate::item::ItemType<$version> for $ident {
                const METADATA: &'static crate::item::ItemMetadata = {
                    static METADATA: crate::item::ItemMetadata = unsafe { crate::item::ItemMetadata::new::<$ident, $version>(
                        froglight_common::identifier::Identifier::new_static($string),
                        $global,
                        ComponentData::empty()
                    ) };
                    &METADATA
                };
            }
        )*

        static ARRAY: &'static [&'static crate::item::ItemMetadata] = &[
            $(
                <$ident as crate::item::ItemType<$version>>::METADATA,
            )*
        ];
        crate::implement_items!($version => unsafe {
            crate::storage::ItemStorage::new_static(ARRAY)
        });
    };
}

pub mod component;
pub mod item;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;
