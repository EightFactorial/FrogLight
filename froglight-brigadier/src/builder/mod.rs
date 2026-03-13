//! TODO

use core::marker::PhantomData;

use bevy_reflect::{func::DynamicFunction, prelude::IntoFunction};

mod argument;
pub use argument::{ArgumentBundle, ArgumentParseError, CommandArgument};
use petgraph::prelude::NodeIndex;

use crate::{builder::argument::BundleFunction, prelude::CommandGraph};

/// A builder for constructing a [`GameCommand`].
pub struct GameCommandBuilder<'w, A> {
    graph: &'w mut CommandGraph,
    function: &'w mut Option<DynamicFunction<'static>>,
    entrypoint: NodeIndex,
    _args: PhantomData<A>,
}

impl<'w> GameCommandBuilder<'w, ()> {
    /// Create a new [`GameCommandBuilder`] for the given command.
    pub fn new(
        graph: &'w mut CommandGraph,
        function: &'w mut Option<DynamicFunction<'static>>,
        entrypoint: NodeIndex,
    ) -> Self {
        Self { graph, function, entrypoint, _args: PhantomData }
    }

    /// Add an argument to the command being built.
    #[must_use]
    pub fn with<T: CommandArgument>(self) -> GameCommandBuilder<'w, T> {
        GameCommandBuilder {
            graph: self.graph,
            function: self.function,
            entrypoint: self.entrypoint,
            _args: PhantomData,
        }
    }
}

impl<'w, A: ArgumentBundle> GameCommandBuilder<'w, A>
where
    Self: CommandBuilderExt<'w>,
{
    /// Add a function to the command being built.
    ///
    /// # Panics
    ///
    /// Panics if `build` is called twice with the same arguments.
    pub fn build<F: BundleFunction<A, Marker>, Marker>(&mut self, f: F) { self.merge(f); }

    /// Add a function to the command being built,
    /// and return a new builder with another argument.
    #[must_use]
    pub fn build_and<T: CommandArgument, F: BundleFunction<A, Marker>, Marker>(
        mut self,
        f: F,
    ) -> <Self as CommandBuilderExt<'w>>::WithBuilder<'w, T> {
        self.build(f);
        self.with::<T>()
    }

    /// Create a new builder and add argument to the command.
    #[must_use]
    pub fn branch<'b, T: CommandArgument>(
        &'b mut self,
    ) -> GameCommandBuilder<'b, <Self as CommandBuilderExt<'w>>::WithBundle<T>> {
        GameCommandBuilder {
            graph: self.graph,
            function: self.function,
            entrypoint: self.entrypoint,
            _args: PhantomData,
        }
    }

    /// Add a function as an overload to the current command.
    fn merge<F: IntoFunction<'static, Marker>, Marker>(&mut self, f: F) {
        let function = IntoFunction::into_function(f);
        match self.function.as_ref() {
            Some(func) => *self.function = Some(func.clone().with_overload(function)),
            None => *self.function = Some(function),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An extension trait for [`GameCommandBuilder`] that provides extra methods.
pub trait CommandBuilderExt<'w> {
    /// The [`ArgumentBundle`] type that will be used for the next builder.
    type WithBundle<T: CommandArgument>: ArgumentBundle;
    /// The builder type that will be returned by the `with` method.
    type WithBuilder<'b, T: CommandArgument + Sized>: Sized;

    /// Add an argument to the command being built.
    #[must_use]
    fn with<T: CommandArgument>(self) -> Self::WithBuilder<'w, T>;
}

macro_rules! impl_command_builder {
    ($(#[$meta:meta])* $($A:ident),*) => {
        $(#[$meta])*
        impl<'w, $($A: CommandArgument),*> CommandBuilderExt<'w> for GameCommandBuilder<'w, ($($A,)*)> {
            type WithBundle<T: CommandArgument> = ($($A,)* T,);
            type WithBuilder<'b, T: CommandArgument> = GameCommandBuilder<'b, Self::WithBundle<T>>;

            fn with<T: CommandArgument>(self) -> Self::WithBuilder<'w, T>{
                GameCommandBuilder {
                    graph: self.graph,
                    function: self.function,
                    entrypoint: self.entrypoint,
                    _args: PhantomData,
                }
            }
        }
    };
}

variadics_please::all_tuples!(
    #[doc(fake_variadic)]
    impl_command_builder,
    1,
    14,
    A
);
