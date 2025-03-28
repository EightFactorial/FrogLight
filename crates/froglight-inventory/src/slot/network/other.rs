use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};

use super::{RawInventorySlot, RawInventorySlotRef};

macro_rules! impl_networking {
    ($version:ident) => {
        impl FrogReadVersion<froglight_common::version::$version>
            for RawInventorySlot<froglight_common::version::$version>
        {
            fn frog_read(_buffer: &mut impl std::io::Read) -> Result<Self, ReadError> { todo!() }
        }

        impl FrogWriteVersion<froglight_common::version::$version>
            for RawInventorySlot<froglight_common::version::$version>
        {
            #[inline]
            fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
                RawInventorySlotRef::from_raw(self).frog_write(buffer)
            }

            #[inline]
            fn frog_len(&self) -> usize { RawInventorySlotRef::from_raw(self).frog_len() }
        }

        impl FrogWriteVersion<froglight_common::version::$version>
            for RawInventorySlotRef<'_, froglight_common::version::$version>
        {
            fn frog_write(&self, _buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
                todo!()
            }

            fn frog_len(&self) -> usize { todo!() }
        }
    };
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "v1_21_5")]
impl_networking!(V1_21_5);
