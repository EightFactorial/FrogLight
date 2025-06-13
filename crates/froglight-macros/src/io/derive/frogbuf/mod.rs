use darling::{FromDeriveInput, FromField, util::Flag};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::CrateManifest;

mod enums;
mod structs;

#[derive(FromDeriveInput)]
#[darling(attributes(frog))]
struct FrogBufMacro {
    version: Flag,
}

#[derive(FromField)]
#[darling(attributes(frog))]
struct FrogBufField {
    var: Flag,
    json: Flag,
}

// -------------------------------------------------------------------------------------------------

pub(crate) fn derive_frogbuf(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let FrogBufMacro { version } = FrogBufMacro::from_derive_input(&input).unwrap();
    let DeriveInput { ident, data, .. } = input;

    // Attempt to find the following crates in order:
    // `froglight::network::io`, `froglight-network::io`, and `froglight-io`.
    let path = CrateManifest::try_find("froglight-network", "froglight").map_or_else(
        || CrateManifest::froglight("froglight-io"),
        |path| syn::parse2(quote!(#path::io)).unwrap(),
    );

    // Derive the `FrogRead` and `FrogWrite` traits.
    match data {
        Data::Struct(data) => structs::derive_struct(ident, data, version, path),
        Data::Enum(data) => enums::derive_enum(ident, data, version, path),
        Data::Union(..) => panic!("`FrogBuf` cannot be derived for unions!"),
    }
}
