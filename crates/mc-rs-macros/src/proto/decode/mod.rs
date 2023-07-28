use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta};

mod data_enum;
mod data_struct;

/// Derive `Decode`
pub fn derive_decode(input: proc_macro::TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse(input).unwrap();

    match data {
        Data::Struct(data) => data_struct::decode_struct(attrs, ident, data),
        Data::Enum(data) => data_enum::decode_enum(attrs, ident, data),
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
