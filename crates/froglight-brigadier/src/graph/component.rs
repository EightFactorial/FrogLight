use std::{any::TypeId, borrow::Cow};

use derive_more::From;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrigadierNode {
    pub(crate) function: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub(crate) enum BrigadierEdge {
    /// A literal argument.
    Literal(SmolStr),
    /// The type of an argument parser.
    Argument(TypeId),
}
