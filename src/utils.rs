use base64::{self, Engine};
use base64::engine::general_purpose;
use std::collections::HashMap;

pub fn url_safe_str_to_bytes(s: &str) -> Vec<u8> {
    let padded_string = format!("{}{}", s, "==".chars().take(4 - s.len() % 4).collect::<String>());
    general_purpose::URL_SAFE.decode(&padded_string).expect("Failed to decode URL-safe base64 string")
}

pub fn bytes_to_base64(b: &[u8]) -> String {
    general_purpose::STANDARD.encode(b)
}

pub fn base64_to_bytes(s: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(s).expect("Failed to decode base64 string")
}

pub fn json_to_dict(json_str: &str) -> HashMap<String, String> {
    serde_json::from_str(json_str).unwrap_or_else(|_| HashMap::new()) 
}
