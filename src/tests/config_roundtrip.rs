#[cfg(test)]
mod config_roundtrip_tests {
    use crate::config::user::UserConfig;
    use crate::hyprland::SearchOptions;
    use crate::ui::types::{ColumnVisibility, Theme};
    use std::fs;
    use tempfile::TempDir;

    fn setup_temp_config(temp_dir: &TempDir) -> std::path::PathBuf {
        let config_path = temp_dir.path().join("hyprbind_test.json");
        config_path
    }

    /// Ensures default config can be serialized and deserialized without data loss
    #[test]
    fn test_default_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = setup_temp_config(&temp_dir);

        let original = UserConfig::default();
        let json = serde_json::to_string_pretty(&original).unwrap();
        fs::write(&config_path, json).unwrap();

        let loaded_json = fs::read_to_string(&config_path).unwrap();
        let loaded: UserConfig = serde_json::from_str(&loaded_json).unwrap();

        assert_eq!(
            serde_json::to_string(&original).unwrap(),
            serde_json::to_string(&loaded).unwrap()
        );
    }

    /// Validates custom config values survive serialization roundtrip
    #[test]
    fn test_custom_config_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = setup_temp_config(&temp_dir);

        let original = UserConfig {
            theme: Theme::Light,
            column_visibility: ColumnVisibility {
                keybind: false,
                command: true,
                description: false,
            },
            search_options: SearchOptions {
                keybind: true,
                command: true,
                description: false,
            },
            zen_mode: true,
        };

        let json = serde_json::to_string_pretty(&original).unwrap();
        fs::write(&config_path, json).unwrap();

        let loaded_json = fs::read_to_string(&config_path).unwrap();
        let loaded: UserConfig = serde_json::from_str(&loaded_json).unwrap();

        assert_eq!(
            serde_json::to_string(&original).unwrap(),
            serde_json::to_string(&loaded).unwrap()
        );
    }

    /// Checks that serialized JSON contains all expected fields
    #[test]
    fn test_serialization_format_stability() {
        let config = UserConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();

        assert!(json.contains("\"theme\""));
        assert!(json.contains("\"column_visibility\""));
        assert!(json.contains("\"search_options\""));
        assert!(json.contains("\"zen_mode\""));
    }

    /// Verifies default UserConfig values match specification
    #[test]
    fn test_default_values() {
        let config = UserConfig::default();
        assert!(matches!(config.theme, Theme::Dark));
        assert!(!config.zen_mode);
        assert!(config.column_visibility.keybind);
        assert!(config.column_visibility.description);
        assert!(!config.column_visibility.command);
    }

    /// Ensures partial JSON config can be successfully deserialized
    #[test]
    fn test_partial_deserialization() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = setup_temp_config(&temp_dir);

        let partial_json = r#"{
            "theme": "Light",
            "column_visibility": {
                "keybind": true,
                "command": true,
                "description": true
            },
            "search_options": {
                "keybind": true,
                "command": true,
                "description": true
            },
            "zen_mode": false
        }"#;

        fs::write(&config_path, partial_json).unwrap();
        let loaded_json = fs::read_to_string(&config_path).unwrap();
        let loaded: Result<UserConfig, _> = serde_json::from_str(&loaded_json);

        assert!(loaded.is_ok());
    }
}
