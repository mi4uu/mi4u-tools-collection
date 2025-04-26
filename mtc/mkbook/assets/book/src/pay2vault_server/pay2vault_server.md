# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `pay2vault_server`

## Modules

## Module `config`

```rust
pub mod config { /* ... */ }
```

### Types

#### Enum `Version`

```rust
pub enum Version {
    V1,
}
```

##### Variants

###### `V1`

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::result::Result<(), ::core::fmt::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Version) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DeserializeOwned**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Config`

**Attributes:**

- `#[config(name = "server_config", format = "toml")]`

```rust
pub struct Config {
    pub name: String,
    pub version: Version,
    pub debug_mode: bool,
    pub port: usize,
    pub addr: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |
| `version` | `Version` |  |
| `debug_mode` | `bool` |  |
| `port` | `usize` |  |
| `addr` | `String` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **DeserializeOwned**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Config) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Configuration**
  - ```rust
    fn config_name() -> String { /* ... */ }
    ```

  - ```rust
    fn format() -> pay2vault_config::ConfigFormat { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
