# i18n Module

Simple i18n module used to translate strings in a Rust program. It is designed to be simple and easy to use. This is a personal module developed to be used within some of my personal projects written in Rust.

## Features

- Lazy-loaded translations using `once_cell`
- Environment variable configuration
- Fluent builder pattern for parameterized translations
- Based on Mozilla's Fluent localization system
- No runtime allocations for simple string lookups
- Simple API with both static and builder approaches

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fluent_templates = "0.8"
once_cell = "1.17"
unic-langid = "0.9"
```

## Usage

### Basic Translation

To retrieve a simple translation:

```rust
let hello = i18n::get("hello");  // Returns "Hello" for en-US
```

### Parameterized Translations

For translations with parameters:

```rust
// Method 1: Using builder pattern with build()
let greeting = i18n::new("greeting")
    .set_args("name", "Alice")
    .build();  // Returns "Hello, Alice!" for en-US

// Method 2: Using builder pattern with args()
let builder = i18n::new("user_info")
    .set_args("user", "Bob")
    .set_args("time", "morning");
    
let message = builder.args("welcome_message");  // Returns "Good morning, Bob!" for en-US
```

## Configuration

The module can be configured using environment variables:

- `I18N_ID`: Language identifier (e.g., "en-US", "es-MX")
- `I18N_DIR`: Directory containing locale files (default: "./assets/locales/")

If not specified, the module defaults to "en-US" locale.

## File Structure

Locale files should follow this directory structure:

```
./assets/locales/
├── en-US/
│   └── main.ftl
├── es-MX/
│   └── main.ftl
└── ...
```

## FTL File Example

Example content for `./assets/locales/en-US/main.ftl`:

```ftl
hello = Hello
greeting = Hello, { $name }!
welcome_message = Good { $time }, { $user }!
```

## API Reference

### Functions

- `get(key)`: Retrieves a translation for the given key
- `new(key)`: Creates a new builder for parameterized translations

### Builder Methods

- `set_args(key, value)`: Sets a parameter for the translation
- `args(key)`: Looks up a translation with the current parameters
- `build()`: Executes translation using the builder's key and arguments