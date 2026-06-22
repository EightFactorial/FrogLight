//! TODO

use facet::Facet;
use froglight_facet_iter::{
    Reader, ReaderError, Writer, WriterError, deserialize::DeserializeItem,
    serialize::SerializeItem,
};

pub mod template {
    //! Re-exports of everything needed for implementing [`FacetTemplate`].

    pub use froglight_facet_iter::{
        Reader, ReaderError, Writer, WriterError, deserialize::DeserializeItem,
        serialize::SerializeItem,
    };

    pub use crate::facet::{FacetBorrowedTemplate, FacetTemplate};
}

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

// -------------------------------------------------------------------------------------------------

/// A [`Facet`] attribute containing custom serialization and deserialization
/// functions.
#[derive(Debug, Clone, Copy, Facet)]
#[facet(opaque)]
pub struct WithFnAttr {
    ser: SerFn,
    de_owned: DeFn<false>,
    de_owned_borrow: DeFn<true>,
    de_borrowed: Option<DeBorrowFn>,
}

impl WithFnAttr {
    /// Create a new [`WithFns`] using the provided template type.
    #[inline]
    #[must_use]
    pub const fn template<T: FacetTemplate>() -> Self {
        Self::using(T::serialize, T::deserialize::<false>, T::deserialize::<true>)
    }

    /// Create a new [`WithFns`] using the provided functions.
    #[inline]
    #[must_use]
    pub const fn using(ser: SerFn, de_owned: DeFn<false>, de_owned_borrow: DeFn<true>) -> Self {
        Self { ser, de_owned, de_owned_borrow, de_borrowed: None }
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
    /// # Note
    ///
    /// Despite the `BORROW` generic,
    /// this does not borrow any data from the [`Reader`].
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    #[inline]
    pub fn deserialize<'facet, const BORROW: bool>(
        &self,
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        if BORROW {
            let f = self.de_owned_borrow;

            // SAFETY: `BORROW` is `true`,
            // so partial is already a `DeserializeItem<'facet, true>`.
            let partial: DeserializeItem<'facet, true> = unsafe { core::mem::transmute(item) };
            let partial: DeserializeItem<'facet, BORROW> =
                unsafe { core::mem::transmute(f(partial, reader)?) };

            Ok(partial)
        } else {
            let f = self.de_owned;

            // SAFETY: `BORROW` is `false`,
            // so partial is already a `DeserializeItem<'facet, false>`.
            let partial: DeserializeItem<'facet, false> = unsafe { core::mem::transmute(item) };
            let partial: DeserializeItem<'facet, BORROW> =
                unsafe { core::mem::transmute(f(partial, reader)?) };

            Ok(partial)
        }
    }

    /// Returns `true` if this attribute has a borrowed deserialization
    /// function.
    #[inline]
    #[must_use]
    pub const fn has_borrowed(&self) -> bool { self.de_borrowed.is_some() }

    /// Deserialize using this attribute's borrowed deserialization function.
    ///
    /// # Note
    ///
    /// This attempts to use the fully borrowed deserialization function if it
    /// exists, otherwise it falls back to owned deserialization.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    pub fn deserialize_borrowed<'facet>(
        &self,
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError> {
        let f = self.de_borrowed.unwrap_or(self.de_owned_borrow);

        f(item, reader)
    }
}

/// A serialization function.
pub type SerFn = fn(SerializeItem<'_, '_>, &mut Writer<'_>) -> Result<(), WriterError>;
/// A deserialization function.
pub type DeFn<const BORROW: bool> =
    for<'facet> fn(
        DeserializeItem<'facet, BORROW>,
        &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError>;
/// A borrowed deserialization function.
pub type DeBorrowFn = for<'facet> fn(
    DeserializeItem<'facet, true>,
    &mut Reader<'facet>,
) -> Result<DeserializeItem<'facet, true>, ReaderError>;

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
/// use froglight_facet::{self as mc, facet::template::*, from_slice, to_vec};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
/// #[facet(mc::with = MyType::WITH)]
/// struct MyType(u32);
///
/// impl FacetTemplate for MyType {
///     fn serialize(
///         item: SerializeItem<'_, '_>,
///         writer: &mut Writer<'_>,
///     ) -> Result<(), WriterError> {
///         let val = item.peek().get::<MyType>().unwrap();
///         writer.write_bytes(&val.0.to_le_bytes())
///     }
///
///     fn deserialize<'facet, const BORROW: bool>(
///         item: DeserializeItem<'facet, BORROW>,
///         reader: &mut Reader<'_>,
///     ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
///         let val: u32 = u32::from_le_bytes(*reader.get_array()?);
///         item.set(MyType(val))
///     }
/// }
///
/// // Check that `MyType` was serialized correctly.
/// let serialized = to_vec(&MyType(42)).unwrap();
/// assert_eq!(serialized, [42, 0, 0, 0]);
///
/// // Check that `MyType` was deserialized correctly.
/// let deserialized: MyType = from_slice(&serialized).unwrap();
/// assert_eq!(deserialized, MyType(42));
/// ```
pub trait FacetTemplate: Sized {
    /// A [`WithFnAttr`] to be used with
    /// `#[derive(Facet)]` in a `#[facet(mc::with = ...)]`
    /// attribute.
    ///
    /// See [`FacetTemplate`] for more details and an example.
    const WITH: WithFnAttr = WithFnAttr::template::<Self>();

    /// The serialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError>;

    /// The deserialization function.
    ///
    /// # Note
    ///
    /// Despite the `BORROW` generic,
    /// this does not borrow any data from the [`Reader`].
    ///
    /// See [`FacetBorrowedTemplate`] for actually borrowed deserialization.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError>;
}

/// A template trait for custom borrowed deserialization functions.
///
/// Must be used with the `#[facet(mc::with = T::WITH_BORROW)]` attribute to
/// take effect.
///
/// See [`FacetTemplate`] for more details and an example.
pub trait FacetBorrowedTemplate: FacetTemplate {
    /// A [`WithFnAttr`] to be used with
    /// `#[derive(Facet)]` in a `#[facet(mc::with = ...)]`
    /// attribute.
    ///
    /// See [`FacetTemplate`] for more details and an example.
    const WITH_BORROW: WithFnAttr =
        WithFnAttr::template::<Self>().with_borrow(Self::deserialize_borrowed);

    /// The borrowed deserialization function.
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails.
    fn deserialize_borrowed<'facet>(
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError>;
}
