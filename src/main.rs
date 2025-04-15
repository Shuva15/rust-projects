mod vault;
use vault::Vault;
use std::io::{self, Write};
use std::fs;
use serde_json;

fn main() {
    let mut vault_entries = Vault::new();
    println!("🔐 Welcome to your password vault!");
    let json_vault = fs::read_to_string("vault.json");
    match json_vault {
        Ok(vault) => {
            match serde_json::from_str(&vault) {
                Ok(parsed) => vault_entries = parsed,
                Err(_) => println!("⚠️ Could not parse vault.json. Starting fresh."),
            }
        }
        Err(_) => println!("File clean"),
    }
    
    loop {
        println!("Commands: add | get | list | exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim().to_lowercase();

        match cmd.as_str() {
            "add" => {
                let site = prompt("Site");
                let username = prompt("Username");
                let password = prompt("Password");

                vault_entries.add_entry(site, username, password);
                vault_entries.save_to_file();
                println!("✅ Added.", );
            }
            "get" => {
                let site = prompt("Site");
                let vault = vault_entries.entries.get(&site);
                match vault {
                    Some(entry) => {
                        println!("👤 Username: {}\n🔑 Password: {}", entry.username, entry.password);
                    }
                    None => println!("❌ No entry found for that site."),
                }
            }
            "list" => {
                println!("📋 Stored sites:");
                vault_entries.list_sites();
            }
            "exit" => break,
            _ => println!("❓ Unknown command."),
        }
    }
}

fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("msg");
    input.trim().to_lowercase()
}
