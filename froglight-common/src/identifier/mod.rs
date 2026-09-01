//! TODO

mod error;
pub use error::IdentifierError;

mod ident;
pub use ident::Ident;

#[cfg(feature = "alloc")]
mod identifier;
#[cfg(feature = "alloc")]
pub use identifier::Identifier;
