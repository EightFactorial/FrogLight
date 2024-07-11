use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

pub(crate) fn generate_convertid(tokens: proc_macro::TokenStream) -> TokenStream {
    let RegistryImpls { version, registries } =
        syn::parse(tokens).expect("Failed to parse registry impls");
    let mut tokenstream = TokenStream::new();

    for Registry { ident, entries } in registries {
        let mut from_tokens = TokenStream::new();
        let mut to_tokens = TokenStream::new();

        for (index, entry) in entries.into_iter().enumerate() {
            let index = u32::try_from(index).expect("Failed to convert index to u32");

            from_tokens.extend(quote! {
                #index => Some(Self::#entry),
            });
            to_tokens.extend(quote! {
                Self::#entry => #index,
            });
        }

        tokenstream.extend(quote! {
            impl crate::ConvertId<#version> for #ident {
                fn from_id(id: u32) -> Option<Self> {
                    match id {
                        #from_tokens
                        _ => None,
                    }
                }
                fn to_id(&self) -> u32 {
                    match self {
                        #to_tokens
                    }
                }
            }
        });
    }

    tokenstream
}

/// Example:
///
/// ```rust,ignore
/// frog_create_registry_impls! {
///     Version,
///     Registry {
///         Entry1, Entry2, Entry3, Entry4
///     },
///     Registry2 {
///         Entry1, Entry2, Entry3
///     },
/// }
#[derive(Debug, Clone, PartialEq, Eq)]
struct RegistryImpls {
    version: Ident,
    registries: Vec<Registry>,
}

impl Parse for RegistryImpls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let version = input.parse().expect("Failed to parse version");
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let mut registries = Vec::new();
        while !input.is_empty() {
            registries.push(input.parse()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { version, registries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Registry {
    ident: Ident,
    entries: Vec<Ident>,
}

impl Parse for Registry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;

        let mut entries = Vec::new();
        {
            let braced;
            syn::braced!(braced in input);

            while !braced.is_empty() {
                entries.push(
                    braced.parse().unwrap_or_else(|_| panic!("Failed to parse entry: \"{ident}\"")),
                );

                if braced.peek(Token![,]) {
                    braced.parse::<Token![,]>()?;
                }
            }
        }

        Ok(Self { ident, entries })
    }
}
