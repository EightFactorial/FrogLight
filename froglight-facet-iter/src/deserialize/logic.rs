//! TODO

use facet::{Def, KnownPointer, Partial, SequenceType, Type, UserType};
use smallvec::SmallVec;

use crate::{
    ReaderError,
    deserialize::{
        DeserializeError,
        item::{DeserializeDesc, DeserializeItem, Item, StackItem},
    },
};

/// TODO
pub struct Deserializer<'facet, 'core, const BORROW: bool, C: 'core> {
    iter: Result<DeserializeIterator<'facet, BORROW>, DeserializeError>,
    core: &'core mut C,
}

/// TODO
struct DeserializeIterator<'facet, const BORROW: bool> {
    start: usize,
    namespace: Option<&'static str>,

    partial: Partial<'facet, BORROW>,
    stack: SmallVec<[StackItem; 10]>,
}

impl<
    'facet,
    'core,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> Deserializer<'facet, 'core, BORROW, C>
{
    /// Create a new [`Deserializer`] for the given type.
    #[inline]
    pub fn new(
        partial: Partial<'facet, BORROW>,
        variable: bool,
        core: &'core mut C,
        namespace: Option<&'static str>,
    ) -> Self {
        let mut stack = SmallVec::new_const();
        stack.push(StackItem::Other(DeserializeDesc::new(variable, None)));

        Deserializer {
            iter: Ok(DeserializeIterator {
                start: partial.frame_count(),
                namespace,
                partial,
                stack,
            }),
            core,
        }
    }

    /// Returns the starting frame count of this [`Deserializer`].
    ///
    /// Returns `0` if the deserializer returned an error.
    #[inline]
    #[must_use]
    pub const fn starting_frame(&self) -> usize {
        if let Ok(iter) = &self.iter { iter.start } else { 0 }
    }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool {
        match &self.iter {
            Ok(iter) => iter.stack.is_empty(),
            Err(_) => true,
        }
    }

    /// Complete the [`Deserializer`] by deserializing the value.
    ///
    /// Returns the initial [`Partial`] if successful.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    pub fn complete(mut self) -> Result<Partial<'facet, BORROW>, DeserializeError> {
        // Drive the iterator to completion.
        while let Some(result) = Iterator::next(&mut self) {
            result?;
        }

        // Make sure the `Partial` is at the correct frame.
        let DeserializeIterator { start, mut partial, .. } = self.iter?;
        while partial.frame_count() > start {
            partial = partial.end()?;
        }

        Ok(partial)
    }

    /// Complete the [`Deserializer`] by deserializing the value.
    ///
    /// Returns the initial [`Partial`] if successful,
    /// or an error if [`complete_mut`](Self::complete_mut) was already
    /// called.
    ///
    /// # Errors
    ///
    /// Returns an error if the deserialization fails.
    pub fn complete_mut(&mut self) -> Result<Partial<'facet, BORROW>, DeserializeError> {
        // Drive the iterator to completion.
        while let Some(result) = Iterator::next(self) {
            result?;
        }

        // Make sure the `Partial` is at the correct frame.
        let DeserializeIterator { start, mut partial, .. } =
            core::mem::replace(&mut self.iter, Err(DeserializeError))?;
        while partial.frame_count() > start {
            partial = partial.end()?;
        }

        Ok(partial)
    }
}

impl<
    'facet,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> Iterator for Deserializer<'facet, '_, BORROW, C>
{
    type Item = Result<(), DeserializeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }

        replace_with::replace_with_and_return(
            &mut self.iter,
            || Err(DeserializeError),
            |iter| match iter.and_then(|iter| iter.process(self.core)) {
                Ok(iter) => (Some(Ok(())), Ok(iter)),
                Err(err) => (Some(Err(err.clone())), Err(err)),
            },
        )
    }
}

// -------------------------------------------------------------------------------------------------

impl<'facet, const BORROW: bool> DeserializeIterator<'facet, BORROW> {
    /// Process one `step` of the deserialization iterator.
    fn process<C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>>(
        mut self,
        core: &mut C,
    ) -> Result<Self, DeserializeError> {
        while let Some(item) = self.stack.pop() {
            #[cfg(feature = "tracing_ext")]
            if matches!(item, StackItem::Item(..)) {
                tracing::debug!(target: "froglight_facet_iter::deserialize", "Deserializing `{}`", self.partial.shape());
            } else {
                tracing::trace!(target: "froglight_facet_iter::deserialize", "Processing `{}` ({})", self.partial.shape(), item.variant_name());
            }

            match item {
                StackItem::Item(desc) => {
                    let item = Item::Item(DeserializeItem::new(self.partial, desc));
                    let Item::Item(item) = core(item)? else { todo!() };
                    self.partial = item.into_inner().0;

                    if self.partial.frame_count() > self.start {
                        self.partial = self.partial.end()?;
                    }

                    return Ok(self);
                }

                StackItem::Fields(len, fields, variable_base) => {
                    let Some((field, fields)) = fields.split_first() else {
                        if self.partial.frame_count() > self.start {
                            self.partial = self.partial.end()?;
                        }
                        continue;
                    };

                    // Push the remaining fields to the stack.
                    self.stack.push(StackItem::Fields(len, fields, variable_base));

                    // Update `variable` using the field's attributes.
                    let variable = variable_base | field.has_attr(Some("mc"), "variable");

                    // Push the current field to the stack.
                    let desc = DeserializeDesc::new(variable, Some(field));
                    self.stack.push(StackItem::Other(desc));

                    // Begin the current field.
                    self.partial = self.partial.begin_nth_field(len - fields.len() - 1)?;
                }

                StackItem::Seq(len, _end_prev, variable) => {
                    while !matches!(self.partial.shape().def, Def::List(..) | Def::Slice(..)) {
                        self.partial = self.partial.end()?;
                    }

                    if len == 0 {
                        self.partial = self.partial.end()?;
                    } else {
                        // Get the next item in the sequence.

                        self.stack.push(StackItem::Seq(len - 1, true, variable));
                        self.stack.push(StackItem::Other(DeserializeDesc::new(variable, None)));
                        self.partial = self.partial.begin_list_item()?;
                    }
                }

                StackItem::Map(len, _end_prev, is_value, variable) => {
                    while !matches!(self.partial.shape().def, Def::Map(..)) {
                        self.partial = self.partial.end()?;
                    }

                    if len == 0 {
                        self.partial = self.partial.end()?;
                    } else if is_value {
                        // `true` means the item is a value

                        self.stack.push(StackItem::Map(len - 1, true, false, variable)); // Next item is a key
                        self.stack.push(StackItem::Other(DeserializeDesc::new(variable, None)));
                        self.partial = self.partial.begin_value()?;
                    } else {
                        // `false` means the item is a key

                        self.stack.push(StackItem::Map(len, true, true, variable)); // Next item is a value
                        self.stack.push(StackItem::Other(DeserializeDesc::new(variable, None)));
                        self.partial = self.partial.begin_key()?;
                    }
                }
                StackItem::Set(len, _end_prev, variable) => {
                    while !matches!(self.partial.shape().def, Def::Set(..)) {
                        self.partial = self.partial.end()?;
                    }

                    if len == 0 {
                        self.partial = self.partial.end()?;
                    } else {
                        // Get the next item in the set.

                        self.stack.push(StackItem::Set(len - 1, true, variable));
                        self.stack.push(StackItem::Other(DeserializeDesc::new(variable, None)));
                        self.partial = self.partial.begin_set_item()?;
                    }
                }

                StackItem::Other(desc) => {
                    self = self.handle_other(desc, core)?;
                }
            }
        }

        Ok(self)
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_other<C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>>(
        mut self,
        mut desc: DeserializeDesc,
        core: &mut C,
    ) -> Result<Self, DeserializeError> {
        // TODO: Pass this into `Deserializer` to allow more flexibility?
        if self.namespace.is_some_and(|ns| ns == "mc") {
            // Set `var` and `with` using the field and type attributes.
            let mut var = desc.is_variable();
            let mut with = false;

            if let Some(attrs) = desc.field_attr() {
                for attr in attrs.iter().filter(|attr| attr.ns.is_some_and(|ns| ns == "mc")) {
                    // #[facet(mc::variable)]
                    var |= attr.key == "variable";
                    // #[facet(mc::with = ...)]
                    with |= attr.key == "with";
                }
            }
            for attr in self.partial.shape().attributes {
                if attr.ns.is_some_and(|ns| ns == "mc") {
                    // #[facet(mc::variable)]
                    var |= attr.key == "variable";
                    // #[facet(mc::with = ...)]
                    with |= attr.key == "with";
                }
            }

            // Update whether `item` is variable.
            desc.set_variable(var);

            // If the type has a custom deserializer, treat it as a value.
            if with {
                #[cfg(feature = "tracing_ext")]
                tracing::trace!(target: "froglight_facet_iter::deserialize", "Using custom deserializer for `{}`", self.partial.shape());

                self.stack.push(StackItem::Item(desc));
                return Ok(self);
            }
        }

        // If the type has a proxy, deserialize the proxy type instead.
        if self.partial.shape().effective_proxy(Some("mc")).is_some() {
            let (proxy_partial, has_proxy) =
                self.partial.begin_custom_deserialization_from_shape_with_format(Some("mc"))?;

            if has_proxy {
                #[cfg(feature = "tracing_ext")]
                tracing::trace!(target: "froglight_facet_iter::deserialize", "Using `{}` as a proxy", proxy_partial.shape());

                // Deserialize the proxied type.
                self.partial =
                    Deserializer::new(proxy_partial, desc.is_variable(), core, self.namespace)
                        .complete()?;

                return Ok(self);
            }

            // Otherwise return the partial unchanged.
            self.partial = proxy_partial;
        }

        // Handle the partial based on its definition.
        match self.partial.shape().def {
            Def::Undefined => self.handle_type(desc, core),
            _ => self.handle_def(desc, core),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_def<C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>>(
        mut self,
        desc: DeserializeDesc,
        core: &mut C,
    ) -> Result<Self, DeserializeError> {
        match self.partial.shape().def {
            // Directly deserialize primitives.
            Def::Scalar => {
                self.stack.push(StackItem::Item(desc));
                Ok(self)
            }

            Def::Map(..) => {
                let Item::Size(len) = core(Item::Size(0))? else { todo!() };
                self.partial = self.partial.init_map()?;

                self.stack.push(StackItem::Map(len as usize, false, false, desc.is_variable()));
                Ok(self)
            }
            Def::Set(..) => {
                let Item::Size(len) = core(Item::Size(0))? else { todo!() };
                self.partial = self.partial.init_set()?;

                self.stack.push(StackItem::Set(len as usize, false, desc.is_variable()));
                Ok(self)
            }

            Def::List(..) | Def::Slice(..) => {
                // Specialize for `Vec<u8>`, `Vec<u32>`, and `Vec<u64>`.
                if self.partial.shape().is_type::<alloc::vec::Vec<u8>>()
                    || self.partial.shape().is_type::<alloc::vec::Vec<u32>>()
                    || self.partial.shape().is_type::<alloc::vec::Vec<u64>>()
                {
                    self.stack.push(StackItem::Item(desc));
                    return Ok(self);
                }

                let Item::Size(len) = core(Item::Size(0))? else { todo!() };
                self.partial = self.partial.init_list_with_capacity(len as usize)?;

                self.stack.push(StackItem::Seq(len as usize, false, desc.is_variable()));
                Ok(self)
            }
            Def::Array(def) => {
                self.partial = self.partial.init_array()?;

                self.stack.push(StackItem::Seq(def.n, false, desc.is_variable()));
                Ok(self)
            }

            Def::NdArray(..) => todo!(),

            Def::Option(..) => {
                let Item::Size(variant) = (core)(Item::Size(0))? else { todo!() };

                match variant {
                    0 => {
                        // Set `None`
                        self.partial = self.partial.set_default()?;
                        self.partial = self.partial.end()?;
                        Ok(self)
                    }
                    1 => {
                        // Begin `Some`
                        self.partial = self.partial.begin_some()?;
                        self.stack.push(StackItem::Other(desc));
                        Ok(self)
                    }
                    #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
                    inv => Err(ReaderError::InvalidBool(inv as u8))?,
                }
            }
            Def::Result(..) => {
                let Item::Size(variant) = (core)(Item::Size(0))? else { todo!() };

                match variant {
                    0 => {
                        // Begin `Ok`
                        self.partial = self.partial.begin_ok()?;
                        self.stack.push(StackItem::Other(desc));
                        Ok(self)
                    }
                    1 => {
                        // Begin `Err`
                        self.partial = self.partial.begin_err()?;
                        self.stack.push(StackItem::Other(desc));
                        Ok(self)
                    }
                    _ => todo!("Invalid ID `{variant}` for `Result`"),
                }
            }

            Def::Pointer(def) if def.constructible_from_pointee() => match def.known {
                Some(KnownPointer::Cow) => {
                    self.stack.push(StackItem::Item(desc));
                    Ok(self)
                }
                _ => todo!("\n\nTODO: PointerType {} -> {def:?}\n\n", self.partial.shape()),
            },

            // Fallback to `Type` for other/undefined types.
            _ => self.handle_type(desc, core),
        }
    }

    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Used once per `C`")]
    fn handle_type<C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>>(
        mut self,
        desc: DeserializeDesc,
        core: &mut C,
    ) -> Result<Self, DeserializeError> {
        match self.partial.shape().ty {
            // Directly deserialize primitives.
            Type::Primitive(..) => {
                self.stack.push(StackItem::Item(desc));
                Ok(self)
            }

            // Type::Sequence(SequenceType::Slice(..)) => {}
            Type::Sequence(SequenceType::Array(ty)) => {
                self.stack.push(StackItem::Seq(ty.n, false, desc.is_variable()));
                Ok(self)
            }

            Type::User(UserType::Struct(ty)) => {
                // Determine whether the struct should pass the variable flag to its fields.
                let variable_base = if self.partial.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    desc.is_variable()
                } else {
                    false
                };

                // Push the fields to the stack.
                self.stack.push(StackItem::Fields(ty.fields.len(), ty.fields, variable_base));

                Ok(self)
            }
            Type::User(UserType::Enum(..)) => {
                // Determine whether the struct should pass the variable flag to its fields.
                let variable_base = if self.partial.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    desc.is_variable()
                } else {
                    false
                };

                #[expect(clippy::cast_possible_wrap, reason = "Expected behavior")]
                {
                    // Deserialize the discriminant of the enum.
                    let Item::Size(discriminant) = core(Item::Size(0))? else { todo!() };
                    self.partial = self.partial.select_variant(i64::from(discriminant as i32))?;
                }

                let variant = self.partial.selected_variant().unwrap();

                #[cfg(feature = "tracing_ext")]
                tracing::debug!(target: "froglight_facet_iter::deserialize", "Deserializing `{}::{}`", self.partial.shape(), variant.effective_name());

                // Push the fields to the stack.
                self.stack.push(StackItem::Fields(
                    variant.data.fields.len(),
                    variant.data.fields,
                    variable_base,
                ));

                Ok(self)
            }
            Type::User(..) => todo!(),

            Type::Pointer(..) => todo!(),

            _ => todo!("Unsupported type `{}`", self.partial.shape()),
        }
    }
}
