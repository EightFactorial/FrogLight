use bevy_ecs::entity::Entity;

use super::{CommandBuilder, WorldRef};
use crate::argument::ArgumentParser;

/// A trait for building functions to add to the
/// [`BrigadierGraph`](super::BrigadierGraph).
pub trait FunctionBuilder<'env, Parser: ArgumentParser, Function, Marker> {
    /// Add an argument to the function.
    fn argument(self) -> CommandBuilder<'env, Marker>;
}

/// A macro for implementing the [`FunctionBuilder`] trait.
macro_rules! impl_builder {
    ($ignored:ident $(,)? $($arg:ident),*) => {
        impl<'env, $($arg,)* Parser: ArgumentParser, Function>
            FunctionBuilder<'env, Parser, fn(Entity, $($arg,)* WorldRef), fn(Entity, $($arg,)* Parser::Arg, WorldRef)>
            for CommandBuilder<'env, Function>
        {
            fn argument(self) -> CommandBuilder<'env, fn(Entity, $($arg,)* Parser::Arg, WorldRef)> { self.convert() }
        }

        impl_builder! { $($arg),* }
    };
    () => {};
}

// Implement [`FunctionBuilder`] builders with up to 10 arguments.
impl_builder! { Ignored, Arg9, Arg8, Arg7, Arg6, Arg5, Arg4, Arg3, Arg2, Arg1, Arg0 }
