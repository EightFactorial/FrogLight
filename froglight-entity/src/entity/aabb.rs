use glam::{Vec2, Vec3};

// Using a larger epsilon to match original behavior.
const EPSILON: f32 = 1e-7;

/// An axis-aligned bounding box (AABB) for an entity.
#[derive(Debug, Clone, Copy)]
pub struct EntityAabb {
    size: Vec2,
    eyes: f32,
}

impl EntityAabb {
    /// Creates a new [`EntityAabb`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new(size: Vec2, eyes: f32) -> Self { Self { size, eyes } }

    /// Creates a new [`EntityAabb`] from the given minimum, maximum, and eye
    /// height.
    #[must_use]
    pub const fn new_xy(min_xz: f32, min_y: f32, max_xz: f32, max_y: f32, eyes: f32) -> Self {
        Self::new(Vec2::new(max_xz - min_xz, max_y - min_y), eyes)
    }

    /// Creates a new [`EntityAabb`] from the given two corner points and eye
    /// height.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new_corners(a: Vec2, b: Vec2, eyes: f32) -> Self {
        Self::new_xy(a.x.min(b.x), a.y.min(b.y), a.x.max(b.x), a.y.max(b.y), eyes)
    }

    /// Creates a new [`EntityAabb`] from the given two corner points.
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    pub fn new_corners(a: Vec2, b: Vec2, eyes: f32) -> Self {
        Self::new_xy(
            libm::fminf(a.x, b.x),
            libm::fminf(a.y, b.y),
            libm::fmaxf(a.x, b.x),
            libm::fmaxf(a.y, b.y),
            eyes,
        )
    }

    /// Compares two [`EntityAabb`]s for equality in a `const` context.
    #[must_use]
    pub const fn const_eq(&self, other: &Self) -> bool {
        (self.size.x - other.size.x).abs() < EPSILON
            && (self.size.y - other.size.y).abs() < EPSILON
            && (self.eyes - other.eyes).abs() < EPSILON
    }

    /// Returns `true` if the given point is contained within the AABB.
    #[must_use]
    pub const fn contains(&self, point: Vec3) -> bool {
        point.x >= 0.0
            && point.x <= self.size.x
            && point.y >= 0.0
            && point.y <= self.size.y
            && point.z >= 0.0
            && point.z <= self.size.x
    }

    /// Returns the eye height of the AABB.
    #[must_use]
    pub const fn eye_height(self) -> f32 { self.eyes }

    /// Returns the eye position inside the AABB.
    #[must_use]
    pub const fn eyes(self) -> Vec3 { Vec3::new(self.size.x / 2., self.eyes, self.size.x / 2.) }

    /// Returns the minimum coordinates of the AABB.
    #[must_use]
    pub const fn min(self) -> Vec3 { Vec3::new(0., 0., 0.) }

    /// Returns the maximum coordinates of the AABB.
    #[must_use]
    pub const fn max(self) -> Vec3 { Vec3::new(self.size.x, self.size.y, self.size.x) }

    /// Returns the minimum and maximum coordinates of the AABB as a tuple.
    #[must_use]
    pub const fn min_max(self) -> (Vec3, Vec3) { (self.min(), self.max()) }
}

impl PartialEq for EntityAabb {
    fn eq(&self, other: &Self) -> bool {
        self.size.abs_diff_eq(other.size, EPSILON) && (self.eyes - other.eyes).abs() < EPSILON
    }
}
