use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

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

    pub fn save_to_file(&self) {
        let json_entries = serde_json::to_string(self).expect("msg");
        let _ = fs::write("vault.json", json_entries);
    }

    pub fn list_sites(&self) {
        for site in self.entries.keys() {
            println!("- {}", site);
        }
    }
}