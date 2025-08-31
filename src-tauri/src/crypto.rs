use anyhow::Result;
use bip39::{Language, Mnemonic};
use hex;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::Rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

pub struct CryptoManager;

impl CryptoManager {
    /// Generate a new BIP39 mnemonic phrase
    pub fn generate_mnemonic() -> Result<String> {
        let mut rng = rand::thread_rng();
        let entropy: [u8; 32] = rng.r#gen();
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;
        Ok(mnemonic.to_string())
    }

    /// Derive private key from mnemonic and path (simplified)
    pub fn derive_private_key(mnemonic: &str, _path: &str) -> Result<SecretKey> {
        let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic)?;
        let seed = mnemonic.to_seed("");

        // Use the first 32 bytes of the seed as private key
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&seed[..32]);

        Ok(SecretKey::from_slice(&private_key)?)
    }

    /// Get public key from private key
    pub fn get_public_key(private_key: &SecretKey) -> PublicKey {
        let secp = Secp256k1::new();
        PublicKey::from_secret_key(&secp, private_key)
    }

    /// Sign data with private key
    pub fn sign_data(private_key: &SecretKey, data: &[u8]) -> Result<Vec<u8>> {
        let secp = Secp256k1::new();
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let message = secp256k1::Message::from_digest(hash.into());
        let signature = secp.sign_ecdsa(&message, private_key);
        Ok(signature.serialize_der().to_vec())
    }

    /// Generate address from public key (Mazzaroth format)
    pub fn generate_address(public_key: &PublicKey) -> String {
        let public_key_bytes = public_key.serialize_uncompressed();
        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        let result = hasher.finalize();
        hex::encode(&result[..20]) // Use first 20 bytes for address
    }

    /// Encrypt data with password
    pub fn encrypt_data(data: &[u8], password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        let mut key = [0u8; 32];
        let _ = pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, 10000, &mut key);

        // Simple XOR encryption (in production, use proper encryption)
        let mut encrypted = Vec::new();
        for (i, &byte) in data.iter().enumerate() {
            encrypted.push(byte ^ key[i % 32]);
        }
        Ok(encrypted)
    }

    /// Decrypt data with password
    pub fn decrypt_data(encrypted_data: &[u8], password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        let mut key = [0u8; 32];
        let _ = pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, 10000, &mut key);

        // Simple XOR decryption
        let mut decrypted = Vec::new();
        for (i, &byte) in encrypted_data.iter().enumerate() {
            decrypted.push(byte ^ key[i % 32]);
        }
        Ok(decrypted)
    }
}
