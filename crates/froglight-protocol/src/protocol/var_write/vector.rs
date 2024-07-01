use super::FrogVarWrite;
use crate::protocol::WriteError;

impl<T: FrogVarWrite, const N: usize> FrogVarWrite for [T; N] {
    #[inline]
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.len().fg_var_write(buf)?;
        for item in self {
            item.fg_var_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogVarWrite> FrogVarWrite for Vec<T> {
    #[inline]
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.len().fg_var_write(buf)?;
        for item in self {
            item.fg_var_write(buf)?;
        }
        Ok(())
    }
}

#[cfg(feature = "smallvec")]
impl<T: FrogVarWrite, const N: usize> FrogVarWrite for smallvec::SmallVec<[T; N]> {
    #[inline]
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.len().fg_var_write(buf)?;
        for item in self {
            item.fg_var_write(buf)?;
        }
        Ok(())
    }
}
