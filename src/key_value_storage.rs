use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Result};
use serde_json::Value;

pub struct FileKeyValueStorage {
    config_file_location: String,
    data: HashMap<String, String>,
}

impl FileKeyValueStorage {
    pub fn new(config_file_location: Option<&str>) -> Self {
        let default_location = "client-config.json";
        let location = config_file_location.unwrap_or(default_location).to_string();
        let mut storage = FileKeyValueStorage {
            config_file_location: location,
            data: HashMap::new(),
        };
        storage.load();
        storage
    }

    fn load(&mut self) {
        let content = self.read_file_content().unwrap_or_default();
        if !content.is_empty() {
            self.data = serde_json::from_str(&content).unwrap_or_default();
        }
    }

    pub fn read_file_content(&self) -> Result<String> {
        let mut file = File::open(&self.config_file_location)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
        self.save();
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }

    fn save(&self) {
        let json = serde_json::to_string(&self.data).expect("Failed to serialize data");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.config_file_location)
            .expect("Failed to open config file");
        file.write_all(json.as_bytes()).expect("Failed to write to config file");
    }
}
