use darling::{FromDeriveInput, FromMeta, util::Flag};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Path,
    Variant, punctuated::Punctuated, token::Comma,
};

use crate::CrateManifest;

#[derive(FromDeriveInput)]
#[darling(attributes(frog))]
struct FrogBufMacro {
    #[darling(default, rename = "proto")]
    traits: FrogBufTraits,
}

/// Which traits to implement for the struct or enum.
#[derive(FromMeta)]
struct FrogBufTraits {
    /// Whether to implement `FrogRead` and `FrogWrite`.
    std: Flag,
    /// Whether to implement `FrogVarRead` and `FrogVarWrite`.
    var: Flag,
}
/// Default to implementing `FrogRead` and `FrogWrite`,
/// otherwise follow the `proto` attribute.
impl Default for FrogBufTraits {
    fn default() -> Self { Self { std: Flag::from(true), var: Flag::from(false) } }
}

/// Whether to read or write from the buffer.
#[derive(Debug, Clone, Copy)]
enum ReadWriteMode {
    Read,
    Write,
    WriteLength,
}

/// Whether to use standard or variable length encoding.
#[derive(Debug, Clone, Copy)]
enum TraitMethod {
    Standard,
    Variable,
}

// -------------------------------------------------------------------------------------------------

pub(crate) fn derive_frogbuf(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let FrogBufMacro { traits } = FrogBufMacro::from_derive_input(&input).unwrap();
    let DeriveInput { ident, data, .. } = input;

    let path = CrateManifest::try_find("froglight-network", "froglight")
        .unwrap_or_else(|| CrateManifest::froglight("froglight-io"));

    match data {
        syn::Data::Struct(data) => derive_struct(ident, data, traits, path),
        syn::Data::Enum(data) => derive_enum(ident, data, traits, path),
        syn::Data::Union(..) => panic!("`FrogBuf` cannot be derived for unions!"),
    }
}

// -------------------------------------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn derive_struct(ident: Ident, data: DataStruct, traits: FrogBufTraits, path: Path) -> TokenStream {
    let (read, write, write_len, var_read, var_write, var_write_len) = match &data.fields {
        // Handle standard structs.
        Fields::Named(fields) => {
            // Add `FrogRead` and `FrogWrite` implementations.
            let (read, write, write_len) = if traits.std.is_present() {
                let read =
                    derive_named_fields(fields, ReadWriteMode::Read, TraitMethod::Standard, &path);

                let write =
                    derive_named_fields(fields, ReadWriteMode::Write, TraitMethod::Standard, &path);
                let write_len = derive_field_length(&data.fields, TraitMethod::Standard, &path);

                (Some(quote!(Ok(Self{#read}))), Some(write), Some(write_len))
            } else {
                (None, None, None)
            };

            // Add `FrogVarRead` and `FrogVarWrite` implementations.
            let (var_read, var_write, var_write_len) = if traits.var.is_present() {
                let read =
                    derive_named_fields(fields, ReadWriteMode::Read, TraitMethod::Variable, &path);

                let write =
                    derive_named_fields(fields, ReadWriteMode::Write, TraitMethod::Variable, &path);
                let write_len = derive_field_length(&data.fields, TraitMethod::Variable, &path);

                (Some(quote!(Ok(Self{#read}))), Some(write), Some(write_len))
            } else {
                (None, None, None)
            };

            (read, write, write_len, var_read, var_write, var_write_len)
        }
        // Handle tuple structs.
        Fields::Unnamed(fields) => {
            // Add `FrogRead` and `FrogWrite` implementations.
            let (read, write, write_len) = if traits.std.is_present() {
                let read = derive_unnamed_fields(
                    fields,
                    ReadWriteMode::Read,
                    TraitMethod::Standard,
                    &path,
                );

                let write = derive_unnamed_fields(
                    fields,
                    ReadWriteMode::Write,
                    TraitMethod::Standard,
                    &path,
                );
                let write_len = derive_field_length(&data.fields, TraitMethod::Standard, &path);

                (Some(quote!(Ok(Self(#read)))), Some(write), Some(write_len))
            } else {
                (None, None, None)
            };

            // Add `FrogVarRead` and `FrogVarWrite` implementations.
            let (var_read, var_write, var_write_len) = if traits.var.is_present() {
                let read = derive_unnamed_fields(
                    fields,
                    ReadWriteMode::Read,
                    TraitMethod::Variable,
                    &path,
                );

                let write = derive_unnamed_fields(
                    fields,
                    ReadWriteMode::Write,
                    TraitMethod::Variable,
                    &path,
                );
                let write_len = derive_field_length(&data.fields, TraitMethod::Variable, &path);

                (Some(quote!(Ok(Self(#read)))), Some(write), Some(write_len))
            } else {
                (None, None, None)
            };

            (read, write, write_len, var_read, var_write, var_write_len)
        }
        // Handle unit structs.
        Fields::Unit => {
            // Add `FrogRead` and `FrogWrite` implementations.
            let (read, write, write_len) = if traits.std.is_present() {
                (Some(quote!(Ok(Self))), Some(quote!()), Some(quote!(0)))
            } else {
                (None, None, None)
            };

            // Add `FrogVarRead` and `FrogVarWrite` implementations.
            let (var_read, var_write, var_write_len) = if traits.var.is_present() {
                (Some(quote!(Ok(Self))), Some(quote!()), Some(quote!(0)))
            } else {
                (None, None, None)
            };

            (read, write, write_len, var_read, var_write, var_write_len)
        }
    };

    let mut tokens = TokenStream::new();

    // Add `FrogRead`
    if let Some(read) = read {
        tokens.extend(quote! {
            #[automatically_derived]
            impl #path::standard::FrogRead for #ident {
                fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                    #read
                }
            }
        });
    }
    // Add `FrogWrite`
    if let (Some(write), Some(write_len)) = (write, write_len) {
        tokens.extend(quote! {
            #[automatically_derived]
            impl #path::standard::FrogWrite for #ident {
                fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                    let mut written = 0;
                    #write
                    Ok(written)
                }
                fn frog_len(&self) -> usize { #write_len }
            }
        });
    }

    // Add `FrogVarRead`
    if let Some(var_read) = var_read {
        tokens.extend(quote! {
            #[automatically_derived]
            impl #path::variable::FrogVarRead for #ident {
                fn frog_var_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                    #var_read
                }
            }
        });
    }
    // Add `FrogVarWrite`
    if let (Some(var_write), Some(var_write_len)) = (var_write, var_write_len) {
        tokens.extend(quote! {
            #[automatically_derived]
            impl #path::variable::FrogVarWrite for #ident {
                fn frog_var_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                    let mut written = 0;
                    #var_write
                    Ok(written)
                }
                fn frog_var_len(&self) -> usize { #var_write_len }
            }
        });
    }

    // Return the generated tokens.
    tokens
}

// -------------------------------------------------------------------------------------------------

fn derive_enum(ident: Ident, data: DataEnum, traits: FrogBufTraits, path: Path) -> TokenStream {
    let mut tokens = TokenStream::new();

    // Add `FrogRead` and `FrogWrite` implementations.
    if traits.std.is_present() {
        let read =
            derive_variants(&data.variants, ReadWriteMode::Read, TraitMethod::Standard, &path);

        let write =
            derive_variants(&data.variants, ReadWriteMode::Write, TraitMethod::Standard, &path);
        let write_length = derive_variant_length(&data.variants, TraitMethod::Standard, &path);

        tokens.extend(quote! {
            impl #path::standard::FrogRead for #ident {
                fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                    match <u32 as #path::variable::FrogVarRead>::frog_read(buffer)? {
                        #read
                    }
                }
            }
            impl #path::standard::FrogWrite for #ident {
                fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                    let mut written = 0;
                    match self { #write }
                    Ok(written)
                }
                fn frog_len(&self) -> usize { match self { #write_length } }
            }
        });
    }

    // Add `FrogVarRead` and `FrogVarWrite` implementations.
    if traits.var.is_present() {
        let read =
            derive_variants(&data.variants, ReadWriteMode::Read, TraitMethod::Variable, &path);

        let write =
            derive_variants(&data.variants, ReadWriteMode::Write, TraitMethod::Variable, &path);
        let write_length = derive_variant_length(&data.variants, TraitMethod::Variable, &path);

        tokens.extend(quote! {
            impl #path::variable::FrogVarRead for #ident {
                fn frog_var_read(buffer: &mut impl std::io::Read) -> Result<Self, #path::standard::ReadError> {
                    match <u32 as #path::variable::FrogVarRead>::frog_var_read(buffer)? {
                        #read
                    }
                }
            }
            impl #path::variable::FrogVarWrite for #ident {
                fn frog_var_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, #path::standard::WriteError> {
                    let mut written = 0;
                    match self { #write }
                    Ok(written)
                }
                fn frog_var_len(&self) -> usize { match self { #write_length } }
            }
        });
    }

    // Return the generated tokens.
    tokens
}

fn derive_variants(
    _variants: &Punctuated<Variant, Comma>,
    _mode: ReadWriteMode,
    _traits: TraitMethod,
    _path: &Path,
) -> TokenStream {
    TokenStream::new()
}

fn derive_variant_length(
    _variants: &Punctuated<Variant, Comma>,
    _traits: TraitMethod,
    _path: &Path,
) -> TokenStream {
    TokenStream::new()
}

// -------------------------------------------------------------------------------------------------

fn derive_named_fields(
    fields: &FieldsNamed,
    mode: ReadWriteMode,
    traits: TraitMethod,
    path: &Path,
) -> TokenStream {
    fields.named.iter().fold(TokenStream::new(), |mut acc, field| {
        let ident = field.ident.as_ref().unwrap();
        let field = derive_field(Some(ident.to_token_stream()), mode, traits, path);

        match mode {
            ReadWriteMode::Read => acc.extend(quote!(#ident: #field,)),
            ReadWriteMode::Write => acc.extend(quote!(written += #field)),
            ReadWriteMode::WriteLength => unreachable!(),
        }

        acc
    })
}

fn derive_unnamed_fields(
    fields: &FieldsUnnamed,
    mode: ReadWriteMode,
    traits: TraitMethod,
    path: &Path,
) -> TokenStream {
    fields.unnamed.iter().enumerate().fold(TokenStream::new(), |mut acc, (index, field)| {
        let field_name = field
            .ident
            .as_ref()
            .map_or_else(|| Index::from(index).into_token_stream(), Ident::to_token_stream);
        let field = derive_field(Some(field_name), mode, traits, path);

        match mode {
            ReadWriteMode::Read => acc.extend(quote!(#field,)),
            ReadWriteMode::Write => acc.extend(quote!(written += #field)),
            ReadWriteMode::WriteLength => unreachable!(),
        }

        acc
    })
}

fn derive_field_length(fields: &Fields, traits: TraitMethod, path: &Path) -> TokenStream {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let field_name = field
                .ident
                .as_ref()
                .map_or_else(|| Index::from(index).into_token_stream(), Ident::to_token_stream);

            let field = derive_field(Some(field_name), ReadWriteMode::WriteLength, traits, path);

            if index < fields.len().saturating_sub(1) { quote!(#field +) } else { field }
        })
        .collect()
}

fn derive_field(
    field: Option<TokenStream>,
    mode: ReadWriteMode,
    traits: TraitMethod,
    path: &Path,
) -> TokenStream {
    match (field, mode, traits) {
        // Read the field from the buffer.
        (_, ReadWriteMode::Read, TraitMethod::Standard) => {
            quote!(#path::standard::FrogRead::frog_read(buffer)?)
        }
        (_, ReadWriteMode::Read, TraitMethod::Variable) => {
            quote!(#path::variable::FrogVarRead::frog_var_read(buffer)?)
        }
        // Write the field to the buffer.
        (Some(field), ReadWriteMode::Write, TraitMethod::Standard) => {
            quote!(#path::standard::FrogWrite::frog_write(&self.#field, buffer)?;)
        }
        (Some(field), ReadWriteMode::Write, TraitMethod::Variable) => {
            quote!(#path::variable::FrogVarWrite::frog_var_write(&self.#field, buffer)?;)
        }
        (None, ReadWriteMode::Write, _) => panic!("Attempted to write unnamed field into buffer!"),

        // Get the length of the field.
        (Some(field), ReadWriteMode::WriteLength, TraitMethod::Standard) => {
            quote!(#path::standard::FrogWrite::frog_len(&self.#field))
        }
        (Some(field), ReadWriteMode::WriteLength, TraitMethod::Variable) => {
            quote!(#path::variable::FrogVarWrite::frog_var_len(&self.#field))
        }
        (None, ReadWriteMode::WriteLength, _) => {
            panic!("Attempted to get length of unnamed field!")
        }
    }
}
