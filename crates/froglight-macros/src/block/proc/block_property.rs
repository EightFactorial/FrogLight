use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitBool, LitInt, LitStr, Path, Token,
};

use crate::CrateManifest;

#[allow(clippy::obfuscated_if_else, clippy::too_many_lines)]
pub(crate) fn block_properties(input: TokenStream) -> TokenStream {
    let MacroInput { path, version, blocks } = syn::parse2(input).unwrap();
    let block_path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-block"));
    let common_path = CrateManifest::froglight("froglight-common");

    // Generate the block implementations
    let mut output = blocks.iter().fold(
        TokenStream::new(),
        |mut tokens, BlockInput { block, properties: PropertyInput { ident, default, is_air }, attributes: AttributeInput { names, idents } }| {
            // If there is not exactly one attribute, make a unit or tuple struct
            let attrs = idents.len().ne(&1).then(|| quote!((#idents))).unwrap_or_else(|| quote!(#idents));

            tokens.extend(quote! {
                #[automatically_derived]
                impl #block_path::block::BlockType<#version> for #block {
                    #[must_use]
                    fn get_attr_str(&self, state: u16, attr: &str) -> Option<&'static str> {
                        if <<#block as #block_path::block::BlockTypeExt<#version>>::Attributes as #block_path::storage::BlockAttributes>::COUNT > state as usize {
                            #block_path::block::Block::<#block, #version>::new(state.into()).get_attr_str(attr)
                        } else {
                            None
                        }
                    }

                    #[inline]
                    #[must_use]
                    fn identifier(&self) -> &'static #common_path::identifier::Identifier {
                        static IDENTIFIER: #common_path::identifier::Identifier = #common_path::identifier::Identifier::const_new(<#block as #block_path::block::BlockTypeExt<#version>>::IDENTIFIER);
                        &IDENTIFIER
                    }

                    #[inline]
                    #[must_use]
                    fn is_air(&self) -> bool { <#block as #block_path::block::BlockTypeExt<#version>>::IS_AIR }
                }
                #[automatically_derived]
                impl #block_path::block::BlockTypeExt<#version> for #block {
                    type Attributes = #attrs;
                    const ATTRIBUTES: &'static [&'static str] = &[#names];
                    const DEFAULT: u16 = #default;

                    const IDENTIFIER: &'static str = #ident;
                    const IS_AIR: bool = #is_air;
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
        let mut resolver_tests = TokenStream::new();

        let mut block_tests = TokenStream::new();

        for BlockInput {
            block,
            properties: PropertyInput { ident, default, .. },
            attributes: AttributeInput { names, idents },
        } in blocks
        {

            // Build the `VersionBlocks` enum
            blocks_enum.extend(quote! { #block(#block_path::block::Block<#block, #version>), });
            blocks_from_impls.extend(quote! {
                #[automatically_derived]
                impl From<#block_path::block::Block<#block, #version>> for VersionBlocks {
                    #[inline]
                    fn from(block: #block_path::block::Block<#block, #version>) -> Self {
                        Self::#block(block)
                    }
                }
            });

            // Register the blocks with the resolver
            vanilla_register.extend(quote! { storage.register::<#block>(); });
            vanilla_resolve.extend(quote! {
                #ident => { return block.downcast::<#block>().map(VersionBlocks::#block) },
            });

            // Create resolver tests
            resolver_tests.extend(quote! {{
                let block = storage.get_untyped(GlobalBlockId::new_unchecked(global)).expect("No block found for expected GlobalBlockId!");
                assert_eq!(block.identifier().as_str(), #ident, "Block \"{}\" identifier mismatch!", #ident);
                assert_eq!(block.resolve::<Vanilla>(), block.downcast().map(|block| VersionBlocks::#block(block)), "Failed to resolve \"{}\"!", #ident);
                #[expect(clippy::cast_possible_truncation)]
                { global += <#block as #block_path::block::BlockTypeExt<#version>>::Attributes::COUNT as u32; }
            }});

            // Create block tests
            let get_attr_tokens: TokenStream = idents
                .iter()
                .zip(names.iter())
                .map(|(attr_ident, attr_name)| {
                    quote! {
                        assert!(block.get_attr::<#attr_ident>().is_some(), "Block \"{}\" missing attribute \"{}\"!", #ident, stringify!(#attr_ident));
                        assert!(block.get_attr_str(#attr_name).is_some(), "Block \"{}\" missing string attribute \"{}\"!", #ident, #attr_name);
                    }
                })
                .collect();

            let default: u16 = default.base10_parse().unwrap();
            block_tests.extend(quote! {{
                    let mut block = #block_path::block::Block::<#block, #version>::default();
                    assert_eq!(block.state(), &#default.into(), "Block \"{}\" default state mismatch!", #ident);
                    assert_eq!(block, block, "Block \"{}\" equality failed!", #ident);
                    assert_eq!(block, block.into_version::<#version>(), "Block \"{}\" `into_version` failed!", #ident);
                    assert_eq!(block, #block_path::block::Block::from_attr(block.into_attr()), "Block \"{}\" attribute round-trip failed!", #ident);

                    assert_eq!(<#block as #block_path::block::BlockTypeExt<#version>>::IDENTIFIER, #ident, "Block \"{}\" typed identifier mismatch!", #ident);
                    assert_eq!(block.identifier().as_str(), #ident, "Block \"{}\" typed identifier mismatch!", #ident);
                    assert_eq!(block.into_untyped().identifier().as_str(), #ident, "Block \"{}\" untyped identifier mismatch!", #ident);
                    assert_eq!(block.into_untyped().downcast().unwrap(), block, "Block \"{}\" downcast failed!", #ident);

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

            #[automatically_derived]
            impl #block_path::resolve::BlockResolver<#version> for #common_path::vanilla::Vanilla {
                type BlockEnum = VersionBlocks;
                fn register(storage: &mut #block_path::storage::BlockStorage<#version>) {
                    #vanilla_register
                }
                fn resolve(block: #block_path::block::UntypedBlock<#version>) -> Option<VersionBlocks> {
                    hashify::fnc_map!(
                        block.identifier().as_bytes(),
                        #vanilla_resolve
                        _ => { return None }
                    );
                    unreachable!("All possible cases handled by `hashify::fnc_map` macro")
                }
            }

            #[cfg(test)]
            mod test {
                use super::*;
                use #block_path::{prelude::*, storage::BlockAttributes};
                use #common_path::vanilla::Vanilla;

                #[test]
                fn blocks() {
                    #block_tests
                }

                #[test]
                fn resolver() {
                    let mut storage = #block_path::storage::BlockStorage::<#version>::new();
                    let mut global = 0u32;

                    #resolver_tests
                }
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
/// properties: { ident: "minecraft:andesite_slab", default: 3, is_air: false }
/// ```
struct PropertyInput {
    ident: LitStr,
    default: LitInt,
    is_air: LitBool,
}
impl Parse for PropertyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = None;
        let mut default = None;
        let mut is_air = None;

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
                "is_air" => {
                    is_air = Some(input.parse()?);
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
            if ident.is_some() && default.is_some() && is_air.is_some() {
                break;
            }
        }

        Ok(Self {
            ident: ident
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'ident'"))?,
            default: default
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'default'"))?,
            is_air: is_air
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'is_air'"))?,
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
