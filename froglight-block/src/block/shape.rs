#[cfg(feature = "alloc")]
use alloc::vec;
use core::ops::Add;

use glam::DVec3;

// Using a larger epsilon to match original behavior.
const EPSILON: f64 = 1e-7;

/// The shape of a block.
///
/// Defined as zero or more [`BlockAabb`]s.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockShape<'a> {
    /// An empty block shape with no AABBs.
    None,
    /// A block shape with a single AABB.
    Single(BlockAabb),
    /// A block shape with multiple AABBs.
    #[cfg(not(feature = "alloc"))]
    Collection(&'a [BlockAabb]),
    /// A block shape with multiple AABBs.
    #[cfg(feature = "alloc")]
    Collection(alloc::borrow::Cow<'a, [BlockAabb]>),
}

impl BlockShape<'_> {
    /// An empty block.
    pub const EMPTY: Self = BlockShape::None;
    /// A full block.
    pub const FULL: Self = BlockShape::Single(BlockAabb::FULL);

    /// Creates a new [`BlockShape`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new(min: DVec3, max: DVec3) -> Self {
        if (max.x - min.x).abs() > EPSILON
            && (max.y - min.y).abs() > EPSILON
            && (max.z - min.z).abs() > EPSILON
        {
            BlockShape::None
        } else {
            BlockShape::Single(BlockAabb::new(min, max))
        }
    }

    /// Creates a new [`BlockShape`] from the given two corner points.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new_from_corners(a: DVec3, b: DVec3) -> Self {
        if (a.x - b.x).abs() > EPSILON && (a.y - b.y).abs() > EPSILON && (a.z - b.z).abs() > EPSILON
        {
            BlockShape::None
        } else {
            BlockShape::Single(BlockAabb::new_corners(b, a))
        }
    }

    /// Creates a new [`BlockShape`] from the given two corner points.
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    pub fn new_from_corners(a: DVec3, b: DVec3) -> Self {
        if (a.x - b.x).abs() > EPSILON && (a.y - b.y).abs() > EPSILON && (a.z - b.z).abs() > EPSILON
        {
            BlockShape::None
        } else {
            BlockShape::Single(BlockAabb::new_corners(b, a))
        }
    }

    /// Returns `true` if the given point is contained within the
    /// [`BlockShape`].
    #[must_use]
    pub const fn contains(&self, point: DVec3) -> bool {
        let mut index = 0;
        while index < self.as_slice().len() {
            if self.as_slice()[index].contains(point) {
                return true;
            }
            index += 1;
        }
        false
    }

    /// Returns `true` if the block shape is empty (i.e., has no AABBs).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            BlockShape::None => true,
            BlockShape::Single(shape) => shape.const_eq(&BlockAabb::EMPTY),
            #[cfg(not(feature = "alloc"))]
            BlockShape::Collection(slice) => slice.is_empty(),
            #[cfg(feature = "alloc")]
            BlockShape::Collection(alloc::borrow::Cow::Borrowed(slice)) => slice.is_empty(),
            #[cfg(feature = "alloc")]
            BlockShape::Collection(alloc::borrow::Cow::Owned(vec)) => vec.is_empty(),
        }
    }

    /// Returns the block's [`BlockAabb`]s as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[BlockAabb] {
        match self {
            BlockShape::None => &[],
            BlockShape::Single(aabb) => core::slice::from_ref(aabb),
            #[cfg(not(feature = "alloc"))]
            BlockShape::Collection(slice) => slice,
            #[cfg(feature = "alloc")]
            BlockShape::Collection(alloc::borrow::Cow::Borrowed(slice)) => slice,
            #[cfg(feature = "alloc")]
            BlockShape::Collection(alloc::borrow::Cow::Owned(vec)) => vec.as_slice(),
        }
    }

    /// Adds a new [`BlockAabb`] to the [`BlockShape`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn add_shape(self, shape: BlockShape<'_>) -> BlockShape<'static> {
        match (self, shape) {
            (aabb, BlockShape::None) | (BlockShape::None, aabb) => aabb.into_owned(),
            (BlockShape::Single(aabb_a), BlockShape::Single(aabb_b)) => {
                BlockShape::Collection(alloc::borrow::Cow::Owned(vec![aabb_a, aabb_b]))
            }
            (BlockShape::Single(aabb), BlockShape::Collection(mut cow))
            | (BlockShape::Collection(mut cow), BlockShape::Single(aabb)) => {
                cow.to_mut().push(aabb);
                BlockShape::Collection(cow).into_owned()
            }
            (BlockShape::Collection(mut cow_a), BlockShape::Collection(cow_b)) => {
                cow_a.to_mut().extend(cow_b.into_owned());
                BlockShape::Collection(cow_a).into_owned()
            }
        }
    }

    /// Creates an owned [`BlockShape`], cloning data if necessary.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn into_owned(self) -> BlockShape<'static> {
        match self {
            BlockShape::None => BlockShape::None,
            BlockShape::Single(aabb) => BlockShape::Single(aabb),
            BlockShape::Collection(alloc::borrow::Cow::Borrowed(slice)) => {
                BlockShape::Collection(alloc::borrow::Cow::Owned(slice.to_vec()))
            }
            BlockShape::Collection(alloc::borrow::Cow::Owned(vec)) => {
                BlockShape::Collection(alloc::borrow::Cow::Owned(vec))
            }
        }
    }
}

impl Default for BlockShape<'_> {
    fn default() -> Self { Self::Single(BlockAabb::FULL) }
}

#[cfg(feature = "alloc")]
impl Add<BlockShape<'_>> for BlockShape<'_> {
    type Output = BlockShape<'static>;

    fn add(self, rhs: BlockShape<'_>) -> Self::Output { self.add_shape(rhs) }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct BlockAabb {
    min: DVec3,
    max: DVec3,
}

impl BlockAabb {
    /// An empty block.
    pub const EMPTY: Self = Self::new(DVec3::ZERO, DVec3::ZERO);
    /// A full block.
    pub const FULL: Self = Self::new(DVec3::ZERO, DVec3::ONE);

    /// Creates a new [`BlockAabb`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new(min: DVec3, max: DVec3) -> Self { Self { min, max } }

    /// Creates a new [`BlockAabb`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new_xyz(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> Self {
        Self::new(DVec3::new(min_x, min_y, min_z), DVec3::new(max_x, max_y, max_z))
    }

    /// Creates a new [`BlockAabb`] from the given two corner points.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new_corners(a: DVec3, b: DVec3) -> Self {
        Self::new_xyz(
            a.x.min(b.x),
            a.y.min(b.y),
            a.z.min(b.z),
            a.x.max(b.x),
            a.y.max(b.y),
            a.z.max(b.z),
        )
    }

    /// Creates a new [`BlockAabb`] from the given two corner points.
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    pub fn new_corners(a: DVec3, b: DVec3) -> Self {
        Self::new_xyz(
            libm::fmin(a.x, b.x),
            libm::fmin(a.y, b.y),
            libm::fmin(a.z, b.z),
            libm::fmax(a.x, b.x),
            libm::fmax(a.y, b.y),
            libm::fmax(a.z, b.z),
        )
    }

    /// Compares two [`BlockAabb`]s for equality in a `const` context.
    #[must_use]
    pub const fn const_eq(&self, other: &Self) -> bool {
        (self.min.x - other.min.x).abs() < EPSILON
            && (self.min.y - other.min.y).abs() < EPSILON
            && (self.min.z - other.min.z).abs() < EPSILON
            && (self.max.x - other.max.x).abs() < EPSILON
            && (self.max.y - other.max.y).abs() < EPSILON
            && (self.max.z - other.max.z).abs() < EPSILON
    }

    /// Returns `true` if the given point is contained within the AABB.
    #[must_use]
    pub const fn contains(&self, point: DVec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    /// Returns the minimum coordinates of the AABB.
    #[must_use]
    pub const fn min(self) -> DVec3 { self.min }

    /// Returns the maximum coordinates of the AABB.
    #[must_use]
    pub const fn max(self) -> DVec3 { self.max }

    /// Returns the minimum and maximum coordinates of the AABB as a tuple.
    #[must_use]
    pub const fn min_max(self) -> (DVec3, DVec3) { (self.min, self.max) }
}

impl Default for BlockAabb {
    fn default() -> Self { Self::FULL }
}

impl PartialEq for BlockAabb {
    fn eq(&self, other: &Self) -> bool {
        self.min.abs_diff_eq(other.min, EPSILON) && self.max.abs_diff_eq(other.max, EPSILON)
    }
}
