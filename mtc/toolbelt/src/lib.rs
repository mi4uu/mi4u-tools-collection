pub mod env_utils;

#[macro_export]
macro_rules! get_tool_storage_path {
    () => {{
        // Default behavior when no parameters are passed
        let storage_dir = std::env::var("OUT_TOOL_STORAGE_DIR").unwrap_or_else(|_| "/tmp".to_string());
        let tool_storage = std::path::PathBuf::from(storage_dir);
        std::fs::create_dir_all(&tool_storage).unwrap();
        tool_storage
    }};
    ($($args:tt)*) => {{
        // Behavior when parameters are passed
        let storage_dir = std::env::var("OUT_TOOL_STORAGE_DIR").unwrap_or_else(|_| "/tmp".to_string());
        let tool_storage = std::path::PathBuf::from(storage_dir);
        std::fs::create_dir_all(&tool_storage).unwrap();
        tool_storage.join(format!($($args)*))
    }};
}
