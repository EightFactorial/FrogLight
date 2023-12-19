use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::proto::{decode::read_fields, get_discriminant};

/// Decode an enum
pub(super) fn decode_enum(input: &DeriveInput) -> TokenStream {
    let DeriveInput {
        ident,
        data: Data::Enum(data),
        ..
    } = input
    else {
        panic!("Expected enum");
    };

    let mut variants = Vec::new();
    let mut discriminant = 0;

    // Decode each variant
    for variant in &data.variants {
        let disc = get_discriminant(&variant.discriminant, &mut discriminant);
        let variant_ident = &variant.ident;

        // Get a list of fields
        let mut field_list = Vec::new();
        read_fields(&variant.fields, &mut field_list);

        // Generate the decode method
        let decode_method = match &variant.fields {
            Fields::Named(_) => {
                quote! {
                    Ok(Self::#variant_ident {
                        #(#field_list)*
                    })
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    Ok(Self::#variant_ident(
                        #(#field_list)*
                    ))
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(Self::#variant_ident)
                }
            }
        };

        variants.push(quote! {
            #disc => #decode_method,
        });
    }

    // Finish the impl
    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                match crate::buffer::VarDecode::var_decode(buf)? {
                    #(#variants)*
                    id => Err(crate::buffer::DecodeError::InvalidEnumId(id)),
                }
            }
        }
    }
}
