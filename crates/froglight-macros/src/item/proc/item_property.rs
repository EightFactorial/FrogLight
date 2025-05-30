use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

#[allow(clippy::too_many_lines)]
pub(crate) fn item_properties(input: TokenStream) -> TokenStream {
    let MacroInput { path, version, items } = syn::parse2(input).unwrap();
    let item_path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-item"));
    let common_path = CrateManifest::froglight("froglight-common");
    let nbt_path = CrateManifest::froglight("froglight-nbt");

    // Generate the item implementations
    let mut output = items.iter().fold(
        TokenStream::new(),
        |mut tokens, ItemInput { item, properties: PropertyInput { ident, rarity } }| {

            tokens.extend(quote! {
                #[automatically_derived]
                impl #item_path::item::ItemType<#version> for #item {
                    #[inline]
                    #[must_use]
                    fn identifier(&self) -> &'static #common_path::identifier::Identifier {
                        static IDENTIFIER: #common_path::identifier::Identifier = #common_path::identifier::Identifier::const_new(<#item as #item_path::item::ItemTypeExt<#version>>::IDENTIFIER);
                        &IDENTIFIER
                    }

                    #[inline]
                    #[must_use]
                    fn default_nbt(&self) -> #nbt_path::nbt::UnnamedNbt { <#item as #item_path::item::ItemTypeExt<#version>>::default_nbt() }

                    #[inline]
                    #[must_use]
                    fn rarity(&self) -> #item_path::item::ItemRarity { <#item as #item_path::item::ItemTypeExt<#version>>::RARITY }
                }
                #[automatically_derived]
                impl #item_path::item::ItemTypeExt<#version> for #item {
                    const IDENTIFIER: &'static str = #ident;
                    const RARITY: #item_path::item::ItemRarity = #rarity;
                    fn default_nbt() -> #nbt_path::nbt::UnnamedNbt { #nbt_path::nbt::UnnamedNbt::default() }
                }
            });

            tokens
        },
    );

    // Generate the item resolver implementation and tests
    output.extend({
        let mut items_enum = TokenStream::new();
        let mut items_from_impls = TokenStream::new();

        let mut vanilla_register = TokenStream::new();
        let mut vanilla_resolve = TokenStream::new();
        let mut resolver_tests = TokenStream::new();

        let mut item_tests = TokenStream::new();

        for ItemInput {
            item,
            properties: PropertyInput { ident, rarity },
        } in items
        {

            // Build the `VersionItems` enum
            items_enum.extend(quote! { #item(#item_path::item::Item<#item, #version>), });
            items_from_impls.extend(quote! {
                #[automatically_derived]
                impl From<#item_path::item::Item<#item, #version>> for VersionItems {
                    #[inline]
                    fn from(item: #item_path::item::Item<#item, #version>) -> Self {
                        Self::#item(item)
                    }
                }
            });

            // Register the items with the resolver
            vanilla_register.extend(quote! { storage.register::<#item>(); });
            vanilla_resolve.extend(quote! {
                #ident => { return item.downcast::<#item>().map(VersionItems::#item) },
            });

            // Create resolver tests
            resolver_tests.extend(quote! {{
                let item = storage.get_untyped(GlobalItemId::new_unchecked_u32(global), None).expect("No item found for expected GlobalItemId!");
                assert_eq!(item.identifier().as_str(), #ident, "Item \"{}\" identifier mismatch!", #ident);
                assert_eq!(item.clone().resolve::<Vanilla>(), item.clone().downcast().map(VersionItems::#item), "Failed to resolve \"{}\"!", #ident);
                global += 1;
            }});

            item_tests.extend(quote! {{
                    let mut item = #item_path::item::Item::<#item, #version>::default();
                    assert_eq!(item, item, "Item \"{}\" equality failed!", #ident);

                    assert_eq!(<#item as #item_path::item::ItemTypeExt<#version>>::IDENTIFIER, #ident, "Item \"{}\" typed identifier mismatch!", #ident);
                    assert_eq!(<#item as #item_path::item::ItemTypeExt<#version>>::RARITY, #item_path::item::#rarity, "Item \"{}\" rarity mismatch!", #ident);

                    assert_eq!(item.identifier().as_str(), #ident, "Item \"{}\" typed identifier mismatch!", #ident);
                    assert_eq!(item.rarity(), #item_path::item::#rarity, "Item \"{}\" rarity mismatch!", #ident);

                    let item = item.into_untyped();
                    assert_eq!(item.identifier().as_str(), #ident, "Item \"{}\" untyped identifier mismatch!", #ident);
                    assert_eq!(item.rarity(), #item_path::item::#rarity, "Item \"{}\" untyped rarity mismatch!", #ident);
                    assert!(item.downcast::<#item>().is_ok(), "Item \"{}\" downcast failed!", #ident);
            }});
        }

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum VersionItems {
                #items_enum
            }
            #items_from_impls

            #[automatically_derived]
            impl #item_path::resolve::ItemResolver<#version> for #common_path::vanilla::Vanilla {
                type ItemEnum = VersionItems;
                fn register(storage: &mut #item_path::storage::ItemStorage<#version>) {
                    #vanilla_register
                }
                fn resolve(item: #item_path::item::UntypedItem<#version>) -> Result<VersionItems, #item_path::item::UntypedItem<#version>> {
                    hashify::fnc_map!(
                        item.identifier().as_bytes(),
                        #vanilla_resolve
                        _ => { return Err(item) }
                    );
                    unreachable!("All possible cases handled by `hashify::fnc_map` macro")
                }
            }

            #[cfg(test)]
            mod test {
                use super::*;
                use #item_path::prelude::*;
                use #common_path::vanilla::Vanilla;

                #[test]
                fn items() {
                    #item_tests
                }

                #[test]
                fn resolver() {
                    let mut storage = #item_path::storage::ItemStorage::<#version>::new();
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
/// froglight_macros::item_properties! {
///     crate,
///     version = froglight_common::version::V1_21_4,
///     ...
/// }
/// ```
struct MacroInput {
    path: Option<Path>,
    version: Path,
    items: Vec<ItemInput>,
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
            items: input.parse_terminated(ItemInput::parse, Token![,])?.into_iter().collect(),
        })
    }
}

/// Example:
///
/// ```text
/// AndesiteSlab                         => { properties: { ... } }
/// ```
struct ItemInput {
    item: Ident,
    properties: PropertyInput,
}
impl Parse for ItemInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = input.parse()?;
        input.parse::<Token![=>]>()?;

        let content;
        syn::braced!(content in input);

        let mut properties = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "properties" => {
                    let braced;
                    syn::braced!(braced in content);

                    properties = Some(braced.parse()?);
                }
                unk => {
                    return Err(syn::Error::new(key.span(), format!("unknown item key '{unk}'")));
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
            item,
            properties: properties
                .ok_or_else(|| syn::Error::new(content.span(), "missing item key 'properties'"))?,
        })
    }
}

/// Example:
///
/// ```text
/// properties: { ident: "minecraft:andesite_slab", rarity: ItemRarity::Common }
/// ```
struct PropertyInput {
    ident: LitStr,
    rarity: Path,
}
impl Parse for PropertyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ident = None;
        let mut rarity = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "ident" => {
                    ident = Some(input.parse()?);
                }
                "rarity" => {
                    rarity = Some(input.parse()?);
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
            if ident.is_some() && rarity.is_some() {
                break;
            }
        }

        Ok(Self {
            ident: ident
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'ident'"))?,
            rarity: rarity
                .ok_or_else(|| syn::Error::new(input.span(), "missing property key 'rarity'"))?,
        })
    }
}
