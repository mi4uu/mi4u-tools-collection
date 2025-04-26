use std::{env, path::PathBuf};

#[macro_export]
macro_rules! set_env_var {
    ($key:expr, $value:expr) => {
        unsafe {
            std::env::set_var($key, $value);
        }
    };
}


/// get path buf for package
pub fn get_toolbelt_package_root() ->std::path::PathBuf {
    std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
}

#[macro_export]
macro_rules! get_package_root {
    () => {
        std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
    };
}



fn get_workspace_root_one_lv_up(package_dir:PathBuf) ->Option<std::path::PathBuf> {
    match  package_dir.parent(){
        Some(parent_dir)=> {
        let cargo_maby=parent_dir.join("Cargo.toml");
            if cargo_maby.exists() {
                return Some(parent_dir.to_path_buf())
            }
            return None


        },
        None=> None
        
    }
}

/// get path buf for workspace
/// 
/// ```rust
/// 
/// use mtc_toolbelt::env_utils::get_workspace_root;
/// 
/// let workspace_root=get_workspace_root();
/// assert_eq!(std::fs::canonicalize(workspace_root).unwrap(), std::fs::canonicalize(std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("../..")).unwrap())
/// ```
pub fn get_workspace_root() ->std::path::PathBuf {
    let package_dir= get_package_root!();
    // check 1 lv up
    if let Some(workspace_dir) = get_workspace_root_one_lv_up(package_dir){
        return workspace_dir;
    }
    // check 2 lv up
    if let Some(package_dir)= get_package_root!().parent(){
        if let Some(workspace_dir) = get_workspace_root_one_lv_up(package_dir.to_path_buf()){
            return workspace_dir;
        }
        // check 3 lv up
        if let Some(package_dir)= package_dir.parent(){
            if let Some(workspace_dir) = get_workspace_root_one_lv_up(package_dir.to_path_buf()){
                return workspace_dir;
            }
            // if more nested than you doing it wrong
        }

    }

   

 get_package_root!()
}

/// Change to workspace root.
///
/// Assumed this xtask is located in `[WORKSPACE]/crates/xtask-build-man`.
pub fn cwd_to_workspace_root() -> std::io::Result<()> {
    let ws_root = get_workspace_root();
    std::env::set_current_dir(ws_root)
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
