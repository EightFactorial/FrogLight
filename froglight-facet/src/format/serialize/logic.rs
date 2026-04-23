use facet::{HasFields, Peek, Type, UserType};
use smallvec::SmallVec;

use crate::format::{
    serialize::{
        SerializeError,
        iterator::{IteratorStack, SerializeIterator, StackItem},
    },
    writer::WriterError,
};

/// TODO
pub struct Serializer<'mem, 'facet, C> {
    iter: SerializeIterator<'mem, 'facet>,
    core: C,
}

/// A serializer item.
pub enum Item<'mem, 'facet> {
    /// A size to be serialized.
    Size(u32),
    /// A value to be serialized.
    Peek(Peek<'mem, 'facet>, bool),
}

impl<'mem, 'facet> Serializer<'mem, 'facet, ()> {
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new(
        peek: Peek<'mem, 'facet>,
        core: impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>,
    ) -> Serializer<'mem, 'facet, impl SerializerCore<'mem, 'facet>> {
        Serializer { iter: SerializeIterator::new(peek), core: create_core(core) }
    }
}

impl<'mem, 'facet, C: SerializerCore<'mem, 'facet>> Serializer<'mem, 'facet, C> {
    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool { self.iter.is_empty() }

    /// Returns the inner [`SerializeIterator`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> SerializeIterator<'mem, 'facet> { self.iter }
}

impl<'mem, 'facet, C: SerializerCore<'mem, 'facet>> Iterator for Serializer<'mem, 'facet, C> {
    type Item = Result<(), SerializeError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.iter.next(self.core.as_fn_once()) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for serializer cores.
pub trait SerializerCore<'mem, 'facet> {
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(&mut IteratorStack<'mem, 'facet>) -> Result<(), SerializeError> + '_;
}

impl<'mem, 'facet, F> SerializerCore<'mem, 'facet> for F
where
    F: FnMut(&mut IteratorStack<'mem, 'facet>) -> Result<(), SerializeError>,
{
    #[inline]
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(&mut IteratorStack<'mem, 'facet>) -> Result<(), SerializeError> + '_ {
        self
    }
}

// -------------------------------------------------------------------------------------------------

fn create_core<'mem, 'facet>(
    mut core: impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>,
) -> impl FnMut(&mut IteratorStack<'mem, 'facet>) -> Result<(), SerializeError> {
    move |stack| {
        loop {
            match stack.pop() {
                // Process the item into values.
                Some((peek, StackItem::Other(var))) => {
                    handle_unknown(&mut core, peek, var, stack)?;
                }
                // Return the `core` result.
                Some((peek, StackItem::Value(var))) => {
                    return core(Item::Peek(peek, var)).map_err(SerializeError::from);
                }
                // Return `Ok`.
                None => {
                    return Ok(());
                }
            }
        }
    }
}

#[inline(always)]
#[expect(clippy::inline_always, reason = "Used once per `core` type")]
fn handle_unknown<'mem, 'facet>(
    core: &mut impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>,
    mut peek: Peek<'mem, 'facet>,
    mut var: bool,
    stack: &mut IteratorStack<'mem, 'facet>,
) -> Result<(), SerializeError> {
    let mut with = false;
    for attr in peek.shape().attributes {
        if attr.ns.is_some_and(|ns| ns == "mc") {
            // #[facet(mc::variable)]
            var |= attr.key == "variable";
            // #[facet(mc::with = ...)]
            with |= attr.key == "with";
        }
    }

    if let Some(proxy) = peek.shape().effective_proxy(Some("mc")) {
        let proxy_ptr =
            unsafe { (proxy.convert_out)(peek.data(), proxy.shape.allocate().unwrap()).unwrap() };
        let proxy = unsafe { Peek::unchecked_new(proxy_ptr.as_const(), proxy.shape) };
        peek = proxy;
    }

    if with {
        stack.push((peek, StackItem::Value(var)));
        return Ok(());
    }

    match peek.shape().ty {
        Type::Primitive(_ty) => stack.push((peek, StackItem::Value(var))),

        Type::Sequence(_ty) => {
            let list = peek.into_list_like()?;
            core(Item::Size(list.len().try_into().map_err(|_err| SerializeError)?))?;

            let iter = list.iter();
            for item in iter.collect::<SmallVec<[_; 8]>>().into_iter().rev() {
                stack.push((item, StackItem::Other(var)));
            }
        }

        Type::User(UserType::Struct(_)) => {
            let iter = peek.into_struct()?.fields_for_serialize();
            for (field, item) in iter.collect::<SmallVec<[_; 8]>>().into_iter().rev() {
                if let Some(field) = field.field {
                    var = field.has_attr(Some("mc"), "variable");
                }

                if let Some(field) = field.field
                    && field.has_attr(Some("mc"), "with")
                {
                    stack.push((item, StackItem::Value(var)));
                } else {
                    stack.push((item, StackItem::Other(var)));
                }
            }
        }
        Type::User(UserType::Enum(_)) => {
            let enum_ = peek.into_enum()?;

            #[expect(clippy::cast_sign_loss, reason = "Expected behavior")]
            let disc = (enum_.discriminant() as u64).try_into().map_err(|_err| SerializeError)?;
            core(Item::Size(disc))?;

            let iter = enum_.fields_for_serialize();
            for (field, item) in iter.collect::<SmallVec<[_; 8]>>().into_iter().rev() {
                if let Some(field) = field.field {
                    var = field.has_attr(Some("mc"), "variable");
                }

                if let Some(field) = field.field
                    && field.has_attr(Some("mc"), "with")
                {
                    stack.push((item, StackItem::Value(var)));
                } else {
                    stack.push((item, StackItem::Other(var)));
                }
            }
        }
        Type::User(_) => todo!(),

        Type::Pointer(_ty) => todo!(),

        Type::Undefined => todo!(),
    }

    Ok(())
}
