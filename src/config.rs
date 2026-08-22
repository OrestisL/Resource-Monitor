use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)] // any missing field falls back to Default
pub struct Config {
    pub interval_secs: u64,
    pub icon_width: i32,
    pub icon_height: i32,
    pub font_px: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval_secs: 2,
            icon_width: 32,
            icon_height: 32,
            font_px: 32.0,
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Get the config, loading it from disk on first call.
pub fn config() -> &'static Config {
    CONFIG.get_or_init(Config::load)
}

impl Config {
    fn load() -> Config {
        let Some(path) = config_path() else {
            return Config::default();
        };
        match fs::read_to_string(&path) {
            Ok(text) => toml::from_str(&text).unwrap_or_else(|e| {
                eprintln!("config: parse error in {}: {e}; using defaults", path.display());
                Config::default()
            }),
            Err(_) => Config::default(), // no file -> defaults, silently
        }
    }
}

/// ~/.config/<crate-name>/config.toml, honoring XDG_CONFIG_HOME.
fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join(env!("CARGO_PKG_NAME")).join("config.toml"))
}