use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitInt, LitStr, Path, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::CrateManifest;

#[allow(clippy::too_many_lines)]
pub(crate) fn block_properties(input: TokenStream) -> TokenStream {
    let MacroInput { path, version, blocks } = syn::parse2(input).unwrap();
    let block_path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-block"));
    let common_path = CrateManifest::froglight("froglight-common");

    // Generate the block implementations
    let mut output = blocks.iter().fold(
        TokenStream::new(),
        |mut tokens, BlockInput { block, properties: PropertyInput { ident, default }, attributes: AttributeInput { names, idents } }| {
            tokens.extend(quote! {
                impl #block_path::block::BlockType<#version> for #block {
                    fn identifier(&self) -> &'static #common_path::Identifier {
                        static IDENTIFIER: #common_path::Identifier = #common_path::Identifier::const_new(#ident);
                        &IDENTIFIER
                    }
                }
                impl #block_path::block::BlockTypeExt<#version> for #block {
                    type Attributes = (#idents);
                    const ATTRIBUTES: &'static [&'static str] = &[#names];
                    const DEFAULT: u16 = #default;
                }
            });

            tokens
        },
    );

    // Generate the block resolver implementation and tests
    output.extend({
        let mut blocks_enum = TokenStream::new();
        let mut blocks_from_impls = TokenStream::new();

        let mut vanilla_register = TokenStream::new();
        let mut vanilla_resolve = TokenStream::new();
        // TODO: Write tests for the resolver
        let resolve_tests = TokenStream::new();

        let mut block_tests = TokenStream::new();

        for BlockInput {
            block,
            properties: PropertyInput { ident, default },
            attributes: AttributeInput { names, idents },
        } in blocks
        {
            let test_fn = Ident::new(
                &format!("test_{}", block.to_string().to_case(Case::Snake)),
                block.span(),
            );
            let default: u16 = default.base10_parse().unwrap();

            // Build the `VersionBlocks` enum
            blocks_enum.extend(quote! { #block(#block_path::block::Block<#block, #version>), });
            blocks_from_impls.extend(quote! {
                impl From<#block_path::block::Block<#block, #version>> for VersionBlocks {
                    fn from(block: #block_path::block::Block<#block, #version>) -> Self {
                        Self::#block(block)
                    }
                }
            });

            // Register the blocks with the resolver
            vanilla_register.extend(quote! { storage.register::<#block>(); });
            vanilla_resolve.extend(quote! {
                #ident => |block| block.downcast::<#block>().unwrap().into(),
            });

            // Create block tests
            let get_attr_tokens: TokenStream = idents
                .iter()
                .zip(names.iter())
                .map(|(ident, name)| {
                    quote! {
                        assert!(block.get_attr::<#ident>().is_some());
                        assert!(block.get_attr_str(#name).is_some());
                    }
                })
                .collect();

            block_tests.extend(quote! {
                #[test]
                fn #test_fn() {
                    let mut block = #block_path::block::Block::<#block, #version>::default();
                    assert_eq!(block.state(), &#default.into());
                    assert_eq!(block, block);
                    assert_eq!(block, block.into_version::<#version>());
                    assert_eq!(block, #block_path::block::Block::from_attr(block.into_attr()));

                    assert_eq!(#block_path::block::Block::<#block, #version>::identifier().as_str(), #ident);
                    assert_eq!(block.into_untyped().identifier().as_str(), #ident);
                    assert_eq!(block.into_untyped().downcast().unwrap(), block);

                    #get_attr_tokens
                }
            });
        }


        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum VersionBlocks {
                #blocks_enum
            }
            #blocks_from_impls

            impl #block_path::resolve::BlockResolver<#version> for #block_path::resolve::Vanilla {
                type BlockEnum = VersionBlocks;
                fn register(storage: &mut #block_path::storage::BlockStorage<#version>) {
                    #vanilla_register
                }
                fn resolve(block: #block_path::block::UntypedBlock<#version>) -> Option<VersionBlocks> {
                    type ResolveFn = fn(#block_path::block::UntypedBlock<#version>) -> VersionBlocks;
                    hashify::map! {
                        block.identifier().as_bytes(),
                        ResolveFn,
                        #vanilla_resolve
                    }.map(|f| f(block))
                }
            }

            #[cfg(test)]
            mod tests {
                use super::*;
                use #block_path::prelude::*;

                #block_tests
                #resolve_tests
            }
        }
    });

    output
}

/// Example:
///
/// ```text
/// froglight_macros::block_properties! {
///     crate,
///     version = froglight_common::version::V1_21_4,
///     ...
/// }
/// ```
struct MacroInput {
    path: Option<Path>,
    version: Path,
    blocks: Vec<BlockInput>,
}
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut path = None;
        let mut version = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "path" => {
                    path = Some(input.parse()?);
                }
                "version" => {
                    version = Some(input.parse()?);
                }
                unk => {
                    return Err(syn::Error::new(key.span(), format!("unknown macro key '{unk}'")));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            if path.is_some() && version.is_some() {
                break;
            }
        }

        Ok(Self {
            path,
            version: version
                .ok_or_else(|| syn::Error::new(input.span(), "missing macro key 'version'"))?,
            blocks: input.parse_terminated(BlockInput::parse, Token![,])?.into_iter().collect(),
        })
    }
}

/// Example:
///
/// ```text
/// AndesiteSlab                         => { properties: { ... }, attributes: { ... } }
/// ```
struct BlockInput {
    block: Ident,
    properties: PropertyInput,
    attributes: AttributeInput,
}
impl Parse for BlockInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let block = input.parse()?;
        input.parse::<Token![=>]>()?;

        let content;
        syn::braced!(content in input);

        let mut properties = None;
        let mut attributes = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            let braced;
            syn::braced!(braced in content);

            match key.to_string().as_str() {
                "properties" => {
                    properties = Some(braced.parse()?);
                }
                "attributes" => {
                    attributes = Some(braced.parse()?);
                }
                unk => {
                    return Err(syn::Error::new(key.span(), format!("unknown block key '{unk}'")));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
            if properties.is_some() && attributes.is_some() {
                break;
            }
        }

        Ok(Self {
            block,
            properties: properties
                .ok_or_else(|| syn::Error::new(content.span(), "missing block key 'properties'"))?,
            attributes: attributes.unwrap_or_default(),
        })
    }
}

/// Example:
///
/// ```text
/// properties: { ident: "minecraft:andesite_slab", default: 3 }
/// ```
struct PropertyInput {
    ident: LitStr,
    default: LitInt,
}
impl Parse for PropertyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = None;
        let mut default = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "ident" => {
                    ident = Some(input.parse()?);
                }
                "default" => {
                    default = Some(input.parse()?);
                }
                unk => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown property key '{unk}'"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            if ident.is_some() && default.is_some() {
                break;
            }
        }

        Ok(Self {
            ident: ident
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'ident'"))?,
            default: default
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'default'"))?,
        })
    }
}

/// Example:
///
/// ```text
/// attributes: { ("type", "waterlogged"): (TypeEnum_Top_Bottom_Double, WaterloggedBool) }
/// ```
#[derive(Default)]
struct AttributeInput {
    names: Punctuated<LitStr, Token![,]>,
    idents: Punctuated<Ident, Token![,]>,
}
impl Parse for AttributeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_content;
        syn::parenthesized!(name_content in input);

        input.parse::<Token![:]>()?;

        let ident_content;
        syn::parenthesized!(ident_content in input);

        Ok(Self {
            names: name_content.parse_terminated(<LitStr as Parse>::parse, Token![,])?,
            idents: ident_content.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}
