use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    Fields, Ident, Lit, Token,
};

pub(super) fn impl_generated_components(tokens: TokenStream) -> TokenStream {
    let input = syn::parse2::<MacroInput>(tokens).unwrap();

    input.0.into_iter().fold(TokenStream::new(), |mut tokens, item| {
        tokens.extend(item.into_item());
        tokens
    })
}

struct MacroInput(Vec<MacroItem>);
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self(items))
    }
}

struct MacroItem {
    ident: Ident,
    fields: Fields,
    default: Option<Lit>,
}
impl Parse for MacroItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;

        // If the struct is unit-like, return early.
        if input.peek(Token![,]) {
            return Ok(Self { ident, fields: Fields::Unit, default: None });
        }

        input.parse::<Token![=>]>()?;

        // Parse the fields of the struct.
        let fields = if input.peek(Paren) {
            Fields::Unnamed(input.parse()?)
        } else {
            Fields::Named(input.parse()?)
        };

        // Parse the default value of the struct, if provided.
        let mut default = None;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            default = input.parse()?;
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(Self { ident, fields, default })
    }
}
impl MacroItem {
    fn into_item(self) -> TokenStream {
        let MacroItem { ident, mut fields, default } = self;
        let fields_tokens = fields.to_token_stream().to_string();

        // Only derive `PartialEq`, `Eq`, and `Hash`
        // if the struct doesn't contain any floats.
        let eq_tokens = if fields_tokens.contains("f32") || fields_tokens.contains("f64") {
            TokenStream::new()
        } else {
            quote! { PartialEq, Eq, Hash, }
        };

        // Derive `Deref`, `DerefMut`, `From`, and `Into`
        // if the struct contains a single field.
        let deref_tokens = if fields.len() == 1 {
            quote! { derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into, }
        } else {
            TokenStream::new()
        };

        // Derive `Default` if a default value is provided.
        let default_tokens = if let Some(default) = default {
            quote! {
                impl Default for #ident {
                    fn default() -> Self { Self::from(#default) }
                }
            }
        } else {
            TokenStream::new()
        };

        let derives = if fields_tokens.contains("String") {
            // Mark any `CompactString` fields as ignored.
            for field in &mut fields {
                if let syn::Type::Path(path) = &mut field.ty {
                    if path.path.to_token_stream().to_string().contains("CompactString") {
                        field.attrs.push(
                            syn::parse_quote!(#[cfg_attr(feature = "reflect", reflect(ignore))]),
                        );
                    }
                }
            }

            quote! {
                #[derive(Debug, Clone, #eq_tokens #deref_tokens Component)]
                #[cfg_attr(feature = "reflect", derive(Reflect))]
                #[cfg_attr(feature = "reflect", reflect(Component))]
            }
        } else {
            quote! {
                #[derive(Debug, Clone, Copy, #eq_tokens #deref_tokens Component)]
                #[cfg_attr(feature = "reflect", derive(Reflect))]
                #[cfg_attr(feature = "reflect", reflect(Component))]
            }
        };

        if matches!(fields, Fields::Unit | Fields::Unnamed(..)) {
            quote! {
                #derives
                pub struct #ident #fields;
                #default_tokens
            }
        } else {
            quote! {
                #derives
                pub struct #ident #fields
                #default_tokens
            }
        }
    }
}
