use crate::encrypt::{decrypt, encrypt, derive_key_from_password, generate_salt};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct EncryptedVault {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub salt: [u8; 16]
}

#[derive(Deserialize, Serialize)]
pub struct VaultEntry {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct Vault {
    pub entries: HashMap<String, VaultEntry>
}

impl Vault {
    pub fn new() -> Self {
        Vault { entries: HashMap::new() }
    }

    pub fn add_entry(&mut self, site: String, username: String, password: String) {
        self.entries.insert(site, VaultEntry { username, password });
    }

    pub fn save_to_file(&self, master_password: &str) {
        let json_entries = serde_json::to_string(self).expect("msg");

        let salt = generate_salt();
        let key = derive_key_from_password(master_password, salt);
        let (ciphertext, nonce) = encrypt(&key, &json_entries);

        let data_to_write: EncryptedVault = EncryptedVault { ciphertext, nonce, salt };
        let json_data = serde_json::to_string(&data_to_write).expect("Failed to serialize data");

        let _ = fs::write("vault.json", json_data);
    }

    pub fn list_sites(&self) {
        for site in self.entries.keys() {
            println!("- {}", site);
        }
    }

    pub fn remove_entry(&mut self, site: String) {
        self.entries.remove(&site);
    }
}

impl EncryptedVault {
    pub fn decrypt_vault(&self, master_password: &str) -> Result<Vault, String> {
        let key = derive_key_from_password(master_password, self.salt);

        let decrypt_text = decrypt(&key, &self.nonce, &self.ciphertext);
        match decrypt_text {
            Ok(text) => {
                let vault: Vault = serde_json::from_str(&text).unwrap();
                Ok(vault)
            }
            Err(error) => Err(error)
        }
    }
}