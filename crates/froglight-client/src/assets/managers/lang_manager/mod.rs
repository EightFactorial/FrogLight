use bevy::prelude::*;
use froglight_assets::assets::ResourcePack;
use froglight_network::common::ResourceKey;
use hashbrown::{hash_map::Entry, HashMap};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.init_resource::<LanguageManager>(); }

/// Provides access to the current language and language strings.
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct LanguageManager {
    /// The current language.
    pub current: ResourceKey,

    /// The strings for each language.
    pub languages: HashMap<ResourceKey, HashMap<String, String>>,
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self { current: ResourceKey::new_inline("minecraft:lang/en_us"), languages: HashMap::new() }
    }
}

impl LanguageManager {
    /// Get the current language.
    #[must_use]
    pub fn current_lang(&self) -> Option<&HashMap<String, String>> {
        self.languages.get(&self.current)
    }

    /// Get the current language mutably.
    #[must_use]
    pub fn current_lang_mut(&mut self) -> Option<&mut HashMap<String, String>> {
        self.languages.get_mut(&self.current)
    }

    /// Get a string from the current language.
    ///
    /// This does not replace arguments in the string.
    ///
    /// You are probably looking for
    /// [`get_current_string`](Self::get_current_string)
    /// instead.
    #[must_use]
    pub fn get_current_raw_string(&self, key: &str) -> Option<&str> {
        self.current_lang().and_then(|lang| lang.get(key).map(String::as_str))
    }

    /// Get a string from the current language, with arguments.
    ///
    /// Arguments are inserted into the string in order using `%s` arguments,
    /// or are ordered by `%1$s`, `%2$s`, etc
    ///
    /// # Examples
    /// ```rust
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
    /// manager.languages.insert(ResourceKey::new_inline("minecraft:lang/en_us"), lang);
    ///
    /// let test_single = manager.get_current_string("test_single", &["world"]).unwrap();
    /// assert_eq!(test_single, "Hello, world!");
    ///
    /// let test_multi_ordered =
    ///     manager.get_current_string("test_multi_ordered", &["world", "universe"]).unwrap();
    /// assert_eq!(test_multi_ordered, "Hello, world and universe!");
    ///
    /// let test_multi_reversed =
    ///     manager.get_current_string("test_multi_reversed", &["world", "universe"]).unwrap();
    /// assert_eq!(test_multi_reversed, "Hello, universe and world!");
    /// ```
    #[must_use]
    pub fn get_current_string(&self, key: &str, args: &[&str]) -> Option<String> {
        let mut string = self.get_current_raw_string(key)?.to_string();

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

        Some(string)
    }

    /// Get a language by key.
    #[must_use]
    pub fn get_lang(&self, lang: &str) -> Option<&HashMap<String, String>> {
        self.languages.get(&ResourceKey::new_inline(lang))
    }

    /// Get a string by language and key.
    #[must_use]
    pub fn get_raw_string(&self, lang: &str, key: &str) -> Option<&str> {
        self.get_lang(lang).and_then(|lang| lang.get(key).map(String::as_str))
    }

    /// Clear all language strings.
    #[inline]
    pub(crate) fn clear(&mut self) { self.languages.clear(); }

    /// Take all language strings from a [`ResourcePack`].
    pub(crate) fn insert(&mut self, resourcepack: &mut ResourcePack) {
        for (key, language_file) in resourcepack.lang.drain() {
            match self.languages.entry(key) {
                Entry::Occupied(entry) => {
                    // Merge the language file into the existing language,
                    // withour overwriting existing strings.
                    let language = entry.into_mut();
                    for (key, value) in language_file.0 {
                        language.entry(key).or_insert(value);
                    }
                }
                Entry::Vacant(entry) => {
                    // Insert the language file.
                    entry.insert(language_file.0);
                }
            }
        }
    }
}
