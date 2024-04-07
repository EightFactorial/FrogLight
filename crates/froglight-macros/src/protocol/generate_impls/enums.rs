use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields};

use super::{is_variable, set_discriminant};

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput) -> proc_macro::TokenStream {
    let enum_ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        unreachable!("Enum generator called on non-enum type");
    };

    let mut read_tokens = TokenStream::new();

    let mut discriminant = 0i32;
    for variant in &data.variants {
        let variant_ident = &variant.ident;

        // Update the discriminant if the variant has one.
        set_discriminant(&variant.discriminant, &mut discriminant);

        // Collect the tokens for the variant
        let variant_tokens = match &variant.fields {
            Fields::Named(fields) => {
                // Collect tokens for reading each field
                let mut field_tokens = TokenStream::new();
                for field in &fields.named {
                    let field_ident = field.ident.as_ref().unwrap();

                    if is_variable(&field.attrs) {
                        // Read the field using `FrogVarRead`
                        field_tokens.extend(quote! {
                            #field_ident: ::froglight::protocol::FrogVarRead::fg_var_read(buf)?,
                        });
                    } else {
                        // Read the field using `FrogRead`
                        field_tokens.extend(quote! {
                            #field_ident: ::froglight::protocol::FrogRead::fg_read(buf)?,
                        });
                    }
                }

                // Create the enum variant
                quote! {
                    Ok(Self::#variant_ident {
                        #field_tokens
                    })
                }
            }
            Fields::Unnamed(fields) => {
                // Collect tokens for reading each field
                let mut field_tokens = TokenStream::new();
                for field in &fields.unnamed {
                    if is_variable(&field.attrs) {
                        // Read the field using `FrogVarRead`
                        field_tokens.extend(quote! {
                            ::froglight::protocol::FrogVarRead::fg_var_read(buf)?,
                        });
                    } else {
                        // Read the field using `FrogRead`
                        field_tokens.extend(quote! {
                            ::froglight::protocol::FrogRead::fg_read(buf)?,
                        });
                    }
                }

                // Create the enum variant
                quote! {
                    Ok(Self::#variant_ident(
                        #field_tokens
                    ))
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(Self::#variant_ident)
                }
            }
        };

        // Add the variant tokens to the read tokens
        read_tokens.extend(quote! {
            #discriminant => #variant_tokens,
        });

        // Increment the discriminant
        discriminant += 1;
    }

    quote! {
        impl ::froglight::protocol::FrogRead for #enum_ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ::froglight::protocol::ReadError>
            where
                Self: Sized,
            {
                match ::froglight::protocol::FrogVarRead::fg_var_read(buf)? {
                    #read_tokens
                    unk => Err(::froglight::protocol::ReadError::InvalidEnum(unk, std::any::type_name::<Self>())),
                }
            }
        }
    }
    .into()
}

pub(super) fn generate_write(input: &DeriveInput) -> proc_macro::TokenStream {
    let enum_ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        unreachable!("Enum generator called on non-enum type");
    };

    // Collect tokens for writing each variant
    let mut write_tokens = TokenStream::new();

    let mut discriminant = 0i32;
    for variant in &data.variants {
        let variant_ident = &variant.ident;

        // Update the discriminant if the variant has one.
        set_discriminant(&variant.discriminant, &mut discriminant);

        match &variant.fields {
            Fields::Named(fields) => {
                // Collect tokens for naming each field
                let mut field_ident_tokens = TokenStream::new();

                // Collect tokens for writing each field
                let mut field_tokens = TokenStream::new();

                for field in &fields.named {
                    let field_ident = field.ident.as_ref().unwrap();

                    // Add the field to the field ident tokens
                    field_ident_tokens.extend(quote! {
                        #field_ident,
                    });

                    // Add tokens for writing the field
                    if is_variable(&field.attrs) {
                        // Write the field using `FrogVarWrite`
                        field_tokens.extend(quote! {
                            ::froglight::protocol::FrogVarWrite::fg_var_write(#field_ident, buf)?;
                        });
                    } else {
                        // Write the field using `FrogWrite`
                        field_tokens.extend(quote! {
                            ::froglight::protocol::FrogWrite::fg_write(#field_ident, buf)?;
                        });
                    }
                }

                // Add the variant to the write tokens
                write_tokens.extend(quote! {
                    Self::#variant_ident { #field_ident_tokens } => {
                        ::froglight::protocol::FrogVarWrite::fg_var_write(&#discriminant, buf)?;
                        #field_tokens
                    }
                });
            }
            Fields::Unnamed(fields) => {
                // Collect tokens for naming each field
                let mut field_ident_tokens = TokenStream::new();

                // Collect tokens for writing each field
                let mut field_tokens = TokenStream::new();

                for (i, field) in fields.unnamed.iter().enumerate() {
                    let field_ident = syn::Ident::new(&format!("field_{}", i), field.span());

                    // Add the field to the field ident tokens
                    field_ident_tokens.extend(quote! {
                        #field_ident,
                    });

                    // Add tokens for writing the field
                    if is_variable(&field.attrs) {
                        // Write the field using `FrogVarWrite`
                        field_tokens.extend(quote! {
                            ::froglight::protocol::FrogVarWrite::fg_var_write(#field_ident, buf)?;
                        });
                    } else {
                        // Write the field using `FrogWrite`
                        field_tokens.extend(quote! {
                            ::froglight::protocol::FrogWrite::fg_write(#field_ident, buf)?;
                        });
                    }
                }

                // Add the variant to the write tokens
                write_tokens.extend(quote! {
                    Self::#variant_ident(#field_ident_tokens) => {
                        ::froglight::protocol::FrogVarWrite::fg_var_write(&#discriminant, buf)?;
                        #field_tokens
                    }
                });
            }
            Fields::Unit => {
                // Write only the discriminant
                write_tokens.extend(quote! {
                    Self::#variant_ident => {
                        ::froglight::protocol::FrogVarWrite::fg_var_write(&#discriminant, buf)?;
                    }
                });
            }
        }

        // Increment the discriminant
        discriminant += 1;
    }

    quote! {
        impl ::froglight::protocol::FrogWrite for #enum_ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), ::froglight::protocol::WriteError> {
                match self {
                    #write_tokens
                }
                Ok(())
            }
        }
    }.into()
}
