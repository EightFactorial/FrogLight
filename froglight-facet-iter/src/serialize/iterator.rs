use core::marker::PhantomData;

use facet::{Attr, Field, Peek, Shape};
use smallvec::SmallVec;

/// TODO
pub(crate) struct SerializeIterator<'mem, 'facet> {
    pub(super) stack: IteratorStack<'mem, 'facet>,
    #[expect(clippy::type_complexity, reason = "Force invariance over 'facet")]
    _invariant: PhantomData<(&'mem (), fn(&'facet ()) -> &'facet ())>,
}

/// A stack of serialization frames.
pub type IteratorStack<'mem, 'facet> = SmallVec<[SerializeItem<'mem, 'facet>; 10]>;

/// An item to be serialized.
#[derive(Debug)]
pub struct SerializeItem<'mem, 'facet> {
    peek: Peek<'mem, 'facet>,
    ty: ItemType,
    variable: bool,
    field_attr: Option<&'static [Attr]>,
}

/// An item on the serializer stack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    /// A value to serialize.
    Value,
    /// A value to process.
    Other,
}

impl<'mem, 'facet> SerializeItem<'mem, 'facet> {
    /// Create a new [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn new(peek: Peek<'mem, 'facet>, ty: ItemType, var: bool) -> Self {
        Self { peek, ty, variable: var, field_attr: None }
    }

    /// Get the [`Peek`] for this [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn peek(&self) -> &Peek<'mem, 'facet> { &self.peek }

    /// Get the [`Shape`] for this [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn shape(&self) -> &'static Shape { self.peek.shape() }

    /// Get the [`ItemType`] for this [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn ty(&self) -> ItemType { self.ty }

    /// Set the [`ItemType`] for this [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn with_ty(mut self, ty: ItemType) -> Self {
        self.ty = ty;
        self
    }

    /// Returns `true` if this [`StackItem`] is variable-length.
    #[inline]
    #[must_use]
    pub const fn is_variable(&self) -> bool { self.variable }

    /// Set whether this [`StackItem`] is variable-length.
    #[inline]
    pub const fn set_variable(&mut self, variable: bool) { self.variable = variable; }

    /// Get the [`Attr`]s of the field this [`StackItem`] came from, if any.
    #[inline]
    #[must_use]
    pub const fn field_attr(&self) -> Option<&'static [Attr]> { self.field_attr }

    /// Get the [`Attr`]s of the [`StackItem`]'s type.
    #[inline]
    #[must_use]
    pub const fn shape_attr(&self) -> &'static [Attr] { self.peek.shape().attributes }

    /// Set the [`Field`] [`Attr`]s of this [`StackItem`].
    #[inline]
    #[must_use]
    pub const fn with_field(mut self, field: Option<Field>) -> Self {
        if let Some(field) = field {
            self.field_attr = Some(field.attributes);
        }
        self
    }
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet> SerializeIterator<'mem, 'facet> {
    /// Create a new [`SerializeIterator`] from the given [`Peek`].
    #[must_use]
    pub(crate) fn new(peek: Peek<'mem, 'facet>, variable: bool) -> Self {
        let mut stack = IteratorStack::new_const();
        stack.push(SerializeItem::new(peek, ItemType::Other, variable));
        Self { stack, _invariant: PhantomData }
    }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub(crate) fn is_empty(&self) -> bool { self.stack.is_empty() }
}
