use attribute_derive::FromAttr;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident, Path};

use super::ConvertKeyAttributes;
use crate::registry::ConvertKeyVariantAttributes;

pub(super) fn generate_convertkey(input: DeriveInput, attrs: ConvertKeyAttributes) -> TokenStream {
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
        Path::from_input(quote! { #crate_path::definitions::errors::InvalidKeyError }).unwrap()
    };

    // Collect the tokens for the `from_key`, `to_key_str`, and `to_key` functions
    let mut from_key_tokens = TokenStream::new();
    let mut to_key_str_tokens = TokenStream::new();
    let mut to_key_tokens = TokenStream::new();

    // Define the flag for the `other` variant
    let mut is_other = false;
    let mut other_variant: Option<Ident> = None;

    // Parse the variants
    for variant in &data.variants {
        let variant_ident = &variant.ident;

        // Parse the attributes of the variant
        let variant_attrs =
            ConvertKeyVariantAttributes::from_attributes(variant.attrs.iter()).unwrap();

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
            other_variant = Some(variant.ident.clone());
            is_other = true;

            // Add tokens for the to_key function
            to_key_tokens.extend(quote! {
                #enum_ident::#variant_ident(key) => key.clone().into(),
            });

            // Add tokens for the to_key_str function
            to_key_str_tokens.extend(quote! {
                #enum_ident::#variant_ident(key) => key.as_ref(),
            });
        } else {
            // Add tokens for the to_key function
            to_key_tokens.extend(quote! {
                #enum_ident::#variant_ident => #variant_key,
            });

            // Add tokens for the to_key_str function
            to_key_str_tokens.extend(quote! {
                #enum_ident::#variant_ident => #variant_str,
            });

            // Add tokens for the from_key function
            from_key_tokens.extend(quote! {
                #variant_str => Ok(#enum_ident::#variant_ident),
            });
        }
    }

    // If the registry has an `other` variant, set the error type to `Infallible`
    if is_other {
        error_type = Path::from_input(quote! { #protocol_path::common::ResourceKeyError }).unwrap();
        let other_variant = other_variant.unwrap();

        from_key_tokens.extend(quote! {
            other => Ok(#enum_ident::#other_variant(other.try_into()?)),
        });
    } else {
        from_key_tokens.extend(quote! {
            other => Err(other.into()),
        });
    }

    // Create a registry name
    let registry_name = Ident::new(&format!("{enum_ident}Registry"), Span::call_site());

    quote! {
        type #registry_name = #crate_path::definitions::SimpleRegistry<#enum_ident>;
        impl #crate_path::definitions::ConvertKey for #enum_ident {
            type Error = #error_type;
            fn from_key(key: &(impl ::core::convert::AsRef<str> + ?::core::marker::Sized)) -> Result<Self, Self::Error> {
                match key.as_ref() {
                    #from_key_tokens
                }
            }
            fn to_key_str(&self) -> &str {
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
