use attribute_derive::FromAttr;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident, Path};

use super::RegistryAttributes;
use crate::registry::VariantAttributes;

pub(super) fn generate_convertkey(input: DeriveInput, attrs: RegistryAttributes) -> TokenStream {
    // Get the path to `froglight-registry`
    let crate_path = crate::registry::get_registry_path();

    // Get the path to `froglight-protocol`
    let protocol_path = crate::protocol::get_protocol_path();

    // Get the enum ident and data
    let enum_ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        panic!("Registries must be enums");
    };

    // Define the default error type
    let mut error_type = if let Some(error) = attrs.error {
        error
    } else {
        Path::from_input(quote! { #crate_path::definitions::MissingKeyError }).unwrap()
    };

    // Collect the tokens for the `from_key` and `to_key` functions
    let mut from_key_tokens = TokenStream::new();
    let mut to_key_tokens = TokenStream::new();

    // Define the flag for the `other` variant
    let mut is_other = false;
    let mut other_variant: Ident = Ident::new("Bingus", Span::call_site());

    // Parse the variants
    for variant in &data.variants {
        let variant_ident = &variant.ident;

        // Parse the attributes of the variant
        let variant_attrs = VariantAttributes::from_attributes(variant.attrs.iter()).unwrap();

        // Parse the key attribute
        let variant_str = variant_attrs.key.as_str();
        let variant_key = if variant_str.len() <= 24 {
            // If the key is less than or equal to 24 characters, use an inline key.
            quote! { #protocol_path::common::ResourceKey::new_inline(#variant_str) }
        } else {
            // If the key is greater than 24 characters, use a standard key.
            quote! { #protocol_path::common::ResourceKey::new(#variant_str) }
        };

        assert_eq!(
            variant_attrs.other,
            variant_str.is_empty(),
            "Variant key must be set if not marked as `other`"
        );

        // If the variant is the `other` variant, set the flag
        if is_other && variant_attrs.other {
            panic!("Only one variant can be marked as `other`");
        } else if !is_other && variant_attrs.other {
            other_variant = variant.ident.clone();
            is_other = true;

            // Add tokens for the to_key function
            to_key_tokens.extend(quote! {
                #enum_ident::#variant_ident(key) => key.clone().into(),
            });
        } else {
            // Add tokens for the to_key function
            to_key_tokens.extend(quote! {
                #enum_ident::#variant_ident => #variant_key,
            });

            // Add tokens for the from_key function
            from_key_tokens.extend(quote! {
                #variant_str => Ok(#enum_ident::#variant_ident),
            });
        }
    }

    // If the registry has an `other` variant, set the error type to `Infallible`
    if is_other {
        error_type = Path::from_input(quote! { ::std::convert::Infallible }).unwrap();
        from_key_tokens.extend(quote! {
            _ => Ok(#enum_ident::#other_variant(key.clone().into())),
        });
    } else {
        from_key_tokens.extend(quote! {
            _ => Err(#error_type::from(key.clone())),
        });
    }

    // Create a registry name
    let id_registry_name = Ident::new(&format!("{enum_ident}IdRegistry"), Span::call_site());

    quote! {
        type #id_registry_name = #crate_path::definitions::SimpleIdRegistry<#enum_ident>;

        impl #crate_path::definitions::ConvertKey for #enum_ident {
            type Error = #error_type;
            fn from_key(key: &#protocol_path::common::ResourceKey) -> Result<Self, Self::Error> {
                match key.as_ref() {
                    #from_key_tokens
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
