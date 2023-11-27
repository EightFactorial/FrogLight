use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Meta};

use super::{macro_type::MacroTypeTrait, test::TestType};
use crate::DeriveMacroAttr;

mod data_enum;
mod data_struct;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EncodeMacro;

impl MacroTypeTrait for EncodeMacro {
    const REQUIRED_TESTS: &'static [TestType] = &[];

    fn generate_macro(&self, _attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        match &input.data {
            Data::Struct(_) => data_struct::encode_struct(input),
            Data::Enum(_) => data_enum::encode_enum(input),
            Data::Union(_) => panic!("Cannot derive `Encode` for a union"),
        }
    }
}

/// Append instructions to write each field to the field list
fn read_fields(fields: &Fields, field_list: &mut Vec<TokenStream>) {
    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
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
                        crate::buffer::VarEncode::var_encode(&self.#name, buf)
                    }
                } else {
                    quote! {
                        crate::buffer::Encode::encode(&self.#name, buf)
                    }
                };

                match cfg!(feature = "debug") {
                    false => tokens.extend(quote!(?;)),
                    true => tokens.extend(quote! {
                        .map_err(|e| {
                            tracing::error!("Failed to encode field {}: {:?}", stringify!(#name), e);
                            e
                        })?;
                    }),
                }

                field_list.push(tokens);
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let index = Index::from(i);

                let mut tokens = if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    quote! {
                        crate::buffer::VarEncode::var_encode(&self.#index, buf)
                    }
                } else {
                    quote! {
                        crate::buffer::Encode::encode(&self.#index, buf)
                    }
                };

                match cfg!(feature = "debug") {
                    false => tokens.extend(quote!(?;)),
                    true => {
                        let ty = &field.ty;

                        tokens.extend(quote! {
                            .map_err(|e| {
                                tracing::error!("Failed to encode type {}: {:?}", stringify!(#ty), e);
                                e
                            })?;
                        });
                    }
                }

                field_list.push(tokens);
            }
        }
        Fields::Unit => {}
    }
}
