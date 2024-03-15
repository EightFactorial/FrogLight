use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::protocol::Attributes;

pub(crate) fn read_as_json(input: &DeriveInput, _: &Attributes) -> TokenStream {
    let name = &input.ident;
    quote! {
        impl crate::io::FrogRead for #name {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
            where
                Self: Sized,
            {
                let string = <String as crate::io::FrogRead>::fg_read(buf)?;
                serde_json::from_str(&string).map_err(crate::io::ReadError::Json)
            }
        }
    }
}

pub(crate) fn write_as_json(input: &DeriveInput, _: &Attributes) -> TokenStream {
    let name = &input.ident;
    quote! {
        impl crate::io::FrogWrite for #name {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                let string = serde_json::to_string(self).map_err(crate::io::WriteError::Json)?;
                <String as crate::io::FrogWrite>::fg_write(&string, buf)
            }
        }
    }
}
