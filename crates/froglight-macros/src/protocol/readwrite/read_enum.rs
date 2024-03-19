use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Variant};

use crate::protocol::Attributes;

/// Generate the `FrogRead` implementation for an enum
pub(super) fn read_enum(input: &DeriveInput, _: &Attributes) -> TokenStream {
    let DeriveInput { ident, data, .. } = input;
    let Data::Enum(DataEnum { variants, .. }) = &data else {
        unreachable!("Only enums are supported")
    };

    let mut discriminant = 0i32;
    let ident_string = ident.to_string();

    let mut variant_tokens = TokenStream::new();
    for variant in variants {
        let Variant { ident: var_ident, fields, discriminant: var_discriminant, .. } = variant;

        // If the variant has a discriminant, use that
        super::set_discriminant(var_discriminant, &mut discriminant);

        match fields {
            Fields::Named(fields) => {
                variant_tokens.extend(read_named(discriminant, var_ident, fields));
            }
            Fields::Unnamed(fields) => {
                variant_tokens.extend(read_unnamed(discriminant, var_ident, fields));
            }
            Fields::Unit => variant_tokens.extend(quote! {
                #discriminant => Ok(Self::#var_ident),
            }),
        }

        discriminant += 1;
    }

    quote! {
        impl crate::io::FrogRead for #ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError> {
                match <i32 as crate::io::FrogVarRead>::fg_var_read(buf)? {
                    #variant_tokens
                    unk => Err(crate::io::ReadError::InvalidEnum(unk, #ident_string)),
                }
            }
        }
    }
}

/// Read the named fields of an enum variant
fn read_named(discriminant: i32, var_ident: &Ident, fields: &syn::FieldsNamed) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    for field in &fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        let field_attrs = field.attrs.as_slice();

        // Read the field normally, or read as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                #field_ident: crate::io::FrogVarRead::fg_var_read(buf)?,
            });
        } else {
            field_tokens.extend(quote! {
                #field_ident: crate::io::FrogRead::fg_read(buf)?,
            });
        }
    }

    quote! {
        #discriminant => Ok(Self::#var_ident { #field_tokens }),
    }
}

/// Read the unnamed fields of an enum variant
fn read_unnamed(discriminant: i32, var_ident: &Ident, fields: &syn::FieldsUnnamed) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    for field in &fields.unnamed {
        let field_attrs = field.attrs.as_slice();

        // Read the field normally, or read as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                crate::io::FrogVarRead::fg_var_read(buf)?,
            });
        } else {
            field_tokens.extend(quote! {
                crate::io::FrogRead::fg_read(buf)?,
            });
        }
    }

    if fields.unnamed.len() == 1 {
        // Read as one field
        quote! {
            #discriminant => Ok(Self::#var_ident(#field_tokens)),
        }
    } else {
        // Read as a tuple
        quote! {
            #discriminant => Ok(Self::#var_ident(#field_tokens)),
        }
    }
}
