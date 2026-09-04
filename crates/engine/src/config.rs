//! Server configuration (PRD §25).

use std::path::Path;

use serde::Deserialize;

/// Configuration of a dedicated server, following the `server.toml` layout
/// from PRD §25. Fields omitted from the file fall back to the serde default
/// (`Default`), so a missing file yields a fully valid default config.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Human-readable server name shown in the server browser.
    #[serde(default = "default_name")]
    pub name: String,
    /// Maximum number of concurrently connected players.
    #[serde(default = "default_max_players")]
    pub max_players: u32,
    /// TCP/UDP port the server listens on.
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: default_name(),
            max_players: default_max_players(),
            port: default_port(),
        }
    }
}

fn default_name() -> String {
    "My RustyPixel Server".to_string()
}

fn default_max_players() -> u32 {
    50
}

fn default_port() -> u16 {
    17091
}

impl ServerConfig {
    /// Load a config from `path`.
    ///
    /// Returns `Ok(None)` when the file does not exist (the caller then uses
    /// defaults), and `Err` when the file exists but cannot be parsed.
    pub fn load(path: impl AsRef<Path>) -> Result<Option<Self>, toml::de::Error> {
        let path = path.as_ref();
        let raw = match std::fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Ok(None), // Treat other IO errors as "use defaults".
        };
        toml::from_str(&raw).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let config = ServerConfig::default();
        assert_eq!(config.name, "My RustyPixel Server");
        assert_eq!(config.max_players, 50);
        assert_eq!(config.port, 17091);
    }

    #[test]
    fn parses_example_toml() {
        let raw = r#"
name = "My RustyPixel Server"
max_players = 50
port = 17091
"#;
        let config: ServerConfig = toml::from_str(raw).expect("valid config");
        assert_eq!(config.name, "My RustyPixel Server");
        assert_eq!(config.max_players, 50);
        assert_eq!(config.port, 17091);
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let raw = "port = 1234\n";
        let config: ServerConfig = toml::from_str(raw).expect("valid partial config");
        assert_eq!(config.port, 1234);
        assert_eq!(config.name, "My RustyPixel Server");
        assert_eq!(config.max_players, 50);
    }
}