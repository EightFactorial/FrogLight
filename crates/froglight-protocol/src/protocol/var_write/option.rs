use std::io::Write;

use crate::protocol::{FrogVarWrite, FrogWrite, WriteError};

impl<T: FrogVarWrite> FrogVarWrite for Option<T> {
    fn fg_var_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        match self {
            Some(value) => {
                true.fg_write(buf)?;
                value.fg_var_write(buf)
            }
            None => false.fg_write(buf),
        }
    }
}

// TODO: Create tests for Option<T>, ideally using proptest
