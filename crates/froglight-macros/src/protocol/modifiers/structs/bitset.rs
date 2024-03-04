use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::protocol::Attributes;

/// Generate a `FrogRead` implementation for the given struct.
pub(crate) fn read_as_bitset(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    let syn::Data::Struct(data) = &input.data else { panic!("Only structs can be bitsets") };

    let name = &input.ident;
    let size = attrs.bitset.expect("Bitset size is required");

    match &data.fields {
        syn::Fields::Named(fields) => {
            // Read the bitset from the buffer
            let mut tokens = quote! {
                let bitset: crate::common::BitSet::<#size> = crate::io::FrogRead::fg_read(buf)?;
            };

            // Get the field idents
            let field_idents =
                fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();

            // Get the fields from the bitset
            for (index, &field) in field_idents.iter().enumerate() {
                tokens.extend(quote! {
                    let #field = bitset.get_bit(#index).expect("BitSet size is too small");
                });
            }

            // Return the struct
            tokens.extend(quote! {
                Ok(Self {
                    #(
                        #field_idents,
                    )*
                })
            });

            // Generate the impl
            generate_read(name, tokens)
        }
        syn::Fields::Unnamed(fields) => {
            // Read the bitset from the buffer
            let mut tokens = quote! {
                let bitset: crate::common::BitSet::<#size> = crate::io::FrogRead::fg_read(buf)?;
            };

            // Create identifiers for the fields
            let field_idents = (0..fields.unnamed.len())
                .map(|index| Ident::new(&format!("field_{index}"), Span::call_site()))
                .collect::<Vec<_>>();

            // Get the fields from the bitset
            for (index, field) in field_idents.iter().enumerate() {
                tokens.extend(quote! {
                    let #field = bitset.get_bit(#index).expect("BitSet size is too small");
                });
            }

            // Return the struct
            tokens.extend(quote! {
                Ok(Self(
                    #(
                        #field_idents,
                    )*
                ))
            });

            // Generate the impl
            generate_read(name, tokens)
        }
        syn::Fields::Unit => panic!("Unit structs cannot be bitsets"),
    }
}

/// Generate a `FrogWrite` implementation for the given struct.
pub(crate) fn write_as_bitset(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    let syn::Data::Struct(data) = &input.data else { panic!("Only structs can be bitsets") };

    let name = &input.ident;
    let size = attrs.bitset.expect("Bitset size is required");

    match &data.fields {
        syn::Fields::Named(fields) => {
            // Create the bitset
            let mut tokens = quote! {
                let mut bitset = crate::common::BitSet::<#size>::new();
            };

            // Get the field idents
            let field_idents =
                fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();

            // Set the fields in the bitset
            for (index, &field) in field_idents.iter().enumerate() {
                tokens.extend(quote! {
                    bitset.set_bit(#index, self.#field).expect("BitSet size is too small");
                });
            }

            // Write the bitset to the buffer
            tokens.extend(quote! {
                bitset.fg_write(buf)
            });

            // Generate the impl
            generate_write(name, tokens)
        }
        syn::Fields::Unnamed(fields) => {
            // Create identifiers for the fields
            let field_idents = (0..fields.unnamed.len())
                .map(|index| Ident::new(&format!("field_{index}"), Span::call_site()))
                .collect::<Vec<_>>();

            // Split the fields into individual variables
            let mut tokens = quote! {
                let Self(#(
                    #field_idents,
                )*) = *self;
            };

            // Create the bitset
            tokens.extend(quote! {
                let mut bitset = crate::common::BitSet::<#size>::from([#(
                    #field_idents,
                )*]);
            });

            // Write the bitset to the buffer
            tokens.extend(quote! {
                bitset.fg_write(buf)
            });

            // Generate the impl
            generate_write(name, tokens)
        }
        syn::Fields::Unit => panic!("Unit structs cannot be bitsets"),
    }
}

// ---- Helper functions ----

/// Wrap the collected tokens in a `FrogRead` impl.
fn generate_read(name: &Ident, tokens: TokenStream) -> TokenStream {
    quote! {
        impl crate::io::FrogRead for #name {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
            where
                Self: Sized,
            {
                #tokens
            }
        }
    }
}

/// Wrap the collected tokens in a `FrogWrite` impl.
fn generate_write(name: &Ident, tokens: TokenStream) -> TokenStream {
    quote! {
        impl crate::io::FrogWrite for #name {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                #tokens
            }
        }
    }
}
