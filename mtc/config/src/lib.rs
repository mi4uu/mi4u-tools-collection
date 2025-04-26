//! A crate for easily creating, saving, and loading configuration files.
//!
//! This crate provides a `Configuration` trait that can be derived for
//! any struct that implements `serde::Serialize` and `serde::de::DeserializeOwned`.
//!
//! # Features
//!
//! - Support for multiple serialization formats (TOML, JSON, YAML) via feature flags
//! - Automatic configuration directory creation
//! - Customizable configuration names and paths
//! - Built-in error handling for configuration operations
//!
//! # Example
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use mtc_config::Configuration;
//!
//! #[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
//! #[derive(Configuration)]
//! #[config(name = "app_config", format = "toml")]
//! struct AppConfig {
//!     name: String,
//!     version: String,
//!     debug_mode: bool,
//! }
//!
//! // Create and save a config
//! let config = AppConfig {
//!     name: "My App".to_string(),
//!     version: "1.0.0".to_string(),
//!     debug_mode: true,
//! };
//!
//! // Load an existing config or create default if not found
//! let loaded_config = AppConfig::load_or_default();
//! ```

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;

// Re-export the derive macro
pub use mtc_config_derive::Configuration;
pub use serde::{*};
/// Errors that can occur when working with configurations.
///
/// This enum provides specific error types for different failure scenarios
/// when working with configuration files, including I/O errors, serialization/deserialization
/// issues, and missing configuration files.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// An error occurred during file I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// An error occurred while serializing the configuration to the selected format.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// An error occurred while deserializing the configuration from the selected format.
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// The configuration file was not found at the specified path.
    #[error("Configuration file not found at: {0}")]
    NotFound(PathBuf),
}

/// Supported formats for configuration serialization and deserialization.
///
/// This enum defines the available formats for storing configuration data.
/// The actual support for each format depends on the enabled feature flags:
/// - `json`: Enables JSON format support
/// - `toml`: Enables TOML format support (default)
/// - `yaml`: Enables YAML format support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    /// JSON format (requires the "json" feature)
    Json,
    /// TOML format (requires the "toml" feature)
    Toml,
    /// YAML format (requires the "yaml" feature)
    Yaml,
}

impl ToString for ConfigFormat {
    /// Converts the format to its string representation.
    ///
    /// # Returns
    /// A string representing the format: "json", "toml", or "yaml".
    fn to_string(&self) -> String {
        match self {
            ConfigFormat::Json => "json".to_string(),
            ConfigFormat::Toml => "toml".to_string(),
            ConfigFormat::Yaml => "yaml".to_string(),
        }
    }
}

impl From<String> for ConfigFormat {
    /// Creates a ConfigFormat from a string.
    ///
    /// # Arguments
    /// * `value` - A string representing the format: "json", "toml", or "yaml".
    ///
    /// # Returns
    /// The corresponding ConfigFormat, or ConfigFormat::Toml for unrecognized formats.
    fn from(value: String) -> Self {
        match value.as_str() {
            "json" => ConfigFormat::Json,
            "toml" => ConfigFormat::Toml,
            "yaml" => ConfigFormat::Yaml,
            _ => ConfigFormat::Toml,
        }
    }
}

impl From<&str> for ConfigFormat {
    /// Creates a ConfigFormat from a string slice.
    ///
    /// # Arguments
    /// * `value` - A string slice representing the format: "json", "toml", or "yaml".
    ///
    /// # Returns
    /// The corresponding ConfigFormat, or ConfigFormat::Toml for unrecognized formats.
    fn from(value: &str) -> Self {
        match value {
            "json" => ConfigFormat::Json,
            "toml" => ConfigFormat::Toml,
            "yaml" => ConfigFormat::Yaml,
            _ => ConfigFormat::Toml,
        }
    }
}

impl Default for ConfigFormat {
    /// Returns the default configuration format (TOML).
    ///
    /// # Returns
    /// ConfigFormat::Toml as the default format.
    fn default() -> Self {
        // Default to TOML as the standard configuration format
        ConfigFormat::Toml
    }
}

/// Gets the configs directory in the workspace root.
///
/// This function returns the path to the "configs" directory
/// within the workspace root, where configuration files should be stored.
///
/// # Returns
/// A PathBuf pointing to the configs directory.
pub fn get_configs_dir() -> PathBuf {
    let storage_dir = std::env::var("CONFIG_ROOT_DIR").unwrap_or_else(|_| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("configs")
            .to_string_lossy()
            .to_string()
    });
    let config_dir = std::path::PathBuf::from(storage_dir);
    std::fs::create_dir_all(&config_dir).unwrap();
    config_dir
}

/// The main configuration trait that provides save, load, and load_or_default methods.
///
/// This trait can be derived for any struct that implements `serde::Serialize`,
/// `serde::Deserialize`, and `Default`. It provides methods for saving and loading
/// configuration data to and from files in various formats.
///
/// # Type Requirements
/// - `Serialize`: The type must be serializable
/// - `Deserialize`: The type must be deserializable
/// - `Default`: The type must provide default values
/// - `Sized`: The type must have a known size at compile time
pub trait Configuration: Serialize + for<'de> Deserialize<'de> + Default + Sized {
    /// Gets the name for this configuration.
    ///
    /// This name is used to generate the filename for the configuration file.
    /// By default, the derive macro uses the lowercase name of the struct.
    ///
    /// # Returns
    /// A string representing the configuration name.
    fn config_name() -> String;

    /// Gets the default path for this configuration file.
    ///
    /// This function generates the full path to the configuration file
    /// by combining the configs directory with the configuration name
    /// and appropriate file extension based on the format.
    ///
    /// # Returns
    /// A PathBuf pointing to the default configuration file location.
    fn default_path() -> PathBuf {
        let extension = match Self::format() {
            ConfigFormat::Json => "json",
            ConfigFormat::Toml => "toml",
            ConfigFormat::Yaml => "yaml",
        };

        get_configs_dir().join(format!("{}.{}", Self::config_name(), extension))
    }

    /// Gets the serialization format for this configuration.
    ///
    /// By default, this returns TOML, but it can be overridden by
    /// implementing this method or using the #[config(format = "...")] attribute.
    ///
    /// # Returns
    /// The ConfigFormat to use for this configuration.
    fn format() -> ConfigFormat {
        ConfigFormat::default()
    }

    /// Saves the configuration to the default path.
    ///
    /// This method serializes the configuration to the specified format
    /// and saves it to the default path. It creates any necessary parent
    /// directories if they don't exist.
    ///
    /// # Returns
    /// A Result indicating success or providing an error if the save failed.
    ///
    /// # Errors
    /// - `ConfigError::Io`: If there was an I/O error creating the file or directories
    /// - `ConfigError::Serialization`: If there was an error serializing the configuration
    fn save(&self) -> Result<(), ConfigError> {
        let path = Self::default_path();

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(ConfigError::Io)?;
        }

        let mut file = File::create(&path).map_err(ConfigError::Io)?;

        match Self::format() {
            #[cfg(feature = "json")]
            ConfigFormat::Json => {
                let content = serde_json::to_string_pretty(self)
                    .map_err(|e| ConfigError::Serialization(e.to_string()))?;
                file.write_all(content.as_bytes())
                    .map_err(ConfigError::Io)?;
            }
            #[cfg(feature = "toml")]
            ConfigFormat::Toml => {
                let content =
                    toml::to_string(self).map_err(|e| ConfigError::Serialization(e.to_string()))?;
                file.write_all(content.as_bytes())
                    .map_err(ConfigError::Io)?;
            }
            #[cfg(feature = "yaml")]
            ConfigFormat::Yaml => {
                let content = serde_yaml::to_string(self)
                    .map_err(|e| ConfigError::Serialization(e.to_string()))?;
                file.write_all(content.as_bytes())
                    .map_err(ConfigError::Io)?;
            }
            #[allow(unreachable_patterns)]
            _ => {
                return Err(ConfigError::Serialization(
                    "Selected format is not enabled".into(),
                ));
            }
        }

        Ok(())
    }

    /// Loads the configuration from the default path.
    ///
    /// This method attempts to load and deserialize the configuration
    /// from its default path using the specified format.
    ///
    /// # Returns
    /// A Result containing the loaded configuration or an error if loading failed.
    ///
    /// # Errors
    /// - `ConfigError::NotFound`: If the configuration file doesn't exist
    /// - `ConfigError::Io`: If there was an I/O error reading the file
    /// - `ConfigError::Deserialization`: If there was an error deserializing the configuration
    fn load() -> Result<Self, ConfigError> {
        let path = Self::default_path();

        if !path.exists() {
            return Err(ConfigError::NotFound(path));
        }

        let mut file = File::open(&path).map_err(ConfigError::Io)?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(ConfigError::Io)?;

        match Self::format() {
            #[cfg(feature = "json")]
            ConfigFormat::Json => serde_json::from_str(&content)
                .map_err(|e| ConfigError::Deserialization(e.to_string())),
            #[cfg(feature = "toml")]
            ConfigFormat::Toml => {
                toml::from_str(&content).map_err(|e| ConfigError::Deserialization(e.to_string()))
            }
            #[cfg(feature = "yaml")]
            ConfigFormat::Yaml => serde_yaml::from_str(&content)
                .map_err(|e| ConfigError::Deserialization(e.to_string())),
            #[allow(unreachable_patterns)]
            _ => Err(ConfigError::Deserialization(
                "Selected format is not enabled".into(),
            )),
        }
    }

    /// Loads the configuration from the default path, or creates and saves the default if loading fails.
    ///
    /// This method attempts to load an existing configuration. If that fails because
    /// the file doesn't exist, it creates a default configuration and saves it.
    /// If loading fails for any other reason, it logs a warning and returns the default.
    ///
    /// # Returns
    /// Either the loaded configuration or a default configuration.
    fn load_or_default() -> Self {
        match Self::load() {
            Ok(config) => config,
            Err(ConfigError::NotFound(_)) => {
                let default_config = Self::default();
                // Create parent directories if they don't exist
                if let Some(parent) = Self::default_path().parent() {
                    let _ = fs::create_dir_all(parent);
                }
                // Save the default configuration
                if let Err(e) = default_config.save() {
                    eprintln!("Warning: Failed to save default config: {}", e);
                }
                default_config
            }
            Err(e) => {
                eprintln!("Warning: Failed to load config: {}", e);
                Self::default()
            }
        }
    }
}
