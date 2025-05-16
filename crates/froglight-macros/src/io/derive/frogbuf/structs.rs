use darling::FromField;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Path};

use super::FrogBufField;

pub(super) fn derive_struct(
    ident: Ident,
    DataStruct { fields, .. }: DataStruct,
    path: Path,
) -> TokenStream {
    // If `io-trace` is enabled, emit a trace message when reading the struct.
    let read_trace = if cfg!(feature = "io-trace") {
        let message = format!("Reading struct \"{ident}\"");
        quote! { #path::tracing::trace!(target: "froglight_io::read", #message); }
    } else {
        TokenStream::new()
    };
    // If `io-trace` is enabled, emit a trace message when writing the struct.
    let write_trace = if cfg!(feature = "io-trace") {
        let message = format!("Writing struct \"{ident}\"");
        quote! { #path::tracing::trace!(target: "froglight_io::write", #message); }
    } else {
        TokenStream::new()
    };

    let (read, write, length) = match &fields {
        Fields::Named(fields) => named_fields(fields, &path),
        Fields::Unnamed(fields) => unnamed_fields(fields, &path),
        // Return a specialized implementation for unit structs
        Fields::Unit => {
            return quote! {
                #[automatically_derived]
                impl #path::standard::FrogRead for #ident {
                    #[inline]
                    fn frog_read(_: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                        #read_trace
                        Ok(Self)
                    }
                }

                #[automatically_derived]
                impl #path::standard::FrogWrite for #ident {
                    #[inline]
                    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                        #write_trace
                        Ok(0)
                    }
                    #[inline]
                    fn frog_len(&self) -> usize { 0 }
                }
            };
        }
    };

    quote! {
        #[automatically_derived]
        impl #path::standard::FrogRead for #ident {
            fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                #read_trace
                Ok(Self #read)
            }
        }

        #[automatically_derived]
        impl #path::standard::FrogWrite for #ident {
            fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                #write_trace
                let mut written = 0;
                #write
                Ok(written)
            }

            fn frog_len(&self) -> usize {
                let mut length = 0;
                #length
                length
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate the `read`, `write`, and `length` functions
/// for a struct with named fields.
fn named_fields(fields: &FieldsNamed, path: &Path) -> (TokenStream, TokenStream, TokenStream) {
    let (mut read, mut write, mut length) =
        (TokenStream::new(), TokenStream::new(), TokenStream::new());

    for field in &fields.named {
        let FrogBufField { var, json } = FrogBufField::from_field(field).unwrap();
        let ident = field.ident.as_ref().unwrap();

        if json.is_present() {
            read.extend(quote! {
                #ident: #path::serde::FrogJson::frog_from_json(buffer)?,
            });

            write.extend(quote! {
                written += #path::serde::FrogJson::frog_to_json(&self.#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::serde::FrogJson::frog_json_len(&self.#ident);
            });
        } else if var.is_present() {
            read.extend(quote! {
                #ident: #path::variable::FrogVarRead::frog_var_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::variable::FrogVarWrite::frog_var_write(&self.#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::variable::FrogVarWrite::frog_var_len(&self.#ident);
            });
        } else {
            read.extend(quote! {
                #ident: #path::standard::FrogRead::frog_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::standard::FrogWrite::frog_write(&self.#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::standard::FrogWrite::frog_len(&self.#ident);
            });
        }
    }

    (quote!({ #read }), write, length)
}

// -------------------------------------------------------------------------------------------------

/// Generate the `read`, `write`, and `length` functions
/// for a struct with unnamed fields.
fn unnamed_fields(fields: &FieldsUnnamed, path: &Path) -> (TokenStream, TokenStream, TokenStream) {
    let (mut read, mut write, mut length) =
        (TokenStream::new(), TokenStream::new(), TokenStream::new());

    for (index, field) in fields.unnamed.iter().enumerate() {
        let FrogBufField { var, json } = FrogBufField::from_field(field).unwrap();
        let index = Index::from(index);

        if json.is_present() {
            read.extend(quote! {
                #path::serde::FrogJson::frog_from_json(buffer)?,
            });

            write.extend(quote! {
                written += #path::serde::FrogJson::frog_to_json(self.#index, buffer)?;
            });
            length.extend(quote! {
                length += #path::serde::FrogJson::frog_json_len(&self.#index);
            });
        } else if var.is_present() {
            read.extend(quote! {
                #path::variable::FrogVarRead::frog_var_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::variable::FrogVarWrite::frog_var_write(self.#index, buffer)?;
            });
            length.extend(quote! {
                length += #path::variable::FrogVarWrite::frog_var_len(&self.#index);
            });
        } else {
            read.extend(quote! {
                 #path::standard::FrogRead::frog_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::standard::FrogWrite::frog_write(&self.#index, buffer)?;
            });
            length.extend(quote! {
                length += #path::standard::FrogWrite::frog_len(&self.#index);
            });
        }
    }

    (quote!(( #read )), write, length)
}
