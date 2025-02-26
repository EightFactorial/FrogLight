use std::{any::TypeId, borrow::Cow};

use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrigadierNode {
    pub(crate) function: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BrigadierEdge {
    /// A literal argument.
    Literal(SmolStr),
    /// An argument parser.
    Argument { type_id: TypeId, type_name: &'static str },
}
