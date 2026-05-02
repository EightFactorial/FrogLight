use core::ops::Deref;

/// An immutable reference.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Ref;

/// A mutable reference.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Mut;

// -------------------------------------------------------------------------------------------------

/// A trait for [`Ref`] and [`Mut`].
pub trait NbtMut: Copy + Eq + sealed::Sealed + 'static {
    /// The type of the reference to use.
    type Of<'a, T: ?Sized + 'a>: Deref<Target = T> + 'a;

    /// Shorten the lifetime of a reference to `'b`.
    fn shrink<'a, 'b, T: ?Sized + 'a>(value: Self::Of<'a, T>) -> Self::Of<'b, T>
    where
        'a: 'b;
}

impl NbtMut for Ref {
    type Of<'a, T: ?Sized + 'a> = &'a T;

    fn shrink<'a, 'b, T: ?Sized + 'a>(value: Self::Of<'a, T>) -> Self::Of<'b, T>
    where
        'a: 'b,
    {
        value
    }
}
impl NbtMut for Mut {
    type Of<'a, T: ?Sized + 'a> = &'a mut T;

    fn shrink<'a, 'b, T: ?Sized + 'a>(value: Self::Of<'a, T>) -> Self::Of<'b, T>
    where
        'a: 'b,
    {
        value
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Ref {}
    impl Sealed for super::Mut {}
}
