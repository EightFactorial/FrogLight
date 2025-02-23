//! TODO
#![expect(missing_docs, dead_code)]

pub(super) mod iterator;
pub(super) mod named;

mod component;
pub use component::{NbtComponentRef, NbtTagRef};

mod error;
pub use error::NbtStreamError;
