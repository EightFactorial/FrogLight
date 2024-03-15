use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use crate::protocol::Attributes;

pub(super) fn write_struct(input: &DeriveInput, _: &Attributes) -> TokenStream {
    let DeriveInput { ident, data, .. } = input;
    let Data::Struct(DataStruct { fields, .. }) = &data else {
        unreachable!("Only structs are supported")
    };

    match fields {
        Fields::Named(fields) => {
            let tokens = write_named(fields);
            quote! {
                impl crate::io::FrogWrite for #ident {
                    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                        #tokens
                        Ok(())
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let tokens = write_unnamed(fields);
            quote! {
                impl crate::io::FrogWrite for #ident {
                    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                        #tokens
                        Ok(())
                    }
                }
            }
        }
        Fields::Unit => quote! {
            impl crate::io::FrogWrite for #ident {
                fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                    Ok(())
                }
            }
        },
    }
}

/// Write the fields of a named struct
fn write_named(fields: &FieldsNamed) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    for field in &fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        let field_attrs = field.attrs.as_slice();

        // Write the field normally, or write as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                crate::io::FrogVarWrite::fg_var_write(&self.#field_ident, buf)?;
            });
        } else {
            field_tokens.extend(quote! {
                crate::io::FrogWrite::fg_write(&self.#field_ident, buf)?;
            });
        }
    }

    field_tokens
}

/// Write the fields of an unnamed struct
fn write_unnamed(fields: &FieldsUnnamed) -> TokenStream {
    let mut field_tokens = TokenStream::new();

    for (i, field) in fields.unnamed.iter().enumerate() {
        let field_ident = syn::Index::from(i);
        let field_attrs = field.attrs.as_slice();

        // Write the field normally, or write as a variable length field
        if super::is_variable(field_attrs) {
            field_tokens.extend(quote! {
                crate::io::FrogVarWrite::fg_var_write(&self.#field_ident, buf)?;
            });
        } else {
            field_tokens.extend(quote! {
                crate::io::FrogWrite::fg_write(&self.#field_ident, buf)?;
            });
        }
    }

    field_tokens
}
