use proc_macro::TokenStream;

mod client;
mod proto;

#[proc_macro_derive(Encode, attributes(bitfield, json, var))]
pub fn derive_encode(input: TokenStream) -> TokenStream { proto::derive_encode(input).into() }

#[proc_macro_derive(Decode, attributes(bitfield, json, var))]
pub fn derive_decode(input: TokenStream) -> TokenStream { proto::derive_decode(input).into() }

#[proc_macro_derive(Packet, attributes(bitfield, json, var))]
pub fn derive_packet(input: TokenStream) -> TokenStream { proto::derive_packet(input).into() }
