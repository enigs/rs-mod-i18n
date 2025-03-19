use fluent_templates::{ArcLoader, Loader, fluent_bundle::FluentValue};
use once_cell::sync::Lazy;
use std::{collections::HashMap, env};
use std::borrow::Cow;
use unic_langid::LanguageIdentifier;

const ERROR_PARSING: &str = "Parsing language failed";
const ERROR_BUILDING: &str = "Unable to build loader";
const DEFAULT_LANG: &str = "en-US";
const DEFAULT_DIR: &str = "./assets/locales/";

/// Internationalization (i18n) Configuration
///
/// A static, lazily initialized configuration for handling translations across different languages.
/// Loads locale settings from environment variables or falls back to defaults.
///
/// # Environment Variables
/// - `I18N_ID`: The language identifier (e.g., "en-US")
/// - `I18N_DIR`: Directory containing locale files
static I18N: Lazy<I18n> = Lazy::new(|| {
    let locales: LanguageIdentifier = env::var("I18N_ID")
        .unwrap_or_else(|_| DEFAULT_LANG.to_string())
        .parse()
        .expect(ERROR_PARSING);

    let i18n_dir = env::var("I18N_DIR").unwrap_or_else(|_| DEFAULT_DIR.to_string());

    let loader = ArcLoader::builder(&i18n_dir, locales.clone())
        .customize(|b| b.set_use_isolating(false))
        .build()
        .expect(ERROR_BUILDING);

    I18n { locales, loader }
});

/// Core internationalization structure
///
/// Holds the translation loader and current locale settings for the application.
///
/// # Fields
/// - `loader`: Handles loading and caching of translation files
/// - `locales`: Current language identifier
struct I18n {
    loader: ArcLoader,
    locales: LanguageIdentifier,
}

/// Retrieves a translation for the given key
///
/// # Parameters
/// * `key` - The translation key to look up
///
/// # Return
/// Returns the translated string for the current locale
///
/// # Examples
/// ```
///
/// let hello = i18n::get("hello");  // Returns "Hello" for en-US
/// let greeting = i18n::get("welcome_message");  // Returns "Welcome" for en-US
/// ```
pub fn get<T>(key: T) -> String
where
    T: ToString,
{
    I18N.loader.lookup(&I18N.locales, &key.to_string())
}

/// Builder for handling translations with parameters
///
/// Provides a fluent interface for setting translation arguments and retrieving
/// parameterized translations.
///
/// # Fields
/// * `key` - The translation key to look up
/// * `args` - HashMap storing the parameter key-value pairs
pub struct I18nBuilder {
    key: String,
    args: HashMap<String, String>,
}

impl I18nBuilder {
    /// Sets a parameter for the translation
    ///
    /// # Parameters
    /// * `key` - The parameter key
    /// * `value` - The parameter value
    ///
    /// # Return
    /// Returns self for method chaining
    ///
    /// # Examples
    /// ```
    /// let builder = i18n::new("greeting")
    ///     .set_args("user", "Alice")
    ///     .set_args("count", "3");
    /// ```
    pub fn set_args<T, U>(mut self, key: T, value: U) -> Self
    where
        T: ToString,
        U: ToString
    {
        self.args.insert(key.to_string(), value.to_string());
        self
    }

    /// Looks up a translation with the current parameters
    ///
    /// # Parameters
    /// * `key` - The translation key to look up
    ///
    /// # Return
    /// Returns the translated string with parameters substituted
    ///
    /// # Examples
    /// ```
    ///
    /// let message = i18n::new("greeting")
    ///     .set_args("name", "Bob")
    ///     .args("greeting");  // Returns "Hello, Bob!"
    /// ```
    pub fn args<T>(&self, key: T) -> String
    where
        T: ToString
    {
        if self.args.is_empty() {
            return get(key);
        }

        let key = key.to_string();
        let args =  self.args
            .iter()
            .map(|(k, v)| (
                Cow::from(k.clone()),
                FluentValue::from(v.clone())
            ))
            .collect();

        I18N.loader
            .lookup_with_args(&I18N.locales, &key, &args)
    }

    /// Executes translation using the builder's key and arguments
    ///
    /// Retrieves a translation for the key stored in the builder.
    /// If arguments have been set, they will be used for parameter substitution.
    ///
    /// # Return
    /// Returns the translated string with parameters substituted if applicable
    ///
    /// # Examples
    /// ```
    ///
    /// let message = i18n::new("greeting")
    ///     .set_args("name", "Bob")
    ///     .build();  // Returns "Hello, Bob!"
    /// ```
    pub fn build(&self) -> String {
        if self.args.is_empty() {
            return get(&self.key);
        }

        let args = self.args
            .iter()
            .map(|(k, v)| (
                Cow::from(k.clone()),
                FluentValue::from(v.clone())
            ))
            .collect();

        I18N.loader
            .lookup_with_args(&I18N.locales, &self.key, &args)
    }
}

/// Creates a new I18nBuilder with an initial key-value pair
///
/// # Parameters
/// * `key` - The translation key to look up
///
/// # Return
/// Returns a new I18nBuilder instance
///
/// # Examples
/// ```
///
/// // Using args() with a different key
/// let builder = i18n::new("user_info")
///     .set_args("user", "Carol")
///     .set_args("time", "morning");
///
/// let greeting = builder.args("greeting");  // Returns "Good morning, Carol!"
///
/// // Using build() with the key stored in the builder
/// let user_info = i18n::new("user_info")
///     .set_args("user", "Carol")
///     .set_args("time", "morning")
///     .build();  // Returns translated string for "user_info" key with parameters
/// ```
pub fn new<K>(key: K) -> I18nBuilder
where
    K: ToString
{
    let key = key.to_string();

    I18nBuilder {
        key,
        args: HashMap::new(),
    }
}