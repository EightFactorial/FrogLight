use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt, LitStr, Token, Type,
};

pub(crate) fn impl_atlasdata(input: TokenStream) -> TokenStream {
    let AtlasMacro {
        name,
        size: (width, height),
        path,
        atlas,
        textures,
    } = parse_macro_input!(input as AtlasMacro);

    // Get the constants and coordinates for each texture.
    let mut consts = Vec::new();
    let mut coords = Vec::new();

    for (
        index,
        TextureDefinition {
            tex_name,
            tex_coords: [x1, y1, x2, y2],
        },
    ) in textures.into_iter().enumerate()
    {
        // The index in the atlas starts at 1.
        let actual_index = index + 1;

        // Create a constant for the texture's index in the atlas.
        consts.push(quote!(pub const #tex_name: usize = #actual_index;));

        // Collect the coordinates for the texture into a Rect
        coords.push(quote!(bevy::math::Rect::new(#x1, #y1, #x2, #y2)));
    }

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct #name;

        impl From<#name> for TextureAtlasType {
            fn from(_: #name) -> Self { #atlas }
        }

        impl #name {
            #(#consts)*
        }

        impl crate::assets::textureatlases::TextureAtlasData for #name {
            fn size() -> (u32, u32) { (#width, #height) }

            fn path() -> mc_rs_core::ResourceLocation {
                mc_rs_core::ResourceLocation::from(#path)
            }

            fn coords() -> Vec<bevy::math::Rect> {
                vec![#(#coords),*]
            }
        }

    }
    .into()
}

/// The input for the `impl_atlasdata!` macro.
struct AtlasMacro {
    name: Ident,
    size: (u32, u32),
    path: LitStr,
    atlas: Type,
    textures: Vec<TextureDefinition>,
}

impl Parse for AtlasMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let content;
        parenthesized!(content in input);
        let width = content.parse::<LitInt>()?.base10_parse::<u32>()?;
        content.parse::<Token![,]>()?;
        let height = content.parse::<LitInt>()?.base10_parse::<u32>()?;
        input.parse::<Token![,]>()?;

        let path = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;

        let atlas = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut textures = Vec::new();
        while !input.is_empty() {
            textures.push(input.parse::<TextureDefinition>()?);
        }

        Ok(AtlasMacro {
            name,
            size: (width, height),
            path,
            atlas,
            textures,
        })
    }
}

/// A texture name and its coordinates in the atlas.
struct TextureDefinition {
    tex_name: Ident,
    tex_coords: [f32; 4],
}

impl Parse for TextureDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tex_name = input.parse()?;
        input.parse::<Token![=]>()?;

        let content;
        bracketed!(content in input);

        let mut tex_coords = [0f32; 4];

        #[allow(clippy::cast_precision_loss)]
        for (index, coord) in tex_coords.iter_mut().enumerate() {
            *coord = content.parse::<LitInt>()?.base10_parse::<u32>()? as f32;

            if index < 3 {
                content.parse::<Token![,]>()?;
            }
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(TextureDefinition {
            tex_name,
            tex_coords,
        })
    }
}
