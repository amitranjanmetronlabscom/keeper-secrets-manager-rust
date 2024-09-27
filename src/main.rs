mod utils;
mod crypto;
mod config;
mod key_value_storage;
mod secret_manager;

use crate::key_value_storage::FileKeyValueStorage;
use crate::secret_manager::SecretsManager;
use std::io;

fn main() {
    let mut hostname = String::new();
    let mut token = String::new();

    println!("Enter KSM Server\nPress <Enter> to use keepersecurity.com server: ");
    io::stdin().read_line(&mut hostname).expect("Failed to read hostname");
    let hostname = hostname.trim();
    let hostname = if hostname.is_empty() {
        "keepersecurity.com".to_string()
    } else {
        hostname.to_string()
    };

    println!("Enter one time token: ");
    io::stdin().read_line(&mut token).expect("Failed to read token");
    let token = token.trim().to_string();

    println!("Hostname: [{}]", hostname);
    println!("Token: [{}]", token);

    let key_value_storage = FileKeyValueStorage::new(None);

    match key_value_storage.read_file_content() {
        Ok(content) => {
            println!("Config File Content: {}", content);
        }
        Err(e) => {
            println!("Error reading config file: {}", e);
        }
    }

    let mut secrets_manager = SecretsManager::new(hostname.clone(), token.clone(), key_value_storage);
    
    let uids: Option<Vec<String>> = Some(vec!["uid1".to_string()]);
    let full_response: bool = false;

    let all_records = secrets_manager.get_secrets(uids, full_response);

    for record in all_records {
        println!("Record: {:?}", record);
    }
}
