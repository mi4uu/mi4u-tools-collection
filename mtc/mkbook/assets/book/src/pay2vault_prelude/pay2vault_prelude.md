# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `pay2vault_prelude`

## Modules

## Module `env_utils`

```rust
pub mod env_utils { /* ... */ }
```

### Types

#### Struct `ScopedEnvVar`

for setting scoped env var
example:
```rust
use pay2vault_prelude::env_utils::ScopedEnvVar;

unsafe { std::env::set_var("SOME_ENV_VAR_NAME", "OLD_VAR".to_string()); }
assert_eq!(std::env::var("SOME_ENV_VAR_NAME"), Ok("OLD_VAR".to_string()));
{
let _env_var = ScopedEnvVar::new("SOME_ENV_VAR_NAME", "NEW_VAR");
assert_eq!(std::env::var("SOME_ENV_VAR_NAME"), Ok("NEW_VAR".to_string()));
}
assert_eq!(std::env::var("SOME_ENV_VAR_NAME"), Ok("OLD_VAR".to_string()));
```

```rust
pub struct ScopedEnvVar {
    pub(in ::env_utils) key: String,
    pub(in ::env_utils) original: Option<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `String` |  |
| `original` | `Option<String>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<T: Into<String>>(key: T, value: T) -> Self { /* ... */ }
  ```
  Sets a new environment variable value and stores the original value

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```
    Automatically restores the original environment variable value when dropped

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Macros

### Macro `set_env_var`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! set_env_var {
    /* macro_rules! set_env_var {
    ($key:expr, $value:expr) => { ... };
} */
}
```

### Macro `get_tool_storage_path`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! get_tool_storage_path {
    /* macro_rules! get_tool_storage_path {
    () => { ... };
    ($($args:tt)*) => { ... };
} */
}
```

