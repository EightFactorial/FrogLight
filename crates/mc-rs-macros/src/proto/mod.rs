use proc_macro2::TokenStream;
use quote::quote;

mod encode;
pub use encode::derive_encode;

mod decode;
pub use decode::derive_decode;

/// Derive both `Encode` and `Decode`
pub(super) fn derive_packet(input: proc_macro::TokenStream) -> TokenStream {
    let encode = derive_encode(input.clone());
    let decode = derive_decode(input);

    quote! {
        #encode
        #decode
    }
}
