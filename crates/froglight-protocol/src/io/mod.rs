//! IO functions
//!
//! This module contains the IO functions for
//! reading and writing data to and from buffers.

mod read;
pub use read::FrogRead;

mod var_read;
pub use var_read::FrogVarRead;

mod var_write;
pub use var_write::FrogVarWrite;

mod write;
pub use write::FrogWrite;
