use alloc::vec::Vec;
use core::any::TypeId;

use bevy_ecs::world::World;
use bevy_reflect::prelude::IntoFunction;

/// An argument for a [`GameCommand`](crate::prelude::GameCommand).
pub trait CommandArgument {
    /// The actual type of the argument.
    type Output: Sized + 'static;

    /// A parser for the argument.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is invalid for this argument type.
    fn parse_argument(input: &str) -> Result<(Self::Output, &str), ArgumentParseError>;
}

/// An error that can occur when parsing an argument.
#[derive(Debug, Clone)]
pub enum ArgumentParseError {}

// -------------------------------------------------------------------------------------------------

/// A bundle of [`CommandArgument`]s.
pub trait ArgumentBundle: Sized {
    /// The [`TypeId`]s of the argument types in this bundle, in order.
    ///
    /// TODO: Replace `TypeId` with graph entries.
    fn type_infos() -> Vec<TypeId>;
}

macro_rules! impl_argument_bundle {
    ($(#[$meta:meta])* $($A:ident),*) => {
        $(#[$meta])*
        impl<$($A: CommandArgument),*> ArgumentBundle for ($($A,)*) {
            fn type_infos() -> Vec<TypeId> {
                alloc::vec![$(TypeId::of::<$A::Output>()),*]
            }
        }
    };
}

impl<A: CommandArgument> ArgumentBundle for A {
    fn type_infos() -> Vec<TypeId> { alloc::vec![TypeId::of::<A::Output>()] }
}

variadics_please::all_tuples!(impl_argument_bundle, 1, 15, A);

// -------------------------------------------------------------------------------------------------

pub trait BundleFunction<Bundle, Marker>: IntoFunction<'static, Marker> + Sized {}

macro_rules! impl_bundle_function {
    ($(#[$meta:meta])* $($A:ident),*) => {
        $(#[$meta])*
        impl<T, $($A: CommandArgument),*, Marker> BundleFunction<($($A,)*), Marker> for T
            where T: for<'w> Fn($($A::Output),*, &'w mut World) + IntoFunction<'static, Marker>
        {}
    };
}

impl<T, Marker> BundleFunction<(), Marker> for T where
    T: for<'w> Fn(&'w mut World) + IntoFunction<'static, Marker>
{
}

variadics_please::all_tuples!(impl_bundle_function, 1, 15, A);
