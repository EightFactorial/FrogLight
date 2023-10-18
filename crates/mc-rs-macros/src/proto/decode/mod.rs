use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta};

use super::macro_type::MacroTypeTrait;
use crate::DeriveMacroAttr;

mod data_enum;
mod data_struct;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DecodeMacro;

impl MacroTypeTrait for DecodeMacro {
    fn generate_macro(&self, _attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        match &input.data {
            Data::Struct(_) => data_struct::decode_struct(input),
            Data::Enum(_) => data_enum::decode_enum(input),
            Data::Union(_) => panic!("Cannot derive `Decode` for a union"),
        }
    }
}

/// Append instructions to read each field to the field list
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
