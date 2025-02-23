//! TODO

pub(super) mod iterator;
pub(super) mod named;

mod component;
pub use component::{NbtCompoundRef, NbtListTagRef, NbtTagRef};

mod error;
pub use error::NbtStreamError;
