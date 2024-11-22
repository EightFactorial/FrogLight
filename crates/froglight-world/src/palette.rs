use froglight_protocol::protocol::{FrogVarRead, FrogVarWrite, ReadError, WriteError};

/// A palette type for a container.
///
/// Used to determine how data is stored in a container.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContainerPalette {
    /// A single block ID.
    Single(u32),
    /// A vector of block IDs.
    Vector(Vec<u32>),
    /// A global palette.
    Global,
}

impl Default for ContainerPalette {
    fn default() -> Self { Self::Single(0u32) }
}

impl ContainerPalette {
    /// Reads a palette type from the buffer.
    pub(crate) fn read_type(&self, buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
        match self {
            Self::Single(_) => Ok(Self::Single(u32::fg_var_read(buf)?)),
            Self::Vector(_) => Ok(Self::Vector(Vec::fg_var_read(buf)?)),
            Self::Global => Ok(Self::Global),
        }
    }
}

impl FrogVarWrite for ContainerPalette {
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        match self {
            Self::Single(id) => id.fg_var_write(buf),
            Self::Vector(ids) => ids.fg_var_write(buf),
            Self::Global => Ok(()),
        }
    }
}
