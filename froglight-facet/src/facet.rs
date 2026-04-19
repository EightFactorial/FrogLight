//! TODO

use core::any::TypeId;

use facet::Facet;

facet::define_attr_grammar! {
    ns "mc";
    crate_path ::froglight_facet::facet;

    /// Attributes for customizing serialization and deserialization.
    #[derive(Copy, Facet)]
    pub enum Attr {
        /// Mark a field as using variable-length encoding.
        ///
        /// See the [Minecraft Wiki](https://minecraft.wiki/w/Java_Edition_protocol/VarInt_and_VarLong)
        /// or [Wikipedia](https://en.wikipedia.org/wiki/LEB128) for more details.
        Variable,

        /// Use custom serialization and deserialization functions for a type or field.
        With(fn_ptr WithFnAttr),
    }

}

/// TODO
#[expect(dead_code, reason = "WIP")]
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct WithFnAttr {
    ty: TypeId,
    ser: SerFn,
    de: DeFn,
}

/// A serialization function.
pub type SerFn = fn();
/// A deserialization function.
pub type DeFn = fn();

impl WithFnAttr {
    /// Create a new [`WithFns`].
    #[inline]
    #[must_use]
    pub const fn new<T: 'static>() -> Self { Self::using::<T>(|| {}, || {}) }

    /// Create a new [`WithFns`] using the provided functions.
    #[inline]
    #[must_use]
    pub const fn using<T: 'static>(ser: SerFn, de: DeFn) -> Self {
        Self { ty: TypeId::of::<T>(), ser, de }
    }
}
