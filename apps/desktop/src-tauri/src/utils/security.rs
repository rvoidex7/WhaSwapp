use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use aes_gcm::{
    aead::{Aead, KeyInit, AeadCore},
    Aes256Gcm, Nonce, Key
};
// use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

const SECURITY_FILE: &str = "security.json";

#[derive(Serialize, Deserialize, Debug)]
struct SecurityConfig {
    password_hash: String,
    // We can store other non-sensitive config here
    // But we NEVER store the actual key. The key is derived from the password at runtime.
    salt: String,
}

pub struct SecurityManager {
    app_dir: PathBuf,
    master_key: Arc<Mutex<Option<Vec<u8>>>>,
}

impl SecurityManager {
    pub fn new(app_dir: PathBuf) -> Self {
        Self {
            app_dir,
            master_key: Arc::new(Mutex::new(None)),
        }
    }

    fn config_path(&self) -> PathBuf {
        self.app_dir.join(SECURITY_FILE)
    }

    pub fn is_configured(&self) -> bool {
        self.config_path().exists()
    }

    pub fn init(&self, password: &str) -> anyhow::Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Hashing failed: {}", e))?
            .to_string();

        let config = SecurityConfig {
            password_hash,
            salt: salt.as_str().to_string(),
        };

        let json = serde_json::to_string_pretty(&config)?;
        fs::write(self.config_path(), json)?;

        // Automatically unlock after init
        self.derive_and_store_key(password, &config.salt)?;

        Ok(())
    }

    pub fn unlock(&self, password: &str) -> anyhow::Result<bool> {
        let config_data = fs::read_to_string(self.config_path())?;
        let config: SecurityConfig = serde_json::from_str(&config_data)?;

        let parsed_hash = PasswordHash::new(&config.password_hash)
            .map_err(|e| anyhow::anyhow!("Invalid hash format: {}", e))?;

        let argon2 = Argon2::default();
        if argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
            self.derive_and_store_key(password, &config.salt)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn derive_and_store_key(&self, password: &str, salt: &str) -> anyhow::Result<()> {
        // We use the password and the stored salt to derive a stable 32-byte key
        // Note: In a production app, we might use a separate KDF for the key vs the auth hash
        // to prevent hash cracking leading to key compromise, but Argon2 is strong enough for both here.

        // Actually, let's use a specific KDF for the key material to be safe.
        // We'll use Argon2 with a specific output length (32 bytes) for the key.
        let mut key_material = [0u8; 32];
        let salt_bytes = salt.as_bytes();

        // Custom params for key derivation (slower is better for keys)
        let params = argon2::Params::new(
            argon2::Params::DEFAULT_M_COST,
            argon2::Params::DEFAULT_T_COST,
            argon2::Params::DEFAULT_P_COST,
            Some(32) // Output length
        ).map_err(|e| anyhow::anyhow!("Argon2 params error: {}", e))?;

        let argon2_kdf = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        argon2_kdf.hash_password_into(password.as_bytes(), salt_bytes, &mut key_material)
             .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;

        *self.master_key.lock().unwrap() = Some(key_material.to_vec());

        // Zeroize stack memory
        key_material.zeroize();

        Ok(())
    }

    pub fn get_master_key(&self) -> Option<Vec<u8>> {
        self.master_key.lock().unwrap().clone()
    }

    // --- Encryption Helpers ---

    #[allow(dead_code)]
    pub fn encrypt_file(&self, input_path: &PathBuf, output_path: &PathBuf) -> anyhow::Result<()> {
        let key_vec = self.get_master_key().ok_or(anyhow::anyhow!("Vault locked"))?;
        let key = Key::<Aes256Gcm>::from_slice(&key_vec);
        let cipher = Aes256Gcm::new(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let plaintext = fs::read(input_path)?;

        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Format: [Nonce (12 bytes)][Ciphertext]
        let mut final_data = Vec::new();
        final_data.extend_from_slice(nonce.as_slice());
        final_data.extend_from_slice(&ciphertext);

        fs::write(output_path, final_data)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn decrypt_file(&self, input_path: &PathBuf, output_path: &PathBuf) -> anyhow::Result<()> {
        let key_vec = self.get_master_key().ok_or(anyhow::anyhow!("Vault locked"))?;
        let key = Key::<Aes256Gcm>::from_slice(&key_vec);
        let cipher = Aes256Gcm::new(key);

        let data = fs::read(input_path)?;
        if data.len() < 12 {
            return Err(anyhow::anyhow!("File too short"));
        }

        let nonce = Nonce::from_slice(&data[0..12]);
        let ciphertext = &data[12..];

        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        fs::write(output_path, plaintext)?;
        Ok(())
    }
}
