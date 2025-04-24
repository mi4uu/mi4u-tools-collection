# mtc-config ![License: do-not-use](https://img.shields.io/badge/license-do--not--use-blue) [![mtc-config on crates.io](https://img.shields.io/crates/v/mtc-config)](https://crates.io/crates/mtc-config) [![mtc-config on docs.rs](https://docs.rs/mtc-config/badge.svg)](https://docs.rs/mtc-config) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/mtc/mtc) ![Rust Version: 1.88.0](https://img.shields.io/badge/rustc-1.88.0-orange.svg)

A crate for easily creating, saving, and loading configuration files.

This crate provides a `Configuration` trait that can be derived for
any struct that implements `serde::Serialize` and `serde::de::DeserializeOwned`.

## Features

* Support for multiple serialization formats (TOML, JSON, YAML) via feature flags
* Automatic configuration directory creation
* Customizable configuration names and paths
* Built-in error handling for configuration operations

## Example

```rust
use serde::{Serialize, Deserialize};
use mtc_config::Configuration;

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
