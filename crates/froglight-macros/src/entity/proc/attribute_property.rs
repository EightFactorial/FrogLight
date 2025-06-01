use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{
    Ident, LitStr, PatRange, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

#[allow(clippy::obfuscated_if_else, clippy::too_many_lines)]
pub(crate) fn entity_attribute_properties(input: TokenStream) -> TokenStream {
    let MacroInput { path, version, attributes } = syn::parse2(input).unwrap();
    let entity_path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-entity"));
    let common_path = CrateManifest::froglight("froglight-common");

    // Generate the block implementations
    let mut output = attributes.iter().fold(
        TokenStream::new(),
        |mut tokens,
         AttributeInput {
             attribute,
             properties: PropertyInput { ident, translation, default, range  },
         }| {
            tokens.extend(quote! {
                #[automatically_derived]
                impl #entity_path::entity_attribute::EntityAttributeTrait<#version> for #attribute {
                    #[inline]
                    fn identifier(&self) -> &'static #common_path::identifier::Identifier {
                        static IDENTIFIER: #common_path::identifier::Identifier = #common_path::identifier::Identifier::const_new(<#attribute as #entity_path::entity_attribute::EntityAttributeExt<#version>>::IDENTIFIER);
                        &IDENTIFIER
                    }

                    #[inline]
                    fn translation_key(&self) -> &'static str {
                        static TRANSLATION_KEY: &str = <#attribute as #entity_path::entity_attribute::EntityAttributeExt<#version>>::TRANSLATION_KEY;
                        TRANSLATION_KEY
                    }

                    #[inline]
                    fn default_value(&self) -> f64 {
                        <#attribute as #entity_path::entity_attribute::EntityAttributeExt<#version>>::DEFAULT
                    }

                    fn valid_range(&self) -> core::ops::RangeInclusive<f64> {
                        <#attribute as #entity_path::entity_attribute::EntityAttributeExt<#version>>::RANGE
                    }
                }
                #[automatically_derived]
                impl #entity_path::entity_attribute::EntityAttributeExt<#version> for #attribute {
                    const IDENTIFIER: &'static str = #ident;
                    const TRANSLATION_KEY: &'static str = #translation;

                    const DEFAULT: f64 = #default;
                    const RANGE: core::ops::RangeInclusive<f64> = #range;
                }
            });

            tokens
        },
    );

    // Generate the block resolver implementation and tests
    output.extend({
        let mut entities_enum = TokenStream::new();
        let mut entities_from_impls = TokenStream::new();

        let mut vanilla_register = TokenStream::new();
        let mut vanilla_resolve = TokenStream::new();
        let _resolver_tests = TokenStream::new();

        let _entity_tests = TokenStream::new();

        for AttributeInput {
            attribute,
            properties: PropertyInput { ident, .. },
        } in attributes
        {

            // Build the `VersionEntityAttribute` enum
            entities_enum.extend(quote! { #attribute(#attribute), });
            entities_from_impls.extend(quote! {
                #[automatically_derived]
                impl From<#attribute> for VersionEntityAttribute {
                    #[inline]
                    fn from(item: #attribute) -> Self {
                        Self::#attribute(item)
                    }
                }
            });

            // Register the entity types with the resolver
            vanilla_register.extend(quote! { storage.register::<#attribute>(); });
            vanilla_resolve.extend(quote! {
                #ident => VersionEntityAttribute::#attribute(#attribute),
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
            #[derive(Debug, Clone, Copy, PartialEq)]
            #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Clone, PartialEq))]
            pub enum VersionEntityAttribute {
                #entities_enum
            }
            #entities_from_impls

            #[automatically_derived]
            impl #entity_path::entity_attribute::EntityAttributeResolver<#version> for #common_path::vanilla::Vanilla {
                type AttributeEnum = VersionEntityAttribute;
                fn register(storage: &mut #entity_path::entity_attribute::EntityAttributeStorage<#version>) {
                    #vanilla_register
                }
                fn resolve(attribute: &dyn #entity_path::entity_attribute::EntityAttributeTrait<#version>) -> Option<VersionEntityAttribute> {
                    hashify::map!(
                        attribute.identifier().as_bytes(),
                        VersionEntityAttribute,
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
    attributes: Vec<AttributeInput>,
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
            attributes: input
                .parse_terminated(AttributeInput::parse, Token![,])?
                .into_iter()
                .collect(),
        })
    }
}

/// Example:
///
/// ```text
/// Cat                => { properties: { ... }, attributes: { ... } }
/// ```
struct AttributeInput {
    attribute: Ident,
    properties: PropertyInput,
}
impl Parse for AttributeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute = input.parse()?;
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
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown attribute key '{unk}'"),
                    ));
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
            attribute,
            properties: properties.ok_or_else(|| {
                syn::Error::new(content.span(), "missing attribute key 'properties'")
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
    translation: LitStr,
    default: Literal,
    range: PatRange,
}
impl Parse for PropertyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = None;
        let mut translation = None;
        let mut default = None;
        let mut range = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "ident" => {
                    ident = Some(input.parse()?);
                }
                "key" => {
                    translation = Some(input.parse()?);
                }
                "default" => {
                    default = Some(input.parse()?);
                }
                "range" => {
                    range = Some(input.parse()?);
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
            if ident.is_some() && translation.is_some() && default.is_some() && range.is_some() {
                break;
            }
        }

        Ok(Self {
            ident: ident
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'ident'"))?,
            translation: translation
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'key'"))?,
            default: default
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'default'"))?,
            range: range
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'range'"))?,
        })
    }
}
