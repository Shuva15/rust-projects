use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_key_from_password(password: &str, salt: [u8; 16]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 100_000, &mut key);
    key
}

pub fn encrypt(key: &[u8; 32], text: &str) -> (Vec<u8>, [u8; 12]) {
    let cipher_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(cipher_key);

    // create nonce and change it to Nonce type
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let data = text.as_bytes();
    let ciphertext = cipher.encrypt(nonce, data)
        .expect("Encryption failed");

    (ciphertext, nonce_bytes)
}
pub fn decrypt(key: &[u8; 32], nonce_bytes: &[u8; 12], ciphertext: &[u8]) -> Result<String, String> {
    let cipher_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(cipher_key);

    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = cipher.decrypt(nonce, ciphertext);
    match decrypted_bytes {
        Ok(value) => {
            Ok(String::from_utf8(value).expect("❌ UTF-8 decoding failed!"))
        }
        Err(_) => Err(String::from("You have given wrong password!"))
    }

}