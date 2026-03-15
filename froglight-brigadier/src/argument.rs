//! TODO

use alloc::vec::Vec;

// use bevy_ecs::world::World;
use bevy_reflect::prelude::IntoFunction;

use crate::{
    builder::{CommandBuilderExt, GameCommandBuilder},
    graph::CommandEdge,
    parse::CommandArgument,
};

/// A bundle of [`CommandArgument`]s.
pub trait ArgumentBundle: Sized + 'static {
    /// The [`CommandEdge`]s of the arguments in this bundle, in order.
    fn graph_edges() -> Vec<CommandEdge>;
}

macro_rules! impl_argument_bundle {
    ($(#[$meta:meta])* $($A:ident),*) => {
        $(#[$meta])*
        impl<$($A: CommandArgument),*> ArgumentBundle for ($($A,)*) {
            fn graph_edges() -> Vec<CommandEdge> {
                alloc::vec![$(CommandEdge::new::<$A>()),*]
            }
        }
    };
}

impl ArgumentBundle for () {
    fn graph_edges() -> Vec<CommandEdge> { Vec::new() }
}

impl<A: CommandArgument> ArgumentBundle for A {
    fn graph_edges() -> Vec<CommandEdge> { alloc::vec![CommandEdge::new::<A>()] }
}

variadics_please::all_tuples!(impl_argument_bundle, 1, 15, A);

// -------------------------------------------------------------------------------------------------

/// A trait for appending a bundle of arguments to a [`GameCommandBuilder`].
pub trait BundleAppender<Bundle> {
    /// The builder type after appending the bundle.
    type Appended;
    /// Appends the bundle to the builder.
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

/// A trait for specifying the function type that accepts a bundle of arguments.
pub trait BundleFunction<Bundle, Marker>: IntoFunction<'static, Marker> + Sized {}

macro_rules! impl_bundle_function {
    ($(#[$meta:meta])* $($A:ident),*) => {
        $(#[$meta])*
        impl<T, $($A: CommandArgument),*, Marker> BundleFunction<($($A,)*), Marker> for T
            where T: for<'w> Fn($($A::Output),*) + IntoFunction<'static, Marker>
        {}
    };
}

impl<T, Marker> BundleFunction<(), Marker> for T where
    T: for<'w> Fn() + IntoFunction<'static, Marker>
{
}
impl<T, A: CommandArgument, Marker> BundleFunction<A, Marker> for T where
    T: for<'w> Fn(A::Output) + IntoFunction<'static, Marker>
{
}

variadics_please::all_tuples!(impl_bundle_function, 1, 15, A);
