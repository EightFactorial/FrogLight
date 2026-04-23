use facet::{Def, HasFields, Peek, Type, UserType};
use smallvec::SmallVec;

use crate::format::{
    serialize::{
        SerializeError,
        iterator::{ItemType, IteratorStack, SerializeIterator, StackItem},
    },
    writer::WriterError,
};

/// TODO
pub struct Serializer<'mem, 'facet, C> {
    iter: SerializeIterator<'mem, 'facet>,
    core: C,
}

/// A serializer item.
#[expect(clippy::large_enum_variant, reason = "Yes")]
pub enum Item<'mem, 'facet> {
    /// A size to be serialized.
    Size(u32),
    /// An item to be serialized.
    Item(StackItem<'mem, 'facet>),
}

impl<'mem, 'facet> Serializer<'mem, 'facet, ()> {
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new(
        peek: Peek<'mem, 'facet>,
        variable: bool,
        core: impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>,
    ) -> Serializer<'mem, 'facet, impl SerializerCore<'mem, 'facet>> {
        Serializer { iter: SerializeIterator::new(peek, variable), core: create_core(core) }
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
                Some(item @ StackItem { ty: ItemType::Value, .. }) => {
                    handle_unknown(&mut core, item, stack)?;
                }
                // Return the `core` result.
                Some(item @ StackItem { ty: ItemType::Other, .. }) => {
                    return core(Item::Item(item)).map_err(SerializeError::from);
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
#[allow(clippy::inline_always, reason = "Used once per `core` type")]
#[expect(clippy::too_many_lines, reason = "Complex matching behavior")]
fn handle_unknown<'mem, 'facet>(
    core: &mut impl FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>,
    mut item: StackItem<'mem, 'facet>,
    stack: &mut IteratorStack<'mem, 'facet>,
) -> Result<(), SerializeError> {
    /// A tiny cache for keeping collected values on the stack.
    type Cache<T> = SmallVec<[T; 8]>;

    // Set `item.variable` and `with` using the field and type attributes.
    let mut with = false;

    if let Some(field) = item.field.as_ref() {
        // #[facet(mc::variable)]
        item.variable |= field.has_attr(Some("mc"), "variable");
        // #[facet(mc::with = ...)]
        with |= field.has_attr(Some("mc"), "with");
    }
    for attr in item.peek.shape().attributes {
        if attr.ns.is_some_and(|ns| ns == "mc") {
            // #[facet(mc::variable)]
            item.variable |= attr.key == "variable";
            // #[facet(mc::with = ...)]
            with |= attr.key == "with";
        }
    }

    // If the type has a custom serializer, treat it as a value.
    if with {
        item.ty = ItemType::Value;
        stack.push(item);
        return Ok(());
    }

    // If the type has a proxy, convert `peek` into the proxy type.
    if let Some(proxy) = item.peek.shape().effective_proxy(Some("mc")) {
        let ptr = proxy.shape.allocate().unwrap();
        // SAFETY: `data` and `ptr` are guaranteed to be the `from` and `to` types.
        let ptr = unsafe { (proxy.convert_out)(item.peek.data(), ptr).unwrap() };
        // SAFETY: `ptr` and `shape` are guaranteed to be for the same type.
        item.peek = unsafe { Peek::unchecked_new(ptr.as_const(), proxy.shape) };

        // Restart `handle_unknown` with the proxy type.
        return handle_unknown(core, item, stack);
    }

    match item.peek.shape().def {
        // Directly serialize primitives.
        Def::Scalar => {
            item.ty = ItemType::Value;
            stack.push(item);
        }

        Def::Map(_) => {
            let map = item.peek.into_map()?;
            // Serialize the length of the map.
            core(Item::Size(map.len().try_into().map_err(|_err| SerializeError)?))?;

            // Push the items in reverse order.
            let iter = map.iter();
            for (key, value) in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(StackItem::new(value, ItemType::Other, item.variable));
                stack.push(StackItem::new(key, ItemType::Other, item.variable));
            }
        }
        Def::Set(_) => {
            let set = item.peek.into_set()?;
            // Serialize the length of the set.
            core(Item::Size(set.len().try_into().map_err(|_err| SerializeError)?))?;

            // Push the items in reverse order.
            let iter = set.iter();
            for value in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(StackItem::new(value, ItemType::Other, item.variable));
            }
        }

        Def::List(_) | Def::Slice(_) => {
            let list = item.peek.into_list()?;
            // Serialize the length of the list.
            core(Item::Size(list.len().try_into().map_err(|_err| SerializeError)?))?;

            // Push the items in reverse order.
            let iter = list.iter();
            for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(StackItem::new(list_item, ItemType::Other, item.variable));
            }
        }
        Def::Array(_) => {
            let array = item.peek.into_list_like()?;

            // Push the items in reverse order.
            let iter = array.iter();
            for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(StackItem::new(array_item, ItemType::Other, item.variable));
            }
        }

        Def::NdArray(_) => {
            let array = item.peek.into_ndarray()?;

            // Push the items in reverse order.
            let iter = (0..array.count()).filter_map(|i| array.get(i));
            for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(StackItem::new(array_item, ItemType::Other, item.variable));
            }
        }

        Def::Option(_) => {
            let option = item.peek.into_option()?;
            // Serialize the discriminant of the option.
            core(Item::Size(u32::from(option.is_some())))?;

            // If the option is `Some`, push the value.
            if let Some(value) = option.value() {
                stack.push(StackItem::new(value, ItemType::Other, item.variable));
            }
        }
        Def::Result(_) => {
            let result = item.peek.into_result()?;
            // Serialize the discriminant of the result.
            core(Item::Size(u32::from(result.is_ok())))?;

            if let Some(value) = result.ok() {
                // Push `Ok(_)`.
                stack.push(StackItem::new(value, ItemType::Other, item.variable));
            } else if let Some(value) = result.err() {
                // Push `Err(_)`.
                stack.push(StackItem::new(value, ItemType::Other, item.variable));
            }
        }

        // Fall back to `Type` for undefined types.
        Def::Undefined => {
            match item.peek.shape().ty {
                // Directly serialize primitives.
                Type::Primitive(_) => {
                    item.ty = ItemType::Value;
                    stack.push(item);
                }

                Type::Sequence(_) => {
                    let list = item.peek.into_list_like()?;
                    // Serialize the length of the list.
                    core(Item::Size(list.len().try_into().map_err(|_err| SerializeError)?))?;

                    // Push the items in reverse order.
                    let iter = list.iter();
                    for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                        stack.push(StackItem::new(list_item, ItemType::Other, item.variable));
                    }
                }

                Type::User(UserType::Struct(_)) => {
                    // Push the fields in reverse order.
                    let iter = item.peek.into_struct()?.fields_for_serialize();
                    for (field, field_item) in iter.collect::<Cache<_>>().into_iter().rev() {
                        let mut field_ty = ItemType::Other;

                        if let Some(field) = field.field {
                            // Update `variable` using the field's attributes.
                            item.variable = field.has_attr(Some("mc"), "variable");

                            // If the field has a custom serializer, treat it as a value.
                            if field.has_attr(Some("mc"), "with") {
                                field_ty = ItemType::Value;
                            }
                        }

                        stack.push(
                            StackItem::new(field_item, field_ty, item.variable)
                                .with_field(field.field),
                        );
                    }
                }
                Type::User(UserType::Enum(_)) => {
                    let enum_ = item.peek.into_enum()?;

                    // Serialize the discriminant of the enum.
                    #[expect(clippy::cast_sign_loss, reason = "Expected behavior")]
                    let disc =
                        (enum_.discriminant() as u64).try_into().map_err(|_err| SerializeError)?;
                    core(Item::Size(disc))?;

                    // Push the fields in reverse order.
                    let iter = enum_.fields_for_serialize();
                    for (field, field_item) in iter.collect::<Cache<_>>().into_iter().rev() {
                        let mut field_ty = ItemType::Other;

                        if let Some(field) = field.field {
                            // Update `variable` using the field's attributes.
                            item.variable = field.has_attr(Some("mc"), "variable");

                            // If the field has a custom serializer, treat it as a value.
                            if field.has_attr(Some("mc"), "with") {
                                field_ty = ItemType::Value;
                            }
                        }

                        stack.push(
                            StackItem::new(field_item, field_ty, item.variable)
                                .with_field(field.field),
                        );
                    }
                }
                Type::User(_) => todo!(),

                Type::Pointer(_) => todo!(),

                Type::Undefined => {
                    todo!("Unsupported type `{}`: {:?}", item.peek.shape().type_name(), item.peek)
                }
            }
        }

        _ => todo!("Unsupported type `{}`: {:?}", item.peek.shape().type_name(), item.peek),
    }

    Ok(())
}
