use attribute_derive::FromAttr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(super) fn generate_convertkey(input: DeriveInput) -> TokenStream {
    // Get the path to `froglight-registry`
    let crate_path = crate::registry::get_registry_path();

    // Get the path to `froglight-protocol`
    let protocol_path = crate::protocol::get_protocol_path();

    // Get the enum ident and data
    let enum_ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        panic!("Registries must be enums");
    };

    // Collect the tokens for the `from_key`, `to_key_str`, and `to_key` functions
    let mut from_key_tokens = TokenStream::new();
    let mut to_key_str_tokens = TokenStream::new();
    let mut to_key_tokens = TokenStream::new();

    // Parse the variants
    for variant in &data.variants {
        let variant_ident = &variant.ident;

        // Parse the attributes of the variant
        let variant_attrs = VariantAttributes::from_attributes(variant.attrs.iter()).unwrap();
        let variant_str = variant_attrs.key.as_str();

        // Add tokens for the to_key function
        to_key_tokens.extend(quote! {
            #enum_ident::#variant_ident => #protocol_path::common::ResourceKey::const_new(#variant_str),
        });

        // Add tokens for the to_key_str function
        to_key_str_tokens.extend(quote! {
            #enum_ident::#variant_ident => #variant_str,
        });

        // Add tokens for the from_key function
        from_key_tokens.extend(quote! {
            #variant_str => Some(#enum_ident::#variant_ident),
        });
    }

    quote! {
        impl #crate_path::definitions::ConvertKey for #enum_ident {
            fn from_key(key: &str) -> Option<Self> {
                match key {
                    #from_key_tokens
                    _ => None,
                }
            }
            fn to_key_str(&self) -> &'static str {
                match self {
                    #to_key_str_tokens
                }
            }
            fn to_key(&self) -> #protocol_path::common::ResourceKey {
                match self {
                    #to_key_tokens
                }
            }
        }
    }
}

/// Attributes for registry variants.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
struct VariantAttributes {
    /// The key for the registry variant.
    key: String,
}
