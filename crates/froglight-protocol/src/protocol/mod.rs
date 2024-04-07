//! Protocol traits and macros.

mod errors;
pub use errors::{ReadError, WriteError};
pub use froglight_macros::{FrogReadWrite, FrogTest};

mod read;
pub use froglight_macros::FrogRead;
pub use read::FrogRead;

mod var_read;
pub use var_read::FrogVarRead;

mod write;
pub use froglight_macros::FrogWrite;
pub use write::FrogWrite;

mod var_write;
pub use var_write::FrogVarWrite;
