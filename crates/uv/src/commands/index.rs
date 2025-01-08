use console::Term;
use keyring::Entry;
use uv_auth::auth_config::{get_auth_config, update_auth_config, AuthConfig};
use uv_distribution_types::Index as IndexIndex;

pub fn credentials_set(index: &IndexIndex, username: &str, password: Option<&str>) {
    // Collect all necessary information
    let index_name = index.name.as_ref().unwrap().to_string();
    let url = format!("{}", index.url);

    // TODO: this is not nice
    let final_password: String;
    if password.is_none() {
        let term = Term::stdout();
        final_password =
            uv_console::password("Enter password: ", &term).expect("Could not read password");
    } else {
        final_password = password.unwrap().to_string();
    }

    // Propagate to auth.toml
    update_auth_config(&index_name, &url, username, &final_password);
    // Propagate to keyring
    match Entry::new(&url, &username) {
        Ok(entry) => {
            if let Err(_) = entry.get_password() {
                panic!("Could not access keyring entry");
            }
            entry
                .set_password(&final_password)
                .expect("Could not set password");
        }
        Err(e) => panic!("Could not create keyring entry: {}", e),
    }
}

pub fn credentials_list(auth_config: AuthConfig, configured_index: Vec<IndexIndex>) {
    // List all credentials
    for ind in configured_index {
        if let Some(name) = ind.name {
            let no_credentials_msg = format!("Index: '{}' no credentials.", name);
            if let Some(auth_index) = auth_config.index.get(&name.to_string()) {
                let username = auth_index.clone().username.clone();
                let url = format!("{}", ind.url);
                // Console output for each named index
                match Entry::new(&url, &username)
                    .expect("Unable to access keyring.")
                    .get_password()
                {
                    Ok(_) => {
                        println!(
                            "Index: '{}' authenticates with username '{}'.",
                            name, username
                        );
                    }
                    Err(_) => {
                        println!("{}", no_credentials_msg);
                    }
                }
            } else {
                println!("{}", no_credentials_msg);
            }
        }
    }
}

pub fn credentials_unset(index: &IndexIndex) {
    let index_name = index.name.as_ref().unwrap().to_string();
    let mut config = get_auth_config();
    // config.index.remove(&index_name);
    // let config_file = user_state_dir()
    //     .ok_or("Could not determine user state directory")
    //     .unwrap()
    //     .join("auth.toml");
    // let config = toml::ser::to_string(&config).expect("Could not serialize configuration");
    // std::fs::write(config_file, config).expect("Could not write configuration to disk");
    // let url = format!("{}", index.url);
    // match Entry::new(&url, &index_name) {
    //     Ok(entry) => {
    //         entry.delete().expect("Could not delete keyring entry");
    //     }
    //     Err(e) => {
    //         panic!("Could not create keyring entry: {}", e);
    //     }
    // }
}
