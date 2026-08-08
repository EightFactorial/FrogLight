use core::marker::PhantomData;

use froglight_facet_iter::serialize::ItemType as SerializeItemType;

use crate::facet::prelude::*;

/// A [`FacetTemplate`] that wraps custom serializers and deserializers inside
/// an [`Option`].
///
/// See [`FacetTemplate`] for more details and an example.
///
/// # Note
///
/// This only works for types the implement [`FacetTemplate`]. If you need to
/// wrap an existing [`WithFnAttr`](crate::facet::WithFnAttr), see
/// [`option_with!`](crate::option_with) instead.
///
/// # Example
///
/// ```rust
/// use facet::*;
/// use froglight_facet::{
///     facet::{WithFnAttr, prelude::*},
///     option_with,
/// };
///
/// #[derive(Facet)]
/// pub struct MyStruct {
///     #[facet(mc::with = MyStruct::WITH_CUSTOM)]
///     inner: u32,
/// }
///
/// impl MyStruct {
///     /// A serializer and deserializer.
///     pub const WITH_CUSTOM: WithFnAttr = CustomTemplate::WITH;
/// }
///
/// struct CustomTemplate;
///
/// impl FacetTemplate for CustomTemplate {
///     fn serialize(_: SerializeItem<'_, '_>, _: &mut Writer<'_>) -> Result<(), WriterError> {
///         todo!()
///     }
///
///     fn deserialize<'facet, const BORROW: bool>(
///         _: DeserializeItem<'facet, BORROW>,
///         _: &mut Reader<'_>,
///     ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
///         todo!()
///     }
/// }
/// ```
pub struct OptionTemplate<T: FacetTemplate>(PhantomData<T>);

impl<T: FacetTemplate> FacetTemplate for OptionTemplate<T> {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let option = item.peek().into_option()?;
        if let Some(value) = option.value() {
            writer.write_byte(1)?;

            let item = SerializeItem::new(value, SerializeItemType::Value, item.is_variable());
            T::WITH.serialize(item, writer)
        } else {
            writer.write_byte(0)
        }
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        match reader.read_byte()? {
            0 => item.scoped(|mut partial| {
                partial = partial.set_default()?;
                Ok(partial)
            }),
            1 => {
                let variable = item.is_variable();
                item.scoped(|mut partial| {
                    partial = partial.begin_some()?;

                    let mut item = DeserializeItem::new_partial(partial).with_variable(variable);
                    item = T::WITH.deserialize(item, reader)?;

                    Ok(item.into_inner().0)
                })
            }
            unk => Err(ReaderError::InvalidBool(unk)),
        }
    }
}

impl<T: FacetBorrowedTemplate> FacetBorrowedTemplate for OptionTemplate<T> {
    fn deserialize_borrowed<'facet>(
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError> {
        match reader.read_byte()? {
            0 => item.scoped(|mut partial| {
                partial = partial.set_default()?;
                Ok(partial)
            }),
            1 => {
                let variable = item.is_variable();
                item.scoped(|mut partial| {
                    partial = partial.begin_some()?;

                    let mut item = DeserializeItem::new_partial(partial).with_variable(variable);
                    item = T::WITH.deserialize_borrowed(item, reader)?;

                    Ok(item.into_inner().0)
                })
            }
            unk => Err(ReaderError::InvalidBool(unk)),
        }
    }
}

/// A helper macro that creates a new [`WithFnAttr`](crate::facet::WithFnAttr)
/// that wraps an existing [`WithFnAttr`](crate::facet::WithFnAttr) inside an
/// [`Option`].
///
/// See [`FacetTemplate`] for more details.
///
/// # Note
///
///
/// This only works for [`WithFnAttr`](crate::facet::WithFnAttr) directly. If
/// you have types that implement [`FacetTemplate`], you can use
/// [`OptionTemplate`] instead.
///
///
/// # Example
///
/// ```rust
/// use facet::*;
/// use froglight_facet::{
///     facet::{WithFnAttr, prelude::*},
///     option_with,
/// };
///
/// #[derive(Facet)]
/// pub struct MyStruct {
///     #[facet(mc::with = MyStruct::WITH_CUSTOM)]
///     inner_a: u32,
///     #[facet(mc::with = MyStruct::WITH_OPT_CUSTOM)]
///     inner_b: Option<u32>,
/// }
///
/// impl MyStruct {
///     /// A serializer and deserializer.
///     pub const WITH_CUSTOM: WithFnAttr = CustomTemplate::WITH;
///     /// A serializer and deserializer that wraps [`MyStruct::WITH_CUSTOM`] inside an [`Option`].
///     pub const WITH_OPT_CUSTOM: WithFnAttr = option_with!(MyStruct::WITH_CUSTOM);
/// }
///
/// struct CustomTemplate;
///
/// impl FacetTemplate for CustomTemplate {
///     fn serialize(_: SerializeItem<'_, '_>, _: &mut Writer<'_>) -> Result<(), WriterError> {
///         todo!()
///     }
///
///     fn deserialize<'facet, const BORROW: bool>(
///         _: DeserializeItem<'facet, BORROW>,
///         _: &mut Reader<'_>,
///     ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
///         todo!()
///     }
/// }
/// ```
#[macro_export]
macro_rules! option_with {
    ( $with:path ) => {
        $crate::facet::WithFnAttr::using(
            |item, writer| {
                let option = item.peek().into_option()?;
                if let Some(inner) = option.value() {
                    writer.write_byte(1)?;

                    let item = $crate::facet::prelude::SerializeItem::new(
                        inner,
                        $crate::facet::prelude::SerializeItemType::Value,
                        item.is_variable(),
                    );

                    ($with.ser)(item, writer)
                } else {
                    writer.write_byte(0)
                }
            },
            |mut item, reader| match reader.read_byte()? {
                0 => item.scoped(|mut partial| {
                    partial = partial.set_default()?;
                    Ok(partial)
                }),
                1 => {
                    let variable = item.is_variable();
                    item.scoped(|mut partial| {
                        partial = partial.begin_some()?;

                        let mut item =
                            $crate::facet::prelude::DeserializeItem::new_partial(partial)
                                .with_variable(variable);
                        item = ($with.de_owned)(item, reader)?;

                        Ok(item.into_inner().0)
                    })
                }
                unk => Err($crate::facet::prelude::ReaderError::InvalidBool(unk)),
            },
            |mut item, reader| match reader.read_byte()? {
                0 => item.scoped(|mut partial| {
                    partial = partial.set_default()?;
                    Ok(partial)
                }),
                1 => {
                    let variable = item.is_variable();
                    item.scoped(|mut partial| {
                        partial = partial.begin_some()?;

                        let mut item =
                            $crate::facet::prelude::DeserializeItem::new_partial(partial)
                                .with_variable(variable);
                        item = ($with.de_owned_borrow)(item, reader)?;

                        Ok(item.into_inner().0)
                    })
                }
                unk => Err($crate::facet::prelude::ReaderError::InvalidBool(unk)),
            },
        )
    };
}
