use std::io::Cursor;

use compact_str::CompactString;
use froglight_components::resourcekey::ResourceKey;

use crate::protocol::{FrogRead, FrogWrite, ReadError, WriteError};

impl FrogRead for ResourceKey {
    #[inline]
    fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(ResourceKey::new(CompactString::fg_read(buf)?))
    }
}

impl FrogWrite for ResourceKey {
    #[inline]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        AsRef::<CompactString>::as_ref(self).fg_write(buf)
    }
}
