use std::io::{Read, Write};

use smallvec::{Array, SmallVec};

use super::{FrogRead, FrogWrite, ReadError, WriteError};

impl<A: Array> FrogRead for SmallVec<A>
where
    A::Item: FrogRead,
{
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        Vec::<A::Item>::frog_read(buffer).map(SmallVec::from_vec)
    }
}

impl<A: Array> FrogWrite for SmallVec<A>
where
    A::Item: FrogWrite,
{
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        <[A::Item]>::frog_write(self.as_slice(), buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { <[A::Item]>::frog_len(self.as_slice()) }
}
