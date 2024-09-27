#[derive(Debug)]
pub enum ConfigKeys {
    KEY_CLIENT_KEY,
    KEY_APP_KEY,
    KEY_OWNER_PUBLIC_KEY,
}

impl ConfigKeys {
    pub fn as_str(&self) -> &str {
        match self {
            ConfigKeys::KEY_CLIENT_KEY => "client_key",
            ConfigKeys::KEY_APP_KEY => "app_key",
            ConfigKeys::KEY_OWNER_PUBLIC_KEY => "owner_public_key",
        }
    }
}
