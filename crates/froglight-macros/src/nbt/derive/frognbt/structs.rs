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
        let FieldAttrs { default, inline, tag_name, tag_type, list_type, with_fn, skip_fn } =
            FieldAttrs::from_field(&field).unwrap();
        let Field { ty, ident, .. } = field;

        // Handle when fields are marked `#[frog(inline)]`
        if inline.is_present() {
            from.extend(quote! {
                #ident: #path::convert::FromCompound::from_compound(nbt)?,
            });
            into.extend(quote! {
                todo!();
            });
            continue;
        }

        // Get the name of the field
        let field = ident.expect("Fields without names require a \"name\" attribute!");
        let field_str = field.to_string();
        // Get the name of the tag
        let tag_name = tag_name.unwrap_or_else(|| field.clone()).to_string();

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
                    TryInto::<#ty>::try_into(tag.clone()).map_err(|err| #path::convert::NbtError::ConversionError(::core::any::type_name::<#ty>(), Box::new(err)))?
                },
                quote! {},
            ),
            (.., Some(with_fn)) => (
                quote! {
                    #with_fn::read_from_tag(tag)?
                },
                quote! {},
            ),
        };

        // Prepare missing tag handlers
        let missing_handler = if default.is_present() {
            quote!(<#ty>::default())
        } else {
            quote!(return Err(#path::convert::NbtError::MissingField(::core::any::type_name::<Self>(), #field_str)))
        };

        match tag_type {
            FieldTagType::None => {
                from.extend(quote! {
                    #field: {
                        match nbt.get_tag(#tag_name) {
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
                    #field: {
                        match nbt.get_tag(#tag_name) {
                            Some(#path::nbt::NbtTag::List(#path::nbt::NbtListTag::#list_ident(tag))) => #from_handler,
                            Some(..) => return Err(#path::convert::NbtError::MismatchedTag(::core::any::type_name::<Self>(), #list_type_str)),
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
                    #field: {
                        match nbt.get_tag(#tag_name) {
                            Some(#path::nbt::NbtTag::#tag_type(tag)) => #from_handler,
                            Some(..) => return Err(#path::convert::NbtError::MismatchedTag(::core::any::type_name::<Self>(), #tag_type_str)),
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

#[allow(unused_mut, unused_variables, unreachable_code)]
fn unnamed_fields(fields: FieldsUnnamed, _path: &Path) -> (TokenStream, TokenStream) {
    let mut from = TokenStream::new();
    let mut into = TokenStream::new();

    for field in fields.unnamed {
        let FieldAttrs { default, inline, tag_name, tag_type, list_type, with_fn, skip_fn } =
            FieldAttrs::from_field(&field).unwrap();

        // Handle when fields are marked `#[frog(inline)]`
        if inline.is_present() {
            from.extend(quote! {
                todo!();
            });
            into.extend(quote! {
                todo!();
            });
        }

        let Field { ty, ident, .. } = field;

        // Get the name of the field
        let field: Ident = todo!();
        let field_str = field.to_string();

        // Get the name of the tag
        let tag_name = tag_name.unwrap_or_else(|| field.clone());
    }

    into.extend(quote!(todo!();));

    (quote!( Self( #from ) ), into)
}
