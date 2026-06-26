use alloc::{borrow::Cow, vec};
use core::ops::Add;

use glam::DVec3;

/// Using larger epsilon to match original behavior.
const EPSILON_F64: f64 = 1e-7;

/// The shape of a block, defined as zero or more [`BlockAabb`]s.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockShape<'a> {
    /// An empty block shape with no AABBs.
    None,
    /// A block shape with a single AABB.
    Single(BlockAabb),
    /// A block shape with multiple AABBs.
    Collection(Cow<'a, [BlockAabb]>),
}

/// A block's axis-aligned bounding box (AABB).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockAabb {
    pub min: DVec3,
    pub max: DVec3,
}

impl BlockShape<'_> {
    /// An empty block.
    pub const EMPTY: Self = BlockShape::None;
    /// A full block.
    pub const FULL: Self = BlockShape::Single(BlockAabb { min: DVec3::ZERO, max: DVec3::ONE });

    /// Creates a new [`BlockShape`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new(min: DVec3, max: DVec3) -> Self {
        if (max.x - min.x).abs() > EPSILON_F64
            && (max.y - min.y).abs() > EPSILON_F64
            && (max.z - min.z).abs() > EPSILON_F64
        {
            BlockShape::None
        } else {
            BlockShape::Single(BlockAabb { min, max })
        }
    }

    /// Creates a new [`BlockShape`] from the given minimum and maximum
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

    /// Creates a new [`BlockShape`] from the given two corner points.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new_from_corners(a: DVec3, b: DVec3) -> Self {
        if (a.x - b.x).abs() > EPSILON_F64
            && (a.y - b.y).abs() > EPSILON_F64
            && (a.z - b.z).abs() > EPSILON_F64
        {
            BlockShape::None
        } else {
            BlockShape::Single(BlockAabb {
                min: DVec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
                max: DVec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
            })
        }
    }

    /// Creates a new [`BlockShape`] from the given two corner points.
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    pub fn new_from_corners(a: DVec3, b: DVec3) -> Self {
        if (a.x - b.x).abs() > EPSILON_F64
            && (a.y - b.y).abs() > EPSILON_F64
            && (a.z - b.z).abs() > EPSILON_F64
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
            let aabb = self.as_slice()[index];

            if point.x >= aabb.min.x
                && point.x <= aabb.max.x
                && point.y >= aabb.min.y
                && point.y <= aabb.max.y
                && point.z >= aabb.min.z
                && point.z <= aabb.max.z
            {
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
            BlockShape::Single(shape) => {
                shape.min.x.abs() < EPSILON_F64
                    && shape.min.y.abs() < EPSILON_F64
                    && shape.min.z.abs() < EPSILON_F64
                    && shape.max.x.abs() < EPSILON_F64
                    && shape.max.y.abs() < EPSILON_F64
                    && shape.max.z.abs() < EPSILON_F64
            }
            BlockShape::Collection(Cow::Borrowed(slice)) => slice.is_empty(),
            BlockShape::Collection(Cow::Owned(vec)) => vec.is_empty(),
        }
    }

    /// Returns the block's [`CommonDAabb`]s as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[BlockAabb] {
        match self {
            BlockShape::None => &[],
            BlockShape::Single(aabb) => core::slice::from_ref(aabb),
            BlockShape::Collection(Cow::Borrowed(slice)) => slice,
            BlockShape::Collection(Cow::Owned(vec)) => vec.as_slice(),
        }
    }

    /// Combine this [`BlockShape`] with another.
    #[must_use]
    pub fn with_shape(self, shape: BlockShape<'_>) -> BlockShape<'static> {
        match (self, shape) {
            (aabb, BlockShape::None) | (BlockShape::None, aabb) => aabb.into_owned(),
            (BlockShape::Single(aabb_a), BlockShape::Single(aabb_b)) => {
                BlockShape::Collection(Cow::Owned(vec![aabb_a, aabb_b]))
            }
            (BlockShape::Single(aabb), BlockShape::Collection(cow))
            | (BlockShape::Collection(cow), BlockShape::Single(aabb)) => {
                let mut cow = cow.into_owned();
                cow.push(aabb);
                BlockShape::Collection(Cow::Owned(cow))
            }
            (BlockShape::Collection(cow_a), BlockShape::Collection(cow_b)) => {
                let mut cow_a = cow_a.into_owned();
                cow_a.extend(cow_b.into_owned());
                BlockShape::Collection(Cow::Owned(cow_a))
            }
        }
    }

    /// Creates an owned [`BlockShape`], cloning data if necessary.
    #[must_use]
    pub fn into_owned(self) -> BlockShape<'static> {
        match self {
            BlockShape::None => BlockShape::None,
            BlockShape::Single(aabb) => BlockShape::Single(aabb),
            BlockShape::Collection(Cow::Owned(vec)) => BlockShape::Collection(Cow::Owned(vec)),
            BlockShape::Collection(Cow::Borrowed(slice)) => {
                BlockShape::Collection(Cow::Owned(slice.to_vec()))
            }
        }
    }
}

impl Default for BlockShape<'_> {
    fn default() -> Self { Self::FULL }
}

impl Add<BlockShape<'_>> for BlockShape<'_> {
    type Output = BlockShape<'static>;

    fn add(self, rhs: BlockShape<'_>) -> Self::Output { self.with_shape(rhs) }
}
