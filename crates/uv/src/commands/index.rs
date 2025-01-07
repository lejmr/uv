use anyhow::Context;
use console::Term;
use keyring::Entry;
use uv_auth::auth_config::{get_auth_config, Index};
use uv_auth::Credentials;
use uv_dirs::user_state_dir;

pub fn credentials_add(index: &str, username: &str, password: Option<&str>) {
    let mut config = get_auth_config();
    config.index.insert(
        index.to_string(),
        Index {
            username: username.to_string(),
        },
    );

    // Determine path to configuration file
    let config_file = user_state_dir()
        .ok_or("Could not determine user state directory")
        .unwrap()
        .join("auth.toml");

    // Serialize configuration to TOML
    let config = toml::ser::to_string(&config).expect("Could not serialize configuration");

    // Write configuration to disk
    std::fs::write(config_file, config).expect("Could not write configuration to disk");
    // Add secret to keyring
    let password = match password {
        Some(p) => p,
        None => {
            let term = Term::stdout();
            let password = uv_console::password("Enter password: ", &term).expect("Could not read password");
            &password.clone()
        },
    };
    match Entry::new(&Credentials::keyring_secret_name(index), &username) {
        Ok(entry) => {
            match entry.get_password() {
                Ok(_) => {
                    entry.set_password(password).expect("Could not set password");
                }
                Err(_) => {
                    entry.set_password(password).expect("Could not set password");
                }
            }
        }
        Err(e) => {
            panic!("Could not create keyring entry: {}", e);
        }
    }
}
