//! TODO

#[cfg(all(feature = "bevy", feature = "serde"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

/// A type that can never be constructed.
///
/// Equivalent to [`Infallible`](core::convert::Infallible),
/// but implements both [`Reflect`](bevy_reflect::Reflect) and
/// [`Facet`](facet::Facet).
///
/// Will be removed if/when [`Reflect`](bevy_reflect::Reflect) is implemented
/// for [`Infallible`](core::convert::Infallible).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum Impossible {}

#[cfg(feature = "facet")]
unsafe impl facet::Facet<'_> for Impossible {
    // Normally this would be an insanely bad idea,
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
