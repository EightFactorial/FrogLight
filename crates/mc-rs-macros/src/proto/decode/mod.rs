use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta};

use super::{macro_type::MacroTypeTrait, test::TestType};
use crate::DeriveMacroAttr;

mod data_enum;
mod data_struct;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DecodeMacro;

impl MacroTypeTrait for DecodeMacro {
    const REQUIRED_TESTS: &'static [TestType] = &[];

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
            for field in &fields.named {
                let Some(name) = &field.ident else {
                    continue;
                };

                let mut tokens = if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    quote! {
                        #name: crate::buffer::VarDecode::var_decode(buf)
                    }
                } else {
                    quote! {
                        #name: crate::buffer::Decode::decode(buf)
                    }
                };

                if cfg!(feature = "debug") {
                    tokens.extend(quote! {
                        .map_err(|e| {
                            tracing::error!("Failed to decode field {}: {:?}", stringify!(#name), e);
                            e
                        })?,
                    });
                } else {
                    tokens.extend(quote! {
                            ?,
                    });
                };

                field_list.push(tokens);
            }
        }
        Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                let mut tokens = if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    quote! {
                        crate::buffer::VarDecode::var_decode(buf)
                    }
                } else {
                    quote! {
                        crate::buffer::Decode::decode(buf)
                    }
                };

                if cfg!(feature = "debug") {
                    let ty = &field.ty;
                    tokens.extend(quote! {
                        .map_err(|e| {
                            tracing::error!("Failed to decode type {}: {:?}", stringify!(#ty), e);
                            e
                        })?,
                    });
                } else {
                    tokens.extend(quote! {
                            ?,
                    });
                };

                field_list.push(tokens);
            }
        }
        Fields::Unit => {}
    }
}
