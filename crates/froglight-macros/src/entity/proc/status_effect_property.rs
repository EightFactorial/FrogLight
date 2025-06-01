use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitInt, LitStr, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

#[allow(clippy::obfuscated_if_else, clippy::too_many_lines)]
pub(crate) fn status_effect_properties(input: TokenStream) -> TokenStream {
    let MacroInput { path, version, effects } = syn::parse2(input).unwrap();
    let entity_path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-entity"));
    let text_path = CrateManifest::froglight("froglight-text");
    let common_path = CrateManifest::froglight("froglight-common");

    // Generate the effect implementations
    let mut output = effects.iter().fold(
        TokenStream::new(),
        |mut tokens,
         EffectInput {
             effect,
             properties: PropertyInput { ident, category, color  },
         }| {
            tokens.extend(quote! {
                #[automatically_derived]
                impl #entity_path::status_effect::StatusEffectTrait<#version> for #effect {
                    #[inline]
                    fn identifier(&self) -> &'static #common_path::identifier::Identifier {
                        static IDENTIFIER: #common_path::identifier::Identifier = #common_path::identifier::Identifier::const_new(<#effect as #entity_path::status_effect::StatusEffectExt<#version>>::IDENTIFIER);
                        &IDENTIFIER
                    }

                    #[inline]
                    fn color(&self) -> #text_path::prelude::IntegerColor { <Self as #entity_path::status_effect::StatusEffectExt<#version>>::COLOR }

                    #[inline]
                    fn category(&self) -> #entity_path::status_effect::StatusEffectCategory { <Self as #entity_path::status_effect::StatusEffectExt<#version>>::CATEGORY }

                }
                #[automatically_derived]
                impl #entity_path::status_effect::StatusEffectExt<#version> for #effect {
                    const IDENTIFIER: &'static str = #ident;
                    const COLOR: #text_path::prelude::IntegerColor = #text_path::prelude::IntegerColor::new(#color);
                    const CATEGORY: #entity_path::status_effect::StatusEffectCategory = #entity_path::status_effect::StatusEffectCategory::#category;
                }
            });

            tokens
        },
    );

    // Generate the effect resolver implementation and tests
    output.extend({
        let mut effects_enum = TokenStream::new();
        let mut effects_from_impls = TokenStream::new();

        let mut vanilla_register = TokenStream::new();
        let mut vanilla_resolve = TokenStream::new();
        let _resolver_tests = TokenStream::new();

        let _entity_tests = TokenStream::new();

        for EffectInput {
            effect,
            properties: PropertyInput { ident, .. },
        } in effects
        {

            // Build the `VersionStatusEffect` enum
            effects_enum.extend(quote! { #effect(#effect), });
            effects_from_impls.extend(quote! {
                #[automatically_derived]
                impl From<#effect> for VersionStatusEffect {
                    #[inline]
                    fn from(entity: #effect) -> Self {
                        Self::#effect(entity)
                    }
                }
            });

            // Register the entity types with the resolver
            vanilla_register.extend(quote! { storage.register::<#effect>(); });
            vanilla_resolve.extend(quote! {
                #ident => VersionStatusEffect::#effect(#effect),
            });

            // // Create resolver tests
            // resolver_tests.extend(quote! {{
            //     let block = storage.get_untyped(GlobalBlockId::new_unchecked(global)).expect("No block found for expected GlobalBlockId!");
            //     assert_eq!(block.identifier().as_str(), #ident, "Block \"{}\" identifier mismatch!", #ident);
            //     assert_eq!(block.resolve::<Vanilla>(), block.downcast().map(|block| VersionBlocks::#entity(block)), "Failed to resolve \"{}\"!", #ident);
            //     #[expect(clippy::cast_possible_truncation)]
            //     { global += <#entity as #entity_path::block::BlockTypeExt<#version>>::Attributes::COUNT as u32; }
            // }});

            // // Create block tests
            // let get_attr_tokens: TokenStream = idents
            //     .iter()
            //     .zip(names.iter())
            //     .map(|(attr_ident, attr_name)| {
            //         quote! {
            //             assert!(block.get_attr::<#attr_ident>().is_some(), "Block \"{}\" missing attribute \"{}\"!", #ident, stringify!(#attr_ident));
            //             assert!(block.get_attr_str(#attr_name).is_some(), "Block \"{}\" missing string attribute \"{}\"!", #ident, #attr_name);
            //         }
            //     })
            //     .collect();

            // let default: u16 = default.base10_parse().unwrap();
            // block_tests.extend(quote! {{
            //         let mut block = #entity_path::block::Block::<#entity, #version>::default();
            //         assert_eq!(u16::from(*block.state()), #default, "Block \"{}\" default state mismatch!", #ident);
            //         assert_eq!(block, block, "Block \"{}\" equality failed!", #ident);
            //         assert_eq!(block, block.into_version::<#version>(), "Block \"{}\" `into_version` failed!", #ident);
            //         assert_eq!(block, #entity_path::block::Block::from_attr(block.into_attr()), "Block \"{}\" attribute round-trip failed!", #ident);

            //         assert_eq!(<#entity as #entity_path::block::BlockTypeExt<#version>>::IDENTIFIER, #ident, "Block \"{}\" typed identifier mismatch!", #ident);
            //         assert_eq!(block.identifier().as_str(), #ident, "Block \"{}\" typed identifier mismatch!", #ident);
            //         assert_eq!(block.into_untyped().identifier().as_str(), #ident, "Block \"{}\" untyped identifier mismatch!", #ident);
            //         assert_eq!(block.into_untyped().downcast().unwrap(), block, "Block \"{}\" downcast failed!", #ident);

            //         #get_attr_tokens
            //     }
            // });
        }


        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Clone, PartialEq, Hash))]
            pub enum VersionStatusEffect {
                #effects_enum
            }
            #effects_from_impls

            #[automatically_derived]
            impl #entity_path::status_effect::StatusEffectResolver<#version> for #common_path::vanilla::Vanilla {
                type EffectEnum = VersionStatusEffect;
                fn register(storage: &mut #entity_path::status_effect::StatusEffectStorage<#version>) {
                    #vanilla_register
                }
                fn resolve(status_effect: &dyn #entity_path::status_effect::StatusEffectTrait<#version>) -> Option<VersionStatusEffect> {
                    hashify::map!(
                        status_effect.identifier().as_bytes(),
                        VersionStatusEffect,
                        #vanilla_resolve
                    ).copied()
                }
            }

            // #[cfg(test)]
            // mod test {
            //     use super::*;
            //     use #entity_path::{prelude::*, storage::BlockAttributes};
            //     use #common_path::vanilla::Vanilla;

            //     #[test]
            //     fn entities() {
            //         #block_tests
            //     }

            //     #[test]
            //     fn resolver() {
            //         let mut storage = #entity_path::storage::BlockStorage::<#version>::new();
            //         let mut global = 0u32;

            //         #resolver_tests
            //     }
            // }
        }
    });

    output
}

/// Example:
///
/// ```text
/// froglight_macros::entity_type_properties! {
///     crate,
///     version = froglight_common::version::V1_21_4,
///     ...
/// }
/// ```
struct MacroInput {
    path: Option<Path>,
    version: Path,
    effects: Vec<EffectInput>,
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
            effects: input.parse_terminated(EffectInput::parse, Token![,])?.into_iter().collect(),
        })
    }
}

/// Example:
///
/// ```text
/// Cat                => { properties: { ... }, attributes: { ... } }
/// ```
struct EffectInput {
    effect: Ident,
    properties: PropertyInput,
}
impl Parse for EffectInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let block = input.parse()?;
        input.parse::<Token![=>]>()?;

        let content;
        syn::braced!(content in input);

        let mut properties = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            let braced;
            syn::braced!(braced in content);

            match key.to_string().as_str() {
                "properties" => {
                    properties = Some(braced.parse()?);
                }
                unk => {
                    return Err(syn::Error::new(key.span(), format!("unknown effect key '{unk}'")));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
            if properties.is_some() {
                break;
            }
        }

        Ok(Self {
            effect: block,
            properties: properties.ok_or_else(|| {
                syn::Error::new(content.span(), "missing effect key 'properties'")
            })?,
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
    category: Ident,
    color: LitInt,
}
impl Parse for PropertyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = None;
        let mut category = None;
        let mut color = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "ident" => {
                    ident = Some(input.parse()?);
                }
                "category" => {
                    category = Some(input.parse()?);
                }
                "color" => {
                    color = Some(input.parse()?);
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
            if ident.is_some() && category.is_some() && color.is_some() {
                break;
            }
        }

        Ok(Self {
            ident: ident
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'ident'"))?,
            category: category
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'category'"))?,
            color: color
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'color'"))?,
        })
    }
}
