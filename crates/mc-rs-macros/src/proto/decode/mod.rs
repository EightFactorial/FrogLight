use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta};

use super::generate_tests;

mod data_enum;
mod data_struct;

/// Tests that are always generated
static STATIC_TESTS: &[&str] = &[];

/// Derive `Decode`
pub fn derive_decode(input: proc_macro::TokenStream, create_tests: bool) -> TokenStream {
    // Generate tests if requested
    let extra = match create_tests {
        true => Some(generate_tests(input.clone(), Some(STATIC_TESTS))),
        false => None,
    };

    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse(input).expect("Unable to DeriveInput");

    match data {
        Data::Struct(data) => data_struct::decode_struct(attrs, ident, data, extra),
        Data::Enum(data) => data_enum::decode_enum(attrs, ident, data, extra),
        Data::Union(_) => panic!("Cannot derive `Decode` for a union"),
    }
}

fn read_fields(fields: &Fields, field_list: &mut Vec<TokenStream>) {
    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let Some(name) = &field.ident else {
                    continue;
                };

                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        #name: crate::buffer::VarDecode::var_decode(buf)?,
                    });
                } else {
                    field_list.push(quote! {
                        #name: crate::buffer::Decode::decode(buf)?,
                    });
                }
            }
        }
        Fields::Unnamed(fields) => {
            for field in fields.unnamed.iter() {
                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        crate::buffer::VarDecode::var_decode(buf)?,
                    });
                } else {
                    field_list.push(quote! {
                        crate::buffer::Decode::decode(buf)?,
                    });
                }
            }
        }
        Fields::Unit => {}
    }
}
