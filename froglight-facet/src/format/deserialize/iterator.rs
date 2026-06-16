use core::fmt;

use facet::{Attr, Facet, Field, Partial, Shape};
use smallvec::SmallVec;

use crate::format::ReaderError;

/// TODO
pub(super) struct DeserializeIterator<'facet, const BORROW: bool> {
    partial: Partial<'facet, BORROW>,
    stack: IteratorStack,
}

/// A stack of deserialization frames.
pub type IteratorStack = SmallVec<[StackItem; 10]>;

#[derive(Debug)]
pub enum StackItem {
    Item(DeserializeDesc),
    Fields(usize, &'static [Field], bool),

    Seq(Option<usize>, bool),
    Map(Option<usize>, bool),
    Set(Option<usize>, bool),

    Other(DeserializeDesc),
}

// -------------------------------------------------------------------------------------------------

/// An item to be deserialized.
pub struct DeserializeItem<'facet, const BORROW: bool> {
    partial: Partial<'facet, BORROW>,
    desc: DeserializeDesc,
}

/// A description of a deserialization item.
#[derive(Debug, Clone)]
pub struct DeserializeDesc {
    variable: bool,
    field_attr: Option<&'static [Attr]>,
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

    /// Get the [`Attr`]s of the field this [`DeserializeItem`] came from, if
    /// any.
    #[inline]
    #[must_use]
    pub const fn field_attr(&self) -> Option<&'static [Attr]> { self.desc.field_attr }

    /// Set the [`Field`] [`Attr`]s of this [`DeserializeItem`].
    #[inline]
    #[must_use]
    pub const fn with_field(mut self, field: Option<Field>) -> Self {
        if let Some(field) = field {
            self.desc.field_attr = Some(field.attributes);
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

impl DeserializeDesc {
    /// Create a new [`DeserializeDesc`].
    #[inline]
    #[must_use]
    pub const fn new(variable: bool, field_attr: Option<&'static [Attr]>) -> Self {
        Self { variable, field_attr }
    }

    /// Returns `true` if this [`DeserializeDesc`] is variable-length.
    #[inline]
    #[must_use]
    pub const fn is_variable(&self) -> bool { self.variable }

    /// Set whether this [`DeserializeDesc`] is variable-length.
    #[inline]
    pub const fn set_variable(&mut self, variable: bool) { self.variable = variable; }

    /// Get the [`Attr`]s of the field this [`DeserializeDesc`] came from, if
    /// any.
    #[inline]
    #[must_use]
    pub const fn field_attr(&self) -> Option<&'static [Attr]> { self.field_attr }
}

impl<const BORROW: bool> fmt::Debug for DeserializeItem<'_, BORROW> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeserializeItem")
            .field("ty", &self.partial.shape().type_name())
            .field("desc", &self.desc)
            .finish()
    }
}

// -------------------------------------------------------------------------------------------------

impl<'facet, const BORROW: bool> DeserializeIterator<'facet, BORROW> {
    /// Create a new [`DeserializeIterator`] for the given [`Partial`].
    #[inline]
    #[must_use]
    pub(crate) fn new_partial(partial: Partial<'facet, BORROW>, variable: bool) -> Self {
        let mut stack = IteratorStack::new_const();
        stack.push(StackItem::Other(DeserializeDesc::new(variable, None)));
        Self { partial, stack }
    }

    /// Returns `true` if the deserialization process is finished.
    #[inline]
    #[must_use]
    pub(crate) fn is_finished(&self) -> bool { self.stack.is_empty() }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub(crate) fn next<Err>(
        mut self,
        f: impl FnOnce(
            Partial<'facet, BORROW>,
            &mut IteratorStack,
        ) -> Result<Partial<'facet, BORROW>, Err>,
    ) -> Result<Self, Err> {
        self.partial = f(self.partial, &mut self.stack)?;
        Ok(self)
    }

    /// Get the [`Partial`] from the iterator.
    #[inline]
    #[must_use]
    pub(crate) fn into_partial(self) -> Partial<'facet, BORROW> { self.partial }
}
