use alloc::vec::Vec;
use core::any::TypeId;

use bevy_ecs::world::World;
use bevy_reflect::prelude::IntoFunction;

use crate::builder::{CommandBuilderExt, GameCommandBuilder};

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

pub trait BundleAppender<Bundle> {
    type Appended;

    fn append(bundle: Self) -> Self::Appended;
}

macro_rules! impl_bundle_appender {
    ($(#[$meta:meta])* $($A:ident),*) => {
        $(#[$meta])*
        impl<'a, $($A: CommandArgument,)* B0: CommandArgument> BundleAppender<(B0,)> for GameCommandBuilder<'a, ($($A,)*)>
        {
            type Appended = GameCommandBuilder<'a, ($($A,)* B0)>;

            fn append(bundle: Self) -> Self::Appended {
                let bundle = bundle.with::<B0>();
                bundle
            }
        }
        $(#[$meta])*
        impl<'a, $($A: CommandArgument,)* B0: CommandArgument, B1: CommandArgument> BundleAppender<(B0, B1)> for GameCommandBuilder<'a, ($($A,)*)>
        {
            type Appended = GameCommandBuilder<'a, ($($A,)* B0, B1)>;

            fn append(bundle: Self) -> Self::Appended {
                let bundle = bundle.with::<B0>();
                let bundle = bundle.with::<B1>();
                bundle
            }
        }
        $(#[$meta])*
        impl<'a, $($A: CommandArgument,)* B0: CommandArgument, B1: CommandArgument, B2: CommandArgument> BundleAppender<(B0, B1, B2)> for GameCommandBuilder<'a, ($($A,)*)>
        {
            type Appended = GameCommandBuilder<'a, ($($A,)* B0, B1, B2)>;

            fn append(bundle: Self) -> Self::Appended {
                let bundle = bundle.with::<B0>();
                let bundle = bundle.with::<B1>();
                let bundle = bundle.with::<B2>();
                bundle
            }
        }
        $(#[$meta])*
        impl<'a, $($A: CommandArgument,)* B0: CommandArgument, B1: CommandArgument, B2: CommandArgument, B3: CommandArgument> BundleAppender<(B0, B1, B2, B3)> for GameCommandBuilder<'a, ($($A,)*)>
        {
            type Appended = GameCommandBuilder<'a, ($($A,)* B0, B1, B2, B3)>;

            fn append(bundle: Self) -> Self::Appended {
                let bundle = bundle.with::<B0>();
                let bundle = bundle.with::<B1>();
                let bundle = bundle.with::<B2>();
                let bundle = bundle.with::<B3>();
                bundle
            }
        }
    };
}

variadics_please::all_tuples!(impl_bundle_appender, 1, 11, A);

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
