use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::entity::Entity;

use super::{CommandBuilder, Full, WorldRef};
use crate::{
    argument::ArgumentParser,
    graph::{BrigadierEdge, BrigadierNode},
};

/// A trait for building functions to add to the
/// [`BrigadierGraph`](super::BrigadierGraph).
pub trait FunctionBuilder<'env, Parser: ArgumentParser, Function, NewFunction> {
    /// Add an argument to the function.
    fn argument(self) -> CommandBuilder<'env, NewFunction>;
}

/// A macro for implementing the [`FunctionBuilder`] trait.
macro_rules! impl_builder {
    ($ignored:ident $(,)? $($arg:ident),*) => {
        impl<'env, $($arg,)* Parser: ArgumentParser, Function>
            FunctionBuilder<'env, Parser, fn(Entity, $($arg,)* WorldRef<Full>), fn(Entity, $($arg,)* Parser::Arg, WorldRef<Full>)>
            for CommandBuilder<'env, Function>
        {
            fn argument(self) -> CommandBuilder<'env, fn(Entity, $($arg,)* Parser::Arg, WorldRef<Full>)> { self.convert() }
        }

        impl_builder! { $($arg),* }
    };
    () => {};
}

// Implement [`FunctionBuilder`] builders with up to 10 arguments.
impl_builder! { Ignored, Arg9, Arg8, Arg7, Arg6, Arg5, Arg4, Arg3, Arg2, Arg1, Arg0 }

impl<'env, Function> CommandBuilder<'env, Function> {
    /// Add an argument to the function.
    #[inline]
    #[must_use]
    #[doc(alias = "argument")]
    pub fn arg<Parser: ArgumentParser, NewFunction>(self) -> CommandBuilder<'env, NewFunction>
    where
        Self: FunctionBuilder<'env, Parser, Function, NewFunction>,
    {
        self.argument::<Parser, NewFunction>()
    }

    /// Add an argument to the function.
    #[must_use]
    pub fn argument<Parser: ArgumentParser, NewFunction>(
        mut self,
    ) -> CommandBuilder<'env, NewFunction>
    where
        Self: FunctionBuilder<'env, Parser, Function, NewFunction>,
    {
        self.nodes
            .push((BrigadierEdge::from(TypeId::of::<Parser>()), BrigadierNode { function: None }));
        <Self as FunctionBuilder<'env, Parser, Function, NewFunction>>::argument(self)
    }

    /// Convert the function to a different type.
    ///
    /// # Note
    /// [`FunctionBuilder`] calls this internally,
    /// so this applies to all building functions.
    #[inline]
    #[must_use]
    fn convert<Other>(self) -> CommandBuilder<'env, Other> {
        CommandBuilder {
            command: self.command,
            nodes: self.nodes,
            graph: self.graph,
            registry: self.registry,
            _function: PhantomData,
        }
    }
}
