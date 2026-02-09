//! Generated block types, attributes, and metadata.
//! 
//! Do not edit anything other than the macros in this file!
#![allow(clippy::all, reason = "Ignore all lints for generated code")]

#[expect(unused, reason = "WIP")]
macro_rules! generate {
    (@attributes $($ident:ident => [ $($str:literal => $val:ident),+ ] ),*) => {
        $(
            #[doc = concat!("The [`", stringify!($ident), "`] block attribute.")]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum $ident {
                $(
                    #[doc = concat!("The `", stringify!($val), "` value")]
                    $val,
                )*
            }
            impl crate::block::BlockAttribute for $ident {
                const STATES: &'static [(&'static str, Self)] = &[$(($str, Self::$val)),*];
            }
        )*
    };

    (@blocks $($ident:ident),* $(,)?) => {
        $(
            #[doc = concat!("The [`", stringify!($ident), "`] block type.")]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $ident;
        )*

        /// An enum containing all vanilla block types.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum VanillaBlock {
            $(
                #[doc = concat!("The [`", stringify!($ident), "`] block type.")]
                $ident,
            )*
        }

        $(
            #[automatically_derived]
            impl From<$ident> for VanillaBlock {
                #[inline]
                fn from(_: $ident) -> Self {
                    VanillaBlock::$ident
                }
            }

            #[automatically_derived]
            impl PartialEq<VanillaBlock> for $ident {
                #[inline]
                fn eq(&self, other: &VanillaBlock) -> bool {
                    matches!(other, VanillaBlock::$ident)
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for VanillaBlock {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    matches!(self, VanillaBlock::$ident)
                }
            }

            #[automatically_derived]
            impl PartialEq<crate::block::Block> for $ident {
                #[inline]
                fn eq(&self, other: &crate::block::Block) -> bool {
                    other.is_block::<$ident>()
                }
            }
            #[automatically_derived]
            impl PartialEq<$ident> for crate::block::Block {
                #[inline]
                fn eq(&self, _: &$ident) -> bool {
                    self.is_block::<$ident>()
                }
            }
        )*

        #[automatically_derived]
        impl PartialEq<crate::block::Block> for VanillaBlock {
            #[allow(unreachable_patterns, reason = "Nonexhaustive")]
            fn eq(&self, other: &crate::block::Block) -> bool {
                match self {
                    $(
                        VanillaBlock::$ident => other.is_block::<$ident>(),
                    )*
                    _ => unreachable!("All variants of `VanillaBlock` should be covered in the match arms."),
                }
            }
        }
        #[automatically_derived]
        impl PartialEq<VanillaBlock> for crate::block::Block {
            #[inline]
            fn eq(&self, other: &VanillaBlock) -> bool {
                PartialEq::<crate::block::Block>::eq(other, self)
            }
        }
    };

    (@version $version:ident, $($ident:ident => {
        ident: $string:literal,
        global: $global:literal,
        default: $default:literal,
        ty: [$($name:literal => $ty:ty),*]
    }),*) => {
        $(
            impl crate::block::BlockType<$version> for $ident {
                type Attributes = ($($ty,)*);
                const ATTRDATA: &'static [(&'static str, core::any::TypeId)] = &[$(
                    ($name, core::any::TypeId::of::<$ty>()),
                )*];
                const METADATA: &'static crate::block::BlockMetadata = {
                    static METADATA: crate::block::BlockMetadata = unsafe { crate::block::BlockMetadata::new::<$ident, $version>(
                        froglight_common::identifier::Identifier::new_static($string),
                        $global,
                        $default,
                        crate::block::BlockBehavior::new::<$ident, $version>(),
                    ) };
                    &METADATA
                };
            }
        )*
    };
    (@version @storage $version:ident, $($ident:ident),*) => {
        static ARRAY: &'static [&'static crate::block::BlockMetadata] = &[
            $(<$ident as crate::block::BlockType<$version>>::METADATA),*
        ];
        crate::implement_blocks!($version => unsafe {
            crate::storage::BlockStorage::new_static(ARRAY)
        });
    };

    (@shape) => {};
}

pub mod attribute;
pub mod block;
pub mod shape;

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

#[cfg(feature = "v26_1")]
mod v26_1;

