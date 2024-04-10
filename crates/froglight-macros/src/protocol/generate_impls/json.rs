use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput) -> TokenStream {
    let crate_path = crate::protocol::get_protocol_path();

    let struct_ident = &input.ident;

    quote! {
        impl #crate_path::protocol::FrogRead for #struct_ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, #crate_path::protocol::ReadError>
            where
                Self: Sized,
            {
                let string = <String as #crate_path::protocol::FrogRead>::fg_read(buf)?;
                serde_json::from_str(&string).map_err(#crate_path::protocol::ReadError::Json)
            }
        }
    }
    .into()
}

/// Generate a `FrogWrite` implementation.
pub(super) fn generate_write(input: &DeriveInput) -> TokenStream {
    let crate_path = crate::protocol::get_protocol_path();

    let struct_ident = &input.ident;

    quote! {
        impl #crate_path::protocol::FrogWrite for #struct_ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), #crate_path::protocol::WriteError> {
                let string = serde_json::to_string(self).map_err(#crate_path::protocol::WriteError::Json)?;
                <String as #crate_path::protocol::FrogWrite>::fg_write(&string, buf)
            }
        }
    }.into()
}
