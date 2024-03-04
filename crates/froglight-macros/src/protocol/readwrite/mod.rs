use proc_macro2::TokenStream;
use syn::DeriveInput;

use super::{modifiers, Attributes};

pub(super) fn generate_read(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    if attrs.json {
        return modifiers::structs::read_as_json(input, attrs);
    } else if attrs.bitset.is_some() {
        return modifiers::structs::read_as_bitset(input, attrs);
    }

    TokenStream::new()
}

pub(super) fn generate_write(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    if attrs.json {
        return modifiers::structs::write_as_json(input, attrs);
    } else if attrs.bitset.is_some() {
        return modifiers::structs::write_as_bitset(input, attrs);
    }

    TokenStream::new()
}
