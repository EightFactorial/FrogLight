use bevy::prelude::*;
use froglight_assets::assets::LanguageFile;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<LanguageManager>().register_type::<LanguageManager>();
}

/// A [`Resource`] for managing the current language and language strings.
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct LanguageManager {
    /// The current language.
    pub current: ResourceKey,

    /// All loaded languages.
    #[reflect(ignore)]
    pub languages: HashMap<ResourceKey, LanguageFile>,
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self { current: ResourceKey::new_inline("minecraft:lang/en_us"), languages: HashMap::new() }
    }
}

impl LanguageManager {
    /// Get a language by key.
    #[must_use]
    #[inline]
    pub fn get_lang(&self, lang: &str) -> Option<&LanguageFile> {
        self.languages.get(&ResourceKey::try_new(lang).ok()?)
    }

    /// Get a mutable language by key.
    #[must_use]
    #[inline]
    pub fn get_lang_mut(&mut self, lang: &str) -> Option<&mut LanguageFile> {
        self.languages.get_mut(&ResourceKey::try_new(lang).ok()?)
    }

    /// Get the current language.
    #[must_use]
    #[inline]
    pub fn current_lang(&self) -> Option<&LanguageFile> { self.languages.get(&self.current) }

    /// Get the current language mutably.
    #[must_use]
    #[inline]
    pub fn current_lang_mut(&mut self) -> Option<&mut LanguageFile> {
        self.languages.get_mut(&self.current)
    }

    /// Get a string by language and key.
    ///
    /// Does not replace arguments in the string.
    ///
    /// You are probably looking for
    /// [`get_string`](Self::get_string)
    /// instead.
    #[must_use]
    pub fn get_raw_string(&self, lang: &str, key: &str) -> Option<&str> {
        self.get_lang(lang).and_then(|lang| lang.get(key).map(String::as_str))
    }

    /// Get a string from a language, with arguments.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_assets::assets::LanguageFile;
    /// use froglight_client::assets::LanguageManager;
    /// use froglight_network::common::ResourceKey;
    /// use hashbrown::HashMap;
    ///
    /// let mut manager = LanguageManager::default();
    ///
    /// let mut lang = HashMap::new();
    /// lang.insert("test_single".to_string(), "Hello, %s!".to_string());
    /// lang.insert("test_multi_ordered".to_string(), "Hello, %s and %s!".to_string());
    /// lang.insert("test_multi_reversed".to_string(), "Hello, %2$s and %1$s!".to_string());
    /// manager.languages.insert(ResourceKey::new_inline("minecraft:lang/en_us"), LanguageFile(lang));
    ///
    /// let test_single =
    ///     manager.get_string("minecraft:lang/en_us", "test_single", &["world"]).unwrap();
    /// assert_eq!(test_single, "Hello, world!");
    ///
    /// let test_multi_ordered = manager
    ///     .get_string("minecraft:lang/en_us", "test_multi_ordered", &["world", "universe"])
    ///     .unwrap();
    /// assert_eq!(test_multi_ordered, "Hello, world and universe!");
    ///
    /// let test_multi_reversed = manager
    ///     .get_string("minecraft:lang/en_us", "test_multi_reversed", &["world", "universe"])
    ///     .unwrap();
    /// assert_eq!(test_multi_reversed, "Hello, universe and world!");
    /// ```
    #[must_use]
    pub fn get_string(&self, lang: &str, key: &str, args: &[&str]) -> Option<String> {
        let string = self.get_raw_string(lang, key)?;
        Some(Self::parse_string_arguments(string.to_string(), args))
    }

    /// Get a string from the current language.
    ///
    /// Does not replace arguments in the string.
    ///
    /// You are probably looking for
    /// [`get_current_string`](Self::get_current_string)
    /// instead.
    #[must_use]
    pub fn current_raw_string(&self, key: &str) -> Option<&str> {
        self.current_lang().and_then(|lang| lang.get(key).map(String::as_str))
    }

    /// Get a string from the current language, with arguments.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_assets::assets::LanguageFile;
    /// use froglight_client::assets::LanguageManager;
    /// use froglight_network::common::ResourceKey;
    /// use hashbrown::HashMap;
    ///
    /// let mut manager = LanguageManager::default();
    ///
    /// let mut lang = HashMap::new();
    /// lang.insert("test_single".to_string(), "Hello, %s!".to_string());
    /// lang.insert("test_multi_ordered".to_string(), "Hello, %s and %s!".to_string());
    /// lang.insert("test_multi_reversed".to_string(), "Hello, %2$s and %1$s!".to_string());
    /// manager.languages.insert(ResourceKey::new_inline("minecraft:lang/en_us"), LanguageFile(lang));
    ///
    /// let test_single = manager.current_string("test_single", &["world"]).unwrap();
    /// assert_eq!(test_single, "Hello, world!");
    ///
    /// let test_multi_ordered =
    ///     manager.current_string("test_multi_ordered", &["world", "universe"]).unwrap();
    /// assert_eq!(test_multi_ordered, "Hello, world and universe!");
    ///
    /// let test_multi_reversed =
    ///     manager.current_string("test_multi_reversed", &["world", "universe"]).unwrap();
    /// assert_eq!(test_multi_reversed, "Hello, universe and world!");
    /// ```
    #[must_use]
    pub fn current_string(&self, key: &str, args: &[&str]) -> Option<String> {
        let string = self.current_raw_string(key)?;
        Some(Self::parse_string_arguments(string.to_string(), args))
    }

    /// The language key to fall back to if the current language does not have a
    /// string.
    pub const FALLBACK_LANG: ResourceKey = ResourceKey::new_inline("minecraft:lang/en_us");

    /// Get a string from a language, with arguments.
    ///
    /// If the string is not found in the language, it will fall back to the
    /// [`FALLBACK_LANG`](Self::FALLBACK_LANG).
    ///
    /// If the string is still not found, it will return the key.
    #[must_use]
    pub fn get_string_fallback(&self, lang: &str, key: &str, args: &[&str]) -> String {
        if let Some(string) = self.get_raw_string(lang, key) {
            Self::parse_string_arguments(string.to_string(), args)
        } else if let Some(string) = self.get_raw_string(&Self::FALLBACK_LANG, key) {
            Self::parse_string_arguments(string.to_string(), args)
        } else {
            warn!("Unable to find string for key: \"{}\":\"{key}\"", lang);
            key.to_string()
        }
    }

    /// Get a string from the current language, with arguments.
    ///
    /// If the string is not found in the current language, it will fall back to
    /// the [`FALLBACK_LANG`](Self::FALLBACK_LANG).
    ///
    /// If the string is still not found, it will return the key.
    #[must_use]
    pub fn current_string_fallback(&self, key: &str, args: &[&str]) -> String {
        if let Some(string) = self.current_raw_string(key) {
            Self::parse_string_arguments(string.to_string(), args)
        } else if let Some(string) = self.get_raw_string(&Self::FALLBACK_LANG, key) {
            Self::parse_string_arguments(string.to_string(), args)
        } else {
            warn!("Unable to find string for key: \"{}\":\"{key}\"", self.current);
            key.to_string()
        }
    }

    /// Convert a [`ResourceKey`] to a language key.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_client::assets::LanguageManager;
    /// use froglight_network::common::ResourceKey;
    ///
    /// let dirt_key = ResourceKey::new_inline("minecraft:dirt");
    /// let dirt_lang = LanguageManager::resourcekey_to_langkey("block", &dirt_key);
    /// assert_eq!(dirt_lang, "block.minecraft.dirt");
    ///
    /// let dye_key = ResourceKey::new_inline("minecraft:yellow_dye");
    /// let dye_lang = LanguageManager::resourcekey_to_langkey("item", &dye_key);
    /// assert_eq!(dye_lang, "item.minecraft.yellow_dye");
    /// ```
    #[must_use]
    pub fn resourcekey_to_langkey(domain: &str, key: &ResourceKey) -> String {
        format!("{domain}.{}", key.replace(['/', ':'], "."))
    }

    /// Parse a string with arguments.
    ///
    /// Arguments are inserted into the string in order using `%s` arguments,
    /// or are ordered by `%1$s`, `%2$s`, etc
    #[must_use]
    pub fn parse_string_arguments(mut string: String, args: &[&str]) -> String {
        if string.contains("%s") {
            // Replace all `%s` arguments in order of occurrence.
            for &arg in args {
                string = string.replacen("%s", arg, 1);
            }
        } else if string.contains("$s") {
            // Replace `%#$s` arguments with the corresponding argument.
            for (i, &arg) in args.iter().enumerate() {
                string = string.replace(&format!("%{}$s", i + 1), arg);
            }
        }
        string
    }
}
