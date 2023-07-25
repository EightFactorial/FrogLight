use proc_macro::TokenStream;

mod client;
mod proto;

/// Derive `Encode` for a struct or enum
///
/// This allows the struct or enum to be encoded into a buffer.
#[proc_macro_derive(Encode, attributes(bitfield, json, var))]
pub fn derive_encode(input: TokenStream) -> TokenStream { proto::derive_encode(input).into() }

/// Derive `Decode` for a struct or enum
///
/// This allows the struct or enum to be decoded from a buffer.
#[proc_macro_derive(Decode, attributes(bitfield, json, var))]
pub fn derive_decode(input: TokenStream) -> TokenStream { proto::derive_decode(input).into() }

/// Derive both `Encode` and `Decode` for a struct or enum
///
/// This allows the struct or enum to be encoded into and decoded from a buffer.
#[proc_macro_derive(Transcode, attributes(bitfield, json, var))]
pub fn derive_transcode(input: TokenStream) -> TokenStream { proto::derive_transcode(input).into() }
