use attribute_derive::FromAttr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

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

    // Collect the tokens for the `from_key` and `to_key` functions
    let mut from_key_tokens = TokenStream::new();
    let mut to_key_tokens = TokenStream::new();

    let mut key_tokens = TokenStream::new();

    // Parse the variants
    for variant in &data.variants {
        let variant_ident = &variant.ident;

        // Parse the attributes of the variant
        let variant_attrs = VariantAttributes::from_attributes(variant.attrs.iter()).unwrap();
        let variant_str = variant_attrs.key.as_str();

        // Add tokens for the str constant
        let str_ident = Ident::new(
            &format!("{}_STR", variant_ident.to_string().to_uppercase()),
            variant_ident.span(),
        );
        // Add tokens for the key constant
        let const_ident = Ident::new(
            &format!("{}_KEY", variant_ident.to_string().to_uppercase()),
            variant_ident.span(),
        );

        key_tokens.extend(quote! {
            pub const #str_ident: &'static str = #variant_str;
            pub const #const_ident: &'static #protocol_path::common::ResourceKey = &#protocol_path::common::ResourceKey::const_new(Self::#str_ident);
        });

        // Add tokens for the from_key function
        from_key_tokens.extend(quote! {
            Self::#str_ident => Some(#enum_ident::#variant_ident),
        });

        // Add tokens for the to_key function
        to_key_tokens.extend(quote! {
            #enum_ident::#variant_ident => Self::#const_ident,
        });
    }

    quote! {
        impl #enum_ident {
            #key_tokens
        }
        impl #crate_path::definitions::ConvertKey for #enum_ident {
            fn from_key(key: &str) -> Option<Self> {
                match key {
                    #from_key_tokens
                    _ => None,
                }
            }
            fn to_key(&self) -> &'static #protocol_path::common::ResourceKey {
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
