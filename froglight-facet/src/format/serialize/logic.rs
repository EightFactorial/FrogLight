use facet::{Def, HasFields, Peek, Type, UserType};
use smallvec::SmallVec;

use crate::format::{
    serialize::{
        SerializeError,
        iterator::{ItemType, IteratorStack, SerializeItem, SerializeIterator},
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
    Item(SerializeItem<'mem, 'facet>),
}

impl<'mem, 'facet> Serializer<'mem, 'facet, ()> {
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new(
        peek: Peek<'mem, 'facet>,
        variable: bool,
        core: &mut impl FnMut(Item<'_, '_>) -> Result<(), WriterError>,
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
    core: &mut impl FnMut(Item<'_, '_>) -> Result<(), WriterError>,
) -> impl FnMut(&mut IteratorStack<'mem, 'facet>) -> Result<(), SerializeError> {
    move |stack| {
        while let Some(item) = stack.pop() {
            match item.ty() {
                // Process the item into values.
                ItemType::Other => {
                    handle_unknown(core, item, stack)?;
                }
                // Pass the value to the `core` function.
                ItemType::Value => {
                    return core(Item::Item(item)).map_err(SerializeError::from);
                }
            }
        }
        Ok(())
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `core` type")]
#[expect(clippy::too_many_lines, reason = "Complex matching behavior")]
fn handle_unknown<'mem, 'facet>(
    core: &mut impl FnMut(Item<'_, '_>) -> Result<(), WriterError>,
    mut item: SerializeItem<'mem, 'facet>,
    stack: &mut IteratorStack<'mem, 'facet>,
) -> Result<(), SerializeError> {
    /// A tiny cache for keeping collected values on the stack.
    type Cache<T> = SmallVec<[T; 8]>;

    {
        // Set `var` and `with` using the field and type attributes.
        let mut var = item.is_variable();
        let mut with = false;

        if let Some(field) = item.field() {
            // #[facet(mc::variable)]
            var |= field.has_attr(Some("mc"), "variable");
            // #[facet(mc::with = ...)]
            with |= field.has_attr(Some("mc"), "with");
        }
        for attr in item.shape().attributes {
            if attr.ns.is_some_and(|ns| ns == "mc") {
                // #[facet(mc::variable)]
                var |= attr.key == "variable";
                // #[facet(mc::with = ...)]
                with |= attr.key == "with";
            }
        }

        // Update whether `item` is variable.
        item.set_variable(var);

        // If the type has a custom serializer, treat it as a value.
        if with {
            stack.push(item.with_ty(ItemType::Value));
            return Ok(());
        }
    }

    // If the type has a proxy, convert `peek` into the proxy type.
    if let Some(proxy) = item.shape().effective_proxy(Some("mc")) {
        // SAFETY: `data` and `ptr` are guaranteed to be the `from` and `to` types.
        let proxy_ptr = proxy.shape.allocate().unwrap();
        let proxy_ptr = unsafe { (proxy.convert_out)(item.peek().data(), proxy_ptr).unwrap() };

        // SAFETY: `ptr` and `shape` are guaranteed to be for the same type.
        let proxy_peek = unsafe { Peek::unchecked_new(proxy_ptr.as_const(), proxy.shape) };

        // Create a new serializer and serialize the proxy type.
        let mut ser = Serializer::new(proxy_peek, item.is_variable(), core);
        while let Some(result) = Iterator::next(&mut ser) {
            if let Err(err) = result {
                // !! MUST DROP AND DEALLOC BEFORE RETURNING !!

                // SAFETY: `ptr` is guaranteed a valid value of the proxy type.
                unsafe {
                    proxy.shape.call_drop_in_place(proxy_ptr).unwrap();
                }
                // SAFETY: `ptr` was allocated via `shape.allocate()`.
                unsafe {
                    proxy.shape.deallocate_mut(proxy_ptr).unwrap();
                }

                return Err(err);
            }
        }
        drop(ser);

        // SAFETY: `ptr` is guaranteed a valid value of the proxy type.
        unsafe {
            proxy.shape.call_drop_in_place(proxy_ptr).unwrap();
        }
        // SAFETY: `ptr` was allocated via `shape.allocate()`.
        unsafe {
            proxy.shape.deallocate_mut(proxy_ptr).unwrap();
        }

        return Ok(());
    }

    match item.shape().def {
        // Directly serialize primitives.
        Def::Scalar => stack.push(item.with_ty(ItemType::Value)),

        Def::Map(_) => {
            let map = item.peek().into_map()?;
            // Serialize the length of the map.
            core(Item::Size(map.len().try_into().map_err(WriterError::TryFromInt)?))?;

            // Push the items in reverse order.
            let iter = map.iter();
            for (key, value) in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                stack.push(SerializeItem::new(key, ItemType::Other, item.is_variable()));
            }
        }
        Def::Set(_) => {
            let set = item.peek().into_set()?;
            // Serialize the length of the set.
            core(Item::Size(set.len().try_into().map_err(WriterError::TryFromInt)?))?;

            // Push the items in reverse order.
            let iter = set.iter();
            for value in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            }
        }

        Def::List(_) | Def::Slice(_) => {
            let list = item.peek().into_list()?;
            // Serialize the length of the list.
            core(Item::Size(list.len().try_into().map_err(WriterError::TryFromInt)?))?;

            // Push the items in reverse order.
            let iter = list.iter();
            for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(list_item, ItemType::Other, item.is_variable()));
            }
        }
        Def::Array(_) => {
            let array = item.peek().into_list_like()?;

            // Push the items in reverse order.
            let iter = array.iter();
            for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(array_item, ItemType::Other, item.is_variable()));
            }
        }

        Def::NdArray(_) => {
            let array = item.peek().into_ndarray()?;

            // Push the items in reverse order.
            let iter = (0..array.count()).filter_map(|i| array.get(i));
            for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(array_item, ItemType::Other, item.is_variable()));
            }
        }

        Def::Option(_) => {
            let option = item.peek().into_option()?;
            // Serialize the discriminant of the option.
            core(Item::Size(u32::from(option.is_some())))?;

            // If the option is `Some`, push the value.
            if let Some(value) = option.value() {
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            }
        }
        Def::Result(_) => {
            let result = item.peek().into_result()?;
            // Serialize the discriminant of the result.
            core(Item::Size(u32::from(result.is_ok())))?;

            if let Some(value) = result.ok() {
                // Push `Ok(_)`.
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            } else if let Some(value) = result.err() {
                // Push `Err(_)`.
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            }
        }

        // Fall back to `Type` for undefined types.
        Def::Undefined => {
            match item.shape().ty {
                // Directly serialize primitives.
                Type::Primitive(_) => stack.push(item.with_ty(ItemType::Value)),

                Type::Sequence(_) => {
                    let list = item.peek().into_list_like()?;
                    // Serialize the length of the list.
                    core(Item::Size(list.len().try_into().map_err(WriterError::TryFromInt)?))?;

                    // Push the items in reverse order.
                    let iter = list.iter();
                    for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                        stack.push(SerializeItem::new(
                            list_item,
                            ItemType::Other,
                            item.is_variable(),
                        ));
                    }
                }

                Type::User(UserType::Struct(_)) => {
                    // Push the fields in reverse order.
                    let iter = item.peek().into_struct()?.fields_for_serialize();
                    for (field, field_item) in iter.collect::<Cache<_>>().into_iter().rev() {
                        let mut field_ty = ItemType::Other;
                        let mut variable = false;

                        if let Some(field) = field.field {
                            // Update `variable` using the field's attributes.
                            variable = field.has_attr(Some("mc"), "variable");

                            // If the field has a custom serializer, treat it as a value.
                            if field.has_attr(Some("mc"), "with") {
                                field_ty = ItemType::Value;
                            }
                        }

                        stack.push(
                            SerializeItem::new(field_item, field_ty, variable)
                                .with_field(field.field),
                        );
                    }
                }
                Type::User(UserType::Enum(_)) => {
                    let enum_ = item.peek().into_enum()?;

                    // Serialize the discriminant of the enum.
                    #[expect(clippy::cast_sign_loss, reason = "Expected behavior")]
                    core(Item::Size(
                        (enum_.discriminant() as u64)
                            .try_into()
                            .map_err(WriterError::TryFromInt)?,
                    ))?;

                    // Push the fields in reverse order.
                    let iter = enum_.fields_for_serialize();
                    for (field, field_item) in iter.collect::<Cache<_>>().into_iter().rev() {
                        let mut field_ty = ItemType::Other;
                        let mut variable = false;

                        if let Some(field) = field.field {
                            // Update `variable` using the field's attributes.
                            variable = field.has_attr(Some("mc"), "variable");

                            // If the field has a custom serializer, treat it as a value.
                            if field.has_attr(Some("mc"), "with") {
                                field_ty = ItemType::Value;
                            }
                        }

                        stack.push(
                            SerializeItem::new(field_item, field_ty, variable)
                                .with_field(field.field),
                        );
                    }
                }
                Type::User(_) => todo!(),

                Type::Pointer(_) => todo!(),

                Type::Undefined => {
                    todo!("Unsupported type `{}`: {:?}", item.shape().type_name(), item.peek())
                }
            }
        }

        _ => todo!("Unsupported type `{}`: {:?}", item.shape().type_name(), item.peek()),
    }

    Ok(())
}
