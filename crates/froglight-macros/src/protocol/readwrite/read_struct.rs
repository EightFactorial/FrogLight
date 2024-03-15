use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput};

use crate::protocol::Attributes;

pub(super) fn read_struct(input: &DeriveInput, _: &Attributes) -> TokenStream {
    let DeriveInput { ident, data, .. } = input;
    let Data::Struct(DataStruct { fields, .. }) = &data else {
        unreachable!("Only structs are supported")
    };

    match fields {
        syn::Fields::Named(fields) => {
            let tokens = read_named(fields);
            quote! {
                impl crate::io::FrogRead for #ident {
                    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError> {
                        Ok(Self {
                            #tokens
                        })
                    }
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            let tokens = read_unnamed(fields);
            quote! {
                impl crate::io::FrogRead for #ident {
                    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError> {
                        Ok(Self(#tokens))
                    }
                }

            }
        }
        syn::Fields::Unit => quote! {
           impl crate::io::FrogRead for #ident {
               fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError> {
                   Ok(Self)
               }
           }
        },
    }
}

/// Read the fields of a named struct
fn read_named(fields: &syn::FieldsNamed) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    for field in &fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        let field_attrs = field.attrs.as_slice();

        // Read the field normally, or read as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                #field_ident: crate::io::FrogVarRead::fg_var_read(buf)?,
            });
        } else {
            field_tokens.extend(quote! {
                #field_ident: crate::io::FrogRead::fg_read(buf)?,
            });
        }
    }

    field_tokens
}

/// Read the fields of an unnamed struct
fn read_unnamed(fields: &syn::FieldsUnnamed) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    for field in &fields.unnamed {
        let field_type = &field.ty;
        let field_attrs = field.attrs.as_slice();

        // Read the field normally, or read as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                crate::io::FrogVarRead::fg_var_read(buf)?,
            });
        } else {
            field_tokens.extend(quote! {
                <#field_type as crate::io::FrogRead>::fg_read(buf)?,
            });
        }
    }

    field_tokens
}
