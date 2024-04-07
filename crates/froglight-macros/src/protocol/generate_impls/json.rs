use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput) -> TokenStream {
    let struct_ident = &input.ident;

    quote! {
        impl ::froglight::protocol::FrogRead for #struct_ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ::froglight::protocol::ReadError>
            where
                Self: Sized,
            {
                let string = <String as ::froglight::protocol::FrogRead>::fg_read(buf)?;
                serde_json::from_str(&string).map_err(::froglight::protocol::ReadError::Json)
            }
        }
    }
    .into()
}

/// Generate a `FrogWrite` implementation.
pub(super) fn generate_write(input: &DeriveInput) -> TokenStream {
    let struct_ident = &input.ident;

    quote! {
        impl ::froglight::protocol::FrogWrite for #struct_ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), ::froglight::protocol::WriteError> {
                let string = serde_json::to_string(self).map_err(::froglight::protocol::WriteError::Json)?;
                <String as ::froglight::protocol::FrogWrite>::fg_write(&string, buf)
            }
        }
    }.into()
}
