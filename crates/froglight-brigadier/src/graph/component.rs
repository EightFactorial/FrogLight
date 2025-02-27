use std::{any::TypeId, borrow::Cow};

use smol_str::SmolStr;

use crate::prelude::ArgumentParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrigadierNode {
    pub(crate) function: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BrigadierEdge {
    /// A command.
    Command,
    /// A literal argument.
    Literal(SmolStr),
    /// An argument parser.
    Argument { type_id: TypeId, type_name: &'static str },
}

impl BrigadierEdge {
    /// Create a new [`BrigadierEdge::Literal`].
    #[must_use]
    pub(crate) fn literal(literal: impl Into<SmolStr>) -> Self { Self::Literal(literal.into()) }

    /// Create a new [`BrigadierEdge::Argument`].
    #[must_use]
    pub(crate) fn argument<Parser: ArgumentParser>() -> Self {
        Self::Argument {
            type_id: TypeId::of::<Parser>(),
            type_name: std::any::type_name::<Parser>(),
        }
    }
}
