use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

use crate::error::{AppError, Result};

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

pub struct BackupEncryptor {
    key: [u8; KEY_SIZE],
}

impl BackupEncryptor {
    pub fn new(password: &str) -> Self {
        let key = derive_key(password);
        Self { key }
    }

    pub fn with_key(key: [u8; KEY_SIZE]) -> Self {
        Self { key }
    }

    pub fn encrypt_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| AppError::Encryption(e.to_string()))?;

        let mut file = File::open(input_path)?;
        let mut plaintext = Vec::new();
        file.read_to_end(&mut plaintext)?;

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| AppError::Encryption(e.to_string()))?;

        let mut output = File::create(output_path)?;
        output.write_all(&nonce_bytes)?;
        output.write_all(&ciphertext)?;

        Ok(())
    }

    pub fn decrypt_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| AppError::Encryption(e.to_string()))?;

        let mut file = File::open(input_path)?;
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        file.read_exact(&mut nonce_bytes)?;

        let mut ciphertext = Vec::new();
        file.read_to_end(&mut ciphertext)?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| AppError::Encryption(format!("Decryption failed: {}", e)))?;

        let mut output = File::create(output_path)?;
        output.write_all(&plaintext)?;

        Ok(())
    }

    pub fn get_key_hex(&self) -> String {
        hex::encode(self.key)
    }
}

fn derive_key(password: &str) -> [u8; KEY_SIZE] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut key = [0u8; KEY_SIZE];
    let salt = b"android-backup-v1";

    for (i, (p, s)) in password
        .bytes()
        .zip(salt.iter().cycle())
        .enumerate()
        .take(KEY_SIZE)
    {
        let mut hasher = DefaultHasher::new();
        p.hash(&mut hasher);
        s.hash(&mut hasher);
        let hash = hasher.finish();
        key[i] = (hash as u8).wrapping_add(i as u8);
    }

    let mut final_key = [0u8; KEY_SIZE];
    for (i, chunk) in password.as_bytes().chunks(8).enumerate() {
        let mut hasher = DefaultHasher::new();
        chunk.hash(&mut hasher);
        salt.hash(&mut hasher);
        let hash = hasher.finish();
        if i < KEY_SIZE / 8 {
            final_key[i * 8..(i + 1) * 8].copy_from_slice(&hash.to_le_bytes());
        }
    }

    final_key
}

pub fn generate_random_key() -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    rand::thread_rng().fill(&mut key);
    key
}

pub fn key_from_hex(hex_str: &str) -> Result<[u8; KEY_SIZE]> {
    let bytes = hex::decode(hex_str).map_err(|e| AppError::Encryption(e.to_string()))?;

    if bytes.len() != KEY_SIZE {
        return Err(AppError::Encryption(format!(
            "Invalid key length: expected {} got {}",
            KEY_SIZE,
            bytes.len()
        )));
    }

    let mut key = [0u8; KEY_SIZE];
    key.copy_from_slice(&bytes);
    Ok(key)
}
