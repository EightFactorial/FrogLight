use std::borrow::Cow;

use bevy_reflect::func::FunctionError;
use smol_str::SmolStr;

use crate::argument::ArgumentError;

/// An error that occurred while parsing and executing a command.
#[derive(Debug, thiserror::Error)]
pub enum BrigadierError {
    /// A command with the same name already exists.
    #[error("duplicate command \"{0}\"")]
    DuplicateCommand(SmolStr),
    /// An unknown command was provided.
    #[error("unknown command \"{0}\"")]
    UnknownCommand(Cow<'static, str>),

    /// An unknown function was provided.
    #[error("unknown function \"{0}\"")]
    UnknownFunction(Cow<'static, str>),

    /// An unknown parser was provided.
    #[error("unknown parser \"{0:?}\"")]
    UnknownParser(Option<&'static str>),

    /// An unexpected end of the command was reached.
    #[error("unexpected end of command \"{0}\"")]
    UnexpectedEnd(SmolStr),

    /// An error occurred while parsing an argument.
    #[error("invalid argument, {0}")]
    Argument(#[from] ArgumentError),
    /// An error occurred while executing a function.
    #[error("function error, {0}")]
    Function(#[from] FunctionError),
}
