#[cfg(test)]
mod test {

use mtc_config::{ConfigError, ConfigFormat, Configuration};
use serde::{Deserialize, Serialize};


// Create a test configuration struct
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[cfg_attr(test, derive(mtc_config::Configuration))]
#[config(name = "test_config_1")]
struct TestConfig {
    name: String,
    value: i32,
    enabled: bool,
}

// Create a test configuration with a custom name
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[cfg_attr(test, derive(mtc_config::Configuration))]
#[config(name = "custom_config")]
struct CustomNameConfig {
    custom_field: String,
}

// Create a test configuration with a custom format
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[cfg_attr(test, derive(mtc_config::Configuration))]
#[config(name = "json_config", format = "json")]
struct JsonFormatConfig {
    json_field: String,
}

// Create another test configuration for modification tests
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[cfg_attr(test, derive(mtc_config::Configuration))]
#[config(name = "test_config_2")]
struct TestConfigForModification {
    name: String,
    value: i32,
    enabled: bool,
}

// Create a test configuration for save/load tests
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[cfg_attr(test, derive(mtc_config::Configuration))]
#[config(name = "test_config_3")]
struct TestConfigForSaveLoad {
    name: String,
    value: i32,
    enabled: bool,
}



#[test]
fn test_save_and_load() {


    // Save the config using our temp directory
    //    let tmpdir= std::env::var("CONFIG_ROOT_DIR").unwrap();
    //    dbg!(tmpdir);
    // Create a test config
    let config = TestConfigForSaveLoad {
        name: "Test".to_string(),
        value: 42,
        enabled: true,
    };
        config.save().expect("Failed to save config");

        // Check that the file exists
        let config_path = TestConfigForSaveLoad::default_path();
        assert!(config_path.exists(), "Config file was not created");

        // Debug: print the file content
        let content = std::fs::read_to_string(&config_path).unwrap();
        println!("Saved config content:\n{}", content);

        // Load the config and verify it matches
        let loaded_config = TestConfigForSaveLoad::load().expect("Failed to load config");
        assert_eq!(
            config, loaded_config,
            "Loaded config doesn't match saved config"
        );
    
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[cfg_attr(test, derive(mtc_config::Configuration))]
#[config(name = "nonexistent_config")]
struct NonexistentConfig {
    dummy: String,
}

#[test]
fn test_load_nonexistent() {
    
    // Try to load a config that doesn't exist
    let result =  NonexistentConfig::load();

    // Verify it returns a NotFound error
    assert!(matches!(result, Err(ConfigError::NotFound(_))));
}

#[test]
fn test_load_or_default() {

    // Use load_or_default to get a config (should create a default)
        {
            let config = TestConfig::load_or_default();

    // Verify it returned the default
    assert_eq!(config, TestConfig::default());

    // Check that the file was created
    let config_path = TestConfig::default_path();
    assert!(
        config_path.exists(),
        "Config file was not created by load_or_default"
    );

    // Now test that a subsequent call to load_or_default correctly loads from the file
    // rather than returning the default again
    let loaded_config = TestConfig::load_or_default();
    assert_eq!(
        loaded_config,
        TestConfig::default(),
        "Second load_or_default should still return the default config"
    );
}
}

#[test]
fn test_custom_config_name() {

    // Create a custom named config
    let config = CustomNameConfig {
        custom_field: "Custom Value".to_string(),
    };

    // Save the config
        config.save().expect("Failed to save custom config");

        // Check the name is used in the path
        let config_path = CustomNameConfig::default_path();
        assert!(
            config_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains("custom_config")
        );
        assert!(config_path.exists(), "Custom config file was not created");
}

#[test]
fn test_load_or_default_with_modification() {

    // First ensure we have a default config saved
  
    // Now modify the config and save it
    let modified_config: TestConfigForModification = TestConfigForModification {
        name: "Test".to_string(),
        value: 42,
        enabled: true,
    };

        modified_config
            .save()
            .expect("Failed to save modified config");
    let loaded_config= TestConfigForModification::load_or_default();
    assert_eq!(loaded_config.name, "Test");
    assert_eq!(loaded_config.value, 42);
    assert!(loaded_config.enabled);


}
#[cfg(feature = "json")]
#[test]
fn test_custom_format() {

    // Create a config with custom format
    let config = JsonFormatConfig {
        json_field: "JSON Value".to_string(),
    };

    // Save the config
        config.save().expect("Failed to save JSON format config");

        // Check the format is used in the path
        let config_path = JsonFormatConfig::default_path();
        assert!(config_path.extension().unwrap().to_string_lossy() == "json");
        assert!(
            config_path.exists(),
            "JSON format config file was not created"
        );
}

#[test]
fn test_config_format_default() {
    // Verify that the default format is TOML
    assert_eq!(ConfigFormat::default(), ConfigFormat::Toml);
}

#[test]
fn test_config_format_from_string() {
    assert_eq!(ConfigFormat::from("json"), ConfigFormat::Json);
    assert_eq!(ConfigFormat::from("toml"), ConfigFormat::Toml);
    assert_eq!(ConfigFormat::from("yaml"), ConfigFormat::Yaml);
    // Unknown formats should default to TOML
    assert_eq!(ConfigFormat::from("unknown"), ConfigFormat::Toml);
}

}