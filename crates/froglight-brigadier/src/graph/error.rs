use std::borrow::Cow;

use bevy_reflect::func::{FunctionError, FunctionRegistrationError};
use smol_str::SmolStr;

use crate::argument::ArgumentError;

/// An error that occurred while parsing and executing a command.
#[derive(Debug, thiserror::Error)]
pub enum BrigadierError {
    /// A command with the same name already exists.
    #[error("duplicate command \"{0}\"")]
    DuplicateCommand(SmolStr),
    /// An unexpected end of the command was reached.
    #[error("unexpected end of command \"{0}\"")]
    UnexpectedEnd(SmolStr),

    /// An unknown command was provided.
    #[error("unknown command \"{0}\"")]
    UnknownCommand(Cow<'static, str>),
    /// An unknown function was provided.
    #[error("unknown function \"{0}\"")]
    UnknownFunction(Cow<'static, str>),
    /// An unknown parser was provided.
    #[error("unknown parser \"{0:?}\"")]
    UnknownParser(&'static str),

    /// An error occurred while parsing an argument.
    #[error("invalid argument, {0}")]
    Argument(#[from] ArgumentError),
    /// An error occurred while executing a function.
    #[error("function error, {0}")]
    Function(#[from] FunctionError),
    /// An error occurred while registering a function.
    #[error("function registration error, {0}")]
    Registry(#[from] FunctionRegistrationError),
}
