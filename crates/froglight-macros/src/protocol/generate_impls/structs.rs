use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index};

use super::is_variable;

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput) -> proc_macro::TokenStream {
    let crate_path = crate::protocol::get_protocol_path();

    let struct_ident = &input.ident;
    let Data::Struct(data) = &input.data else {
        unreachable!("Struct generator called on non-struct type");
    };

    let mut read_tokens = TokenStream::new();

    #[cfg(feature = "froglight-protocol-debug")]
    {
        let name = struct_ident.to_string();
        read_tokens.extend(quote! {
            #[cfg(all(debug_assertions, feature = "bevy"))]
            {
                let buf_pos = buf.position() as usize;

                let mut buf_ref = &buf.get_ref()[buf_pos..];
                let buf_len = buf_ref.len();

                if buf_len > 64 {
                    buf_ref = &buf_ref[..64];
                }

                bevy_log::trace!(concat!("Reading Struct \"", #name, "\": {} bytes, {:?}"), buf_len, buf_ref);
            }
        });
    }

    match &data.fields {
        Fields::Named(fields) => {
            // Collect tokens for reading each field
            let mut field_tokens = TokenStream::new();
            for field in &fields.named {
                let field_ident = field.ident.as_ref().unwrap();

                if is_variable(&field.attrs) {
                    // Read the field using `FrogVarRead`
                    field_tokens.extend(quote! {
                        #field_ident: #crate_path::protocol::FrogVarRead::fg_var_read(buf)?,
                    });
                } else {
                    // Read the field using `FrogRead`
                    field_tokens.extend(quote! {
                        #field_ident: #crate_path::protocol::FrogRead::fg_read(buf)?,
                    });
                }
            }

            // Create the struct
            read_tokens.extend(quote! {
                Ok(Self {
                    #field_tokens
                })
            });
        }
        Fields::Unnamed(fields) => {
            // Collect tokens for reading each field
            let mut field_tokens = TokenStream::new();
            for field in &fields.unnamed {
                if is_variable(&field.attrs) {
                    // Read the field using `FrogVarRead`
                    field_tokens.extend(quote! {
                        #crate_path::protocol::FrogVarRead::fg_var_read(buf)?,
                    });
                } else {
                    // Read the field using `FrogRead`
                    field_tokens.extend(quote! {
                        #crate_path::protocol::FrogRead::fg_read(buf)?,
                    });
                }
            }

            // Create the struct
            read_tokens.extend(quote! {
                Ok(Self(
                    #field_tokens
                ))
            });
        }
        // Do nothing for unit structs
        Fields::Unit => {
            read_tokens.extend(quote! {
                Ok(Self)
            });
        }
    }

    quote! {
        impl #crate_path::protocol::FrogRead for #struct_ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, #crate_path::protocol::ReadError>
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
    let crate_path = crate::protocol::get_protocol_path();

    let struct_ident = &input.ident;
    let Data::Struct(data) = &input.data else {
        unreachable!("Struct generator called on non-struct type");
    };

    let mut write_tokens = TokenStream::new();
    match &data.fields {
        Fields::Named(fields) => {
            // Write each field
            for field in &fields.named {
                let field_ident = field.ident.as_ref().unwrap();

                if is_variable(&field.attrs) {
                    // Write the field using `FrogVarWrite`
                    write_tokens.extend(quote! {
                        #crate_path::protocol::FrogVarWrite::fg_var_write(&self.#field_ident, buf)?;
                    });
                } else {
                    // Write the field using `FrogWrite`
                    write_tokens.extend(quote! {
                        #crate_path::protocol::FrogWrite::fg_write(&self.#field_ident, buf)?;
                    });
                }
            }
        }
        Fields::Unnamed(fields) => {
            // Write each field
            for (index, field) in fields.unnamed.iter().enumerate() {
                let index_indent = Index::from(index);

                if is_variable(&field.attrs) {
                    // Write the field using `FrogVarWrite`
                    write_tokens.extend(quote! {
                        #crate_path::protocol::FrogVarWrite::fg_var_write(&self.#index_indent, buf)?;
                    });
                } else {
                    // Write the field using `FrogWrite`
                    write_tokens.extend(quote! {
                        #crate_path::protocol::FrogWrite::fg_write(&self.#index_indent, buf)?;
                    });
                }
            }
        }
        // Do nothing for unit structs
        Fields::Unit => {}
    }

    // Emit an Ok(())
    write_tokens.extend(quote! {
        Ok(())
    });

    quote! {
        impl #crate_path::protocol::FrogWrite for #struct_ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), #crate_path::protocol::WriteError> {
                #write_tokens
            }
        }
    }.into()
}
