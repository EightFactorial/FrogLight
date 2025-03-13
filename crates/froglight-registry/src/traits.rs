//! TODO

use downcast_rs::DowncastSync;
use froglight_common::{prelude::Identifier, version::Version};

/// A registry type.
pub trait RegistryType<V: Version>: MaybeReflect {
    /// The type of value stored in the
    /// [`RegistryStorage`](crate::storage::RegistryStorage).
    type Value: RegistryValue;

    /// The default values for this registry.
    fn defaults() -> Vec<(Identifier, Self::Value)>;
}

/// A value stored in a [`RegistryStorage`](crate::storage::RegistryStorage).
pub trait RegistryValue: DowncastSync + MaybeReflect {}

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
