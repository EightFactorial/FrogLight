use bevy_reflect::func::FunctionError;
use smol_str::SmolStr;

use crate::argument::ArgumentError;

/// An error that occurred while parsing and executing a command.
#[derive(Debug, thiserror::Error)]
pub enum BrigadierError {
    /// An unknown command was provided.
    #[error("Unknown command: \"{0}\"")]
    UnknownCommand(SmolStr),
    /// An unknown function was provided.
    #[error("Unknown function: \"{0}\"")]
    UnknownFunction(SmolStr),

    /// An unknown parser was provided.
    #[error("Unknown parser: \"{0:?}\"")]
    UnknownParser(Option<&'static str>),

    /// An unexpected end of the command was reached.
    #[error("Unexpected end of command: \"{0}\"")]
    UnexpectedEnd(SmolStr),

    /// An error occurred while parsing an argument.
    #[error("Invalid argument: {0}")]
    Argument(#[from] ArgumentError),
    /// An error occurred while executing a function.
    #[error("Function error: {0}")]
    Function(#[from] FunctionError),
}
