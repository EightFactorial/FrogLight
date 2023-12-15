use std::{future::Future, path::Path, pin::Pin};

use convert_case::{Case, Casing};
use git2::Repository;
use mc_rs_extract::{modules::ExtractModule, ModuleData};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{Item, ItemImpl, Type, TypePath};
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{debug, error, info};

use crate::modules::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlockVersionModule;

impl ModuleExt for BlockVersionModule {
    fn deps(&self) -> &'static [ExtractModule] { &[ExtractModule::BlockStates] }

    fn run<'a>(
        &self,
        data: &'a mut ModuleData,
        repo: &Repository,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        let mut impl_path = repo.path().parent().unwrap().to_path_buf();
        impl_path.push("crates/mc-rs-world/src/blocks/versions");
        let version_file_name = data.version.to_string().replace('.', "_");
        impl_path.push(format!("v{version_file_name}.rs"));

        Box::pin(async move {
            info!("Generating block impls");
            if let Err(err) = Self::generate_impls(&impl_path, data).await {
                error!("Failed to generate block impls: {err}");
            }
        })
    }
}

impl BlockVersionModule {
    async fn generate_impls(path: &Path, data: &ModuleData) -> Result<(), &'static str> {
        debug!("BlockVersion file: {}", path.display());

        let version_crate = format!("v{}", data.version.to_string().replace('.', "_"));
        let version_crate_ident = Ident::new(&version_crate, Span::call_site());

        let version_struct = format!("V{}", data.version.to_string().replace('.', "_"));
        let version_struct_ident = Ident::new(&version_struct, Span::call_site());

        let mut options = File::options();
        options.read(true).write(true).create(true).truncate(true);

        let mut file = options
            .open(path)
            .await
            .map_err(|_| "Failed to create/open block version file")?;

        let mut output = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: Vec::new(),
        };

        // Add the imports
        {
            output.items.push(Item::Use(syn::parse_quote! {
                use crate::blocks::{
                    attributes::*,
                    structs::*,
                    traits::{BlockTrait, BlocksTrait},
                    Blocks,
                };
            }));
            output.items.push(Item::Use(syn::parse_quote! {
                use mc_rs_protocol::versions::#version_crate_ident::#version_struct_ident;
            }));
            output.items.push(Item::Verbatim(TokenStream::new()));
        }

        // Parse the blocks into a list of Idents
        let mut blocks = Vec::new();
        {
            let error_name = Ident::new("BlockError", Span::call_site());
            let error_variant = Ident::new("Error", Span::call_site());
            blocks.push((error_variant, error_name));

            for block_name in data.output["blocks"]["list"].members() {
                let variant_name = block_name.as_str().unwrap().to_case(Case::Pascal);
                let block_name = format!("Block{variant_name}");

                let variant_name = Ident::new(&variant_name, Span::call_site());
                let block_name = Ident::new(&block_name, Span::call_site());

                blocks.push((variant_name, block_name));
            }
        }

        // Implement BlocksTrait<Version>
        {
            let mut impl_blocks = ItemImpl {
                attrs: Vec::new(),
                defaultness: None,
                unsafety: None,
                impl_token: Default::default(),
                generics: Default::default(),
                trait_: Some((
                    None,
                    syn::parse_quote! { BlocksTrait<#version_struct_ident> },
                    Default::default(),
                )),
                self_ty: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Ident::new("Blocks", Span::call_site()).into(),
                })),
                brace_token: Default::default(),
                items: Vec::new(),
            };

            // Implement resource_location
            {
                let mut tokens = TokenStream::new();
                for (variant_name, _) in &blocks {
                    tokens.extend(quote::quote! {
                        Self::#variant_name(b) => b.resource_location(),
                    });
                }

                impl_blocks.items.push(syn::parse_quote! {
                    fn resource_location(&self) -> &'static str {
                        match self {
                            #tokens
                        }
                    }
                });
            }

            // Implement to_u32
            {
                let mut tokens = TokenStream::new();

                for (variant_name, _) in &blocks {
                    tokens.extend(quote::quote! {
                        Self::#variant_name(b) => b.to_u32(),
                    });
                }

                impl_blocks.items.push(syn::parse_quote! {
                    fn to_u32(&self) -> u32 {
                        match self {
                            #tokens
                        }
                    }
                });
            }

            // Implement from_u32
            {
                let mut tokens = TokenStream::new();

                for (block_name, block_data) in data.output["blocks"]["data"].entries() {
                    let block_ident = format!("Block{}", block_name.to_case(Case::Pascal));
                    let block_ident = Ident::new(&block_ident, Span::call_site());

                    let min = block_data["state_ids"]["min"].as_u32().unwrap();
                    let max = block_data["state_ids"]["max"].as_u32().unwrap();

                    let matcher = if min == max {
                        quote::quote! { #min }
                    } else {
                        quote::quote! { #min..=#max }
                    };

                    tokens.extend(quote::quote! {
                         #matcher => #block_ident::try_from_u32(id).map(Blocks::from).unwrap_or(Self::Error(BlockError)),
                    });
                }

                tokens.extend(quote::quote! {
                    _ => Self::Error(BlockError),
                });

                impl_blocks.items.push(syn::parse_quote! {
                    fn from_u32(id: u32) -> Self {
                        match id {
                            #tokens
                        }
                    }
                });
            }

            // Implement is_air
            {
                let mut tokens = TokenStream::new();

                for (variant_name, _) in &blocks {
                    tokens.extend(quote::quote! {
                        Self::#variant_name(b) => b.is_air(),
                    });
                }

                impl_blocks.items.push(syn::parse_quote! {
                    fn is_air(&self) -> bool {
                        match self {
                            #tokens
                        }
                    }
                });
            }

            // Implement is_opaque
            {
                let mut tokens = TokenStream::new();

                for (variant_name, _) in &blocks {
                    tokens.extend(quote::quote! {
                        Self::#variant_name(b) => b.is_opaque(),
                    });
                }

                impl_blocks.items.push(syn::parse_quote! {
                    fn is_opaque(&self) -> bool {
                        match self {
                            #tokens
                        }
                    }
                });
            }

            // Implement is_collidable
            {
                let mut tokens = TokenStream::new();

                for (variant_name, _) in &blocks {
                    tokens.extend(quote::quote! {
                        Self::#variant_name(b) => b.is_collidable(),
                    });
                }

                impl_blocks.items.push(syn::parse_quote! {
                    fn is_collidable(&self) -> bool {
                        match self {
                            #tokens
                        }
                    }
                });
            }

            output.items.push(syn::Item::Impl(impl_blocks));
            output.items.push(Item::Verbatim(TokenStream::new()));
        }

        // Implement BlockTrait<Version>
        {
            output.items.push(Item::Impl(syn::parse_quote! {
                impl BlockTrait<#version_struct_ident> for BlockError {
                    fn resource_location(&self) -> &'static str {
                        "mc-rs:error"
                    }

                    fn try_from_u32(_: u32) -> Option<Self> {
                        Some(Self)
                    }

                    fn to_u32(&self) -> u32 {
                        u32::MAX
                    }
                }
            }));
            output.items.push(Item::Verbatim(TokenStream::new()));

            for (block_name, block_data) in data.output["blocks"]["data"].entries() {
                let block_ident = format!("Block{}", block_name.to_case(Case::Pascal));
                let block_ident = Ident::new(&block_ident, Span::call_site());

                // Implement `resource_location`
                let resource_location = {
                    let name = format!("minecraft:{block_name}");
                    quote::quote! {
                        fn resource_location(&self) -> &'static str {
                            #name
                        }
                    }
                };

                // Implement `try_from_u32` and `to_u32`
                let (try_from, to_u32) = {
                    let min = block_data["state_ids"]["min"].as_u32().unwrap();
                    let max = block_data["state_ids"]["max"].as_u32().unwrap();

                    if block_data["attributes"].is_empty() && min == max {
                        (
                            quote::quote! {
                               fn try_from_u32(_: u32) -> Option<Self> {
                                   Some(Self)
                               }

                            },
                            quote::quote! {
                               fn to_u32(&self) -> u32 {
                                   #min
                               }
                            },
                        )
                    } else {
                        (
                            quote::quote! {
                               fn try_from_u32(_id: u32) -> Option<Self> {
                                   Some(Self::default())
                               }
                            },
                            quote::quote! {
                              fn to_u32(&self) -> u32 {
                                  #min
                              }
                            },
                        )
                    }
                };

                let (is_air, is_opaque, is_collidable) = {
                    let is_air = block_data["is_air"].as_bool().unwrap();
                    let is_opaque = block_data["opaque"].as_bool().unwrap();
                    let is_collidable = block_data["collidable"].as_bool().unwrap();

                    (
                        match is_air {
                            true => quote::quote! {
                               fn is_air(&self) -> bool {
                                   true
                               }
                            },
                            false => quote::quote!(),
                        },
                        match is_opaque {
                            true => quote::quote!(),
                            false => quote::quote! {
                               fn is_opaque(&self) -> bool {
                                   false
                               }
                            },
                        },
                        match is_collidable {
                            true => quote::quote!(),
                            false => quote::quote! {
                               fn is_collidable(&self) -> bool {
                                   false
                               }
                            },
                        },
                    )
                };

                output.items.push(Item::Impl(syn::parse_quote! {
                    impl BlockTrait<#version_struct_ident> for #block_ident {
                        #resource_location
                        #try_from
                        #to_u32

                        #is_air
                        #is_opaque
                        #is_collidable
                    }
                }));
                output.items.push(Item::Verbatim(TokenStream::new()));
            }
        }

        // Write the block impls to the file.
        {
            let results = prettyplease::unparse(&output);

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
}
