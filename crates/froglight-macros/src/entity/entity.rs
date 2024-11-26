use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

pub(super) fn impl_generated_entities(tokens: TokenStream) -> TokenStream {
    let input = syn::parse2::<MacroInput>(tokens).unwrap();
    let mut output = TokenStream::new();

    for item in &input.0 {
        output.extend(item.to_item());
    }
    output.extend(input.create_register());

    output
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
impl MacroInput {
    fn create_register(&self) -> TokenStream {
        let items = self.0.iter().map(|item| {
            let ident = &item.ident;
            quote! {
                app.register_type::<#ident>();
            }
        });

        quote! {
            #[cfg(feature = "reflect")]
            pub(crate) fn register(app: &mut bevy_app::App) {
                #(#items)*
            }
        }
    }
}

struct MacroItem {
    ident: Ident,
    components: Vec<TokenTree>,
}
impl Parse for MacroItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![=>]>()?;

        let braced;
        syn::braced!(braced in input);

        let mut components = Vec::new();
        while !braced.is_empty() {
            components.push(braced.parse()?);

            if braced.peek(Token![,]) {
                braced.parse::<Token![,]>()?;
            }
        }

        Ok(Self { ident, components })
    }
}
impl MacroItem {
    fn to_item(&self) -> TokenStream {
        let MacroItem { ident, components } = self;

        let mut required_tokens = TokenStream::new();
        for (component, next) in components.iter().zip(components.iter().skip(1)) {
            if component.to_string().contains('(') {
            } else if next.to_string().contains('(') {
                let mut iterator = component.to_token_stream().into_iter();
                let ident = iterator.next().unwrap();

                required_tokens.extend(quote! {
                    #[require(#ident(|| #ident::from(#next)))]
                });
            } else {
                required_tokens.extend(quote! {
                    #[require(#component)]
                });
            }
        }

        quote! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
            #[cfg_attr(feature = "reflect", derive(Reflect))]
            #[cfg_attr(feature = "reflect", reflect(Default, Component))]
            #required_tokens
            pub struct #ident;
        }
    }
}
