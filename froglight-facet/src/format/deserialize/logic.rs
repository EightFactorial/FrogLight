//! TODO

use facet::{Def, Partial, Type, UserType};

use crate::format::{
    ReaderError,
    deserialize::{
        DeserializeError,
        iterator::{
            DeserializeDesc, DeserializeItem, DeserializeIterator, IteratorStack, StackItem,
        },
    },
};

/// TODO
pub struct Deserializer<'facet, 'core, const BORROW: bool, C: 'core> {
    start: usize,
    iter: Result<DeserializeIterator<'facet, BORROW>, DeserializeError>,
    core: &'core mut C,
}

/// A [`Deserializer`] item.
#[derive(Debug)]
pub enum Item<'facet, const BORROW: bool> {
    /// A size to be deserialized.
    Size(u32),
    /// An item to be deserialized.
    Item(DeserializeItem<'facet, BORROW>),
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
    pub(crate) fn new(
        partial: Partial<'facet, BORROW>,
        variable: bool,
        core: &'core mut C,
    ) -> Self {
        Deserializer {
            start: partial.frame_count(),
            iter: Ok(DeserializeIterator::new_partial(partial, variable)),
            core,
        }
    }

    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub fn is_finished(&self) -> bool {
        match &self.iter {
            Ok(iter) => iter.is_finished(),
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
    #[inline]
    pub fn complete(mut self) -> Result<Partial<'facet, BORROW>, DeserializeError> {
        // Drive the iterator to completion.
        while let Some(result) = Iterator::next(&mut self) {
            result?;
        }

        // Make sure the `Partial` is at the correct frame.
        let mut partial = self.iter.map(DeserializeIterator::into_partial)?;
        while partial.frame_count() > self.start {
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
            |iter| match iter.and_then(|iter| Self::process(self.core, iter)) {
                Ok(iter) => (Some(Ok(())), Ok(iter)),
                Err(err) => (Some(Err(err.clone())), Err(err)),
            },
        )
    }
}

// -------------------------------------------------------------------------------------------------

impl<
    'facet,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
> Deserializer<'facet, '_, BORROW, C>
{
    /// Process one `step` of the deserialization iterator.
    fn process(
        core: &mut C,
        mut iter: DeserializeIterator<'facet, BORROW>,
    ) -> Result<DeserializeIterator<'facet, BORROW>, DeserializeError> {
        while let Some(item) = iter.stack.pop() {
            match item {
                StackItem::Item(desc) => {
                    let item = Item::Item(DeserializeItem::new(iter.partial, desc));
                    let Item::Item(item) = (core)(item)? else { todo!() };
                    iter.partial = item.into_inner().0;

                    if iter.partial.frame_count() > 1 {
                        iter.partial = iter.partial.end()?;
                    }

                    return Ok(iter);
                }

                StackItem::Fields(len, fields, variable_base) => {
                    let Some((field, fields)) = fields.split_first() else {
                        if iter.partial.frame_count() > 1 {
                            iter.partial = iter.partial.end()?;
                        }
                        continue;
                    };

                    // Update `variable` using the field's attributes.
                    let variable = variable_base | field.has_attr(Some("mc"), "variable");

                    // Push the remaining fields to the stack.
                    iter.stack.push(StackItem::Fields(len, fields, variable_base));

                    // Push the current field to the stack.
                    let desc = DeserializeDesc::new(variable, Some(field.attributes));
                    iter.stack.push(StackItem::Other(desc));

                    // Begin the current field.
                    iter.partial = iter.partial.begin_nth_field(len - fields.len() - 1)?;
                }

                StackItem::Seq(..) => todo!(),

                StackItem::Map(..) => todo!(),
                StackItem::Set(..) => todo!(),

                StackItem::Other(desc) => {
                    iter.partial = handle_other(iter.partial, desc, core, &mut iter.stack)?;
                }
            }
        }

        Ok(iter)
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `C`")]
fn handle_other<
    'facet,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
>(
    mut partial: Partial<'facet, BORROW>,
    mut desc: DeserializeDesc,
    core: &mut C,
    stack: &mut IteratorStack,
) -> Result<Partial<'facet, BORROW>, DeserializeError> {
    {
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
        for attr in partial.shape().attributes {
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
            stack.push(StackItem::Item(desc));
            return Ok(partial);
        }
    }

    // If the type has a proxy, deserialize the proxy type instead.
    if partial.shape().effective_proxy(Some("mc")).is_some() {
        let (proxy_partial, has_proxy) =
            partial.begin_custom_deserialization_from_shape_with_format(Some("mc"))?;

        if has_proxy {
            // Deserialize the proxied type.
            return Deserializer::new(proxy_partial, desc.is_variable(), core).complete();
        }

        // Otherwise return the partial unchanged.
        partial = proxy_partial;
    }

    // Handle the partial based on its definition.
    match partial.shape().def {
        Def::Undefined => handle_type(partial, desc, core, stack),
        _ => handle_def(partial, desc, core, stack),
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `C`")]
fn handle_def<
    'facet,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
>(
    partial: Partial<'facet, BORROW>,
    desc: DeserializeDesc,
    core: &mut C,
    stack: &mut IteratorStack,
) -> Result<Partial<'facet, BORROW>, DeserializeError> {
    match partial.shape().def {
        // Directly deserialize primitives.
        Def::Scalar => {
            stack.push(StackItem::Item(desc));
            Ok(partial)
        }

        Def::Map(..) => todo!(),
        Def::Set(..) => todo!(),

        Def::List(..) | Def::Slice(..) => todo!(),
        Def::Array(..) => todo!(),

        Def::NdArray(..) => todo!(),

        Def::Option(..) => todo!(),
        Def::Result(..) => todo!(),

        // Fallback to `Type` for undefined types.
        Def::Undefined => handle_type(partial, desc, core, stack),

        _ => todo!("Unsupported type `{}`", partial.shape().type_name()),
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `C`")]
fn handle_type<
    'facet,
    const BORROW: bool,
    C: FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
>(
    partial: Partial<'facet, BORROW>,
    desc: DeserializeDesc,
    core: &mut C,
    stack: &mut IteratorStack,
) -> Result<Partial<'facet, BORROW>, DeserializeError> {
    match partial.shape().ty {
        // Directly deserialize primitives.
        Type::Primitive(..) => {
            stack.push(StackItem::Item(desc));
            Ok(partial)
        }

        Type::Sequence(..) => todo!(),

        Type::User(UserType::Struct(ty)) => {
            // Determine whether the struct should pass the variable flag to its fields.
            let variable_base =
                if partial.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    desc.is_variable()
                } else {
                    false
                };

            // Push the fields to the stack.
            stack.push(StackItem::Fields(ty.fields.len(), ty.fields, variable_base));

            Ok(partial)
        }
        Type::User(UserType::Enum(..)) => {
            // Determine whether the struct should pass the variable flag to its fields.
            let variable_base =
                if partial.shape().attributes.iter().any(|attr| {
                    attr.ns.is_some_and(|ns| ns == "mc") && attr.key == "variable_inner"
                }) {
                    desc.is_variable()
                } else {
                    false
                };

            // Deserialize the discriminant of the enum.
            let Item::Size(discriminant) = core(Item::Size(0))? else { todo!() };
            #[expect(clippy::cast_possible_wrap, reason = "Expected behavior")]
            let partial = partial.select_variant(i64::from(discriminant as i32))?;

            // Push the fields to the stack.
            let variant = partial.selected_variant().unwrap();
            stack.push(StackItem::Fields(
                variant.data.fields.len(),
                variant.data.fields,
                variable_base,
            ));

            Ok(partial)
        }
        Type::User(..) => todo!(),

        Type::Pointer(..) => todo!(),

        Type::Undefined => {
            todo!("Unsupported type `{}`", partial.shape().type_name())
        }
    }
}
