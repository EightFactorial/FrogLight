use compact_str::CompactString;

use crate::io::{FrogVarWrite, FrogWrite, WriteError};

impl FrogWrite for String {
    #[inline]
    fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("String too long").frog_var_write(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl FrogWrite for CompactString {
    #[inline]
    fn frog_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("String too long").frog_var_write(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
}
