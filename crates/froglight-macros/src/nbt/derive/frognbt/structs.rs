use darling::FromField;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataStruct, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Path};

use super::{FieldAttrs, FieldTagType};

/// Generate `FromCompound` and `IntoCompound` implementations for a struct.
pub(super) fn generate_struct(data: DataStruct, path: &Path) -> (TokenStream, TokenStream) {
    match data.fields {
        Fields::Named(fields) => named_fields(fields, path),
        Fields::Unnamed(fields) => unnamed_fields(fields, path),
        Fields::Unit => panic!("`FrogNbt` does not support unit structs!"),
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_mut, unused_variables)]
fn named_fields(fields: FieldsNamed, path: &Path) -> (TokenStream, TokenStream) {
    let mut from = TokenStream::new();
    let mut into = TokenStream::new();

    for field in fields.named {
        let FieldAttrs { default, tag_name, tag_type, list_type, with_fn, skip_fn } =
            FieldAttrs::from_field(&field).unwrap();
        let Field { ty, ident, .. } = field;

        let ident = ident.unwrap_or_else(|| {
            tag_name.expect("Fields without names require a \"name\" attribute!")
        });
        let ident_str = ident.to_string();

        // Prepare tag handlers based on the input flags
        let (from_handler, into_handler) = match (tag_type, with_fn) {
            (FieldTagType::None, None) => (
                quote! {
                    <#ty as #path::convert::FromTag>::from_tag(tag)?
                },
                quote! {},
            ),
            (other, None) => (
                quote! {
                    TryInto::<#ty>::try_into(tag.clone()).map_err(|err| #path::convert::ConvertError::ConversionError(::core::any::type_name::<#ty>(), Box::new(err)))?
                },
                quote! {},
            ),
            (.., Some(with_fn)) => (
                quote! {
                    #with_fn::from_tag(tag)?
                },
                quote! {},
            ),
        };

        // Prepare missing tag handlers
        let missing_handler = if default.is_present() {
            quote!(<#ty>::default())
        } else {
            quote!(return Err(#path::convert::ConvertError::MissingField(::core::any::type_name::<Self>(), #ident_str)))
        };

        match tag_type {
            FieldTagType::None => {
                from.extend(quote! {
                    #ident: {
                        match nbt.get_tag(#ident_str) {
                            Some(tag) => #from_handler,
                            None => #missing_handler,
                        }
                    },
                });
                into.extend(quote! {
                    todo!();
                });
            }
            FieldTagType::List => {
                let list_type = list_type.expect("List fields require a \"list\" attribute!");
                let list_ident = Ident::new(&format!("{list_type:?}"), Span::call_site());
                let list_type_str = format!("List of {list_type:?}");

                from.extend(quote! {
                    #ident: {
                        match nbt.get_tag(#ident_str) {
                            Some(#path::nbt::NbtTag::List(#path::nbt::NbtListTag::#list_ident(tag))) => #from_handler,
                            Some(..) => return Err(#path::convert::ConvertError::MismatchedTag(::core::any::type_name::<Self>(), #list_type_str)),
                            None => #missing_handler,
                        }
                    },
                });
                into.extend(quote! {
                    todo!();
                });
            }
            other => {
                let tag_type_str = format!("{other:?}");
                let tag_type = Ident::new(&tag_type_str, Span::call_site());

                from.extend(quote! {
                    #ident: {
                        match nbt.get_tag(#ident_str) {
                            Some(#path::nbt::NbtTag::#tag_type(tag)) => #from_handler,
                            Some(..) => return Err(#path::convert::ConvertError::MismatchedTag(::core::any::type_name::<Self>(), #tag_type_str)),
                            None => #missing_handler,
                        }
                    },
                });
                into.extend(quote! {
                    todo!();
                });
            }
        };
    }

    (quote!( Self { #from } ), into)
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_mut, unused_variables)]
fn unnamed_fields(fields: FieldsUnnamed, _path: &Path) -> (TokenStream, TokenStream) {
    let mut from = TokenStream::new();
    let mut into = TokenStream::new();

    for field in fields.unnamed {
        let FieldAttrs { default, tag_name, tag_type, list_type, with_fn, skip_fn } =
            FieldAttrs::from_field(&field).unwrap();

        let Field { ty, ident, .. } = field;
        let ident = ident.unwrap_or_else(|| {
            tag_name.expect("Fields without names require a \"name\" attribute!")
        });
    }

    into.extend(quote!(todo!();));

    (quote!( Self( #from ) ), into)
}
