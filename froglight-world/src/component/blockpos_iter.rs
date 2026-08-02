use crate::prelude::*;

/// An [`Iterator`] over an area of [`BlockPos`]s.
pub struct BlockPosIter {
    min: BlockPos,
    max: BlockPos,
    current: BlockPos,
}

impl BlockPosIter {
    /// Create a new [`BlockPosIter`] between two corners, inclusive.
    #[inline]
    #[must_use]
    pub const fn new_inclusive(first: BlockPos, second: BlockPos) -> Self {
        let [mut min_x, mut min_y, mut min_z] = first.as_ivec3().to_array();
        let [mut max_x, mut max_y, mut max_z] = second.as_ivec3().to_array();

        if min_x > max_x {
            core::mem::swap(&mut min_x, &mut max_x);
        }
        if min_y > max_y {
            core::mem::swap(&mut min_y, &mut max_y);
        }
        if min_z > max_z {
            core::mem::swap(&mut min_z, &mut max_z);
        }

        let min = BlockPos::new_xyz(min_x, min_y, min_z);
        let max = BlockPos::new_xyz(max_x, max_y, max_z);
        Self { min, max, current: min }
    }

    /// Create a new [`BlockPosIter`] between two corners, exclusive.
    #[must_use]
    pub const fn new_exclusive(first: BlockPos, second: BlockPos) -> Self {
        let mut iter = Self::new_inclusive(first, second);
        iter.min.set_x(iter.min.x() + 1);
        iter.min.set_y(iter.min.y() + 1);
        iter.min.set_z(iter.min.z() + 1);
        iter.max.set_x(iter.max.x() - 1);
        iter.max.set_y(iter.max.y() - 1);
        iter.max.set_z(iter.max.z() - 1);
        iter.current = iter.min;
        iter
    }

    /// Get the maximum corner of this [`BlockPosIter`].
    #[inline]
    #[must_use]
    pub const fn max(&self) -> BlockPos { self.max }

    /// Get the minimum corner of this [`BlockPosIter`].
    #[inline]
    #[must_use]
    pub const fn min(&self) -> BlockPos { self.min }

    /// Get the next [`BlockPos`] in this [`BlockPosIter`].
    ///
    /// Returns `None` if the iterator has reached the end.
    #[must_use]
    pub const fn next(&mut self) -> Option<BlockPos> {
        if self.current.const_le(self.max) {
            let result = Some(self.current);
            self.increment();
            result
        } else {
            None
        }
    }

    /// Increment the current position of this [`BlockPosIter`].
    const fn increment(&mut self) {
        self.current.set_x(self.current.x() + 1);
        if self.current.x() > self.max.x() {
            self.current.set_x(self.min.x());
            self.current.set_y(self.current.y() + 1);
            if self.current.y() > self.max.y() {
                self.current.set_y(self.min.y());
                self.current.set_z(self.current.z() + 1);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl Iterator for BlockPosIter {
    type Item = BlockPos;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.next() }
}
