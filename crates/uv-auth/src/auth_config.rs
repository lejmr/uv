use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml;
use uv_dirs::user_state_dir;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct AuthConfig {
    pub index: HashMap<String, Index>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Index {
    pub username: String,
}

fn get_auth_config_path() -> std::path::PathBuf {
    user_state_dir()
        .ok_or("Could not determine user state directory")
        .unwrap()
        .join("auth.toml")
}

pub fn get_auth_config() -> AuthConfig {
    // Determine path to configuration file
    let config_file = get_auth_config_path();

    // Load file from disk
    match std::fs::read_to_string(config_file) {
        Ok(config) => {
            // Parse configuration file
            toml::de::from_str(&config).expect("Could not parse configuration file")
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return AuthConfig {
                    index: HashMap::new(),
                };
            }
            panic!("Could not read configuration file: {}", e);
        }
    }
}

pub(crate) fn load_username_for_index(index: &str) -> Option<String> {
    let config = get_auth_config();
    match config.index.get(index) {
        Some(index) => Some(index.username.clone()),
        None => None,
    }
}

pub fn update_auth_config(index_name: &str, index_url: &str, username: &str, password: &str) {
    // Update save configuration in memory
    let mut config = get_auth_config();
    config.index.insert(
        index_name.to_string(),
        Index {
            username: username.to_string(),
        },
    );
    // Serialize configuration to TOML
    let config_string = toml::ser::to_string(&config).expect("Could not serialize configuration");
    // Write configuration to disk
    std::fs::write(get_auth_config_path(), config_string)
        .expect("Could not write configuration to disk");
}
