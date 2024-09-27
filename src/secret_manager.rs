use crate::key_value_storage::FileKeyValueStorage;
use crate::config::{ConfigKeys};
use serde_json::Value;
use std::collections::HashMap;
use crate::utils::{url_safe_str_to_bytes, bytes_to_base64, base64_to_bytes, json_to_dict};
use crate::crypto::CryptoUtils;

pub struct QueryOptions {
    pub records_filter: Option<Vec<String>>,
    pub folders_filter: Option<Vec<String>>,
}

pub struct RecordsResponse {
    pub records: Vec<Value>,
    pub just_bound: bool,
    pub warnings: Vec<String>,
    pub bad_records: Vec<Value>,
    pub bad_folders: Vec<Value>,
}

pub struct SecretsManager {
    hostname: String,
    token: String,
    config: FileKeyValueStorage,
}

impl SecretsManager {
    pub fn new(hostname: String, token: String, config: FileKeyValueStorage) -> Self {
        SecretsManager {
            hostname,
            token,
            config,
        }
    }

    pub fn get_secrets(&mut self, uids: Option<Vec<String>>, full_response: bool) -> Vec<Value> {
        let query_options = QueryOptions {
            records_filter: uids,
            folders_filter: None,
        };

        self.get_secrets_with_options(query_options, full_response)
    }

    pub fn get_secrets_with_options(&mut self, query_options: QueryOptions, full_response: bool) -> Vec<Value> {
        let records_resp = self.fetch_and_decrypt_secrets(&query_options);
    
        if records_resp.just_bound {
            let _ = self.fetch_and_decrypt_secrets(&query_options);
        }
    
        if !records_resp.warnings.is_empty() {
            for warning in &records_resp.warnings {
                println!("Warning: {}", warning);
            }
        }
    
        if !records_resp.bad_records.is_empty() {
            for error in &records_resp.bad_records {
                println!("Bad Record: {:?}", error);
            }
        }
    
        if !records_resp.bad_folders.is_empty() {
            for error in &records_resp.bad_folders {
                println!("Bad Folder: {:?}", error);
            }
        }
    
        if full_response {
            let response = serde_json::json!({
                "records": records_resp.records,
                "just_bound": records_resp.just_bound,
                "warnings": records_resp.warnings,
                "bad_records": records_resp.bad_records,
                "bad_folders": records_resp.bad_folders,
            });
    
            println!("{:?}", response);
            
            vec![response]
        } else {
            println!("{:?}", records_resp.records);
    
            records_resp.records
        }
    }
    

    pub fn fetch_and_decrypt_secrets(&mut self, query_options: &QueryOptions) -> RecordsResponse {
        let payload = self.prepare_get_payload(query_options);
        
        let decrypted_response_bytes = self.post_query("get_secret", payload);
        
        let decrypted_response_str = String::from_utf8_lossy(&decrypted_response_bytes).to_string();

        self.process_decrypted_response(&decrypted_response_str);
        
        let decrypted_response_dict: HashMap<String, Value> = serde_json::from_str(&decrypted_response_str).unwrap_or_default();

        let mut records: Vec<Value> = vec![];
        let mut shared_folders: Vec<Value> = vec![];

        let mut just_bound = false;

        if let Some(encrypted_app_key) = decrypted_response_dict.get("encryptedAppKey") {
            just_bound = true;

            let encrypted_master_key = url_safe_str_to_bytes(encrypted_app_key.as_str().unwrap_or(""));

            let client_key = url_safe_str_to_bytes(self.config.get(ConfigKeys::KEY_CLIENT_KEY.as_str()).unwrap_or(&String::new()));
    
            let app_owner_public_key: &str = "your_app_owner_public_key";

            let app_owner_public_key_bytes = url_safe_str_to_bytes(app_owner_public_key);

            let secret_key = CryptoUtils::decrypt_aes(&encrypted_master_key, &client_key).expect("Decryption failed");
            
            self.config.set(ConfigKeys::KEY_APP_KEY.as_str(), bytes_to_base64(&secret_key));
            
            self.config.delete(ConfigKeys::KEY_CLIENT_KEY.as_str());
            
            self.config.set(ConfigKeys::KEY_OWNER_PUBLIC_KEY.as_str(), bytes_to_base64(&app_owner_public_key_bytes));
        
        } else {
            let secret_key = base64_to_bytes(self.config.get(ConfigKeys::KEY_APP_KEY.as_str()).unwrap_or(&String::new()));
        }

        if let Some(records_resp) = decrypted_response_dict.get("records") {
            records = records_resp.as_array().unwrap_or(&vec![]).to_vec();
        }

        if let Some(folders_resp) = decrypted_response_dict.get("folders") {
            shared_folders = folders_resp.as_array().unwrap_or(&vec![]).to_vec();
        }

        println!("Individual record count: {}", records.len());
        println!("Folder count: {}", shared_folders.len());

        RecordsResponse {
            records,
            just_bound,
            warnings: vec![],
            bad_records: vec![],
            bad_folders: vec![],
        }
    }

    fn process_decrypted_response(&mut self, decrypted_response_str: &str) {
        let decrypted_response_dict: HashMap<String, String> = json_to_dict(decrypted_response_str);
    
        if let Some(app_owner_public_key) = decrypted_response_dict.get("appOwnerPublicKey") {
            let app_owner_public_key_bytes = url_safe_str_to_bytes(app_owner_public_key);
            self.config.set(ConfigKeys::KEY_OWNER_PUBLIC_KEY.as_str(), bytes_to_base64(&app_owner_public_key_bytes));
        }
    }

    fn prepare_get_payload(&self, query_options: &QueryOptions) -> String {
        String::new()
        // need to implement this
    }

    fn post_query(&self, endpoint: &str, payload: String) -> Vec<u8> {
        vec![]
        // need to implement this
    }

}
