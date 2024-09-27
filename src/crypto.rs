use aes_gcm::{Aes256Gcm, Key, Nonce}; // Import necessary types
use aes_gcm::aead::{Aead, KeyInit}; // For Aead trait
use aes::Aes256; // Import Aes256 for key size specification

pub struct CryptoUtils;

impl CryptoUtils {
    pub fn decrypt_aes(data: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
        // Check that the key length is valid (32 bytes for AES256)
        if key.len() != 32 {
            return Err("Invalid key length; must be 32 bytes.");
        }

        // Ensure the data is long enough (12 bytes for nonce + ciphertext)
        if data.len() < 12 {
            return Err("Data too short; must include nonce.");
        }

        // Split data into nonce and ciphertext
        let nonce = Nonce::from_slice(&data[..12]); // The first 12 bytes
        let ciphertext = &data[12..]; // The rest is the ciphertext

        // Create the AES-GCM cipher instance with an explicit key type
        let key = Key::<Aes256>::from_slice(key); // Create a key from the provided slice
        let cipher = Aes256Gcm::new(key);

        // Decrypt the data
        cipher.decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed.")
    }
}
