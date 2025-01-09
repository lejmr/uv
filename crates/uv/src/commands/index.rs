use console::Term;
use keyring::Entry;
use uv_auth::auth_config::{drop_index_entry, get_auth_config, update_auth_config, AuthConfig};
use uv_distribution_types::Index as IndexIndex;

pub(crate) fn credentials_set(index: &IndexIndex, username: &str, password: Option<&str>) {
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

pub(crate) fn credentials_list(auth_config: AuthConfig, configured_index: Vec<IndexIndex>) {
    // List all credentials
    for ind in configured_index {
        if let Some(name) = ind.name {
            let no_credentials_msg = format!("Index: '{}' no credentials.", name);
            if let Some(auth_index) = auth_config.index.get(&name.to_string()) {
                let username = auth_index.username.clone();
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

pub(crate) fn credentials_unset(name: &str, indexes: Vec<IndexIndex>) {
    let auth_config = get_auth_config();
    for ind in indexes {
        if let Some(ref index_name) = ind.name {
            if index_name.to_string() == name {
                if let Some(_) = auth_config.index.get(&name.to_string()) {
                    // At this point we are certain that such index has defined credentials in auth.toml
                    drop_index_entry(name, &format!("{}", ind.url));
                    break;
                }
            }
        }
    }
}
