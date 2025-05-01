#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, boxed::Box};
#[cfg(feature = "std")]
use std::borrow::Cow;

use bevy_reflect::PartialReflect;
use smol_str::SmolStr;

use crate::prelude::ArgumentParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrigadierNode {
    pub(crate) function: Option<Cow<'static, str>>,
}

#[derive(Debug)]
pub(crate) enum BrigadierEdge {
    /// A command.
    Command,
    /// A literal argument.
    Literal(SmolStr),
    /// An argument parser.
    Argument(Box<dyn PartialReflect>),
}

impl BrigadierEdge {
    /// Create a new [`BrigadierEdge::Literal`].
    #[must_use]
    pub(crate) fn literal(literal: impl Into<SmolStr>) -> Self { Self::Literal(literal.into()) }

    /// Create a new [`BrigadierEdge::Argument`].
    #[must_use]
    pub(crate) fn argument<Parser: ArgumentParser>(parser: Parser) -> Self {
        Self::Argument(Box::new(parser))
    }
}
