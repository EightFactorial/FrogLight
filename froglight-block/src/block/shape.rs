#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, vec};
use core::ops::Add;

use froglight_common::aabb::CommonAabb;
use glam::DVec3;

// Using a larger epsilon to match original behavior.
const EPSILON: f64 = 1e-7;

/// The shape of a block.
///
/// Defined as zero or more [`CommonAabb`]s.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockShape<'a> {
    /// An empty block shape with no AABBs.
    None,
    /// A block shape with a single AABB.
    Single(CommonAabb),
    /// A block shape with multiple AABBs.
    #[cfg(not(feature = "alloc"))]
    Collection(&'a [CommonAabb]),
    /// A block shape with multiple AABBs.
    #[cfg(feature = "alloc")]
    Collection(Cow<'a, [CommonAabb]>),
}

impl BlockShape<'_> {
    /// An empty block.
    pub const EMPTY: Self = BlockShape::None;
    /// A full block.
    pub const FULL: Self = BlockShape::Single(CommonAabb::ONE);

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
            BlockShape::Single(CommonAabb::new(min, max))
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
        if (a.x - b.x).abs() > EPSILON && (a.y - b.y).abs() > EPSILON && (a.z - b.z).abs() > EPSILON
        {
            BlockShape::None
        } else {
            BlockShape::Single(CommonAabb::new_corners(b, a))
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
            BlockShape::Single(CommonAabb::new_corners(b, a))
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
            BlockShape::Single(shape) => shape.const_eq(&CommonAabb::ZERO),
            #[cfg(not(feature = "alloc"))]
            BlockShape::Collection(slice) => slice.is_empty(),
            #[cfg(feature = "alloc")]
            BlockShape::Collection(Cow::Borrowed(slice)) => slice.is_empty(),
            #[cfg(feature = "alloc")]
            BlockShape::Collection(Cow::Owned(vec)) => vec.is_empty(),
        }
    }

    /// Returns the block's [`CommonAabb`]s as a slice.
    #[must_use]
    pub const fn as_slice(&self) -> &[CommonAabb] {
        match self {
            BlockShape::None => &[],
            BlockShape::Single(aabb) => core::slice::from_ref(aabb),
            #[cfg(not(feature = "alloc"))]
            BlockShape::Collection(slice) => slice,
            #[cfg(feature = "alloc")]
            BlockShape::Collection(Cow::Borrowed(slice)) => slice,
            #[cfg(feature = "alloc")]
            BlockShape::Collection(Cow::Owned(vec)) => vec.as_slice(),
        }
    }

    /// Adds a new [`CommonAabb`] to the [`BlockShape`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn add_shape(self, shape: BlockShape<'_>) -> BlockShape<'static> {
        match (self, shape) {
            (aabb, BlockShape::None) | (BlockShape::None, aabb) => aabb.into_owned(),
            (BlockShape::Single(aabb_a), BlockShape::Single(aabb_b)) => {
                BlockShape::Collection(Cow::Owned(vec![aabb_a, aabb_b]))
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
            BlockShape::Collection(Cow::Borrowed(slice)) => {
                BlockShape::Collection(Cow::Owned(slice.to_vec()))
            }
            BlockShape::Collection(Cow::Owned(vec)) => BlockShape::Collection(Cow::Owned(vec)),
        }
    }
}

impl Default for BlockShape<'_> {
    fn default() -> Self { Self::FULL }
}

#[cfg(feature = "alloc")]
impl Add<BlockShape<'_>> for BlockShape<'_> {
    type Output = BlockShape<'static>;

    fn add(self, rhs: BlockShape<'_>) -> Self::Output { self.add_shape(rhs) }
}
