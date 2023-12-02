use std::path::Path;

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use syn::{
    punctuated::Punctuated, AttrStyle, Attribute, Fields, File, Generics, Ident, Item, ItemMod,
    ItemStruct, MacroDelimiter, Meta, MetaList, Visibility,
};
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
    debug!("Creating file: {short_path}");

    // Skip if the file already exists.
    if file_path.exists() {
        debug!("Skipping {short_path}");
        return Ok(());
    } else if !dir.exists() {
        // Create the directory if it doesn't exist.
        tokio::fs::create_dir_all(dir).await?;
    }

    // Create a new File.
    let mut file = File {
        shebang: None,
        attrs: Vec::new(),
        items: Vec::new(),
    };

    // Add modules.
    if !folder.is_empty() {
        add_modules(&mut file, folder);
    }

    // Create a component.
    create_component(&mut file, &name, &dir_name);

    // Create the file.
    let mut output_file = tokio::fs::File::create(file_path).await?;
    output_file
        .write_all(prettyplease::unparse(&file).as_bytes())
        .await?;

    output_file.flush().await?;
    Ok(())
}

/// Get a shorter path for the file, for logging purposes.
fn short_path(path: &Path) -> String {
    let path = path.to_str().unwrap();
    let index = path.find("crates/").unwrap();
    path[index + 7..].to_string()
}

/// Add a `pub mod` entry for each module.
fn add_modules(file: &mut File, folder: &Folder) {
    // Add modules.
    for (name, _) in folder.iter() {
        let name = name.trim_end_matches("_file");
        file.items.push(Item::Mod(ItemMod {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            unsafety: None,
            mod_token: Default::default(),
            ident: Ident::new(name, Span::call_site()),
            content: None,
            semi: Some(Default::default()),
        }));
    }

    // Add a blank line.
    file.items.push(Item::Verbatim(TokenStream::new()));
}

/// Create a component for the directory.
fn create_component(file: &mut File, name: &str, dir_name: &str) {
    // Create the component name.
    let component_name = if name == "mod" {
        dir_name.to_case(Case::Pascal) + "NodeComponent"
    } else {
        name.to_case(Case::Pascal) + "NodeComponent"
    };

    let component = Item::Struct(ItemStruct {
        attrs: vec![Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            meta: Meta::List(MetaList {
                path: Ident::new("derive", Span::call_site()).into(),
                delimiter: MacroDelimiter::Paren(Default::default()),
                tokens: quote::quote!(Debug, Default, Clone, Copy, PartialEq, Eq, Hash),
            }),
        }],
        vis: Visibility::Public(Default::default()),
        struct_token: Default::default(),
        ident: Ident::new(&component_name, Span::call_site()),
        generics: Generics {
            lt_token: None,
            params: Punctuated::new(),
            gt_token: None,
            where_clause: None,
        },
        fields: Fields::Unit,
        semi_token: None,
    });

    file.items.push(component);
}
