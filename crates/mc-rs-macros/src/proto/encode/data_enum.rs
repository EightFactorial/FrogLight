use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta};

use crate::proto::{encode::read_fields, get_discriminant};

/// Encode an enum
pub(super) fn encode_enum(input: &DeriveInput) -> TokenStream {
    let DeriveInput {
        ident,
        data: Data::Enum(data),
        ..
    } = input
    else {
        panic!("Expected enum");
    };

    let mut variant_list = Vec::new();
    let mut discriminant = 0;

    // Encode each variant
    for variant in data.variants.iter() {
        let disc = get_discriminant(&variant.discriminant, &mut discriminant);
        let ident = &variant.ident;

        // Encode each variant field
        match &variant.fields {
            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| &f.ident);

                let mut field_list = Vec::new();
                read_fields(&variant.fields, &mut field_list);

                variant_list.push(quote! {
                    Self::#ident { #(#names,)* } => {
                        crate::buffer::VarEncode::var_encode(&#disc, buf)?;
                        #(#field_list)*
                    }
                });
            }
            Fields::Unnamed(fields) => {
                let names =
                    (0..fields.unnamed.len()).map(|i| Ident::new(&format!("f{}", i), ident.span()));

                let mut field_list = Vec::new();
                for (field, name) in fields.unnamed.iter().zip(names.clone()) {
                    if field.attrs.iter().any(|f| {
                        if let Meta::Path(path) = &f.meta {
                            path.is_ident("var")
                        } else {
                            false
                        }
                    }) {
                        field_list.push(quote! {
                            crate::buffer::VarEncode::var_encode(#name, buf)?;
                        });
                    } else {
                        field_list.push(quote! {
                            crate::buffer::Encode::encode(#name, buf)?;
                        });
                    }
                }

                variant_list.push(quote! {
                    Self::#ident( #(#names,)* ) => {
                        crate::buffer::VarEncode::var_encode(&#disc, buf)?;
                        #(#field_list)*
                    }
                });
            }
            Fields::Unit => {
                variant_list.push(quote! {
                    Self::#ident => {
                        crate::buffer::VarEncode::var_encode(&#disc, buf)?;
                    }
                });
            }
        }
    }

    // Finish the impl
    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                match self {
                    #(#variant_list,)*
                }
                Ok(())
            }
        }
    }
}
