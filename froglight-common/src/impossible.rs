//! [`Impossible`]

use core::ops::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

/// A type that can never be constructed.
///
/// Equivalent to [`Infallible`](core::convert::Infallible), but implements both
/// `Reflect` and [`Facet`](facet::Facet).
///
/// Will be removed if/when `Reflect` is implemented for
/// [`Infallible`](core::convert::Infallible).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq, Hash))]
pub struct Impossible(!);

#[cfg(feature = "facet")]
unsafe impl facet::Facet<'_> for Impossible {
    // Normally this would be an incredibly unsound,
    // but since it can never be constructed it should be fine.
    const SHAPE: &'static facet::Shape = &const {
        use core::convert::Infallible;

        facet::ShapeBuilder::for_sized::<Impossible>("Impossible")
            .ty(Infallible::SHAPE.ty)
            .def(Infallible::SHAPE.def)
            .vtable(Infallible::SHAPE.vtable)
            .type_ops(Infallible::SHAPE.type_ops.unwrap())
            .eq()
            .copy()
            .send()
            .sync()
            .build()
    };
}

// -------------------------------------------------------------------------------------------------

impl AsRef<!> for Impossible {
    fn as_ref(&self) -> &! { &self.0 }
}
impl AsMut<!> for Impossible {
    fn as_mut(&mut self) -> &mut ! { &mut self.0 }
}

impl Deref for Impossible {
    type Target = !;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for Impossible {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<!> for Impossible {
    fn from(_: !) -> Self { unreachable!() }
}
impl From<Impossible> for ! {
    fn from(_: Impossible) -> Self { unreachable!() }
}
