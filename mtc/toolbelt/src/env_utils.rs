use std::env;

#[macro_export]
macro_rules! set_env_var {
    ($key:expr, $value:expr) => {
        unsafe {
            std::env::set_var($key, $value);
        }
    };
}

/// for setting scoped env var
/// example:
/// ```rust
/// use mtc_toolbelt::env_utils::ScopedEnvVar;
///
/// unsafe { std::env::set_var("SOME_ENV_VAR_NAME", "OLD_VAR".to_string()); }
/// assert_eq!(std::env::var("SOME_ENV_VAR_NAME"), Ok("OLD_VAR".to_string()));
/// {
/// let _env_var = ScopedEnvVar::new("SOME_ENV_VAR_NAME", "NEW_VAR");
/// assert_eq!(std::env::var("SOME_ENV_VAR_NAME"), Ok("NEW_VAR".to_string()));
/// }
/// assert_eq!(std::env::var("SOME_ENV_VAR_NAME"), Ok("OLD_VAR".to_string()));
/// ```
pub struct ScopedEnvVar {
    key: String,
    original: Option<String>, // To store the original value of the variable
}

impl ScopedEnvVar {
    /// Sets a new environment variable value and stores the original value
    pub fn new<T: Into<String>>(key: T, value: T) -> Self {
        let key = key.into();
        let original = env::var(&key).ok(); // Capture the current value (if any)

        // SAFELY set the new value
        unsafe {
            env::set_var(&key, value.into());
        }

        Self { key, original }
    }
}

impl Drop for ScopedEnvVar {
    /// Automatically restores the original environment variable value when dropped
    fn drop(&mut self) {
        unsafe {
            if let Some(ref original) = self.original {
                env::set_var(&self.key, original); // Restore the original value
            } else {
                env::remove_var(&self.key); // Remove the variable if it wasn't set originally
            }
        }
    }
}

#[test]
fn test_with_scoped_env_var() {
    let storage_env_before = format!("{:#?}", std::env::var("OUT_TOOL_STORAGE_DIR"));
    {
        // Set the environment variable with `ScopedEnvVar`
        let _env_var = ScopedEnvVar::new("OUT_TOOL_STORAGE_DIR", "/mocked/path");

        // Inside this scope, the environment variable is mocked
        assert_eq!(
            std::env::var("OUT_TOOL_STORAGE_DIR").unwrap(),
            "/mocked/path"
        );

        // When `_env_var` goes out of scope, the environment variable will be restored
    }
    let storage_env_after = format!("{:#?}", std::env::var("OUT_TOOL_STORAGE_DIR"));

    assert_eq!(storage_env_before, storage_env_after);
}
