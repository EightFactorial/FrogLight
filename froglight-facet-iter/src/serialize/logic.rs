use core::marker::PhantomData;

use facet::{Def, HasFields, Peek, Type, UserType};
use smallvec::SmallVec;

use crate::{
    serialize::{
        SerializeError,
        item::{Item, ItemType, SerializeItem},
    },
    writer::WriterError,
};

/// TODO
pub struct Serializer<'mem, 'facet, 'core, C: 'core> {
    stack: SmallVec<[SerializeItem<'mem, 'facet>; 10]>,

    core: &'core mut C,
    namespace: Option<&'core str>,

    #[expect(clippy::type_complexity, reason = "Force invariance over 'facet")]
    _invariant: PhantomData<(&'mem (), fn(&'facet ()) -> &'facet ())>,
}

impl<'mem, 'facet, 'core, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>
    Serializer<'mem, 'facet, 'core, C>
{
    /// Create a new [`Serializer`] for the given type.
    #[inline]
    #[must_use]
    pub fn new(
        peek: Peek<'mem, 'facet>,
        variable: bool,
        core: &'core mut C,
        namespace: Option<&'core str>,
    ) -> Self {
        let mut stack = SmallVec::new_const();
        stack.push(SerializeItem::new(peek, ItemType::Other, variable));
        Serializer { stack, core, namespace, _invariant: PhantomData }
    }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool { self.stack.is_empty() }

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
        if self.is_finished() { None } else { Some(self.process()) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'mem, 'facet, C: FnMut(Item<'mem, 'facet>) -> Result<(), WriterError>>
    Serializer<'mem, 'facet, '_, C>
{
    /// Process one `step` of the serialization iterator.
    fn process(&mut self) -> Result<(), SerializeError> {
        while let Some(item) = self.stack.pop() {
            match item.ty() {
                // Process the item into values.
                ItemType::Other => self.handle_other(item)?,
                // Pass the value to the `core` function.
                ItemType::Value => {
                    return (self.core)(Item::Item(item)).map_err(SerializeError::from);
                }
            }
        }
        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_other(
        &mut self,
        mut item: SerializeItem<'mem, 'facet>,
    ) -> Result<(), SerializeError> {
        // TODO: Pass this into `Serializer` to allow more flexibility?
        if self.namespace.is_some_and(|nc| nc == "mc") {
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
                self.stack.push(item.with_ty(ItemType::Value));
                return Ok(());
            }
        }

        // Handle the item's proxy if it has one.
        if self.handle_proxy(&item)?.is_some() {
            return Ok(());
        }

        // Handle the item based on its definition.
        if let Def::Undefined = item.shape().def {
            self.handle_type(item)
        } else {
            self.handle_def(item)
        }
    }

    /// If the type has a proxy, serialize the proxy type instead.
    /// Can't be pushed to the stack since the proxy does not live long enough.
    ///
    /// Based on [`https://github.com/facet-rs/facet-format/blob/b583926d6837756faaf5b6931780de0b7b961230/facet-format/src/serializer.rs#L2167`]
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_proxy(
        &mut self,
        item: &SerializeItem<'mem, 'facet>,
    ) -> Result<Option<()>, SerializeError> {
        let Some(proxy) = item.shape().effective_proxy(self.namespace) else { return Ok(None) };

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
        let mut ser = Serializer::new(proxy_peek, item.is_variable(), self.core, self.namespace);
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

        Ok(Some(()))
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_def(&mut self, item: SerializeItem<'mem, 'facet>) -> Result<(), SerializeError> {
        /// A tiny cache for keeping collected values on the stack.
        type Cache<T> = SmallVec<[T; 8]>;

        match item.shape().def {
            // Directly serialize primitives.
            Def::Scalar => {
                self.stack.push(item.with_ty(ItemType::Value));
                Ok(())
            }

            Def::Map(..) => {
                let map = item.peek().into_map()?;
                // Serialize the length of the map.
                (self.core)(Item::Size(map.len().try_into().map_err(WriterError::other)?))?;

                // Push the items in reverse order.
                let iter = map.iter();
                for (key, value) in iter.collect::<Cache<_>>().into_iter().rev() {
                    self.stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                    self.stack.push(SerializeItem::new(key, ItemType::Other, item.is_variable()));
                }

                Ok(())
            }
            Def::Set(..) => {
                let set = item.peek().into_set()?;
                // Serialize the length of the set.
                (self.core)(Item::Size(set.len().try_into().map_err(WriterError::other)?))?;

                // Push the items in reverse order.
                let iter = set.iter();
                for value in iter.collect::<Cache<_>>().into_iter().rev() {
                    self.stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                }

                Ok(())
            }

            Def::List(..) | Def::Slice(..) => {
                let list = item.peek().into_list()?;
                // Serialize the length of the list.
                (self.core)(Item::Size(list.len().try_into().map_err(WriterError::other)?))?;

                // Push the items in reverse order.
                let iter = list.iter();
                for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                    self.stack.push(SerializeItem::new(
                        list_item,
                        ItemType::Other,
                        item.is_variable(),
                    ));
                }

                Ok(())
            }
            Def::Array(..) => {
                let array = item.peek().into_list_like()?;

                // Push the items in reverse order.
                let iter = array.iter();
                for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                    self.stack.push(SerializeItem::new(
                        array_item,
                        ItemType::Other,
                        item.is_variable(),
                    ));
                }

                Ok(())
            }

            Def::NdArray(..) => {
                let array = item.peek().into_ndarray()?;

                // Push the items in reverse order.
                let iter = (0..array.count()).filter_map(|i| array.get(i));
                for array_item in iter.collect::<Cache<_>>().into_iter().rev() {
                    self.stack.push(SerializeItem::new(
                        array_item,
                        ItemType::Other,
                        item.is_variable(),
                    ));
                }

                Ok(())
            }

            Def::Option(..) => {
                let option = item.peek().into_option()?;
                // Serialize the discriminant of the option.
                (self.core)(Item::Size(u32::from(option.is_some())))?;

                // If the option is `Some`, push the value.
                if let Some(value) = option.value() {
                    self.stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                }

                Ok(())
            }
            Def::Result(..) => {
                let result = item.peek().into_result()?;
                // Serialize the discriminant of the result.
                (self.core)(Item::Size(u32::from(result.is_ok())))?;

                if let Some(value) = result.ok() {
                    // Push `Ok(..)`.
                    self.stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                } else if let Some(value) = result.err() {
                    // Push `Err(..)`.
                    self.stack.push(SerializeItem::new(value, ItemType::Other, item.is_variable()));
                }

                Ok(())
            }

            // Fallback to `Type` for undefined types.
            Def::Undefined => self.handle_type(item),

            _ => todo!("Unsupported type `{}`: {:?}", item.shape(), item.peek()),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_type(&mut self, item: SerializeItem<'mem, 'facet>) -> Result<(), SerializeError> {
        /// A tiny cache for keeping collected values on the stack.
        type Cache<T> = SmallVec<[T; 8]>;

        match item.shape().ty {
            // Directly serialize primitives.
            Type::Primitive(..) => {
                self.stack.push(item.with_ty(ItemType::Value));
                Ok(())
            }

            Type::Sequence(..) => {
                let list = item.peek().into_list_like()?;
                // Serialize the length of the list.
                (self.core)(Item::Size(list.len().try_into().map_err(WriterError::other)?))?;

                // Push the items in reverse order.
                let iter = list.iter();
                for list_item in iter.collect::<Cache<_>>().into_iter().rev() {
                    self.stack.push(SerializeItem::new(
                        list_item,
                        ItemType::Other,
                        item.is_variable(),
                    ));
                }

                Ok(())
            }

            Type::User(UserType::Struct(..)) => {
                // Push the fields in reverse order.
                let iter = item.peek().into_struct()?.fields_for_binary_serialize();

                // Determine whether the struct should pass the variable flag to its fields.
                let variable_base = if item.shape().attributes.iter().any(|attr| {
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

                    self.stack.push(
                        SerializeItem::new(field_item, field_ty, variable).with_field(field.field),
                    );
                }

                Ok(())
            }
            Type::User(UserType::Enum(..)) => {
                let enum_ = item.peek().into_enum()?;

                // Determine whether the enum should pass the variable flag to its fields.
                let variable_base = if item.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    item.is_variable()
                } else {
                    false
                };

                // Serialize the discriminant of the enum.
                #[expect(clippy::cast_sign_loss, reason = "Expected behavior")]
                (self.core)(Item::Size(
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

                    self.stack.push(
                        SerializeItem::new(field_item, field_ty, variable).with_field(field.field),
                    );
                }

                Ok(())
            }
            Type::User(..) => todo!(),

            Type::Pointer(..) => todo!(),

            _ => todo!("Unsupported type `{}`: {:?}", item.shape(), item.peek()),
        }
    }
}
