use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ExprArray, Ident, LitBool, LitStr, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

#[allow(clippy::obfuscated_if_else, clippy::too_many_lines)]
pub(crate) fn entity_type_properties(input: TokenStream) -> TokenStream {
    let MacroInput { path, version, entities: blocks } = syn::parse2(input).unwrap();
    let entity_path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-entity"));
    let common_path = CrateManifest::froglight("froglight-common");

    // Generate the block implementations
    let mut output = blocks.iter().fold(
        TokenStream::new(),
        |mut tokens,
         EntityInput {
             entity,
             properties: PropertyInput { ident, group, dimensions, fire_immune },
         }| {
            let (mut dim_x, mut dim_y, mut dim_z) = (None, None, None);
            for (i, dim) in dimensions.elems.iter().take(3).enumerate() {
                match i {
                    0 => dim_x = Some(dim),
                    1 => dim_y = Some(dim),
                    2 => dim_z = Some(dim),
                    _ => unreachable!(),
                }
            }
            #[expect(clippy::manual_let_else)]
            let (dim_x, dim_y, dim_z) = match (dim_x, dim_y, dim_z) {
                (Some(dim_x), Some(dim_y), Some(dim_z)) => (dim_x, dim_y, dim_z),
                _ => panic!("Invalid dimensions array length!"),
            };

            tokens.extend(quote! {
                #[automatically_derived]
                impl #entity_path::entity_type::EntityTypeTrait<#version> for #entity {
                    #[inline]
                    fn identifier(&self) -> &'static #common_path::identifier::Identifier {
                        static IDENTIFIER: #common_path::identifier::Identifier = #common_path::identifier::Identifier::const_new(<#entity as #entity_path::entity_type::EntityTypeExt<#version>>::IDENTIFIER);
                        &IDENTIFIER
                    }

                    #[inline]
                    fn spawn_group(&self) -> &'static #common_path::identifier::Identifier {
                        static SPAWN_GROUP: #common_path::identifier::Identifier = #common_path::identifier::Identifier::const_new(<#entity as #entity_path::entity_type::EntityTypeExt<#version>>::SPAWN_GROUP);
                        &SPAWN_GROUP
                    }

                    #[inline]
                    fn dimensions(&self) -> glam::Vec3 { <Self as #entity_path::entity_type::EntityTypeExt<#version>>::DIMENSIONS }

                    #[inline]
                    fn fire_immunity(&self) -> bool { <Self as #entity_path::entity_type::EntityTypeExt<#version>>::FIRE_IMMUNITY }

                    #[inline]
                    #[cfg(feature = "bevy")]
                    fn insert_bundle(&self, entity: &mut bevy_ecs::world::EntityWorldMut) {
                        entity.insert(<Self as #entity_path::entity_type::EntityTypeExt<#version>>::BUNDLE);
                    }
                }
                #[automatically_derived]
                impl #entity_path::entity_type::EntityTypeExt<#version> for #entity {
                    #[cfg(feature = "bevy")]
                    type BundleType = (Self, #entity_path::entity_type::EntityCollider, #entity_path::entity_type::EntityEyeHeight);
                    #[cfg(feature = "bevy")]
                    const BUNDLE: Self::BundleType = (
                        Self,
                        #entity_path::entity_type::EntityCollider::new(#entity_path::entity_type::Aabb3d { min: glam::Vec3A::ZERO, max: glam::Vec3A::new(#dim_x, #dim_y, #dim_x) }),
                        #entity_path::entity_type::EntityEyeHeight::new(#dim_z),
                    );

                    const IDENTIFIER: &'static str = #ident;
                    const SPAWN_GROUP: &'static str = #group;

                    const DIMENSIONS: glam::Vec3 = glam::Vec3::new(#dim_x, #dim_y, #dim_z);
                    const FIRE_IMMUNITY: bool = #fire_immune;
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

        for EntityInput {
            entity,
            properties: PropertyInput { ident, .. },
        } in blocks
        {

            // Build the `VersionEntityType` enum
            entities_enum.extend(quote! { #entity(#entity), });
            entities_from_impls.extend(quote! {
                #[automatically_derived]
                impl From<#entity> for VersionEntityType {
                    #[inline]
                    fn from(entity: #entity) -> Self {
                        Self::#entity(entity)
                    }
                }
            });

            // Register the entity types with the resolver
            vanilla_register.extend(quote! { storage.register::<#entity>(); });
            vanilla_resolve.extend(quote! {
                #ident => VersionEntityType::#entity(#entity),
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
            pub enum VersionEntityType {
                #entities_enum
            }
            #entities_from_impls

            #[automatically_derived]
            impl #entity_path::entity_type::EntityTypeResolver<#version> for #common_path::vanilla::Vanilla {
                type EntityEnum = VersionEntityType;
                fn register(storage: &mut #entity_path::entity_type::EntityTypeStorage<#version>) {
                    #vanilla_register
                }
                fn resolve(entity_type: &dyn #entity_path::entity_type::EntityTypeTrait<#version>) -> Option<VersionEntityType> {
                    hashify::map!(
                        entity_type.identifier().as_bytes(),
                        VersionEntityType,
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
    entities: Vec<EntityInput>,
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
            entities: input.parse_terminated(EntityInput::parse, Token![,])?.into_iter().collect(),
        })
    }
}

/// Example:
///
/// ```text
/// Cat                => { properties: { ... }, attributes: { ... } }
/// ```
struct EntityInput {
    entity: Ident,
    properties: PropertyInput,
}
impl Parse for EntityInput {
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
                    return Err(syn::Error::new(key.span(), format!("unknown entity key '{unk}'")));
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
            entity: block,
            properties: properties.ok_or_else(|| {
                syn::Error::new(content.span(), "missing entity key 'properties'")
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
    group: LitStr,
    dimensions: ExprArray,
    fire_immune: LitBool,
}
impl Parse for PropertyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = None;
        let mut group = None;
        let mut dimensions = None;
        let mut fire_immune = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "ident" => {
                    ident = Some(input.parse()?);
                }
                "group" => {
                    group = Some(input.parse()?);
                }
                "dimensions" => {
                    dimensions = Some(input.parse()?);
                }
                "fire_immune" => {
                    fire_immune = Some(input.parse()?);
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
            if ident.is_some() && group.is_some() && dimensions.is_some() && fire_immune.is_some() {
                break;
            }
        }

        Ok(Self {
            ident: ident
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'ident'"))?,
            group: group
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'group'"))?,
            dimensions: dimensions.ok_or_else(|| {
                syn::Error::new(input.span(), "missing property key 'dimensions'")
            })?,
            fire_immune: fire_immune.ok_or_else(|| {
                syn::Error::new(input.span(), "missing property key 'fire_immune'")
            })?,
        })
    }
}
