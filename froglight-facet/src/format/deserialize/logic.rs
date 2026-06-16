//! TODO

use facet::{Def, Partial, Type, UserType};

use super::iterator::StackItem;
use crate::format::{
    ReaderError,
    deserialize::{
        DeserializeError,
        iterator::{DeserializeDesc, DeserializeItem, DeserializeIterator, IteratorStack},
    },
};

/// TODO
pub struct Deserializer<'facet, const BORROW: bool, C> {
    start: usize,
    iter: Result<DeserializeIterator<'facet, BORROW>, DeserializeError>,
    core: C,
}

/// A deserializer item.
#[derive(Debug)]
pub enum Item<'facet, const BORROW: bool> {
    /// A size to be deserialized.
    Size(u32),
    /// An item to be deserialized.
    Item(DeserializeItem<'facet, BORROW>),
}

impl<'facet, const BORROW: bool> Deserializer<'facet, BORROW, ()> {
    /// Create a new [`Deserializer`] for the given type.
    #[inline]
    pub(crate) fn new(
        partial: Partial<'facet, BORROW>,
        variable: bool,
        core: &mut impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
    ) -> Deserializer<'facet, BORROW, impl DeserializerCore<'facet, BORROW>> {
        Deserializer {
            start: partial.frame_count(),
            iter: Ok(DeserializeIterator::new_partial(partial, variable)),
            core: create_core(core),
        }
    }
}

impl<'facet, const BORROW: bool, C: DeserializerCore<'facet, BORROW>>
    Deserializer<'facet, BORROW, C>
{
    /// Returns `true` if the iterator is finished.
    #[inline]
    #[must_use]
    pub(crate) fn is_finished(&self) -> bool {
        match &self.iter {
            Ok(iter) => iter.is_finished(),
            Err(_) => true,
        }
    }

    /// Build the final value from the deserialized data.
    ///
    /// # Errors
    ///
    /// Returns an error if some data was not initialized,
    /// or the output type does not match the input type.
    #[inline]
    pub(crate) fn into_partial(self) -> Result<Partial<'facet, BORROW>, DeserializeError> {
        let mut partial = self.iter.map(DeserializeIterator::into_partial)?;
        while partial.frame_count() > self.start {
            partial = partial.end()?;
        }
        Ok(partial)
    }
}

impl<'facet, const BORROW: bool, C: DeserializerCore<'facet, BORROW>> Iterator
    for Deserializer<'facet, BORROW, C>
{
    type Item = Result<(), DeserializeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }

        replace_with::replace_with_and_return(
            &mut self.iter,
            || Err(DeserializeError),
            |iter| match iter.and_then(|iter| iter.next(self.core.as_fn_once())) {
                Ok(iter) => (Some(Ok(())), Ok(iter)),
                Err(err) => (Some(Err(err.clone())), Err(err)),
            },
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for deserializer cores.
pub trait DeserializerCore<'facet, const BORROW: bool> {
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, DeserializeError>
    + '_;
}

impl<'facet, const BORROW: bool, T> DeserializerCore<'facet, BORROW> for T
where
    T: FnMut(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, DeserializeError>,
{
    #[inline]
    fn as_fn_once(
        &mut self,
    ) -> impl FnOnce(
        Partial<'facet, BORROW>,
        &mut IteratorStack,
    ) -> Result<Partial<'facet, BORROW>, DeserializeError>
    + '_ {
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// A generic [`DeserializerCore`] wrapper that only calls the provided
/// function on values to be deserialized.
fn create_core<'facet, const BORROW: bool>(
    mut core: &mut impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
) -> impl FnMut(
    Partial<'facet, BORROW>,
    &mut IteratorStack,
) -> Result<Partial<'facet, BORROW>, DeserializeError> {
    move |mut partial, stack| {
        while let Some(item) = stack.pop() {
            match item {
                StackItem::Item(desc) => {
                    let item = Item::Item(DeserializeItem::new(partial, desc));
                    let Item::Item(item) = core(item)? else { todo!() };
                    partial = item.into_inner().0;

                    if partial.frame_count() > 1 {
                        partial = partial.end()?;
                    }

                    return Ok(partial);
                }

                StackItem::Fields(len, fields, variable_base) => {
                    let Some((field, fields)) = fields.split_first() else {
                        if partial.frame_count() > 1 {
                            partial = partial.end()?;
                        }
                        continue;
                    };

                    // Update `variable` using the field's attributes.
                    let variable = variable_base | field.has_attr(Some("mc"), "variable");

                    // Push the remaining fields to the stack.
                    stack.push(StackItem::Fields(len, fields, variable_base));

                    // Push the current field to the stack.
                    let desc = DeserializeDesc::new(variable, Some(field.attributes));
                    stack.push(StackItem::Other(desc));

                    // Begin the current field.
                    partial = partial.begin_nth_field(len - fields.len() - 1)?;
                }

                StackItem::Seq(..) => todo!(),

                StackItem::Map(..) => todo!(),
                StackItem::Set(..) => todo!(),

                StackItem::Other(desc) => {
                    partial = handle_other(partial, desc, &mut core, stack)?;
                }
            }
        }

        Ok(partial)
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `core` type")]
fn handle_other<'facet, const BORROW: bool>(
    mut partial: Partial<'facet, BORROW>,
    mut desc: DeserializeDesc,
    core: &mut impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
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
            partial = Deserializer::new(proxy_partial, desc.is_variable(), core).into_partial()?;
        } else {
            // Otherwise return the partial unchanged.
            partial = proxy_partial;
        }
    }

    // Handle the partial based on its definition.
    match partial.shape().def {
        Def::Undefined => handle_type(partial, desc, core, stack),
        _ => handle_def(partial, desc, core, stack),
    }
}

#[inline(always)]
#[allow(clippy::inline_always, reason = "Used once per `core` type")]
fn handle_def<'facet, const BORROW: bool>(
    partial: Partial<'facet, BORROW>,
    desc: DeserializeDesc,
    core: &mut impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
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
#[allow(clippy::inline_always, reason = "Used once per `core` type")]
fn handle_type<'facet, const BORROW: bool>(
    partial: Partial<'facet, BORROW>,
    desc: DeserializeDesc,
    core: &mut impl FnMut(Item<'facet, BORROW>) -> Result<Item<'facet, BORROW>, ReaderError>,
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
