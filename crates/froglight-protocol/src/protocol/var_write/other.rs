use std::io::Write;

use simdnbt::owned::Nbt;

use super::FrogVarWrite;
use crate::protocol::WriteError;

impl FrogVarWrite for Nbt {
    #[inline]
    fn fg_var_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        let mut vec = Vec::new();
        self.write_unnamed(&mut vec);
        buf.write_all(&vec).map_err(WriteError::Io)
    }
}
