use proc_macro2::{TokenStream, TokenTree};
use syn::{DeriveInput, Meta};

use super::modifiers;

pub(super) fn generate_read(input: &DeriveInput) -> TokenStream {
    for attr in &input.attrs {
        if attr.path().is_ident("frog") {
            let Meta::List(attr) = &attr.meta else { panic!("Invalid frog attribute") };
            for token in attr.tokens.clone() {
                if let TokenTree::Ident(attr_ident) = token {
                    match attr_ident.to_string().as_str() {
                        "json" => return modifiers::structs::read_as_json(input),
                        "bitset" => return modifiers::structs::read_as_bitset(input),
                        _ => {}
                    }
                }
            }
        }
    }

    TokenStream::new()
}

pub(super) fn generate_write(input: &DeriveInput) -> TokenStream {
    for attr in &input.attrs {
        if attr.path().is_ident("frog") {
            let Meta::List(attr) = &attr.meta else { panic!("Invalid frog attribute") };
            for token in attr.tokens.clone() {
                if let TokenTree::Ident(attr_ident) = token {
                    match attr_ident.to_string().as_str() {
                        "json" => return modifiers::structs::write_as_json(input),
                        "bitset" => return modifiers::structs::write_as_bitset(input),
                        _ => {}
                    }
                }
            }
        }
    }

    TokenStream::new()
}
