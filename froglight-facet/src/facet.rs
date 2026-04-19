//! TODO

use core::any::TypeId;

pub use ::facet;
use facet::{Facet, Partial, Peek, ReflectError, ReflectErrorKind};

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

/// A [`Facet`] attribute containing custom serialization and deserialization
/// functions.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct WithFnAttr {
    ty: TypeId,
    ser: SerFn,
    de_owned: DeFn,
    de_borrowed: Option<DeBorrowFn>,
}

impl WithFnAttr {
    /// Create a new [`WithFns`].
    #[inline]
    #[must_use]
    pub const fn new<T: FacetTemplate>() -> Self { Self::using::<T>(T::serialize, T::deserialize) }

    /// Create a new [`WithFns`] using the provided functions.
    #[inline]
    #[must_use]
    pub const fn using<T: 'static>(ser: SerFn, de: DeFn) -> Self {
        Self { ty: TypeId::of::<T>(), ser, de_owned: de, de_borrowed: None }
    }

    /// Set the borrowed deserialization function.
    #[inline]
    #[must_use]
    pub const fn with_borrow(mut self, borrow: DeBorrowFn) -> Self {
        self.de_borrowed = Some(borrow);
        self
    }

    /// Returns `true` if this attribute is for the type `T`.
    #[inline]
    #[must_use]
    pub fn is_for<T: 'static>(&self) -> bool { self.ty == TypeId::of::<T>() }

    /// Serialize using this attribute's serialization function.
    #[inline]
    pub fn serialize(&self, peek: Peek<'_, 'static>) { (self.ser)(peek) }

    /// Deserialize using this attribute's deserialization function.
    ///
    /// # Errors
    ///
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn deserialize(
        &self,
        partial: Partial<'static, false>,
    ) -> Result<Partial<'static, false>, ReflectError> {
        (self.de_owned)(partial)
    }

    /// Returns `true` if this attribute has a borrowed deserialization
    /// function.
    #[inline]
    #[must_use]
    pub const fn has_borrowed(&self) -> bool { self.de_borrowed.is_some() }

    /// Deserialize using this attribute's borrowed deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if this attribute does not have a borrowed
    /// deserialization function or if deserialization fails.
    pub fn deserialize_borrowed<'facet>(
        &self,
        partial: Partial<'facet, true>,
    ) -> Result<Partial<'facet, true>, ReflectError> {
        match self.de_borrowed {
            Some(de) => de(partial),
            None => Err(ReflectError {
                path: partial.path(),
                kind: ReflectErrorKind::OperationFailed {
                    shape: partial.shape(),
                    operation: "borrowed deserialization",
                },
            }),
        }
    }
}

/// A serialization function.
pub type SerFn = fn(Peek<'_, 'static>);
/// A deserialization function.
pub type DeFn = fn(Partial<'static, false>) -> Result<Partial<'static, false>, ReflectError>;
/// A borrowed deserialization function.
pub type DeBorrowFn =
    for<'facet> fn(Partial<'facet, true>) -> Result<Partial<'facet, true>, ReflectError>;

// -------------------------------------------------------------------------------------------------

/// A template trait for custom serialization and deserialization functions.
///
/// Must be used with [`WithFnAttr::new`] or [`WithFnAttr::using`] to take
/// effect.
pub trait FacetTemplate: 'static {
    /// The serialization function.
    fn serialize(peek: Peek<'_, 'static>);

    /// The deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    fn deserialize(
        partial: Partial<'static, false>,
    ) -> Result<Partial<'static, false>, ReflectError>;
}

/// A template trait for custom borrowed deserialization functions.
///
/// Must be used with [`WithFnAttr::with_borrow`] to take effect.
pub trait FacetBorrowdedTemplate<'facet> {
    /// The borrowed deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    fn deserialize_borrowed(
        partial: Partial<'facet, true>,
    ) -> Result<Partial<'facet, true>, ReflectError>;
}
