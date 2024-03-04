use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::protocol::Attributes;

pub(crate) fn read_as_json(input: &DeriveInput, _attrs: &Attributes) -> TokenStream {
    let name = &input.ident;
    quote! {
        impl FrogRead for #name {
            fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
            where
                Self: Sized,
            {
                serde_json::from_str(&String::fg_read(buf)?).map_err(ReadError::Json)
            }
        }
    }
}

pub(crate) fn write_as_json(input: &DeriveInput, _attrs: &Attributes) -> TokenStream {
    let name = &input.ident;
    quote! {
        impl FrogWrite for #name {
            fn fg_write(&self, buf: &mut Vec<u8>) -> Result<(), WriteError> {
                serde_json::to_string(self).map_err(WriteError::Json)?.fg_write(buf)
            }
        }
    }
}
