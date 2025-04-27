use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields, FieldsUnnamed, LitInt, Variant};

use crate::CrateManifest;

#[derive(FromDeriveInput)]
#[darling(attributes(frog))]
struct FrogPacketMacro {}

// -------------------------------------------------------------------------------------------------

pub(crate) fn derive_frogpackets(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let FrogPacketMacro {} = FrogPacketMacro::from_derive_input(&input).unwrap();
    let DeriveInput { ident, data, .. } = input;

    // Attempt to find the following crates in order:
    // `froglight::network::io`, `froglight-network::io`, and `froglight-io`.
    let path = CrateManifest::try_find("froglight-network", "froglight").map_or_else(
        || CrateManifest::froglight("froglight-io"),
        |path| syn::parse2(quote!(#path::io)).unwrap(),
    );

    // Attempt to find the following crates in order:
    // `froglight::common`, `froglight-common`.
    let common = CrateManifest::froglight("froglight-common");

    let Data::Enum(DataEnum { variants, .. }) = data else {
        panic!("`FrogPackets` requires an enum of packets!")
    };

    // Return an unreachable error if the enum has no variants
    if variants.is_empty() {
        let message =
            format!("Enum \"{ident}\" has no variants, this code should never be reached!");
        return quote! {
            #[automatically_derived]
            impl<V: #common::version::Version> #path::version::FrogReadVersion<V> for #ident {
                fn frog_read(_: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> { unreachable!(#message) }
            }

            #[automatically_derived]
            impl<V: #common::version::Version> #path::version::FrogWriteVersion<V> for #ident {
                fn frog_write(&self, _: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> { unreachable!(#message) }
                fn frog_len(&self) -> usize { unreachable!(#message) }
            }
        };
    }

    let mut read_tokens = TokenStream::new();
    let mut write_tokens = TokenStream::new();
    let mut length_tokens = TokenStream::new();

    let mut requirements = TokenStream::new();

    for Variant { ident, fields, discriminant, .. } in &variants {
        let Some((_, discriminant)) = discriminant else {
            panic!("`FrogPackets` requires each packet to have a discriminant!")
        };
        let discriminant: LitInt = syn::parse_quote!(#discriminant);

        let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = fields else {
            panic!("`FrogPackets` requires each packet to have a single unnamed field!")
        };
        assert!(
            unnamed.len() == 1,
            "`FrogPackets` requires each packet to have a single unnamed field!"
        );

        // Add to the list of types required to implement ReadVersion/WriteVersion
        let ty = &unnamed[0].ty;
        requirements.extend(
            quote!{ #ty: #path::version::FrogReadVersion<V> + #path::version::FrogWriteVersion<V>, },
        );

        read_tokens.extend(quote! {
            #discriminant => Ok(Self::#ident(#path::version::FrogReadVersion::<V>::frog_read(buffer)?)),
        });

        write_tokens.extend(quote! {
            Self::#ident(value) => {
                written += <i32 as #path::variable::FrogVarWrite>::frog_var_write(&#discriminant, buffer)?;
                written += #path::version::FrogWriteVersion::frog_write(value, buffer)?;
            },
        });
        length_tokens.extend(quote! {
            Self::#ident(value) => {
                length += <i32 as #path::variable::FrogVarWrite>::frog_var_len(&#discriminant);
                length += #path::version::FrogWriteVersion::frog_len(value);
            },
        });
    }

    quote! {
        #[automatically_derived]
        impl<V: #common::version::Version> #path::version::FrogReadVersion<V> for #ident
        where #requirements
        {
            fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                match <i32 as #path::variable::FrogVarRead>::frog_var_read(buffer)? {
                    #read_tokens
                    unk => Err(#path::standard::ReadError::InvalidEnum(core::any::type_name::<Self>(), unk)),
                }
            }
        }

        #[automatically_derived]
        impl<V: #common::version::Version> #path::version::FrogWriteVersion<V> for #ident
        where #requirements
        {
            fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                let mut written = 0;
                match self {
                    #write_tokens
                }
                Ok(written)
            }

            fn frog_len(&self) -> usize {
                let mut length = 0;
                match self {
                    #length_tokens
                }
                length
            }
        }
    }
}
