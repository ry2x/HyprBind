use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("hyprbind");
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config").join("hyprbind")
}

pub fn export_dir() -> PathBuf {
    config_dir().join("exports")
}

pub(super) fn config_path() -> PathBuf {
    config_dir().join("config.json")
}
