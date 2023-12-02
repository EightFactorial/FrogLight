use std::path::Path;

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{File, Ident, ImplItem, Item, ItemImpl};
use tokio::io::AsyncWriteExt;
use tracing::debug;

use super::Folder;

pub(super) async fn create_file(dir: &Path, name: &str, folder: &Folder) -> tokio::io::Result<()> {
    let name = name.trim_end_matches("_file").to_string();

    // Get the directory name.
    let dir_name = dir.file_name().unwrap().to_str().unwrap().to_string();

    // Create the file path.
    let file_path = dir.join(name.clone() + ".rs");
    let short_path = short_path(&file_path);

    // Skip if the file already exists.
    if file_path.exists() {
        debug!("Skipping: {short_path}");
        return Ok(());
    } else if !dir.exists() {
        // Create the directory if it doesn't exist.
        tokio::fs::create_dir_all(dir).await?;
    }
    debug!("Creating: {short_path}");

    // Create a new File.
    let mut file = File {
        shebang: None,
        attrs: Vec::new(),
        items: Vec::new(),
    };

    // Add the necessary imports.
    import_modules(&mut file);

    // Add modules.
    if !folder.is_empty() {
        add_modules(&mut file, folder);
    }

    // Create a component.
    let component_name = component_name(&name, &dir_name);
    create_component(&mut file, &component_name);

    // Implement MenuComponent.
    impl_menu(&mut file, &component_name, &dir_name, folder);

    // Create the file.
    let mut output_file = tokio::fs::File::create(file_path).await?;

    let output = prettyplease::unparse(&file);
    output_file.write_all(output.as_bytes()).await?;

    output_file.flush().await?;
    Ok(())
}

/// Get a shorter path for the file, for logging purposes.
fn short_path(path: &Path) -> String {
    let path = path.to_str().unwrap();
    let index = path.find("crates/").unwrap();
    path[index + 7..].to_string()
}

/// Get the component name.
fn component_name(name: &str, dir_name: &str) -> String {
    if name == "mod" {
        dir_name.to_case(Case::Pascal) + "NodeComponent"
    } else {
        name.to_case(Case::Pascal) + "NodeComponent"
    }
}

/// Import the necessary modules.
fn import_modules(file: &mut File) {
    // Add the necessary imports.
    file.items.extend(vec![
        Item::Use(syn::parse_quote!(
            use bevy::prelude::*;
        )),
        Item::Verbatim(TokenStream::new()),
        Item::Use(syn::parse_quote!(
            use crate::menus::traits::MenuComponent;
        )),
    ]);

    // Add a blank line.
    file.items.push(Item::Verbatim(TokenStream::new()));
}

/// Add a `pub mod` entry for each module.
fn add_modules(file: &mut File, folder: &Folder) {
    // Add modules.
    for (name, _) in folder.iter() {
        let name = name.trim_end_matches("_file");
        let ident = Ident::new(name, Span::call_site());
        file.items.push(Item::Mod(syn::parse_quote! {
            pub mod #ident;
        }));
    }

    // Add a blank line.
    file.items.push(Item::Verbatim(TokenStream::new()));
}

/// Create a component for the directory.
fn create_component(file: &mut File, comp_name: &str) {
    // Create the component.
    let ident = Ident::new(comp_name, Span::call_site());
    file.items.push(syn::parse_quote! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
            pub struct #ident;
    });

    // Add a blank line.
    file.items.push(Item::Verbatim(TokenStream::new()));
}

/// Implement MenuComponent for the component.
fn impl_menu(file: &mut File, comp_name: &str, dir_name: &str, folder: &Folder) {
    let ident = Ident::new(comp_name, Span::call_site());
    let mut setup_tokens = TokenStream::new();
    let mut build_tokens = TokenStream::new();

    // Add the node bundle.
    build_tokens.extend(quote! {
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };
    });

    if folder.is_empty() {
        // Add the node to the world.
        build_tokens.extend(quote! {
            world.spawn((#ident, node)).set_parent(parent);
        });
    } else {
        // Add the node to the world, and get its ID.
        build_tokens.extend(quote! {
            let entity = world.spawn((#ident, node)).set_parent(parent).id();
        });

        for (name, _) in folder.iter() {
            let module_name = name.trim_end_matches("_file");
            let module_ident = Ident::new(module_name, Span::call_site());

            let component_name = component_name(name, dir_name);
            let component_ident = Ident::new(&component_name, Span::call_site());

            // Add child systems.
            setup_tokens.extend(quote! {
                #module_ident::#component_ident::setup(app);
            });
            // Build child modules.
            build_tokens.extend(quote! {
                #module_ident::#component_ident::build(entity, world);
            });
        }
    }

    // Build the component implementation.
    let comp_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: Default::default(),
        trait_: Some((None, syn::parse_quote!(MenuComponent), Default::default())),
        self_ty: syn::parse_quote!(#ident),
        brace_token: Default::default(),
        items: vec![
            // Add the setup function.
            if setup_tokens.is_empty() {
                syn::parse2(quote! {
                    fn setup(_app: &mut App) {}
                })
                .unwrap()
            } else {
                syn::parse2(quote! {
                    fn setup(app: &mut App) {
                        #setup_tokens
                    }
                })
                .unwrap()
            },
            // Add a blank line.
            ImplItem::Verbatim(TokenStream::new()),
            // Add the build function.
            syn::parse2(quote! {
                fn build(parent: Entity, world: &mut World) {
                    #build_tokens
                }
            })
            .unwrap(),
        ],
    };

    // Add the implementation.
    file.items.push(Item::Impl(comp_impl));
    file.items.push(Item::Verbatim(TokenStream::new()));
}
