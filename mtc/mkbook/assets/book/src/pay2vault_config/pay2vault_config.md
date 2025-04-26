# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `pay2vault_config`

A crate for easily creating, saving, and loading configuration files.

This crate provides a `Configuration` trait that can be derived for
any struct that implements `serde::Serialize` and `serde::de::DeserializeOwned`.

# Features

- Support for multiple serialization formats (TOML, JSON, YAML) via feature flags
- Automatic configuration directory creation
- Customizable configuration names and paths
- Built-in error handling for configuration operations

# Example

```rust
use serde::{Serialize, Deserialize};
use pay2vault_config::Configuration;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[derive(Configuration)]
#[config(name = "app_config", format = "toml")]
struct AppConfig {
    name: String,
    version: String,
    debug_mode: bool,
}

// Create and save a config
let config = AppConfig {
    name: "My App".to_string(),
    version: "1.0.0".to_string(),
    debug_mode: true,
};

// Load an existing config or create default if not found
let loaded_config = AppConfig::load_or_default();
```

## Types

### Enum `ConfigError`

Errors that can occur when working with configurations.

This enum provides specific error types for different failure scenarios
when working with configuration files, including I/O errors, serialization/deserialization
issues, and missing configuration files.

```rust
pub enum ConfigError {
    Io(std::io::Error),
    Serialization(String),
    Deserialization(String),
    NotFound(std::path::PathBuf),
}
```

#### Variants

##### `Io`

An error occurred during file I/O operations.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::io::Error` |  |

##### `Serialization`

An error occurred while serializing the configuration to the selected format.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### `Deserialization`

An error occurred while deserializing the configuration from the selected format.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### `NotFound`

The configuration file was not found at the specified path.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::path::PathBuf` |  |

#### Implementations

##### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, __formatter: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private::Error + ''static> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(source: std::io::Error) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Enum `ConfigFormat`

Supported formats for configuration serialization and deserialization.

This enum defines the available formats for storing configuration data.
The actual support for each format depends on the enabled feature flags:
- `json`: Enables JSON format support
- `toml`: Enables TOML format support (default)
- `yaml`: Enables YAML format support

```rust
pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}
```

#### Variants

##### `Json`

JSON format (requires the "json" feature)

##### `Toml`

TOML format (requires the "toml" feature)

##### `Yaml`

YAML format (requires the "yaml" feature)

#### Implementations

##### Trait Implementations

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ConfigFormat { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ConfigFormat) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Eq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```
    Converts the format to its string representation.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: String) -> Self { /* ... */ }
    ```
    Creates a ConfigFormat from a string.

  - ```rust
    fn from(value: &str) -> Self { /* ... */ }
    ```
    Creates a ConfigFormat from a string slice.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Returns the default configuration format (TOML).

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
## Traits

### Trait `Configuration`

The main configuration trait that provides save, load, and load_or_default methods.

This trait can be derived for any struct that implements `serde::Serialize`,
`serde::Deserialize`, and `Default`. It provides methods for saving and loading
configuration data to and from files in various formats.

# Type Requirements
- `Serialize`: The type must be serializable
- `Deserialize`: The type must be deserializable
- `Default`: The type must provide default values
- `Sized`: The type must have a known size at compile time

```rust
pub trait Configuration: Serialize + for<''de> Deserialize<''de> + Default + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `config_name`: Gets the name for this configuration.

#### Provided Methods

- ```rust
  fn default_path() -> PathBuf { /* ... */ }
  ```
  Gets the default path for this configuration file.

- ```rust
  fn format() -> ConfigFormat { /* ... */ }
  ```
  Gets the serialization format for this configuration.

- ```rust
  fn save(self: &Self) -> Result<(), ConfigError> { /* ... */ }
  ```
  Saves the configuration to the default path.

- ```rust
  fn load() -> Result<Self, ConfigError> { /* ... */ }
  ```
  Loads the configuration from the default path.

- ```rust
  fn load_or_default() -> Self { /* ... */ }
  ```
  Loads the configuration from the default path, or creates and saves the default if loading fails.

## Functions

### Function `get_configs_dir`

Gets the configs directory in the workspace root.

This function returns the path to the "configs" directory
within the workspace root, where configuration files should be stored.

# Returns
A PathBuf pointing to the configs directory.

```rust
pub fn get_configs_dir() -> std::path::PathBuf { /* ... */ }
```

## Re-exports

### Re-export `Configuration`

```rust
pub use pay2vault_config_derive::Configuration;
```

### Re-export `Deserialize`

```rust
pub use serde::Deserialize;
```

### Re-export `Deserialize`

```rust
pub use serde::Deserialize;
```

### Re-export `Serialize`

```rust
pub use serde::Serialize;
```

### Re-export `Serialize`

```rust
pub use serde::Serialize;
```

