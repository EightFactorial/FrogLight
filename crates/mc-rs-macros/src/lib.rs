use proc_macro::TokenStream;

mod client;
mod proto;

/// Derive `Encode` for a struct or enum
///
/// This allows the struct or enum to be encoded into a buffer.
#[proc_macro_derive(Encode, attributes(bitset, json, var))]
pub fn derive_encode(input: TokenStream) -> TokenStream { proto::derive_encode(input).into() }

/// Derive `Decode` for a struct or enum
///
/// This allows the struct or enum to be decoded from a buffer.
#[proc_macro_derive(Decode, attributes(bitset, json, var))]
pub fn derive_decode(input: TokenStream) -> TokenStream { proto::derive_decode(input).into() }

/// Derive both `Encode` and `Decode` for a struct or enum
///
/// This allows the struct or enum to be encoded into and decoded from a buffer.
#[proc_macro_derive(Transcode, attributes(bitset, json, var))]
pub fn derive_transcode(input: TokenStream) -> TokenStream { proto::derive_transcode(input).into() }

/// Derive `State<V>` for a state
///
/// This allows the state to be used in a connection for that version.
#[proc_macro]
pub fn impl_state(input: TokenStream) -> TokenStream { proto::impl_state(input) }
