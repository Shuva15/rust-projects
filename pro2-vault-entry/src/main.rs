mod vault;
mod encrypt;
use rpassword;
use serde_json;
use std::fs;
use std::io::{self, Write};
use vault::{EncryptedVault, Vault};

fn main() {
    let master_password = rpassword::prompt_password("Enter your master password: ").unwrap();

    let ciphervault: EncryptedVault;
    let mut vault_entries = Vault::new();
    
    let json_ciphervault = fs::read_to_string("vault.json");
    match json_ciphervault {
        Ok(ciphervalue) => match serde_json::from_str(&ciphervalue) {
            Ok(parsed) => {
                ciphervault = parsed;
                match ciphervault.decrypt_vault(&master_password) {
                    Ok(vault) => {
                        vault_entries = vault
                    }
                    Err(error) => {
                        println!("{}", error);
                        return;
                    }
                }
            },
            Err(_) => println!("⚠️ Could not parse vault.json. Starting fresh."),
        },
        Err(_) => println!("File clean"),
    }
    println!("🔐 Welcome to your password vault!");

    loop {
        println!("Commands: add | get | list | remove | cmp | help | exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim().to_lowercase();

        match cmd.as_str() {
            "add" => {
                let site = prompt("Site");
                let username = prompt("Username");
                let password = rpassword::prompt_password("Password: ").unwrap();

                vault_entries.add_entry(site, username, password);
                vault_entries.save_to_file(&master_password);
                println!("✅ Added.",);
            }
            "get" => {
                let site = prompt("Site");
                let vault = vault_entries.entries.get(&site);
                match vault {
                    Some(entry) => {
                        println!(
                            "👤 Username: {}\n🔑 Password: {}",
                            entry.username, entry.password
                        );
                    }
                    None => println!("❌ No entry found for that site."),
                }
            }
            "list" => {
                println!("📋 Stored sites:");
                vault_entries.list_sites();
            }
            "remove" => {
                let site = prompt("Site name you want to remove");
                match vault_entries.entries.get(&site) {
                    Some(_) => {
                        vault_entries.remove_entry(site);
                        vault_entries.save_to_file(&master_password);
                        println!("🗑️ Entry removed successfully.");
                    }
                    None => println!("❌ No entry found for that site."),
                }
            }
            "cmp" => {
                change_master_password(&master_password, &vault_entries);
                break;
            }
            "help" => {
                println!("Commands you can use:");
                println!("- add    → Add a new site with username & password");
                println!("- get    → Retrieve username & password for a site");
                println!("- list   → List all saved site names");
                println!("- remove → Remove a site’s entry");
                println!("- cmp    → Change Master Password");
                println!("- help   → Show this help message");
                println!("- exit   → Exit the password vault");
            }
            "exit" => break,
            _ => println!("❓ Unknown command."),
        }
    }
}

fn prompt(label: &str) -> String {
    loop {
        print!("{}: ", label);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("msg");
        if !input.trim().is_empty() {
            return input.trim().to_lowercase();
        };
        println!("Input can't be empty! Enter a valid {}:", label);
    }
}

fn change_master_password(master_password: &String, vault_entries: &Vault) {
    let old_password = rpassword::prompt_password("Enter your master password: ").unwrap();
    if master_password != &old_password {
        println!("You have given wrong password! To change master password try again later with correct password.");
        return;
    }
    let new_password = prompt("Set new master password");
    let confirm_password = rpassword::prompt_password("Type the new password again: ").unwrap();
    if new_password != confirm_password {
        println!("Your new password doesn't match! Exiting Vault try again later.");
        return;
    }
    vault_entries.save_to_file(&new_password);
    println!("Your master password is changed successfully! Enter with your new password to access Vault.");
    return;
}