use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

pub(crate) fn impl_generated_registries(tokens: proc_macro::TokenStream) -> TokenStream {
    let RegistryImpls { version, registries } =
        syn::parse(tokens).expect("Failed to parse registry impls");

    let mut tokenstream = TokenStream::new();

    for RegistryImpl { name, entries } in registries {
        let mut as_id = TokenStream::new();
        let mut from_id = TokenStream::new();
        let mut as_key = TokenStream::new();
        let mut from_key = TokenStream::new();

        for (index, entry) in entries.into_iter().enumerate() {
            let id = u32::try_from(index).unwrap();
            let key = Ident::new(
                &format!("KEY_{}", entry.to_string().to_ascii_uppercase()),
                entry.span(),
            );

            as_id.extend(quote! {
                Self::#entry => Some(#id),
            });
            from_id.extend(quote! {
                #id => Some(Self::#entry),
            });

            as_key.extend(quote! {
                Self::#entry => Some(Self::#key),
            });
            from_key.extend(quote! {
                Self::#key => Some(Self::#entry),
            });
        }

        tokenstream.extend(quote! {
            impl RegistryId<#version> for #name {
                fn as_id(&self) -> Option<u32> {
                    match self {
                        #as_id
                        _ => None,
                    }
                }
                fn from_id(id: u32) -> Option<Self> {
                    match id {
                        #from_id
                        _ => None,
                    }
                }
            }

            impl RegistryKey<#version> for #name {
                fn as_key(&self) -> Option<&'static str> {
                    match self {
                        #as_key
                        _ => None,
                    }
                }
                fn from_key(key: &str) -> Option<Self> {
                    match key {
                        #from_key
                        _ => None,
                    }
                }
            }
        });
    }

    tokenstream
}

/// ```rust,ignore
/// impl_generated_registries! {
///    V1_21_0 => {
///        Registry1 { Entry1, Entry2 },
///    }
/// }
/// ```
struct RegistryImpls {
    version: Ident,
    registries: Vec<RegistryImpl>,
}

impl Parse for RegistryImpls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let version = input.parse()?;
        input.parse::<Token![=>]>()?;

        let braced;
        syn::braced!(braced in input);

        let mut registries = Vec::new();
        while !braced.is_empty() {
            let registry = braced.parse()?;
            if braced.peek(Token![,]) {
                braced.parse::<Token![,]>()?;
            }

            registries.push(registry);
        }

        Ok(Self { version, registries })
    }
}

struct RegistryImpl {
    name: Ident,
    entries: Vec<Ident>,
}

impl Parse for RegistryImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let braced;
        syn::braced!(braced in input);

        let mut entries = Vec::new();
        while !braced.is_empty() {
            let entry = braced.parse()?;
            if braced.peek(Token![,]) {
                braced.parse::<Token![,]>()?;
            }

            entries.push(entry);
        }

        Ok(Self { name, entries })
    }
}
