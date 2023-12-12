use std::{future::Future, path::Path, pin::Pin};

use convert_case::{Case, Casing};
use git2::Repository;
use json::JsonValue;
use mc_rs_extract::{modules::ExtractModule, ModuleData};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{
    punctuated::Punctuated, Field, Fields, FieldsNamed, Item, ItemEnum, ItemStruct, Type, TypePath,
    Variant, Visibility,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{debug, error, info};

use crate::modules::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlockListModule;

impl ModuleExt for BlockListModule {
    fn deps(&self) -> &'static [ExtractModule] { &[ExtractModule::BlockStates] }

    fn run<'a>(
        &self,
        data: &'a mut ModuleData,
        repo: &Repository,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        let mut attr_path = repo.path().parent().unwrap().to_path_buf();
        attr_path.push("crates/mc-rs-world/src/blocks");

        let block_structs_path = attr_path.join("structs.rs");
        let blocks_enum_path = attr_path.join("mod.rs");
        attr_path.push("attributes.rs");

        Box::pin(async move {
            info!("Generating block attributes");
            if let Err(err) = Self::generate_attributes(&attr_path, data).await {
                error!("Failed to generate block attributes: {err}");
            }

            info!("Generating block structs");
            if let Err(err) = Self::generate_structs(&block_structs_path, data).await {
                error!("Failed to generate block structs: {err}");
            }

            info!("Generating blocks enum");
            if let Err(err) = Self::generate_blocks_enum(&blocks_enum_path, data).await {
                error!("Failed to generate blocks enum: {err}");
            }
        })
    }
}

impl BlockListModule {
    async fn generate_attributes(path: &Path, data: &ModuleData) -> Result<(), &'static str> {
        debug!("BlockAttributes file: {}", path.display());

        let mut options = File::options();
        options.read(true).write(true).create(true).truncate(true);

        let mut file = options.open(path).await.map_err(|err| {
            debug!("{err}");
            "Failed to create/open block attributes file"
        })?;

        // Parse the block attributes.
        let mut parsed;
        {
            let mut contents = String::new();
            file.read_to_string(&mut contents).await.map_err(|err| {
                debug!("{err}");
                "Failed to read block attributes file"
            })?;

            parsed = syn::parse_file(&contents).map_err(|err| {
                debug!("{err}");
                "Failed to parse block attributes"
            })?;
        }

        // Remove the old block attributes.
        parsed.items.retain(|item| {
            if let syn::Item::Struct(item) = item {
                if let syn::Visibility::Public(_) = item.vis {
                    return false;
                }
            }

            true
        });

        // Add the new block attributes.
        for (attr_name, attr_data) in data.output["blocks"]["attributes"]["values"].entries() {
            let item = Self::generate_attribute(attr_name, attr_data);
            parsed.items.push(item);
            parsed.items.push(Item::Verbatim(TokenStream::new()));
        }

        // Write the block attributes to the file.
        {
            let results = prettyplease::unparse(&parsed);
            file.write_all(results.as_bytes()).await.map_err(|err| {
                debug!("{err}");
                "Failed to write block attributes file"
            })?;
            file.flush().await.map_err(|err| {
                debug!("{err}");
                "Failed to flush block attributes file"
            })?;
        }

        Ok(())
    }

    fn generate_attribute(name: &str, data: &JsonValue) -> Item {
        let name = format!("{}Attribute", name.to_case(Case::Pascal));
        let name = Ident::new(&name, Span::call_site());

        let kind = data["type"].as_str().unwrap();
        match kind {
            "boolean" => {
                let item: ItemStruct = syn::parse_quote! {
                    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
                    pub struct #name(pub bool);
                };

                Item::Struct(item)
            }
            "integer" => {
                let mut variants: Vec<Variant> = Vec::new();

                let min = data["min"].as_u32().unwrap();
                let max = data["max"].as_u32().unwrap();

                for i in min..=max {
                    variants.push(Variant {
                        attrs: if i == min {
                            vec![syn::parse_quote!(#[default])]
                        } else {
                            Vec::new()
                        },
                        ident: Ident::new(&format!("_{i}"), Span::call_site()),
                        fields: Fields::Unit,
                        discriminant: None,
                    });
                }

                let item: ItemEnum = syn::parse_quote! {
                    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
                    pub enum #name {
                        #(#variants),*
                    }
                };

                Item::Enum(item)
            }
            "direction" | "enum" => {
                let mut variants: Vec<Variant> = Vec::new();

                for (index, variant) in data["values"].members().enumerate() {
                    let variant = Ident::new(
                        &variant.as_str().unwrap().to_case(Case::Pascal),
                        Span::call_site(),
                    );

                    variants.push(Variant {
                        attrs: if index == 0 {
                            vec![syn::parse_quote!(#[default])]
                        } else {
                            Vec::new()
                        },
                        ident: variant,
                        fields: Fields::Unit,
                        discriminant: None,
                    });
                }

                let item: ItemEnum = syn::parse_quote! {
                    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
                    pub enum #name {
                        #(#variants),*
                    }
                };

                Item::Enum(item)
            }
            unk => panic!("Unknown attribute type: {unk}"),
        }
    }

    async fn generate_structs(path: &Path, data: &ModuleData) -> Result<(), &'static str> {
        debug!("BlockStructs file: {}", path.display());

        let mut options = File::options();
        options.read(true).write(true).create(true).truncate(true);

        let mut file = options
            .open(path)
            .await
            .map_err(|_| "Failed to create/open block structs file")?;

        // Parse the block structs.
        let mut parsed;
        {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .await
                .map_err(|_| "Failed to read block structs file")?;

            parsed = syn::parse_file(&contents).map_err(|_| "Failed to parse block structs")?;
        }

        // Remove the old block structs.
        parsed.items.retain(|item| {
            if let syn::Item::Struct(item) = item {
                if let syn::Visibility::Public(_) = item.vis {
                    return false;
                }
            }

            true
        });

        // Add modules
        parsed.items.push(Item::Use(syn::parse_quote!(
            use super::attributes::*;
        )));
        parsed.items.push(Item::Verbatim(TokenStream::new()));

        // Add an Error block
        parsed.items.push(Item::Struct(syn::parse_quote! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            pub struct BlockError;
        }));
        parsed.items.push(Item::Verbatim(TokenStream::new()));

        // Add the new block structs.
        for (block_name, block_data) in data.output["blocks"]["data"].entries() {
            let item = Self::create_struct(block_name, block_data);
            parsed.items.push(item.into());
            parsed.items.push(Item::Verbatim(TokenStream::new()));
        }

        // Write the block structs to the file.
        {
            let results = prettyplease::unparse(&parsed);

            file.write_all(results.as_bytes())
                .await
                .map_err(|_| "Failed to write block structs file")?;

            file.flush().await.map_err(|err| {
                debug!("{err}");
                "Failed to flush blocks structs file"
            })?;
        }

        Ok(())
    }

    fn create_struct(name: &str, data: &JsonValue) -> ItemStruct {
        let name = format!("Block{}", name.to_case(Case::Pascal));
        let name = Ident::new(&name, Span::call_site());

        let mut fields = Punctuated::new();
        for attribute in data["attributes"].members() {
            let attribute = attribute.as_str().unwrap().to_lowercase();
            let attr_name = Ident::new(&attribute, Span::call_site());

            let attr_type = format!("{}Attribute", attribute.to_case(Case::Pascal));
            let attr_type = Ident::new(&attr_type, Span::call_site());

            let field = Field {
                attrs: Vec::new(),
                vis: Visibility::Public(Default::default()),
                mutability: syn::FieldMutability::None,
                ident: Some(attr_name),
                colon_token: Default::default(),
                ty: Type::Path(TypePath {
                    qself: None,
                    path: attr_type.into(),
                }),
            };

            fields.push(field);
        }

        let fields = if fields.is_empty() {
            Fields::Unit
        } else {
            Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: fields,
            })
        };

        ItemStruct {
            attrs: vec![syn::parse_quote!(#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)])],
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident: name,
            generics: Default::default(),
            fields,
            semi_token: None,
        }
    }

    async fn generate_blocks_enum(path: &Path, data: &ModuleData) -> Result<(), &'static str> {
        debug!("BlocksEnum file: {}", path.display());

        let mut options = File::options();
        options.read(true).write(true).create(true).truncate(true);

        let mut file = options
            .open(path)
            .await
            .map_err(|_| "Failed to create/open blocks enum file")?;

        // Parse the blocks enum.
        let mut parsed;
        {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .await
                .map_err(|_| "Failed to read blocks enum file")?;

            parsed = syn::parse_file(&contents).map_err(|_| "Failed to parse blocks enum")?;
        }

        // Remove the old blocks enum.
        parsed.items.retain(|item| {
            if let syn::Item::Enum(item) = item {
                if let syn::Visibility::Public(_) = item.vis {
                    return false;
                }
            }

            true
        });

        // Add modules
        parsed.items.push(Item::Mod(syn::parse_quote!(
            pub mod attributes;
        )));
        parsed.items.push(Item::Mod(syn::parse_quote!(
            mod versions;
        )));
        parsed.items.push(Item::Mod(syn::parse_quote!(
            pub mod traits;
        )));
        parsed.items.push(Item::Verbatim(TokenStream::new()));
        parsed.items.push(Item::Mod(syn::parse_quote!(
            pub mod structs;
        )));
        parsed.items.push(Item::Use(syn::parse_quote!(
            use structs::*;
        )));
        parsed.items.push(Item::Verbatim(TokenStream::new()));

        // Add the new blocks enum.
        {
            let mut variants = Punctuated::new();

            for block_name in data.output["blocks"]["list"].members() {
                let variant_name = block_name.as_str().unwrap().to_case(Case::Pascal);
                let block_name = format!("Block{variant_name}");

                let variant_name = Ident::new(&variant_name, Span::call_site());
                let block_name = Ident::new(&block_name, Span::call_site());

                variants.push(syn::parse_quote! {
                    #variant_name(#block_name)
                });
            }

            parsed.items.push(Item::Enum(ItemEnum {
                attrs: vec![syn::parse_quote!(#[derive(Debug, Clone, Copy, PartialEq, Eq)])],
                vis: Visibility::Public(Default::default()),
                enum_token: Default::default(),
                ident: Ident::new("Blocks", Span::call_site()),
                generics: Default::default(),
                brace_token: Default::default(),
                variants,
            }));
        }

        // Write the blocks enum to the file.
        {
            let results = prettyplease::unparse(&parsed);
            file.write_all(results.as_bytes()).await.map_err(|err| {
                debug!("{err}");
                "Failed to write blocks enum file"
            })?;
            file.flush().await.map_err(|err| {
                debug!("{err}");
                "Failed to flush blocks enum file"
            })?;
        }

        Ok(())
    }
}
