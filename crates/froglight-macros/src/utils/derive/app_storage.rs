use darling::{FromDeriveInput, FromMeta, usage::GenericsExt, util::Flag};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Ident, LitStr, TypePath};

pub(crate) fn derive_app_storage(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let AppStorageMacro { index, manual, reflect, bevy } =
        AppStorageMacro::from_derive_input(&input).unwrap();

    let DeriveInput { vis, ident, generics, .. } = input;
    let app_ident = Ident::new(&format!("App{ident}"), ident.span());

    let mut generic_params =
        generics.declared_type_params().into_iter().fold(TokenStream::new(), |mut acc, param| {
            acc.extend(quote! { #param, });
            acc
        });
    if !generic_params.is_empty() {
        generic_params = quote! { <#generic_params> };
    }

    let mut index_tokens = TokenStream::new();
    if let Some(index) = index {
        let index_ident = &index.ident;

        let mut attr_tokens = TokenStream::new();

        // Add the `Reflect` derive if a flag for the feature is given.
        if let Some(reflect) = &reflect {
            attr_tokens.extend(quote! {
                #[cfg_attr(feature = #reflect, derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
            });
        }

        // Generate the index struct
        index_tokens.extend(quote! {
            /// An index for accessing items in type storage.
            ///
            /// # Warning
            /// There is no guarantee that the given index is valid or represents the
            /// same index between versions. Indices may even change between program runs!
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref)]
            #attr_tokens
            #vis struct #index;
        });

        // Only generate `new_unchecked` if the index is not done manually.
        if !manual.is_present() {
            index_tokens.extend(quote! {
                impl #index_ident {
                    /// Create a new index with the given value.
                    ///
                    /// # Warning
                    /// There is no guarantee that the given index is valid or represents the
                    /// same index between versions. Indices may even change between program runs!
                    #[inline]
                    #[must_use]
                    pub const fn new_unchecked(index: usize) -> Self { Self(index as _) }
                }
            });
        }
    }

    let mut attr_tokens = TokenStream::new();

    // Add the `Reflect` derive if a flag for the feature is given.
    if let Some(reflect) = &reflect {
        attr_tokens.extend(quote! {
            #[cfg_attr(feature = #reflect, derive(Reflect), reflect(Clone, AppStorage))]
        });
    }
    // Add the `Resource` derive if a flag for the feature is given.
    if let Some(bevy) = &bevy {
        attr_tokens.extend(quote! {
            #[cfg_attr(feature = #bevy, derive(Resource))]
        });
    }
    // Reflect the `Resource` trait if flags for both
    // the `reflect` and `bevy` features are given.
    if let (Some(reflect), Some(bevy)) = (&reflect, &bevy) {
        attr_tokens.extend(quote! {
            #[cfg_attr(all(feature = #bevy, feature = #reflect), reflect(Resource))]
        });
    }

    quote! {
        /// A thread-safe, run-time modifiable storage for values
        /// accessible by either a [`TypeId`] or an index.
        #[derive(Clone, derive_more::Deref, derive_more::From, derive_more::Into)]
        #[repr(transparent)]
        #attr_tokens
        #vis struct #app_ident #generics (Arc<::parking_lot::RwLock<#ident #generic_params >>);

        impl #generics #app_ident #generic_params {
            /// Create a new `AppStorage` with the given storage.
            #[must_use]
            pub fn from_storage(storage: #ident #generic_params) -> Self {
                Self(Arc::new(::parking_lot::RwLock::new(storage)))
            }
        }


        #index_tokens
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(FromDeriveInput)]
#[darling(attributes(storage))]
struct AppStorageMacro {
    #[darling(default)]
    index: Option<IndexItem>,
    manual: Flag,

    #[darling(default)]
    reflect: Option<LitStr>,
    #[darling(default)]
    bevy: Option<LitStr>,
}

#[derive(FromMeta)]
struct IndexItem {
    ident: Ident,
    inner: TypePath,
}

impl ToTokens for IndexItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IndexItem { ident, inner } = self;
        tokens.extend(quote! {
            #ident(#inner)
        });
    }
}
