use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    token::Brace,
    Ident, LitInt, Token,
};

/// Input:
/// ```rust,ignore
/// frog_create_block_impls! {
///     // Version
///     V1_21_0,
///     Attributes => {
///         Attrib: [One, Two, Three],
///     },
///     Blocks => {
///         EmptyBlock,
///         Block {
///             // Permutation index
///             default: 1,
///             fields: {
///                 // Attribute index
///                 field_name: 0
///             },
///             permutations: [
///                 // Attribute value index
///                 [0], [1], [2]
///             ],
///         },
///     },
/// }
/// ```
///
/// Output:
/// ```rust,ignore
/// impl BlockExt<V1_21_0> for EmptyBlock {
///     fn default_state() -> Self { EmptyBlock }
/// }
/// impl BlockExt<V1_21_0> for Block {
///     const BLOCK_STATES: u32 = 3;
///     fn from_relative_id(id: u32) -> Option<Self> {
///         match id {
///             0 => Some(Self { field_name: Attrib::One }),
///             1 => Some(Self { field_name: Attrib::Two }),
///             2 => Some(Self { field_name: Attrib::Three }),
///             _ => None,
///         }
///     }
///     fn to_relative_id(&self) -> u32 {
///         match self {
///             Self { field_name: Attrib::One } => 0,
///             Self { field_name: Attrib::Two } => 1,
///             Self { field_name: Attrib::Three } => 2,
///        }
///     }
///     fn default_state() -> Self {
///         Self {
///             field_name: Attrib::Two
///         }
///     }
/// }
/// ```
pub(crate) fn generate_block_impls(tokens: proc_macro::TokenStream) -> TokenStream {
    let BlockImplMacro { version, attributes, blocks } =
        syn::parse(tokens).expect("Failed to parse block list");
    let mut tokenstream = TokenStream::new();

    // Create an implementation for `VanillaResolver`
    {
        let mut register_tokens = TokenStream::new();
        for block in &blocks {
            let name = &block.name;
            register_tokens.extend(quote! {
                storage.register::<#name>();
            });
        }

        tokenstream.extend(quote! {
            impl BlockStateResolver<#version> for VanillaResolver {
                type Resolved = Blocks;
                fn resolve_state(blockstate_id: u32, storage: &BlockStorage<#version>) -> Self::Resolved { todo!() }
                fn register_blocks(storage: &mut BlockStorage<#version>) {
                    #register_tokens
                }
            }
        });
    }

    // Create an implementation for `BlockExt<V>`
    {
        for block in blocks {
            let name = block.name;

            match block.data {
                BlockData::Default => {
                    tokenstream.extend(quote! {
                        impl BlockExt<#version> for #name {
                            fn default_state() -> Self { #name }
                        }
                    });
                }
                BlockData::Fields { default, fields, permutations } => {
                    let mut from_relative_id_tokens = TokenStream::new();
                    let mut to_relative_id_tokens = TokenStream::new();
                    let mut default_state_tokens = TokenStream::new();

                    for (i, permutation) in permutations.iter().enumerate() {
                        let mut fields_tokens = TokenStream::new();
                        for (field_index, field) in fields.iter().enumerate() {
                            let field_name = &field.name;
                            let attribute_index = field.attribute_index;

                            let attribute_type = &attributes[attribute_index].name;
                            let attribute_value =
                                &attributes[attribute_index].values[permutation[field_index]];

                            if attribute_type.to_string().contains("Boolean") {
                                let attribute_value = attribute_value.to_string();
                                if attribute_value == "True" {
                                    fields_tokens.extend(quote! {
                                        #field_name: #attribute_type(true),
                                    });
                                } else {
                                    fields_tokens.extend(quote! {
                                        #field_name: #attribute_type(false),
                                    });
                                }
                            } else {
                                fields_tokens.extend(quote! {
                                    #field_name: #attribute_type::#attribute_value,
                                });
                            }
                        }

                        let i_u32 = u32::try_from(i).expect("Too many permutations for u32");

                        from_relative_id_tokens.extend(quote! {
                            #i_u32 => Some(Self { #fields_tokens }),
                        });

                        to_relative_id_tokens.extend(quote! {
                            Self { #fields_tokens } => #i_u32,
                        });

                        if i == default {
                            default_state_tokens.extend(quote! {
                                Self { #fields_tokens }
                            });
                        }
                    }

                    let permutations_len =
                        u32::try_from(permutations.len()).expect("Too many permutations for u32");

                    tokenstream.extend(quote! {
                        impl BlockExt<#version> for #name {
                            const BLOCK_STATES: u32 = #permutations_len;
                            fn from_relative_id(id: u32) -> Option<Self> {
                                match id {
                                    #from_relative_id_tokens
                                    _ => None,
                                }
                            }
                            fn to_relative_id(&self) -> u32 {
                                match self {
                                    #to_relative_id_tokens
                                }
                            }
                            fn default_state() -> Self {
                                #default_state_tokens
                            }
                        }
                    });
                }
            }
        }
    }

    tokenstream
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlockImplMacro {
    version: Ident,
    attributes: Vec<Attribute>,
    blocks: Vec<Block>,
}

impl Parse for BlockImplMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let version: Ident = input.parse()?;
        let _comma: Token![,] = input.parse()?;

        let mut attributes: Vec<Attribute> = Vec::new();
        {
            let _attributes: Ident = input.parse()?;
            let _fat_arrow: Token![=>] = input.parse()?;

            let braced;
            syn::braced!(braced in input);

            while !braced.is_empty() {
                match braced.parse::<Attribute>() {
                    Ok(attribute) => attributes.push(attribute),
                    Err(err) => {
                        panic!("Failed to parse attribute: {err}");
                    }
                }
            }
        }
        if input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
        }

        let mut blocks: Vec<Block> = Vec::new();
        {
            let _attributes: Ident = input.parse()?;
            let _fat_arrow: Token![=>] = input.parse()?;

            let braced;
            syn::braced!(braced in input);

            while !braced.is_empty() {
                match braced.parse::<Block>() {
                    Ok(block) => blocks.push(block),
                    Err(err) => {
                        panic!("Failed to parse block: {err}");
                    }
                }
            }
        }
        if input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
        }

        Ok(BlockImplMacro { version, attributes, blocks })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Attribute {
    name: Ident,
    values: Vec<Ident>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrib_type: Ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;

        let mut values = Vec::new();
        {
            let bracketed;
            syn::bracketed!(bracketed in input);

            while !bracketed.is_empty() {
                values.push(bracketed.parse()?);

                if bracketed.peek(Token![,]) {
                    let _comma: Token![,] = bracketed.parse()?;
                }
            }
        }

        if input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
        }

        Ok(Attribute { name: attrib_type, values })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    name: Ident,
    data: BlockData,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        if input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;

            Ok(Block { name, data: BlockData::Default })
        } else if input.peek(Brace) {
            let data = {
                let braced;
                syn::braced!(braced in input);

                match braced.parse() {
                    Ok(data) => data,
                    Err(err) => {
                        panic!("Failed to parse block \"{name}\": {err}");
                    }
                }
            };

            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }

            Ok(Block { name, data })
        } else {
            panic!("Unexpected token after block name");
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BlockData {
    Default,
    Fields { default: usize, fields: Vec<Field>, permutations: Vec<Vec<usize>> },
}

impl Parse for BlockData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut default = None;
        let mut fields = Vec::new();
        let mut permutations = Vec::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _colon: Token![:] = input.parse()?;

            if ident == "default" {
                default = Some(input.parse::<LitInt>()?.base10_parse()?);
            } else if ident == "fields" {
                let braced;
                syn::braced!(braced in input);

                while !braced.is_empty() {
                    fields.push(braced.parse()?);

                    if braced.peek(Token![,]) {
                        let _comma: Token![,] = braced.parse()?;
                    }
                }
            } else if ident == "permutations" {
                // permutations: [ ... ]
                let bracketed;
                syn::bracketed!(bracketed in input);

                while !bracketed.is_empty() {
                    let mut permutation = Vec::new();

                    // permutations: [ [ ... ], ... ]
                    let bracketed_sub;
                    syn::bracketed!(bracketed_sub in bracketed);

                    // permutations: [ [0, 1, 2], ... ]
                    while !bracketed_sub.is_empty() {
                        permutation.push(bracketed_sub.parse::<LitInt>()?.base10_parse()?);

                        if bracketed_sub.peek(Token![,]) {
                            let _comma: Token![,] = bracketed_sub.parse()?;
                        }
                    }

                    if bracketed.peek(Token![,]) {
                        let _comma: Token![,] = bracketed.parse()?;
                    }

                    permutations.push(permutation);
                }
            } else {
                panic!("Found unexpected token in block data: \"{ident}\"");
            }

            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(BlockData::Fields {
            default: default.expect("Missing default permutation"),
            fields,
            permutations,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    name: Ident,
    attribute_index: usize,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;
        let attribute_index = input.parse::<LitInt>()?.base10_parse()?;

        Ok(Field { name, attribute_index })
    }
}
