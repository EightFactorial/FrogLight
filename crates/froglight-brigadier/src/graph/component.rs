use std::any::TypeId;

use derive_more::From;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BridagierNode {
    pub(crate) function: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub(crate) enum BridagierEdge {
    /// A literal argument.
    Literal(SmolStr),
    /// The type of an argument parser.
    Argument(TypeId),
}
