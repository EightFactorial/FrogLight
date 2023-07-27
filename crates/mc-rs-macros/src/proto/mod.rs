use proc_macro2::TokenStream;
use quote::quote;

mod encode;
pub use encode::derive_encode;

mod decode;
pub use decode::derive_decode;

mod state;
pub use state::impl_state;

/// Derive both `Encode` and `Decode`
pub(super) fn derive_transcode(input: proc_macro::TokenStream) -> TokenStream {
    let encode = derive_encode(input.clone());
    let decode = derive_decode(input);

    quote! {
        #encode
        #decode
    }
}
