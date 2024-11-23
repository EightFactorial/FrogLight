use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    token::Paren,
    Ident, Lit, Token,
};

pub(super) fn impl_block_traits(tokens: TokenStream) -> TokenStream {
    let input = syn::parse2::<MacroInput>(tokens).unwrap();
    input.blocks.iter().fold(TokenStream::new(), |mut output, block| {
        output.extend(block.as_tokens(&input.version));
        output
    })
}

struct MacroInput {
    version: Ident,
    blocks: Vec<BlockTraitImpl>,
}
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let version = input.parse()?;
        let _ = input.parse::<Token![=>]>()?;

        let braced;
        syn::braced!(braced in input);

        let mut blocks = Vec::new();
        while !braced.is_empty() {
            blocks.push(braced.parse()?);

            if braced.peek(Token![,]) {
                let _ = braced.parse::<Token![,]>();
            }
        }

        Ok(MacroInput { version, blocks })
    }
}

struct BlockTraitImpl {
    ident: Ident,
    attributes: Vec<Ident>,
    default: Option<Lit>,

    resource_key: Lit,
    material: Lit,
    diggable: Lit,
    hardness: Lit,
    resistance: Lit,
    transparent: Lit,
    emit_light: Lit,
    bounding_box: Lit,
}
impl Parse for BlockTraitImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _ = input.parse::<Token![=>]>()?;

        let mut attributes = Vec::new();
        if input.peek(Paren) {
            // Parse the attributes
            let parenthesized;
            syn::parenthesized!(parenthesized in input);

            while !parenthesized.is_empty() {
                attributes.push(parenthesized.parse()?);

                if parenthesized.peek(Token![,]) {
                    let _ = parenthesized.parse::<Token![,]>();
                }
            }

            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>();
            }
        }

        // Parse the block properties
        let bracketed;
        syn::bracketed!(bracketed in input);

        let resource_key = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let material = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let diggable = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let hardness = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let resistance = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let transparent = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let emit_light = bracketed.parse()?;
        let _ = bracketed.parse::<Token![,]>();
        let bounding_box = bracketed.parse()?;

        // Parse the default state, if present
        let mut default = None;
        if bracketed.peek(Token![,]) {
            let _ = bracketed.parse::<Token![,]>();
            default = Some(bracketed.parse()?);
        }

        Ok(BlockTraitImpl {
            ident,
            attributes,
            default,
            resource_key,
            material,
            diggable,
            hardness,
            resistance,
            transparent,
            emit_light,
            bounding_box,
        })
    }
}
impl BlockTraitImpl {
    fn as_tokens(&self, version: &Ident) -> TokenStream {
        let BlockTraitImpl {
            ident,
            attributes,
            default,
            resource_key,
            material,
            diggable,
            hardness,
            resistance,
            transparent,
            emit_light,
            bounding_box,
        } = &self;

        let blockstateext = if let Some(default) = default {
            // Collect the attributes into a tuple
            let mut attr_tokens = TokenStream::new();
            for (index, attr) in attributes.iter().enumerate() {
                attr_tokens.extend(quote!(#attr));
                if index < attributes.len() - 1 {
                    attr_tokens.extend(quote!(,));
                }
            }

            // Wrap the attributes in parentheses if there are more than one
            match attributes.len() {
                0 => panic!("BlockState has a default state, but no attributes!"),
                1 => {}
                2.. => attr_tokens = quote!((#attr_tokens)),
            }

            // Generate the BlockStateExt implementation
            quote! {
                impl BlockStateExt<#version> for #ident {
                    type Attributes = #attr_tokens;
                    const DEFAULT: Self = Self(#default);
                    fn to_relative(&self) -> u16 { self.0 }
                    fn from_relative(relative: u16) -> Option<Self> {
                        if usize::from(relative) < <Self as BlockStateExt<#version>>::STATE_COUNT { Some(Self(relative)) } else { None }
                    }
                }
            }
        } else {
            // Generate the BlockStateExt implementation
            quote! {
                impl BlockStateExt<#version> for #ident {
                    type Attributes = ();
                    const DEFAULT: Self = Self;
                    fn to_relative(&self) -> u16 { 0u16 }
                    fn from_relative(relative: u16) -> Option<Self> {
                        if relative == 0 { Some(Self) } else { None }
                    }
                }
            }
        };

        quote! {
            impl BlockState<#version> for #ident {
                fn resource_key(&self) -> &'static str { #resource_key }
                fn material(&self) -> &'static str { #material }
                fn diggable(&self) -> bool { #diggable }
                fn hardness(&self) -> f32 { #hardness }
                fn resistance(&self) -> f32 { #resistance }
                fn transparent(&self) -> bool { #transparent }
                fn emit_light(&self) -> u8 { #emit_light }
                fn bounding_box(&self) -> &'static str { #bounding_box }
            }
            #blockstateext
        }
    }
}
