use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index};

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput) -> proc_macro::TokenStream {
    let struct_ident = &input.ident;
    let Data::Struct(data) = &input.data else {
        unreachable!("Bitset items must be structs");
    };

    let mut read_tokens = TokenStream::new();
    match &data.fields {
        Fields::Named(fields) => {
            let field_count = fields.named.len();

            // Read the bitset from the buffer
            read_tokens.extend(quote! {
                let bitset: ::froglight::common::BitSet::<#field_count> = ::froglight::protocol::FrogRead::fg_read(buf)?;
            });

            // Collect tokens for reading each field
            let mut field_tokens = TokenStream::new();
            for (index, field) in fields.named.iter().enumerate() {
                let field_ident = field.ident.as_ref().unwrap();
                field_tokens.extend(quote! {
                    #field_ident: bitset.get_bit(#index).expect("Failed to get bit"),
                });
            }

            // Create the struct
            read_tokens.extend(quote! {
                Ok(Self {
                    #field_tokens
                })
            });
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();

            // Read the bitset from the buffer
            read_tokens.extend(quote! {
                let bitset: ::froglight::common::BitSet::<#field_count> = ::froglight::protocol::FrogRead::fg_read(buf)?;
            });

            // Collect tokens for reading each field
            let mut field_tokens = TokenStream::new();
            for index in 0..field_count {
                field_tokens.extend(quote! {
                    bitset.get_bit(#index).expect("Failed to get bit"),
                });
            }

            // Create the struct
            read_tokens.extend(quote! {
                Ok(Self(
                    #field_tokens
                ))
            });
        }
        Fields::Unit => panic!("Cannot generate a BitSet read for a unit struct"),
    }

    quote! {
        impl ::froglight::protocol::FrogRead for #struct_ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ::froglight::protocol::ReadError>
            where
                Self: Sized,
            {
                #read_tokens
            }
        }
    }
    .into()
}

/// Generate a `FrogWrite` implementation.
pub(super) fn generate_write(input: &DeriveInput) -> proc_macro::TokenStream {
    let struct_ident = &input.ident;
    let Data::Struct(data) = &input.data else {
        unreachable!("Bitset items must be structs");
    };

    let mut write_tokens = TokenStream::new();
    match &data.fields {
        Fields::Named(fields) => {
            let field_count = fields.named.len();

            // Create the bitset
            write_tokens.extend(quote! {
                let mut bitset = ::froglight::common::BitSet::<#field_count>::new();
            });

            // Collect tokens for writing each field
            let mut field_tokens = TokenStream::new();
            for (index, field) in fields.named.iter().enumerate() {
                let field_ident = field.ident.as_ref().unwrap();
                field_tokens.extend(quote! {
                    bitset.set_bit(#index, self.#field_ident).expect("Failed to set bit");
                });
            }

            // Write the bitset to the buffer
            write_tokens.extend(quote! {
                #field_tokens
                bitset.fg_write(buf)
            });
        }
        Fields::Unnamed(fields) => {
            let field_count = fields.unnamed.len();

            // Create the bitset
            write_tokens.extend(quote! {
                let mut bitset = ::froglight::common::BitSet::<#field_count>::new();
            });

            // Collect tokens for writing each field
            let mut field_tokens = TokenStream::new();
            for index in 0..field_count {
                let index_ident = Index::from(index);
                field_tokens.extend(quote! {
                    bitset.set_bit(#index, self.#index_ident).expect("Failed to set bit");
                });
            }

            // Write the bitset to the buffer
            write_tokens.extend(quote! {
                #field_tokens
                bitset.fg_write(buf)
            });
        }
        Fields::Unit => panic!("Cannot generate a BitSet write for a unit struct"),
    }

    quote! {
        impl ::froglight::protocol::FrogWrite for #struct_ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), ::froglight::protocol::WriteError> {
                #write_tokens
            }
        }
    }
    .into()
}
