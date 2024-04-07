use std::hash::{BuildHasher, Hash};

use crate::protocol::{FrogVarWrite, FrogWrite};

impl<K: FrogWrite + Hash + Eq, V: FrogVarWrite, S: BuildHasher> FrogVarWrite
    for std::collections::HashMap<K, V, S>
{
    #[inline]
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::protocol::WriteError> {
        u32::try_from(self.len()).expect("Map length too long").fg_var_write(buf)?;
        for (key, value) in self {
            key.fg_write(buf)?;
            value.fg_var_write(buf)?;
        }
        Ok(())
    }
}

impl<K: FrogWrite + Hash + Eq, V: FrogVarWrite, S: BuildHasher> FrogVarWrite
    for hashbrown::HashMap<K, V, S>
{
    #[inline]
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::protocol::WriteError> {
        u32::try_from(self.len()).expect("Map length too long").fg_var_write(buf)?;
        for (key, value) in self {
            key.fg_write(buf)?;
            value.fg_var_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogVarWrite, S: BuildHasher> FrogVarWrite for std::collections::HashSet<T, S> {
    #[inline]
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::protocol::WriteError> {
        u32::try_from(self.len()).expect("Set length too long").fg_var_write(buf)?;
        for value in self {
            value.fg_var_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogVarWrite, S: BuildHasher> FrogVarWrite for hashbrown::HashSet<T, S> {
    #[inline]
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::protocol::WriteError> {
        u32::try_from(self.len()).expect("Set length too long").fg_var_write(buf)?;
        for value in self {
            value.fg_var_write(buf)?;
        }
        Ok(())
    }
}

impl<K: FrogWrite + Hash + Eq, V: FrogVarWrite> FrogVarWrite for std::collections::BTreeMap<K, V> {
    #[inline]
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::protocol::WriteError> {
        u32::try_from(self.len()).expect("Map length too long").fg_var_write(buf)?;
        for (key, value) in self {
            key.fg_write(buf)?;
            value.fg_var_write(buf)?;
        }
        Ok(())
    }
}

impl<K: FrogVarWrite + Hash + Eq> FrogVarWrite for std::collections::BTreeSet<K> {
    #[inline]
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::protocol::WriteError> {
        u32::try_from(self.len()).expect("Set length too long").fg_var_write(buf)?;
        for value in self {
            value.fg_var_write(buf)?;
        }
        Ok(())
    }
}

// TODO: Create tests for the above implementations, ideally using proptest
