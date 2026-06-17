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
pub struct Serializer<'mem, 'facet, 'core, C: 'core> {
    iter: SerializeIterator<'mem, 'facet>,
    core: &'core mut C,
}

/// A [`Serializer`] item.
pub enum Item<'mem, 'facet> {
    /// A size to be serialized.
    Size(u32),
    /// An item to be serialized.
    Item(SerializeItem<'mem, 'facet>),
}

impl<'mem, 'facet, 'core, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>
    Serializer<'mem, 'facet, 'core, C>
{
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new(peek: Peek<'mem, 'facet>, variable: bool, core: &'core mut C) -> Self {
        Serializer { iter: SerializeIterator::new(peek, variable), core }
    }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool { self.iter.is_empty() }

    /// Complete the [`Serializer`] by fully serializing the value.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn complete(mut self) -> Result<(), SerializeError> {
        // Drive the iterator to completion.
        while let Some(result) = Iterator::next(&mut self) {
            result?;
        }

        Ok(())
    }
}

impl<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>> Iterator
    for Serializer<'mem, 'facet, '_, C>
{
    type Item = Result<(), SerializeError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_empty() { None } else { Some(self.process()) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>
    Serializer<'mem, 'facet, '_, C>
{
    /// Process one `step` of the serialization iterator.
    fn process(&mut self) -> Result<(), SerializeError> {
        while let Some(item) = self.iter.stack.pop() {
            match item.ty() {
                // Process the item into values.
                ItemType::Other => handle_other(self.core, item, &mut self.iter.stack)?,
                // Pass the value to the `core` function.
                ItemType::Value => {
                    return (self.core)(Item::Item(item)).map_err(SerializeError::from);
                }
            }
        }
        Ok(())
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `C`")]
fn handle_other<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>(
    core: &mut C,
    mut item: SerializeItem<'mem, 'facet>,
    stack: &mut IteratorStack<'mem, 'facet>,
) -> Result<(), SerializeError> {
    {
        // Set `var` and `with` using the field and type attributes.
        let mut var = item.is_variable();
        let mut with = false;

        if let Some(attrs) = item.field_attr() {
            for attr in attrs.iter().filter(|attr| attr.ns.is_some_and(|ns| ns == "mc")) {
                // #[facet(mc::variable)]
                var |= attr.key == "variable";
                // #[facet(mc::with = ...)]
                with |= attr.key == "with";
            }
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

    // If the type has a proxy, serialize the proxy type instead.
    // Can't be pushed to the stack since the proxy does not live long enough.
    //
    // Based on [`https://github.com/facet-rs/facet-format/blob/b583926d6837756faaf5b6931780de0b7b961230/facet-format/src/serializer.rs#L2167`]
    if let Some(proxy) = item.shape().effective_proxy(Some("mc")) {
        let proxy_shape = proxy.shape;
        let proxy_layout = proxy_shape.layout.sized_layout().map_err(|_| SerializeError)?;

        // SAFETY: `data` and `uninit` are guaranteed to be the `from` and `to` types.
        let proxy_uninit = facet::alloc_for_layout(proxy_layout);
        let convert_result = unsafe { (proxy.convert_out)(item.peek().data(), proxy_uninit) };

        let proxy_ptr = match convert_result {
            Ok(ptr) => ptr,
            Err(_err) => {
                // !! MUST DEALLOC BEFORE RETURNING !!

                // SAFETY: `proxy_uninit` was allocated via `alloc_for_layout`.
                unsafe { facet::dealloc_for_layout(proxy_uninit.assume_init(), proxy_layout) };

                return Err(SerializeError);
            }
        };

        // SAFETY: `ptr` and `shape` are guaranteed to be for the same type.
        let proxy_peek = unsafe { Peek::unchecked_new(proxy_ptr.as_const(), proxy.shape) };

        // Create a new serializer and serialize the proxy value.
        let mut ser = Serializer::new(proxy_peek, item.is_variable(), core);
        while let Some(result) = Iterator::next(&mut ser) {
            if let Err(err) = result {
                // !! MUST DROP AND DEALLOC BEFORE RETURNING !!

                // SAFETY: `ptr` is guaranteed a valid value of the proxy type.
                // SAFETY: `ptr` was allocated via `alloc_for_layout`.
                unsafe {
                    drop(ser);
                    let _ = proxy.shape.call_drop_in_place(proxy_ptr);
                    facet::dealloc_for_layout(proxy_ptr, proxy_layout);
                }

                return Err(err);
            }
        }

        // !! MUST DROP AND DEALLOC BEFORE RETURNING !!

        // SAFETY: `ptr` is guaranteed a valid value of the proxy type.
        // SAFETY: `ptr` was allocated via `alloc_for_layout`.
        unsafe {
            drop(ser);
            let _ = proxy.shape.call_drop_in_place(proxy_ptr);
            facet::dealloc_for_layout(proxy_ptr, proxy_layout);
        }

        return Ok(());
    }

    // Handle the item based on its definition.
    match item.shape().def {
        Def::Undefined => handle_type(core, item, stack),
        _ => handle_def(core, item, stack),
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `C`")]
fn handle_def<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>(
    core: &mut C,
    item: SerializeItem<'mem, 'facet>,
    stack: &mut IteratorStack<'mem, 'facet>,
) -> Result<(), SerializeError> {
    /// A tiny cache for keeping collected values on the stack.
    type Cache<T> = SmallVec<[T; 8]>;

    match item.shape().def {
        // Directly serialize primitives.
        Def::Scalar => {
            stack.push(item.with_ty(ItemType::Value));
            Ok(())
        }

        Def::Map(..) => {
            let map = item.peek().into_map()?;
            // Serialize the length of the map.
            core(Item::Size(map.len().try_into().map_err(WriterError::other)?))?;

            // Push the items in reverse order.
            let iter = map.iter();
            for (key, value) in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                stack.push(SerializeItem::new(key, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }
        Def::Set(..) => {
            let set = item.peek().into_set()?;
            // Serialize the length of the set.
            core(Item::Size(set.len().try_into().map_err(WriterError::other)?))?;

            // Push the items in reverse order.
            let iter = set.iter();
            for value in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }

        Def::List(..) | Def::Slice(..) => {
            let list = item.peek().into_list()?;
            // Serialize the length of the list.
            core(Item::Size(list.len().try_into().map_err(WriterError::other)?))?;

            // Push the items in reverse order.
            let iter = list.iter();
            for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(list_item, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }
        Def::Array(..) => {
            let array = item.peek().into_list_like()?;

            // Push the items in reverse order.
            let iter = array.iter();
            for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(array_item, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }

        Def::NdArray(..) => {
            let array = item.peek().into_ndarray()?;

            // Push the items in reverse order.
            let iter = (0..array.count()).filter_map(|i| array.get(i));
            for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(array_item, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }

        Def::Option(..) => {
            let option = item.peek().into_option()?;
            // Serialize the discriminant of the option.
            core(Item::Size(u32::from(option.is_some())))?;

            // If the option is `Some`, push the value.
            if let Some(value) = option.value() {
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }
        Def::Result(..) => {
            let result = item.peek().into_result()?;
            // Serialize the discriminant of the result.
            core(Item::Size(u32::from(result.is_ok())))?;

            if let Some(value) = result.ok() {
                // Push `Ok(..)`.
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            } else if let Some(value) = result.err() {
                // Push `Err(..)`.
                stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }

        // Fallback to `Type` for undefined types.
        Def::Undefined => handle_type(core, item, stack),

        _ => todo!("Unsupported type `{}`: {:?}", item.shape().type_name(), item.peek()),
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `C`")]
fn handle_type<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>(
    core: &mut C,
    item: SerializeItem<'mem, 'facet>,
    stack: &mut IteratorStack<'mem, 'facet>,
) -> Result<(), SerializeError> {
    /// A tiny cache for keeping collected values on the stack.
    type Cache<T> = SmallVec<[T; 8]>;

    match item.shape().ty {
        // Directly serialize primitives.
        Type::Primitive(..) => {
            stack.push(item.with_ty(ItemType::Value));
            Ok(())
        }

        Type::Sequence(..) => {
            let list = item.peek().into_list_like()?;
            // Serialize the length of the list.
            core(Item::Size(list.len().try_into().map_err(WriterError::other)?))?;

            // Push the items in reverse order.
            let iter = list.iter();
            for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                stack.push(SerializeItem::new(list_item, ItemType::Other, item.is_variable()));
            }

            Ok(())
        }

        Type::User(UserType::Struct(..)) => {
            // Push the fields in reverse order.
            let iter = item.peek().into_struct()?.fields_for_binary_serialize();

            // Determine whether the struct should pass the variable flag to its fields.
            let variable_base =
                if item.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    item.is_variable()
                } else {
                    false
                };

            for (field, field_item) in iter.collect::<Cache<_>>().into_iter().rev() {
                let mut field_ty = ItemType::Other;
                let mut variable = variable_base;

                if let Some(field) = field.field {
                    // Update `variable` using the field's attributes.
                    variable |= field.has_attr(Some("mc"), "variable");

                    // If the field has a custom serializer, treat it as a value.
                    if field.has_attr(Some("mc"), "with") {
                        field_ty = ItemType::Value;
                    }
                }

                stack.push(
                    SerializeItem::new(field_item, field_ty, variable).with_field(field.field),
                );
            }

            Ok(())
        }
        Type::User(UserType::Enum(..)) => {
            let enum_ = item.peek().into_enum()?;

            // Determine whether the enum should pass the variable flag to its fields.
            let variable_base =
                if item.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    item.is_variable()
                } else {
                    false
                };

            // Serialize the discriminant of the enum.
            #[expect(clippy::cast_sign_loss, reason = "Expected behavior")]
            core(Item::Size(
                (enum_.discriminant() as u64).try_into().map_err(WriterError::other)?,
            ))?;

            // Push the fields in reverse order.
            let iter = enum_.fields_for_binary_serialize();
            for (field, field_item) in iter.collect::<Cache<_>>().into_iter().rev() {
                let mut field_ty = ItemType::Other;
                let mut variable = variable_base;

                if let Some(field) = field.field {
                    // Update `variable` using the field's attributes.
                    variable |= field.has_attr(Some("mc"), "variable");

                    // If the field has a custom serializer, treat it as a value.
                    if field.has_attr(Some("mc"), "with") {
                        field_ty = ItemType::Value;
                    }
                }

                stack.push(
                    SerializeItem::new(field_item, field_ty, variable).with_field(field.field),
                );
            }

            Ok(())
        }
        Type::User(..) => todo!(),

        Type::Pointer(..) => todo!(),

        Type::Undefined => {
            todo!("Unsupported type `{}`: {:?}", item.shape().type_name(), item.peek())
        }
    }
}
