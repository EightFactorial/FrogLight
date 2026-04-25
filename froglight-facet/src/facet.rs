//! TODO

pub use ::facet;
use facet::{Facet, Partial, ReflectError, ReflectErrorKind};

use crate::format::{Reader, Writer, WriterError, serialize::SerializeItem};

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

        /// Mark whether a struct or enum should pass it's variable-length encoding state to its fields.
        ///
        /// This is useful for newtypes that want to pass outer `#[facet(mc::variable)]` attributes to their inner fields.
        VariableInner,

        /// Use custom serialization and deserialization functions for a type or field.
        With(fn_ptr WithFnAttr),
    }
}

/// A [`Facet`] attribute containing custom serialization and deserialization
/// functions.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct WithFnAttr {
    ser: SerFn,
    de_owned: DeFn,
    de_borrowed: Option<DeBorrowFn>,
}

impl WithFnAttr {
    /// Create a new [`WithFns`] using the provided template type.
    #[inline]
    #[must_use]
    pub const fn template<T: FacetTemplate>() -> Self { Self::using(T::serialize, T::deserialize) }

    /// Create a new [`WithFns`] using the provided functions.
    #[inline]
    #[must_use]
    pub const fn using(ser: SerFn, de: DeFn) -> Self {
        Self { ser, de_owned: de, de_borrowed: None }
    }

    /// Set the borrowed deserialization function.
    #[inline]
    #[must_use]
    pub const fn with_borrow(mut self, borrow: DeBorrowFn) -> Self {
        self.de_borrowed = Some(borrow);
        self
    }

    /// Serialize using this attribute's serialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    #[inline]
    pub fn serialize(
        &self,
        item: SerializeItem<'_, '_>,
        writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        (self.ser)(item, writer)
    }

    /// Deserialize using this attribute's deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn deserialize(
        &self,
        partial: Partial<'static, false>,
        reader: &mut Reader<'_>,
    ) -> Result<Partial<'static, false>, ReflectError> {
        (self.de_owned)(partial, reader)
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
    /// deserializer function or if deserialization fails.
    pub fn deserialize_borrowed<'facet>(
        &self,
        partial: Partial<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<Partial<'facet, true>, ReflectError> {
        match self.de_borrowed {
            Some(de) => de(partial, reader),
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
pub type SerFn = fn(SerializeItem<'_, '_>, &mut Writer<'_>) -> Result<(), WriterError>;
/// A deserialization function.
pub type DeFn =
    fn(Partial<'static, false>, &mut Reader<'_>) -> Result<Partial<'static, false>, ReflectError>;
/// A borrowed deserialization function.
pub type DeBorrowFn = for<'facet> fn(
    Partial<'facet, true>,
    &mut Reader<'facet>,
) -> Result<Partial<'facet, true>, ReflectError>;

// -------------------------------------------------------------------------------------------------

/// A template trait for custom serialization and deserialization functions.
///
/// Must be used with the `#[facet(mc::with = T::WITH)]` attribute to take
/// effect.
///
/// # Example
///
/// ```rust
/// use facet::{Facet, Partial, ReflectError};
/// use froglight_facet::{
///     self as mc,
///     facet::FacetTemplate,
///     format::{Reader, Writer, WriterError, serialize::SerializeItem},
///     to_vec,
/// };
///
/// #[derive(Facet)]
/// #[facet(mc::with = MyType::WITH)]
/// struct MyType(u32);
///
/// impl FacetTemplate for MyType {
///     fn serialize(
///         item: SerializeItem<'_, '_>,
///         writer: &mut Writer<'_>,
///     ) -> Result<(), WriterError> {
///         let val = item.peek().get::<MyType>().unwrap();
///         writer.write_bytes(&val.0.to_be_bytes())
///     }
///
///     fn deserialize(
///         _: Partial<'static, false>,
///         _: &mut Reader<'_>,
///     ) -> Result<Partial<'static, false>, ReflectError> {
///         todo!()
///     }
/// }
///
/// // Check that `MyType` was serialized correctly.
/// let serialized = to_vec(&MyType(42)).unwrap();
/// assert_eq!(serialized, [0, 0, 0, 42]);
/// ```
pub trait FacetTemplate: 'static + Sized {
    /// A [`WithFnAttr`] that uses this template.
    const WITH: WithFnAttr = WithFnAttr::template::<Self>();

    /// The serialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError>;

    /// The deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    fn deserialize(
        partial: Partial<'static, false>,
        reader: &mut Reader<'_>,
    ) -> Result<Partial<'static, false>, ReflectError>;
}

/// A template trait for custom borrowed deserialization functions.
///
/// Must be used with the `#[facet(mc::with = T::WITH_BORROW)]` attribute to
/// take effect.
pub trait FacetBorrowedTemplate: FacetTemplate {
    /// A [`WithFnAttr`] that uses this template.
    const WITH_BORROW: WithFnAttr =
        WithFnAttr::template::<Self>().with_borrow(Self::deserialize_borrowed);

    /// The borrowed deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    fn deserialize_borrowed<'facet>(
        partial: Partial<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<Partial<'facet, true>, ReflectError>;
}
