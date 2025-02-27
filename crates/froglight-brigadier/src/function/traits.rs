use bevy_ecs::entity::Entity;

use super::{BuilderState, CommandBuilder, Full, WorldRef};
use crate::argument::ArgumentParser;

/// A trait for building functions to add to the
/// [`BrigadierGraph`](super::BrigadierGraph).
pub(super) trait FunctionBuilder<
    'env,
    Parser: ArgumentParser,
    State: BuilderState,
    Function,
    NewFunction,
>
{
    /// Add an argument to the function.
    fn argument(self) -> CommandBuilder<'env, State, NewFunction>;
}

/// A macro for implementing the [`FunctionBuilder`] trait.
macro_rules! impl_builder {
    ($ignored:ident $(,)? $($arg:ident),*) => {
        impl<'env, $($arg,)* Parser: ArgumentParser, State: BuilderState, Function>
            FunctionBuilder<'env, Parser, State, fn(Entity, $($arg,)* WorldRef<Full>), fn(Entity, $($arg,)* Parser::Arg, WorldRef<Full>)>
            for CommandBuilder<'env, State, Function>
        {
            fn argument(self) -> CommandBuilder<'env, State, fn(Entity, $($arg,)* Parser::Arg, WorldRef<Full>)> { self.convert() }
        }

        impl_builder! { $($arg),* }
    };
    () => {};
}

// Implement [`FunctionBuilder`] builders with up to 10 arguments.
impl_builder! { Ignored, Arg9, Arg8, Arg7, Arg6, Arg5, Arg4, Arg3, Arg2, Arg1, Arg0 }
