use core::fmt;

use facet::{Attr, Facet, Field, Partial, Shape};

use crate::ReaderError;

#[derive(Debug)]
pub(super) enum StackItem {
    Item(DeserializeDesc),
    Fields(usize, &'static [Field], bool),

    Seq(usize, bool, bool),
    Map(usize, bool, bool, bool),
    Set(usize, bool, bool),

    Other(DeserializeDesc),
}

/// A [`Deserializer`] item.
#[derive(Debug)]
pub enum Item<'facet, const BORROW: bool> {
    /// A size to be deserialized.
    Size(u32),
    /// An item to be deserialized.
    Item(DeserializeItem<'facet, BORROW>),
}

impl StackItem {
    /// Get the name of the variant of this [`StackItem`].
    #[must_use]
    #[allow(dead_code, reason = "Used for tracing")]
    pub(super) const fn variant_name(&self) -> &'static str {
        match self {
            StackItem::Item(_) => "Item",
            StackItem::Fields(..) => "Fields",
            StackItem::Seq(..) => "Seq",
            StackItem::Map(..) => "Map",
            StackItem::Set(..) => "Set",
            StackItem::Other(_) => "Other",
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A description of a deserialization item.
#[derive(Debug, Clone)]
pub struct DeserializeDesc {
    variable: bool,
    field: Option<&'static Field>,
}

impl DeserializeDesc {
    /// Create a new [`DeserializeDesc`].
    #[inline]
    #[must_use]
    pub const fn new(variable: bool, field: Option<&'static Field>) -> Self {
        Self { variable, field }
    }

    /// Returns `true` if this [`DeserializeDesc`] is variable-length.
    #[inline]
    #[must_use]
    pub const fn is_variable(&self) -> bool { self.variable }

    /// Set whether this [`DeserializeDesc`] is variable-length.
    #[inline]
    pub const fn set_variable(&mut self, variable: bool) { self.variable = variable; }

    /// Get the [`Field`] this [`DeserializeDesc`] came from, if any.
    #[inline]
    #[must_use]
    pub const fn field(&self) -> Option<&'static Field> { self.field }

    /// Get the [`Field`] [`Attr`]s of this [`DeserializeDesc`], if any.
    #[inline]
    #[must_use]
    pub const fn field_attr(&self) -> Option<&'static [Attr]> {
        if let Some(field) = self.field { Some(field.attributes) } else { None }
    }
}

// -------------------------------------------------------------------------------------------------

/// An item to be deserialized.
pub struct DeserializeItem<'facet, const BORROW: bool> {
    partial: Partial<'facet, BORROW>,
    desc: DeserializeDesc,
}

impl<'facet, const BORROW: bool> DeserializeItem<'facet, BORROW> {
    /// Create a new [`DeserializeItem`] for the given [`Partial`] and
    /// [`DeserializeDesc`].
    #[inline]
    #[must_use]
    pub const fn new(partial: Partial<'facet, BORROW>, desc: DeserializeDesc) -> Self {
        Self { partial, desc }
    }

    /// Get the inner [`Partial`] of the [`DeserializeItem`].
    #[inline]
    #[must_use]
    pub const fn partial(&self) -> &Partial<'facet, BORROW> { &self.partial }

    /// Returns `true` if this [`DeserializeItem`] is variable-length.
    #[inline]
    #[must_use]
    pub const fn is_variable(&self) -> bool { self.desc.variable }

    /// Set whether this [`DeserializeItem`] is variable-length.
    #[inline]
    pub const fn set_variable(&mut self, variable: bool) { self.desc.variable = variable; }

    /// Get the [`Shape`] of the [`DeserializeItem`].
    #[inline]
    #[must_use]
    pub fn shape(&self) -> &'static Shape { self.partial.shape() }

    /// Get the [`Attr`]s of the [`DeserializeItem`]'s type.
    #[inline]
    #[must_use]
    pub fn shape_attr(&self) -> &'static [Attr] { self.partial.shape().attributes }

    /// Get the [`Field`] [`Attr`]s of the [`DeserializeItem`], if any.
    #[inline]
    #[must_use]
    pub const fn field_attr(&self) -> Option<&'static [Attr]> { self.desc.field_attr() }

    /// Set the [`Field`] of this [`DeserializeItem`].
    #[inline]
    #[must_use]
    pub const fn with_field(mut self, field: Option<&'static Field>) -> Self {
        if let Some(field) = field {
            self.desc.field = Some(field);
        }
        self
    }

    /// Returns `true` if this [`DeserializeItem`] is of the given type.
    #[inline]
    #[must_use]
    pub fn is_type<U: Facet<'facet>>(&self) -> bool { U::SHAPE == self.partial.shape() }

    /// Set the value of this [`DeserializeItem`].
    ///
    /// # Errors
    ///
    /// Returns an error if the value is of the incorrect type.
    #[inline]
    pub fn set<U: Facet<'facet>>(self, value: U) -> Result<Self, ReaderError> {
        match self.partial.set::<U>(value) {
            Ok(partial) => Ok(Self { partial, desc: self.desc }),
            Err(err) => Err(ReaderError::Reflect(err)),
        }
    }

    /// Perform a scoped operation on the inner [`Partial`] of this
    /// [`DeserializeItem`].
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[inline]
    pub fn scoped<Err>(
        self,
        scope: impl FnOnce(Partial<'facet, BORROW>) -> Result<Partial<'facet, BORROW>, Err>,
    ) -> Result<Self, Err> {
        Ok(Self { partial: scope(self.partial)?, desc: self.desc })
    }

    /// Perform a scoped operation on the inner [`Partial`] of this
    /// [`DeserializeItem`] without the possibility of failure.
    #[inline]
    #[must_use]
    pub fn scoped_infallible(
        self,
        scope: impl FnOnce(Partial<'facet, BORROW>) -> Partial<'facet, BORROW>,
    ) -> Self {
        Self { partial: scope(self.partial), desc: self.desc }
    }

    /// Break the [`DeserializeItem`] into its inner components.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> (Partial<'facet, BORROW>, DeserializeDesc) {
        (self.partial, self.desc)
    }
}

impl<const BORROW: bool> fmt::Debug for DeserializeItem<'_, BORROW> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeserializeItem")
            .field("ty", &self.partial.shape())
            .field("desc", &self.desc)
            .finish()
    }
}
