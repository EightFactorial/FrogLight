use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DeriveInput, Fields, Ident, Variant};

use crate::protocol::Attributes;

pub(super) fn write_enum(input: &DeriveInput, _: &Attributes) -> TokenStream {
    let DeriveInput { ident, data, .. } = input;
    let Data::Enum(DataEnum { variants, .. }) = &data else {
        unreachable!("Only enums are supported")
    };

    let mut discriminant = 0i32;

    let mut variant_tokens = TokenStream::new();
    for variant in variants {
        let Variant { ident: var_ident, fields, discriminant: var_discriminant, .. } = variant;

        // If the variant has a discriminant, use that
        super::set_discriminant(var_discriminant, &mut discriminant);

        match fields {
            Fields::Named(fields) => {
                variant_tokens.extend(write_named(discriminant, var_ident, fields));
            }
            Fields::Unnamed(fields) => {
                variant_tokens.extend(write_unnamed(discriminant, var_ident, fields));
            }
            Fields::Unit => variant_tokens.extend(quote! {
                Self::#var_ident => <i32 as crate::io::FrogVarWrite>::fg_var_write(&#discriminant, buf)?,
            }),
        }

        discriminant += 1;
    }

    quote! {
        impl crate::io::FrogWrite for #ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                match self {
                    #variant_tokens
                }
                Ok(())
            }
        }
    }
}

/// Write the named fields of an enum variant
fn write_named(
    discriminant: i32,
    var_ident: &syn::Ident,
    fields: &syn::FieldsNamed,
) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    let field_idents: Vec<&Ident> =
        fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    for field in &fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_attrs = field.attrs.as_slice();

        // Write the field normally, or write as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                <#field_type as crate::io::FrogVarWrite>::fg_var_write(#field_ident, buf)?;
            });
        } else {
            field_tokens.extend(quote! {
                <#field_type as crate::io::FrogWrite>::fg_write(#field_ident, buf)?;
            });
        }
    }

    quote! {
        Self::#var_ident { #(#field_idents),* }=> {
            <i32 as crate::io::FrogVarWrite>::fg_var_write(&#discriminant, buf)?;
            #field_tokens
        }
    }
}

/// Write the unnamed fields of an enum variant
fn write_unnamed(
    discriminant: i32,
    var_ident: &syn::Ident,
    fields: &syn::FieldsUnnamed,
) -> TokenStream {
    let mut field_tokens = TokenStream::new();
    let mut variant_tokens = TokenStream::new();

    for (i, field) in fields.unnamed.iter().enumerate() {
        let field_attrs = field.attrs.as_slice();

        let field_name = Ident::new(&format!("field_{i}"), field.span());
        variant_tokens.extend(quote!(#field_name,));

        // Write the field normally, or write as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                crate::io::FrogVarWrite::fg_var_write(#field_name, buf)?;
            });
        } else {
            field_tokens.extend(quote! {
                crate::io::FrogWrite::fg_write(#field_name, buf)?;
            });
        }
    }

    quote! {
        Self::#var_ident(#variant_tokens) => {
            <i32 as crate::io::FrogVarWrite>::fg_var_write(&#discriminant, buf)?;
            #field_tokens
        }
    }
}
