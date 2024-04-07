use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput) -> proc_macro::TokenStream {
    let enum_ident = &input.ident;
    let Data::Enum(_data) = &input.data else {
        unreachable!("Enum generator called on non-enum type");
    };

    let read_tokens = TokenStream::new();
    // for variant in &data.variants {}

    quote! {
        impl crate::protocol::FrogRead for #enum_ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
            where
                Self: Sized,
            {
                #read_tokens
            }
        }
    }
    .into()
}

pub(super) fn generate_write(input: &DeriveInput) -> proc_macro::TokenStream {
    let enum_ident = &input.ident;
    let Data::Enum(_data) = &input.data else {
        unreachable!("Enum generator called on non-enum type");
    };

    let write_tokens = TokenStream::new();
    // for variant in &data.variants {}

    quote! {
        impl crate::protocol::FrogWrite for #enum_ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::protocol::WriteError> {
                #write_tokens
            }
        }
    }.into()
}
