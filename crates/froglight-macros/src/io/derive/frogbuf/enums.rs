use darling::FromField;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed, Ident, LitInt, Path, Variant};

use super::FrogBufField;
use crate::{CrateManifest, io::derive::frogbuf::FrogBufMacro};

pub(super) fn derive_enum(
    ident: Ident,
    DataEnum { variants, .. }: DataEnum,
    flags: FrogBufMacro,
    path: Path,
) -> TokenStream {
    let (mut read_tokens, mut write_tokens, mut length_tokens) =
        (TokenStream::new(), TokenStream::new(), TokenStream::new());
    let mut discriminant = 0i32;

    let common = CrateManifest::froglight("froglight-common");
    let (read_trait, write_trait, generics) = if flags.version.is_present() {
        (
            quote!(#path::version::FrogReadVersion<V>),
            quote!(#path::version::FrogWriteVersion<V>),
            quote!(<V: #common::version::Version>),
        )
    } else {
        (quote!(#path::standard::FrogRead), quote!(#path::standard::FrogWrite), quote!())
    };

    // Return an unreachable error if the enum has no variants
    if variants.is_empty() {
        let message =
            format!("Enum \"{ident}\" has no variants, this code should never be reached!");
        return quote! {
            #[automatically_derived]
            impl #generics #read_trait for #ident {
                fn frog_read(_: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> { unreachable!(#message) }
            }

            #[automatically_derived]
            impl #generics #write_trait for #ident {
                fn frog_write(&self, _: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> { unreachable!(#message) }
                fn frog_len(&self) -> usize { unreachable!(#message) }
            }
        };
    }

    for Variant { ident: variant, fields, discriminant: d, .. } in variants {
        // Update the discriminant if one is provided
        if let Some((_, d)) = d {
            let integer: LitInt = syn::parse_quote!(#d);
            discriminant = integer.base10_parse().unwrap();
        }

        // Generate the read and write tokens for the variant
        let (fields, read, write, length) = match &fields {
            Fields::Named(fields) => named_fields(fields, &flags, &path),
            Fields::Unnamed(fields) => unnamed_fields(fields, &flags, &path),
            Fields::Unit => (quote!(), quote!(), quote!(), quote!()),
        };

        // If `io-trace` is enabled, emit a trace message when reading the struct.
        let read_trace = if cfg!(feature = "io-trace") {
            let message = format!("Reading enum variant \"{ident}::{variant}\" ({discriminant})");
            quote! { #path::tracing::trace!(target: "froglight_io::read", #message); }
        } else {
            TokenStream::new()
        };
        // If `io-trace` is enabled, emit a trace message when writing the struct.
        let write_trace = if cfg!(feature = "io-trace") {
            let message = format!("Writing enum variant \"{ident}::{variant}\" ({discriminant})");
            quote! { #path::tracing::trace!(target: "froglight_io::write", #message); }
        } else {
            TokenStream::new()
        };

        // Add outer syntax and handle variant discriminants
        read_tokens.extend(quote! {
            #discriminant => {
                #read_trace
                Ok(Self::#variant #read)
            },
        });
        write_tokens.extend(quote! {
            Self::#variant #fields => {
                #write_trace
                written += #path::variable::FrogVarWrite::frog_var_write(&#discriminant, buffer)?;
                #write
            }
        });
        length_tokens.extend(quote! {
            Self::#variant #fields => {
                length += #path::variable::FrogVarWrite::frog_var_len(&#discriminant);
                #length
            }
        });

        discriminant += 1;
    }

    quote! {
        #[automatically_derived]
        impl #generics #read_trait for #ident {
            fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                match #path::variable::FrogVarRead::frog_var_read(buffer)? {
                    #read_tokens
                    unk => Err(#path::standard::ReadError::InvalidEnum(core::any::type_name::<Self>(), unk)),
                }
            }
        }

        #[automatically_derived]
        impl #generics #write_trait for #ident {
            fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                let mut written = 0;
                match self {
                    #write_tokens
                }
                Ok(written)
            }

            fn frog_len(&self) -> usize {
                let mut length = 0;
                match self {
                    #length_tokens
                }
                length
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate the input fields and the `read`, `write`, and `length` functions
/// for an enum with named fields.
fn named_fields(
    fields: &FieldsNamed,
    flags: &FrogBufMacro,
    path: &Path,
) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    let (mut fields_out, mut read, mut write, mut length) =
        (TokenStream::new(), TokenStream::new(), TokenStream::new(), TokenStream::new());

    for field in &fields.named {
        let FrogBufField { var, json } = FrogBufField::from_field(field).unwrap();
        let ident = field.ident.as_ref().unwrap();

        fields_out.extend(quote!(#ident,));

        if json.is_present() {
            read.extend(quote! {
                #ident: #path::json::FrogJson::frog_json(buffer)?,
            });

            write.extend(quote! {
                written += #ident.frog_json(buffer)?;
            });
            length.extend(quote! {
                length += #ident.frog_json_length();
            });
        } else if var.is_present() {
            read.extend(quote! {
                #ident: #path::variable::FrogVarRead::frog_var_read(buffer)?,
            });

            write.extend(quote! {
                written += #ident.frog_var_write(buffer)?;
            });
            length.extend(quote! {
                length += #ident.frog_var_length();
            });
        } else if flags.version.is_present() {
            read.extend(quote! {
                #ident: #path::version::FrogReadVersion::<V>::frog_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::version::FrogWriteVersion::<V>::frog_write(&self.#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::version::FrogWriteVersion::<V>::frog_length(&self.#ident);
            });
        } else {
            read.extend(quote! {
                 #ident: #path::standard::FrogRead::frog_read(buffer)?,
            });

            write.extend(quote! {
                written += #ident.frog_write(buffer)?;
            });
            length.extend(quote! {
                length += #ident.frog_length();
            });
        }
    }

    (quote!({ #fields_out }), quote!({ #read }), write, length)
}

// -------------------------------------------------------------------------------------------------

const FIELDS: &[&str; 36] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "aa", "ab", "ac", "ad", "ae", "af", "ag", "ah", "ai", "aj",
];

/// Generate the input fields and the `read`, `write`, and `length` functions
/// for an enum with unnamed fields.
fn unnamed_fields(
    fields: &FieldsUnnamed,
    flags: &FrogBufMacro,
    path: &Path,
) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    let (mut fields_out, mut read, mut write, mut length) =
        (TokenStream::new(), TokenStream::new(), TokenStream::new(), TokenStream::new());

    for (index, field) in fields.unnamed.iter().enumerate() {
        let FrogBufField { var, json } = FrogBufField::from_field(field).unwrap();
        let ident = Ident::new(FIELDS[index], Span::call_site());

        fields_out.extend(quote!(#ident,));

        if json.is_present() {
            read.extend(quote! {
                #path::serde::FrogJson::frog_from_json(buffer)?,
            });

            write.extend(quote! {
                written += #path::serde::FrogJson::frog_to_json(#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::serde::FrogJson::frog_json_len(#ident);
            });
        } else if var.is_present() {
            read.extend(quote! {
                #path::variable::FrogVarRead::frog_var_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::variable::FrogVarWrite::frog_var_write(#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::variable::FrogVarWrite::frog_var_len(#ident);
            });
        } else if flags.version.is_present() {
            read.extend(quote! {
                #path::version::FrogReadVersion::<V>::frog_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::version::FrogWriteVersion::<V>::frog_write(#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::version::FrogWriteVersion::<V>::frog_len(#ident);
            });
        } else {
            read.extend(quote! {
                 #path::standard::FrogRead::frog_read(buffer)?,
            });

            write.extend(quote! {
                written += #path::standard::FrogWrite::frog_write(#ident, buffer)?;
            });
            length.extend(quote! {
                length += #path::standard::FrogWrite::frog_len(#ident);
            });
        }
    }

    (quote!( (#fields_out) ), quote!( (#read) ), write, length)
}
