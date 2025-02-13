use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Path, Token, Variant, Visibility,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
};

use crate::CrateManifest;

pub(crate) fn block_attributes(input: TokenStream) -> TokenStream {
    let MacroInput { path, attributes } = syn::parse2(input).unwrap();
    let block = path.unwrap_or_else(|| CrateManifest::froglight("froglight-block"));

    // Generate the block attribute enums.
    let mut tokens = attributes.iter().fold(TokenStream::new(), |mut tokens, attr| {
        tokens.extend(attr.as_tokens(&block));
        tokens
    });

    // Generate block attribute tests.
    tokens.extend(BlockAttribute::create_tests(&attributes, &block));

    tokens
}

struct MacroInput {
    path: Option<Path>,
    attributes: Punctuated<BlockAttribute, Token![,]>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse::<Path>().map_or(None, |v| {
                let _ = input.parse::<Token![,]>();
                Some(v)
            }),
            attributes: input.parse_terminated(BlockAttribute::parse, Token![,])?,
        })
    }
}

#[derive(Hash)]
struct BlockAttribute {
    vis: Visibility,
    token: Token![enum],
    ident: Ident,
    _variants_brace: Brace,
    variants: Punctuated<Variant, Token![,]>,
    _strings_brace: Brace,
    strings: Punctuated<LitStr, Token![,]>,
}

impl Parse for BlockAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant_content;
        let string_content;
        Ok(Self {
            vis: input.parse()?,
            token: input.parse()?,
            ident: input.parse()?,
            _variants_brace: syn::braced!(variant_content in input),
            variants: variant_content.parse_terminated(Variant::parse, Token![,])?,
            _strings_brace: syn::braced!(string_content in input),
            strings: string_content.parse_terminated(<LitStr as Parse>::parse, Token![,])?,
        })
    }
}
impl BlockAttribute {
    fn as_tokens(&self, path: &Path) -> TokenStream {
        let Self { vis, token, ident, variants, strings, .. } = self;

        let state_tokens =
            variants.iter().enumerate().fold(TokenStream::new(), |mut tokens, (i, v)| {
                if i == 0 {
                    tokens.extend(quote! { #ident::#v });
                } else {
                    tokens.extend(quote! { , #ident::#v });
                }
                tokens
            });

        let usize_tokens =
            variants.iter().enumerate().fold(TokenStream::new(), |mut tokens, (i, v)| {
                tokens.extend(quote! { #ident::#v => #i, });
                tokens
            });

        let bool_tokens = if variants.len() == 2
            && matches!(variants.first().unwrap().ident.to_string().as_str(), "True" | "False")
            && matches!(variants.last().unwrap().ident.to_string().as_str(), "True" | "False")
        {
            quote! {
                impl From<bool> for #ident {
                    fn from(v: bool) -> Self {
                        if v { #ident::True } else { #ident::False }
                    }
                }
                impl From<#ident> for bool {
                    fn from(v: #ident) -> Self {
                        match v {
                            #ident::True => true,
                            #ident::False => false,
                        }
                    }
                }
            }
        } else {
            TokenStream::new()
        };

        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect), reflect(Debug, PartialEq, Hash))]
            #vis #token #ident { #variants }
            impl #path::storage::Attribute for #ident {
                const STATES: &'static [#ident] = &[#state_tokens];
                const VALUES: &'static [&'static str] = &[#strings];
            }
            impl From<#ident> for usize {
                fn from(v: #ident) -> Self {
                    match v { #usize_tokens }
                }
            }
            #bool_tokens
        }
    }

    fn create_tests(attrs: &Punctuated<Self, Token![,]>, path: &Path) -> TokenStream {
        let attribute_tests = attrs.iter().fold(
            TokenStream::new(),
            |mut tokens, BlockAttribute { ident, variants, .. }| {
                let attribute_tests = variants.iter().enumerate().fold(TokenStream::new(), |mut tokens, (i, v)| {
                    tokens.extend(quote! {{
                        assert_eq!(Into::<usize>::into(#ident::#v), #i, "Invalid Attribute index for `{:?}`!", #ident::#v);
                        assert_eq!(#ident::#v.into_index(), #i, "Invalid BlockAttribute index for `{:?}`!", #ident::#v);
                        assert_eq!(#ident::#v, #ident::from_index(#i), "Invalid BlockAttribute index for `{:?}`!", #ident::#v);

                        let mut attr = #ident::#v;
                        assert_eq!(attr.get_attr::<#ident>(), Some(#ident::#v), "Unable to get attribute `{:?}`!", #ident::#v);
                        assert_eq!(attr.get_attr_str(0), #ident::VALUES[#i], "String value mismatch for `{:?}`!", #ident::#v);
                        assert_eq!(attr.set_attr_str(0, #ident::VALUES[#i]), Some(#ident::VALUES[#i]), "String value mismatch for `{:?}`!", #ident::#v);
                        assert_eq!(attr, #ident::#v, "String value mismatch for `{:?}`!", #ident::#v);
                    }});
                    tokens
                });

                tokens.extend(quote! {{
                        assert_eq!(#ident::STATES.len(), #ident::COUNT, "Invalid state count for `{}`!", stringify!(#ident));
                        assert_eq!(&[std::any::TypeId::of::<#ident>()], #ident::TYPES, "Invalid type ID for `{}`!", stringify!(#ident));

                        #attribute_tests
                }});

                tokens
            },
        );

        quote! {
            #[cfg(test)]
            mod test {
                use super::*;
                use #path::storage::{BlockAttributes, Attribute};

                #[test]
                fn attributes() {
                    #attribute_tests
                }
            }
        }
    }
}
