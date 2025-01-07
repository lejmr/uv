use crate::Credentials;
use keyring::Entry;
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

pub fn get_auth_config() -> AuthConfig {
    // Determine path to configuration file
    let config_file = user_state_dir()
        .ok_or("Could not determine user state directory")
        .unwrap()
        .join("auth.toml");

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

pub fn update_auth_config(index: &str, username: &str, password: &str) {
    let auth_config = get_auth_config();

    // Write configuration to disk
    // std::fs::write(config_file, config).expect("Could not write configuration to disk");
}
