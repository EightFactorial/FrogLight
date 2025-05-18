//! TODO

use alloc::{borrow::Cow, string::String};

use downcast_rs::DowncastSync;
use froglight_common::{prelude::Identifier, version::Version};
use smol_str::SmolStr;

/// A registry type.
pub trait RegistryType<V: Version>: MaybeReflect + 'static {
    /// The type of value stored in the
    /// [`RegistryStorage`](crate::storage::RegistryStorage).
    type Value: RegistryValue + Clone;

    /// The default values for this registry.
    const DEFAULTS: &'static [(Identifier, Self::Value)];
}

// -------------------------------------------------------------------------------------------------

/// A value stored in a [`RegistryStorage`](crate::storage::RegistryStorage).
pub trait RegistryValue: DowncastSync + MaybeReflect {}

impl RegistryValue for () {}
impl RegistryValue for bool {}

impl RegistryValue for i8 {}
impl RegistryValue for i16 {}
impl RegistryValue for i32 {}
impl RegistryValue for i64 {}
impl RegistryValue for i128 {}
impl RegistryValue for isize {}

impl RegistryValue for u8 {}
impl RegistryValue for u16 {}
impl RegistryValue for u32 {}
impl RegistryValue for u64 {}
impl RegistryValue for u128 {}
impl RegistryValue for usize {}

impl RegistryValue for f32 {}
impl RegistryValue for f64 {}

impl RegistryValue for &'static str {}
impl RegistryValue for Cow<'static, str> {}
impl RegistryValue for String {}
impl RegistryValue for SmolStr {}
impl RegistryValue for Identifier {}

// -------------------------------------------------------------------------------------------------

use sealed::MaybeReflect;
mod sealed {
    #[cfg(feature = "bevy")]
    use bevy_reflect::Reflect;

    #[cfg(feature = "bevy")]
    pub trait MaybeReflect: Reflect {}
    #[cfg(feature = "bevy")]
    impl<T: Reflect> MaybeReflect for T {}

    #[cfg(not(feature = "bevy"))]
    pub trait MaybeReflect {}
    #[cfg(not(feature = "bevy"))]
    impl<T> MaybeReflect for T {}
}
